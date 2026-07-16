[CmdletBinding()]
param(
    [string]$ManifestPath,
    [string]$BinaryName = "sl-daemon",
    [string]$SourceDateEpoch,
    [switch]$Strict,
    [switch]$PolicyOnly
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot

function Assert-SourceDateEpochPolicy {
    $docsPath = Join-Path $repoRoot "docs/ops/reproducible-builds.md"
    $releasePath = Join-Path $repoRoot ".github/workflows/release.yml"

    if (-not (Test-Path -LiteralPath $docsPath -PathType Leaf)) {
        throw "Missing reproducible builds doc at '$docsPath'."
    }
    if (-not (Test-Path -LiteralPath $releasePath -PathType Leaf)) {
        throw "Missing release workflow at '$releasePath'."
    }

    $docs = Get-Content -LiteralPath $docsPath -Raw
    $release = Get-Content -LiteralPath $releasePath -Raw

    if ($docs -notmatch 'SOURCE_DATE_EPOCH') {
        throw "docs/ops/reproducible-builds.md must document SOURCE_DATE_EPOCH."
    }
    if ($docs -notmatch '(?i)release packaging') {
        throw "docs/ops/reproducible-builds.md must mandate SOURCE_DATE_EPOCH for release packaging."
    }
    if ($release -notmatch 'SOURCE_DATE_EPOCH') {
        throw ".github/workflows/release.yml must export SOURCE_DATE_EPOCH for release builds."
    }
    if ($release -notmatch '(?m)^\s*SOURCE_DATE_EPOCH\s*[:=]') {
        throw ".github/workflows/release.yml must assign SOURCE_DATE_EPOCH (env export) for packaging builds."
    }

    Write-Host "SOURCE_DATE_EPOCH policy OK (docs mandate + release.yml export)."
}

Assert-SourceDateEpochPolicy

if ($PolicyOnly) {
    Write-Host "Policy-only mode: skipping dual release builds."
    exit 0
}

if (-not $ManifestPath) {
    $ManifestPath = Join-Path $repoRoot "crates/sl-daemon/Cargo.toml"
}
$ManifestPath = (Resolve-Path -LiteralPath $ManifestPath).Path

if (-not $SourceDateEpoch) {
    if ($env:SOURCE_DATE_EPOCH -match '^\d+$') {
        $SourceDateEpoch = $env:SOURCE_DATE_EPOCH
        Write-Host "Using exported SOURCE_DATE_EPOCH=$SourceDateEpoch"
    }
    else {
        $SourceDateEpoch = (& git -C $repoRoot log -1 --format=%ct).Trim()
        if ($LASTEXITCODE -ne 0 -or $SourceDateEpoch -notmatch '^\d+$') {
            throw "Could not derive SOURCE_DATE_EPOCH from the current Git commit."
        }
    }
}
if ($SourceDateEpoch -notmatch '^\d+$') {
    throw "SourceDateEpoch must be a Unix timestamp."
}

$originalSourceDateEpoch = $env:SOURCE_DATE_EPOCH
$originalCargoIncremental = $env:CARGO_INCREMENTAL
$workDir = Join-Path ([IO.Path]::GetTempPath()) "sessionledger-repro-$([guid]::NewGuid())"
$isWindowsHost = $IsWindows -or $env:OS -eq "Windows_NT"
$extension = if ($isWindowsHost) { ".exe" } else { "" }
$hashes = [System.Collections.Generic.List[string]]::new()

try {
    $env:SOURCE_DATE_EPOCH = $SourceDateEpoch
    $env:CARGO_INCREMENTAL = "0"

    foreach ($attempt in 1..2) {
        $targetDir = Join-Path $workDir "build-$attempt"
        & cargo build --manifest-path $ManifestPath --release --locked --target-dir $targetDir
        if ($LASTEXITCODE -ne 0) {
            throw "Reproducibility build $attempt failed with exit code $LASTEXITCODE."
        }

        $binaryPath = Join-Path $targetDir "release/$BinaryName$extension"
        if (-not (Test-Path -LiteralPath $binaryPath -PathType Leaf)) {
            throw "Expected release binary not found at '$binaryPath'."
        }

        $hash = (Get-FileHash -LiteralPath $binaryPath -Algorithm SHA256).Hash.ToLowerInvariant()
        $hashes.Add($hash)
        Write-Host "build $attempt SHA-256: $hash"
    }

    if ($hashes[0] -ne $hashes[1]) {
        $message = "Reproducibility check failed: release binary digests differ."
        if ($isWindowsHost -and -not $Strict) {
            Write-Warning "$message Windows checks are best-effort; rerun with -Strict to make this mismatch blocking. Archive/PE metadata is not asserted."
        }
        else {
            throw $message
        }
    }
    else {
        Write-Host "Reproducibility check passed for $BinaryName (SOURCE_DATE_EPOCH=$SourceDateEpoch)."
    }
}
finally {
    $env:SOURCE_DATE_EPOCH = $originalSourceDateEpoch
    $env:CARGO_INCREMENTAL = $originalCargoIncremental
    Remove-Item -LiteralPath $workDir -Recurse -Force -ErrorAction SilentlyContinue
}
