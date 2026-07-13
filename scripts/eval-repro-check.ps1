[CmdletBinding()]
param(
    [string]$ManifestPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
if (-not $ManifestPath) {
    $ManifestPath = Join-Path $repoRoot "docs/ops/eval-manifest.json"
}
$ManifestPath = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($ManifestPath)

if (-not (Test-Path -LiteralPath $ManifestPath -PathType Leaf)) {
    throw "Eval manifest not found at '$ManifestPath'."
}

$manifest = Get-Content -LiteralPath $ManifestPath -Raw | ConvertFrom-Json
$cargoLockPath = Join-Path $repoRoot "Cargo.lock"
$fixtureRoot = Join-Path $repoRoot "docs/reference/conformance/fixtures"
$benchPolicyPath = Join-Path $repoRoot ([string]$manifest.bench_policy_path)
$benchGatePath = Join-Path $repoRoot ([string]$manifest.bench_gate_script)

function Get-Sha256Hex {
    param([Parameter(Mandatory = $true)][string]$Path)
    (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash.ToLowerInvariant()
}

if (-not (Test-Path -LiteralPath $cargoLockPath -PathType Leaf)) {
    throw "Cargo.lock missing at '$cargoLockPath'."
}

$lockHash = Get-Sha256Hex -Path $cargoLockPath
if ($lockHash -ne [string]$manifest.cargo_lock_sha256) {
    throw "Cargo.lock SHA-256 mismatch. Expected $($manifest.cargo_lock_sha256); got $lockHash. Update docs/ops/eval-manifest.json when lockfile changes intentionally."
}

$rustcVersion = (& rustc --version) -replace '^rustc ', ''
$rustcSemver = ($rustcVersion -split '\s+', 2)[0]
if ([version]$rustcSemver -lt [version][string]$manifest.rust_msrv) {
    throw "rustc $rustcSemver is below eval manifest MSRV $($manifest.rust_msrv)."
}

$fixtureCount = (Get-ChildItem -LiteralPath $fixtureRoot -Filter *.okf.json -File).Count
if ($fixtureCount -ne [int]$manifest.fixture_count) {
    throw "Fixture count mismatch. Expected $($manifest.fixture_count); found $fixtureCount under '$fixtureRoot'."
}

foreach ($required in @($benchPolicyPath, $benchGatePath)) {
    if (-not (Test-Path -LiteralPath $required)) {
        throw "Required eval artifact missing: '$required'."
    }
}

$commit = (& git -C $repoRoot rev-parse HEAD).Trim()
Write-Host "Eval reproducibility check passed."
Write-Host "  commit=$commit"
Write-Host "  fixture_seed=$($manifest.fixture_seed) fixture_count=$fixtureCount"
Write-Host "  cargo_lock_sha256=$lockHash"
Write-Host "  rustc=$rustcSemver bench_policy=$($manifest.bench_policy_path)"
