<#
.SYNOPSIS
Optional dhat heap-profiling smoke for the session-ledger process_session pipeline (L8).

.DESCRIPTION
Parses docs/ops/alloc-profile.json and optionally runs the feature-gated dhat
integration test (cargo test --test alloc_profile_dhat --features alloc-profile).
This is the cheap L8 companion beyond scripts/allocation-budget-check.ps1: it
records real allocator statistics without jemalloc or continuous daemon profiling.

Hermetic. Use -SelfCheck to validate args + ceiling config without compiling dhat.
#>
[CmdletBinding()]
param(
    [string]$ProfileConfig = "",

    [ValidateRange(0, 9223372036854775807)]
    [long]$MaxBytesCeiling = 0,

    [ValidateRange(0, 9223372036854775807)]
    [long]$TotalBlocksCeiling = 0,

    [switch]$SelfCheck,

    [switch]$RunTest
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptRoot = $PSScriptRoot
$repoRoot = Split-Path -Parent $scriptRoot

if ([string]::IsNullOrWhiteSpace($ProfileConfig)) {
    $ProfileConfig = Join-Path $repoRoot "docs/ops/alloc-profile.json"
}

function Get-AllocProfileBudget {
    param(
        [Parameter(Mandatory = $true)][string]$ConfigPath,
        [long]$OverrideMaxBytes,
        [long]$OverrideTotalBlocks
    )

    if (-not (Test-Path -LiteralPath $ConfigPath -PathType Leaf)) {
        throw "Allocator profile config not found at '$ConfigPath'."
    }

    $config = Get-Content -LiteralPath $ConfigPath -Raw | ConvertFrom-Json
    if ($null -eq $config.schema -or $config.schema -ne "sessionledger.alloc-profile.v1") {
        throw "Profile config '$ConfigPath' must declare schema sessionledger.alloc-profile.v1."
    }
    if ($null -eq $config.max_bytes_ceiling) {
        throw "Profile config '$ConfigPath' is missing max_bytes_ceiling."
    }
    if ($null -eq $config.total_blocks_ceiling) {
        throw "Profile config '$ConfigPath' is missing total_blocks_ceiling."
    }
    if ($null -eq $config.profiler -or $config.profiler -ne "dhat") {
        throw "Profile config '$ConfigPath' must declare profiler dhat."
    }

    $maxBytes = if ($OverrideMaxBytes -gt 0) { $OverrideMaxBytes } else { [long]$config.max_bytes_ceiling }
    $blocks = if ($OverrideTotalBlocks -gt 0) { $OverrideTotalBlocks } else { [long]$config.total_blocks_ceiling }

    if ($maxBytes -le 0) {
        throw "max_bytes_ceiling must be a positive integer (got $maxBytes)."
    }
    if ($blocks -le 0) {
        throw "total_blocks_ceiling must be a positive integer (got $blocks)."
    }

    return [pscustomobject]@{
        MaxBytesCeiling     = $maxBytes
        TotalBlocksCeiling  = $blocks
        Workload            = [string]$config.workload
        Profiler            = [string]$config.profiler
        ConfigPath          = $ConfigPath
    }
}

$budget = Get-AllocProfileBudget `
    -ConfigPath $ProfileConfig `
    -OverrideMaxBytes $MaxBytesCeiling `
    -OverrideTotalBlocks $TotalBlocksCeiling

Write-Host ("Max bytes ceiling: {0:N0} ({1:N1} MiB)" -f $budget.MaxBytesCeiling, ($budget.MaxBytesCeiling / 1MB))
Write-Host ("Total blocks ceiling: {0:N0}" -f $budget.TotalBlocksCeiling)
Write-Host ("Workload: {0}" -f $budget.Workload)
Write-Host ("Profiler: {0}" -f $budget.Profiler)
Write-Host ("Config: {0}" -f $budget.ConfigPath)

if ($SelfCheck) {
    Write-Host "Self-check passed: alloc-profile config parses and ceilings are positive."
    if (-not $RunTest) {
        exit 0
    }
}

if (-not $RunTest -and -not $SelfCheck) {
    $RunTest = $true
}

if ($RunTest) {
    if (-not $env:CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR = Join-Path $repoRoot "target-w29-c00-dhat"
        Write-Host "Using worktree-local CARGO_TARGET_DIR=$($env:CARGO_TARGET_DIR)"
    }

    Write-Host "Running cargo test --test alloc_profile --locked (hermetic wiring) ..."
    Push-Location $repoRoot
    try {
        & cargo test --test alloc_profile --locked
        if ($LASTEXITCODE -ne 0) {
            throw "alloc_profile tests failed with exit code $LASTEXITCODE."
        }

        Write-Host "Running cargo test --test alloc_profile_dhat --features alloc-profile --locked ..."
        & cargo test --test alloc_profile_dhat --features alloc-profile --locked -- --nocapture
        if ($LASTEXITCODE -ne 0) {
            throw "alloc_profile_dhat tests failed with exit code $LASTEXITCODE."
        }
    }
    finally {
        Pop-Location
    }

    Write-Host "Allocator profile smoke passed."
}
