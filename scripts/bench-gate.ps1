[CmdletBinding()]
param(
    [string]$BaselinePath,

    [ValidateRange(10, 1000000)]
    [int]$SampleSize = 10,

    [ValidateRange(0.1, 3600)]
    [double]$WarmUpSeconds = 1,

    [ValidateRange(0.1, 3600)]
    [double]$MeasurementSeconds = 2,

    [double]$ThresholdPercent = -1,

    [switch]$UpdateBaseline
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
if (-not $BaselinePath) {
    $BaselinePath = Join-Path $repoRoot "docs/ops/perf-baseline.json"
}
$BaselinePath = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($BaselinePath)

if ($env:CARGO_TARGET_DIR) {
    if ([System.IO.Path]::IsPathRooted($env:CARGO_TARGET_DIR)) {
        $targetDir = $env:CARGO_TARGET_DIR
    }
    else {
        $targetDir = Join-Path $repoRoot $env:CARGO_TARGET_DIR
    }
}
else {
    $targetDir = Join-Path $repoRoot "target"
}
$criterionDir = Join-Path $targetDir "criterion"

$benchmarkNames = @(
    "distill_compile_200_messages",
    "okf_export_200_messages",
    "inject_render_200_messages"
)

function Get-CriterionEstimate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$BenchmarkName
    )

    $estimatePath = Join-Path $criterionDir "pipeline/$BenchmarkName/current/estimates.json"
    if (-not (Test-Path -LiteralPath $estimatePath -PathType Leaf)) {
        throw "Criterion estimate not found at '$estimatePath'."
    }

    $estimate = Get-Content -LiteralPath $estimatePath -Raw | ConvertFrom-Json
    [double]$estimate.mean.point_estimate
}

function Get-CurrentMeasurements {
    $measurements = [ordered]@{}
    foreach ($name in $benchmarkNames) {
        $key = "pipeline/$name"
        $measurements[$key] = Get-CriterionEstimate -BenchmarkName $name
    }
    $measurements
}

$oldSampleSize = $env:SESSION_LEDGER_BENCH_SAMPLE_SIZE
$oldWarmUp = $env:SESSION_LEDGER_BENCH_WARM_UP_SECONDS
$oldMeasurement = $env:SESSION_LEDGER_BENCH_MEASUREMENT_SECONDS

try {
    $env:SESSION_LEDGER_BENCH_SAMPLE_SIZE = [string]$SampleSize
    $env:SESSION_LEDGER_BENCH_WARM_UP_SECONDS = [string]$WarmUpSeconds
    $env:SESSION_LEDGER_BENCH_MEASUREMENT_SECONDS = [string]$MeasurementSeconds

    Push-Location $repoRoot
    try {
        Write-Host "Running pipeline Criterion suite with sample_size=$SampleSize, warm_up=${WarmUpSeconds}s, measurement=${MeasurementSeconds}s..."
        & cargo bench --locked --bench pipeline -- --save-baseline current
        if ($LASTEXITCODE -ne 0) {
            throw "cargo bench failed with exit code $LASTEXITCODE."
        }
    }
    finally {
        Pop-Location
    }

    $current = Get-CurrentMeasurements

    if ($UpdateBaseline) {
        $benchmarkBaselines = [ordered]@{}
        foreach ($entry in $current.GetEnumerator()) {
            $benchmarkBaselines[$entry.Key] = [ordered]@{
                mean_ns = [Math]::Round([double]$entry.Value, 3)
            }
        }

        $baseline = [ordered]@{
            schema_version = 1
            suite = "benches/pipeline.rs"
            criterion_baseline = "current"
            units = "nanoseconds"
            policy = [ordered]@{
                threshold_percent = 25.0
                sample_size = $SampleSize
                warm_up_seconds = $WarmUpSeconds
                measurement_seconds = $MeasurementSeconds
                update_command = "./scripts/bench-gate.ps1 -UpdateBaseline"
            }
            benchmarks = $benchmarkBaselines
        }

        $baselineDir = Split-Path -Parent $BaselinePath
        New-Item -ItemType Directory -Force -Path $baselineDir | Out-Null
        $baseline | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $BaselinePath -Encoding utf8
        Write-Host "Updated performance baseline at $BaselinePath."
        return
    }

    if (-not (Test-Path -LiteralPath $BaselinePath -PathType Leaf)) {
        throw "Baseline not found at '$BaselinePath'. Run ./scripts/bench-gate.ps1 -UpdateBaseline after reviewing local measurements."
    }

    $baseline = Get-Content -LiteralPath $BaselinePath -Raw | ConvertFrom-Json
    if ($ThresholdPercent -lt 0) {
        $ThresholdPercent = [double]$baseline.policy.threshold_percent
    }

    Write-Host ("Gate threshold: fail when current mean is > {0:N1}% slower than baseline." -f $ThresholdPercent)

    $failures = [System.Collections.Generic.List[string]]::new()
    foreach ($entry in $baseline.benchmarks.PSObject.Properties) {
        $key = $entry.Name
        if (-not $current.Contains($key)) {
            throw "Current Criterion output is missing '$key'."
        }

        $baselineMean = [double]$entry.Value.mean_ns
        $currentMean = [double]$current[$key]
        $allowedMean = $baselineMean * (1.0 + ($ThresholdPercent / 100.0))
        $deltaPercent = (($currentMean - $baselineMean) / $baselineMean) * 100.0

        Write-Host ("{0}: baseline={1:N0} ns current={2:N0} ns delta={3:N1}% limit={4:N0} ns" -f `
            $key, $baselineMean, $currentMean, $deltaPercent, $allowedMean)

        if ($currentMean -gt $allowedMean) {
            $failures.Add(("{0} regressed by {1:N1}% (allowed {2:N1}%)." -f $key, $deltaPercent, $ThresholdPercent))
        }
    }

    if ($failures.Count -gt 0) {
        $failures | ForEach-Object { Write-Error $_ }
        exit 1
    }

    Write-Host "Pipeline performance gate passed."
}
finally {
    $env:SESSION_LEDGER_BENCH_SAMPLE_SIZE = $oldSampleSize
    $env:SESSION_LEDGER_BENCH_WARM_UP_SECONDS = $oldWarmUp
    $env:SESSION_LEDGER_BENCH_MEASUREMENT_SECONDS = $oldMeasurement
}
