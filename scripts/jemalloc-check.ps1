<#
.SYNOPSIS
  Machine-check soft optional jemalloc feature anchors (C00 L8).

.DESCRIPTION
  Verifies docs/ops/jemalloc.md documents the soft jemalloc feature and that
  sl-daemon Cargo.toml / main.rs / ops-load soft job stay wired.
  Hermetic: no cargo jemalloc compile — suitable for default Windows cargo test.

  Does not claim always-on production jemalloc or continuous profiling.

.PARAMETER SelfCheck
  Explicit docs/path smoke (CI unit proof). Same checks as the default path.

.PARAMETER Build
  On Unix CI: cargo build --features jemalloc in crates/sl-daemon (soft).

.EXAMPLE
  pwsh ./scripts/jemalloc-check.ps1 -SelfCheck

.EXAMPLE
  pwsh ./scripts/jemalloc-check.ps1 -SelfCheck -Build
#>
[CmdletBinding()]
param(
    [switch]$SelfCheck,
    [switch]$Build
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$docPath = Join-Path $repoRoot "docs/ops/jemalloc.md"
$workflowPath = Join-Path $repoRoot ".github/workflows/ops-load.yml"
$daemonCargoPath = Join-Path $repoRoot "crates/sl-daemon/Cargo.toml"
$daemonMainPath = Join-Path $repoRoot "crates/sl-daemon/src/main.rs"
$selfPath = Join-Path $repoRoot "scripts/jemalloc-check.ps1"
$softTestPath = Join-Path $repoRoot "tests/jemalloc_soft.rs"

function Assert-File {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Label
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Label at '$Path'."
    }
}

function Write-Check {
    param([string]$Label, [bool]$Ok)
    $mark = if ($Ok) { "PASS" } else { "FAIL" }
    Write-Host "  [$mark] $Label"
    return $Ok
}

function Test-DocContains {
    param(
        [Parameter(Mandatory = $true)][string]$Doc,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Label,
        [string]$Context = "docs/ops/jemalloc.md"
    )
    $ok = $Doc.Contains($Needle)
    [void](Write-Check -Label $Label -Ok $ok)
    if (-not $ok) {
        throw "$Context missing required anchor: '$Needle'"
    }
}

Write-Host "Soft jemalloc feature check (C00 L8)"
if ($SelfCheck -or -not $Build) {
    Write-Host "Mode: SelfCheck (docs + Cargo/feature/cfg anchors; no jemalloc compile)"
}

Assert-File -Path $docPath -Label "jemalloc doc"
Assert-File -Path $workflowPath -Label "ops-load workflow"
Assert-File -Path $daemonCargoPath -Label "sl-daemon Cargo.toml"
Assert-File -Path $daemonMainPath -Label "sl-daemon main.rs"
Assert-File -Path $selfPath -Label "jemalloc check script"
Assert-File -Path $softTestPath -Label "jemalloc_soft test"

$doc = Get-Content -LiteralPath $docPath -Raw
$workflow = Get-Content -LiteralPath $workflowPath -Raw
$cargoToml = Get-Content -LiteralPath $daemonCargoPath -Raw
$mainRs = Get-Content -LiteralPath $daemonMainPath -Raw

Write-Host "jemalloc doc anchors:"
Test-DocContains -Doc $doc -Needle "Optional jemalloc allocator" `
    -Label "jemalloc section heading"
Test-DocContains -Doc $doc -Needle "scripts/jemalloc-check.ps1" `
    -Label "SelfCheck script reference"
Test-DocContains -Doc $doc -Needle "-SelfCheck" `
    -Label "SelfCheck invocation"
Test-DocContains -Doc $doc -Needle "Soft jemalloc SelfCheck | **done**" `
    -Label "SelfCheck gate marked done"
Test-DocContains -Doc $doc -Needle "tikv-jemallocator" `
    -Label "tikv-jemallocator reference"
Test-DocContains -Doc $doc -Needle "--features jemalloc" `
    -Label "Cargo feature enable note"
Test-DocContains -Doc $doc -Needle "continue-on-error" `
    -Label "soft continue-on-error note"
Test-DocContains -Doc $doc -Needle "Windows-safe" `
    -Label "Windows-safe default note"
Test-DocContains -Doc $doc -Needle "Continuous jemalloc profiling / production always-on jemalloc | **unpaid**" `
    -Label "always-on jemalloc unpaid gate"

Write-Host "sl-daemon Cargo / main anchors:"
if ($cargoToml -notmatch '(?m)^jemalloc\s*=') {
    throw "crates/sl-daemon/Cargo.toml missing jemalloc feature."
}
[void](Write-Check -Label "Cargo.toml jemalloc feature" -Ok $true)

if ($cargoToml -notmatch 'tikv-jemallocator') {
    throw "crates/sl-daemon/Cargo.toml missing tikv-jemallocator dependency."
}
[void](Write-Check -Label "Cargo.toml tikv-jemallocator dep" -Ok $true)

if ($cargoToml -match '(?m)^default\s*=\s*\[[^\]]*jemalloc') {
    throw "jemalloc must not be in default features (Windows-safe default builds)."
}
[void](Write-Check -Label "jemalloc not in default features" -Ok $true)

if ($mainRs -notmatch 'cfg\(all\(feature = "jemalloc", unix\)\)') {
    throw "main.rs must gate #[global_allocator] on cfg(all(feature = `"jemalloc`", unix))."
}
[void](Write-Check -Label "main.rs jemalloc+unix cfg gate" -Ok $true)

if ($mainRs -notmatch '#\[global_allocator\]') {
    throw "main.rs must declare #[global_allocator] for jemalloc."
}
[void](Write-Check -Label "main.rs global_allocator" -Ok $true)

if ($mainRs -notmatch 'tikv_jemallocator::Jemalloc') {
    throw "main.rs must use tikv_jemallocator::Jemalloc."
}
[void](Write-Check -Label "main.rs Jemalloc type" -Ok $true)

Write-Host "Workflow soft-gate anchors:"
if ($workflow -notmatch '(?m)^\s*jemalloc:') {
    throw "ops-load.yml must declare a jemalloc soft job."
}
[void](Write-Check -Label "ops-load jemalloc job" -Ok $true)

if ($workflow -notmatch 'jemalloc-check\.ps1') {
    throw "ops-load.yml jemalloc job must invoke scripts/jemalloc-check.ps1."
}
[void](Write-Check -Label "ops-load invokes jemalloc-check.ps1" -Ok $true)

if ($workflow -notmatch '--features jemalloc') {
    throw "ops-load.yml must cargo build with --features jemalloc."
}
[void](Write-Check -Label "ops-load --features jemalloc" -Ok $true)

# Soft job block should set continue-on-error near the jemalloc job.
$jemallocBlock = if ($workflow -match '(?s)jemalloc:.*?continue-on-error:\s*true') { $true } else { $false }
if (-not $jemallocBlock) {
    throw "ops-load.yml jemalloc job must set continue-on-error: true (soft gate)."
}
[void](Write-Check -Label "jemalloc job continue-on-error: true" -Ok $true)

Write-Host "Soft jemalloc SelfCheck passed"

if ($Build) {
    if ($IsWindows -or $env:OS -match 'Windows') {
        Write-Host "Skip -Build on Windows (jemalloc is Unix-only); SelfCheck already passed."
        exit 0
    }

    if (-not $env:CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR = Join-Path $repoRoot "target-w33-c00-jemalloc"
        Write-Host "Using worktree-local CARGO_TARGET_DIR=$($env:CARGO_TARGET_DIR)"
    }

    Write-Host "Running cargo build --features jemalloc --locked (sl-daemon) ..."
    Push-Location (Join-Path $repoRoot "crates/sl-daemon")
    try {
        & cargo build --features jemalloc --locked
        if ($LASTEXITCODE -ne 0) {
            throw "cargo build --features jemalloc failed with exit code $LASTEXITCODE."
        }
    }
    finally {
        Pop-Location
    }
    Write-Host "Soft jemalloc feature build passed."
}

exit 0
