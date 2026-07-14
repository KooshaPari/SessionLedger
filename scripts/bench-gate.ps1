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

    [string]$ArtifactPath,

    [switch]$UpdateBaseline,

    [switch]$SelfCheck
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
if (-not $BaselinePath) {
    $BaselinePath = Join-Path $repoRoot "docs/ops/perf-baseline.json"
}
$BaselinePath = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($BaselinePath)

if (-not $ArtifactPath) {
    $ArtifactPath = Join-Path $repoRoot "artifacts/pipeline-perf-gate.json"
}

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

function Get-BenchmarkKey {
    param([Parameter(Mandatory = $true)][string]$BenchmarkName)
    "pipeline/$BenchmarkName"
}

function Get-BaselineBenchmarkEntry {
    param(
        [Parameter(Mandatory = $true)]$Benchmarks,
        [Parameter(Mandatory = $true)][string]$Key
    )

    $prop = $Benchmarks.PSObject.Properties[$Key]
    if ($null -eq $prop) {
        throw "Baseline is missing required benchmark '$Key'."
    }
    $prop.Value
}

function Assert-BaselinePolicy {
    param(
        [Parameter(Mandatory = $true)]$Baseline,
        [Parameter(Mandatory = $true)][string]$Path
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Baseline not found at '$Path'."
    }

    if ($null -eq $Baseline.policy) {
        throw "Baseline '$Path' is missing policy."
    }

    $enforced = $false
    if ($Baseline.policy.PSObject.Properties.Name -contains "enforced") {
        $enforced = [bool]$Baseline.policy.enforced
    }
    if (-not $enforced) {
        throw "Baseline policy.enforced must be true — pipeline perf budgets are a blocking CI gate."
    }

    $threshold = [double]$Baseline.policy.threshold_percent
    if ($threshold -le 0 -or $threshold -gt 500) {
        throw "Baseline policy.threshold_percent must be in (0, 500]; got $threshold."
    }

    $props = @($Baseline.benchmarks.PSObject.Properties)
    if ($props.Count -eq 0) {
        throw "Baseline benchmarks object is empty."
    }

    foreach ($name in $benchmarkNames) {
        $key = Get-BenchmarkKey -BenchmarkName $name
        $entry = Get-BaselineBenchmarkEntry -Benchmarks $Baseline.benchmarks -Key $key

        $meanNs = [double]$entry.mean_ns
        if ($meanNs -le 0) {
            throw "Baseline '$key'.mean_ns must be > 0."
        }

        if ($entry.PSObject.Properties.Name -contains "budget_mean_ns") {
            $budgetNs = [double]$entry.budget_mean_ns
        }
        else {
            $budgetNs = $meanNs * (1.0 + ($threshold / 100.0))
        }

        $expectedMin = $meanNs * (1.0 + ($threshold / 100.0))
        if ($budgetNs + 0.001 -lt $expectedMin) {
            throw ("Baseline '{0}'.budget_mean_ns ({1}) is tighter than mean_ns + {2}% ({3})." -f `
                $key, $budgetNs, $threshold, $expectedMin)
        }
        if ($budgetNs -lt $meanNs) {
            throw "Baseline '$key'.budget_mean_ns must be >= mean_ns."
        }
    }

    return $threshold
}

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
        $key = Get-BenchmarkKey -BenchmarkName $name
        $measurements[$key] = Get-CriterionEstimate -BenchmarkName $name
    }
    $measurements
}

function Write-GateSummary {
    param(
        [Parameter(Mandatory = $true)][string]$Title,
        [Parameter(Mandatory = $true)][string]$Body
    )

    if ($env:GITHUB_STEP_SUMMARY) {
        @"
## $Title

$Body
"@ | Out-File -FilePath $env:GITHUB_STEP_SUMMARY -Append -Encoding utf8
    }
}

if ($SelfCheck) {
    if (-not (Test-Path -LiteralPath $BaselinePath -PathType Leaf)) {
        throw "Baseline not found at '$BaselinePath'."
    }
    if (-not (Test-Path -LiteralPath $PSCommandPath -PathType Leaf)) {
        throw "bench-gate script missing at '$PSCommandPath'."
    }

    $baseline = Get-Content -LiteralPath $BaselinePath -Raw | ConvertFrom-Json
    $threshold = Assert-BaselinePolicy -Baseline $baseline -Path $BaselinePath

    $rows = New-Object System.Collections.Generic.List[string]
    $rows.Add("| Benchmark | Baseline mean (ns) | Budget ceiling (ns) | Threshold |")
    $rows.Add("|---|---:|---:|---:|")
    foreach ($name in $benchmarkNames) {
        $key = Get-BenchmarkKey -BenchmarkName $name
        $entry = Get-BaselineBenchmarkEntry -Benchmarks $baseline.benchmarks -Key $key
        $meanNs = [double]$entry.mean_ns
        if ($entry.PSObject.Properties.Name -contains "budget_mean_ns") {
            $budgetNs = [double]$entry.budget_mean_ns
        }
        else {
            $budgetNs = $meanNs * (1.0 + ($threshold / 100.0))
        }
        $rows.Add(("| `{0}` | {1:N3} | {2:N3} | {3:N1}% |" -f $key, $meanNs, $budgetNs, $threshold))
    }

    Write-Host "bench-gate SelfCheck passed (enforced=true, threshold=${threshold}%)."
    Write-Host ("Documented budgets: {0}" -f ($benchmarkNames -join ", "))
    Write-GateSummary -Title "Pipeline perf-budget SelfCheck" -Body @"
Enforced blocking gate policy OK (``policy.enforced=true``).

Threshold: **${threshold}%** slower than checked-in ``mean_ns``.

$($rows -join "`n")

CI fails when any Criterion mean exceeds ``budget_mean_ns``.
"@
    exit 0
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
        if ($ThresholdPercent -lt 0) {
            if (Test-Path -LiteralPath $BaselinePath -PathType Leaf) {
                $existing = Get-Content -LiteralPath $BaselinePath -Raw | ConvertFrom-Json
                $ThresholdPercent = [double]$existing.policy.threshold_percent
            }
            else {
                $ThresholdPercent = 25.0
            }
        }

        $benchmarkBaselines = [ordered]@{}
        foreach ($entry in $current.GetEnumerator()) {
            $meanNs = [Math]::Round([double]$entry.Value, 3)
            $budgetNs = [Math]::Round($meanNs * (1.0 + ($ThresholdPercent / 100.0)), 3)
            $benchmarkBaselines[$entry.Key] = [ordered]@{
                mean_ns        = $meanNs
                budget_mean_ns = $budgetNs
            }
        }

        $baseline = [ordered]@{
            schema_version      = 2
            suite               = "benches/pipeline.rs"
            criterion_baseline  = "current"
            units               = "nanoseconds"
            policy              = [ordered]@{
                enforced             = $true
                threshold_percent    = [double]$ThresholdPercent
                sample_size          = $SampleSize
                warm_up_seconds      = $WarmUpSeconds
                measurement_seconds  = $MeasurementSeconds
                update_command       = "./scripts/bench-gate.ps1 -UpdateBaseline"
                self_check_command   = "./scripts/bench-gate.ps1 -SelfCheck"
                notes                = @(
                    "CI job 'pipeline perf regression gate' is blocking (no continue-on-error).",
                    "A run fails when any current mean exceeds budget_mean_ns (baseline mean + threshold_percent)."
                )
            }
            benchmarks          = $benchmarkBaselines
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
    $policyThreshold = Assert-BaselinePolicy -Baseline $baseline -Path $BaselinePath
    if ($ThresholdPercent -lt 0) {
        $ThresholdPercent = $policyThreshold
    }

    Write-Host ("Gate threshold: fail when current mean exceeds budget_mean_ns (>{0:N1}% slower than baseline)." -f $ThresholdPercent)
    Write-Host "Enforcement: blocking (policy.enforced=true)."

    $failures = [System.Collections.Generic.List[string]]::new()
    $resultRows = [System.Collections.Generic.List[object]]::new()
    $summaryRows = New-Object System.Collections.Generic.List[string]
    $summaryRows.Add("| Benchmark | Baseline | Current | Budget | Delta | Result |")
    $summaryRows.Add("|---|---:|---:|---:|---:|:---:|")

    foreach ($name in $benchmarkNames) {
        $key = Get-BenchmarkKey -BenchmarkName $name
        if (-not $current.Contains($key)) {
            throw "Current Criterion output is missing '$key'."
        }

        $entry = Get-BaselineBenchmarkEntry -Benchmarks $baseline.benchmarks -Key $key
        $baselineMean = [double]$entry.mean_ns
        $currentMean = [double]$current[$key]
        if ($entry.PSObject.Properties.Name -contains "budget_mean_ns") {
            $budgetMean = [double]$entry.budget_mean_ns
        }
        else {
            $budgetMean = $baselineMean * (1.0 + ($ThresholdPercent / 100.0))
        }
        $deltaPercent = (($currentMean - $baselineMean) / $baselineMean) * 100.0
        $passed = $currentMean -le $budgetMean

        Write-Host ("{0}: baseline={1:N0} ns current={2:N0} ns delta={3:N1}% budget={4:N0} ns => {5}" -f `
            $key, $baselineMean, $currentMean, $deltaPercent, $budgetMean, $(if ($passed) { "PASS" } else { "FAIL" }))

        $summaryRows.Add(("| `{0}` | {1:N0} | {2:N0} | {3:N0} | {4:N1}% | {5} |" -f `
            $key, $baselineMean, $currentMean, $budgetMean, $deltaPercent, $(if ($passed) { "PASS" } else { "FAIL" })))

        $resultRows.Add([ordered]@{
                benchmark      = $key
                baseline_ns    = [Math]::Round($baselineMean, 3)
                current_ns     = [Math]::Round($currentMean, 3)
                budget_ns      = [Math]::Round($budgetMean, 3)
                delta_percent  = [Math]::Round($deltaPercent, 3)
                passed         = $passed
            })

        if (-not $passed) {
            $failures.Add(("{0} exceeded budget: current {1:N0} ns > budget {2:N0} ns ({3:N1}% vs allowed {4:N1}%)." -f `
                $key, $currentMean, $budgetMean, $deltaPercent, $ThresholdPercent))
        }
    }

    $artifactDir = Split-Path -Parent $ArtifactPath
    if ($artifactDir) {
        New-Item -ItemType Directory -Force -Path $artifactDir | Out-Null
    }
    $artifact = [ordered]@{
        schema_version     = 1
        enforced           = $true
        threshold_percent  = $ThresholdPercent
        baseline_path      = $BaselinePath
        passed             = ($failures.Count -eq 0)
        results            = @($resultRows)
    }
    $artifact | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $ArtifactPath -Encoding utf8
    Write-Host "Wrote gate artifact: $ArtifactPath"

    if ($failures.Count -gt 0) {
        Write-GateSummary -Title "Pipeline perf-budget gate FAILED" -Body @"
Blocking gate failed (``policy.enforced=true``). Threshold: **${ThresholdPercent}%**.

$($summaryRows -join "`n")

Failures:
$($failures | ForEach-Object { "- $_" } | Out-String)
"@
        foreach ($failure in $failures) {
            Write-Host "ERROR: $failure" -ForegroundColor Red
        }
        exit 1
    }

    Write-GateSummary -Title "Pipeline perf-budget gate passed" -Body @"
Blocking gate passed (``policy.enforced=true``). Threshold: **${ThresholdPercent}%**.

$($summaryRows -join "`n")
"@
    Write-Host "Pipeline performance gate passed."
}
finally {
    $env:SESSION_LEDGER_BENCH_SAMPLE_SIZE = $oldSampleSize
    $env:SESSION_LEDGER_BENCH_WARM_UP_SECONDS = $oldWarmUp
    $env:SESSION_LEDGER_BENCH_MEASUREMENT_SECONDS = $oldMeasurement
}
