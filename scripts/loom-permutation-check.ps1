<#
.SYNOPSIS
  Machine-check loom permutation checker anchors (C00 L7).

.DESCRIPTION
  Verifies docs/ops/concurrency-safety.md documents the loom permutation lane
  (done vs unpaid rows), that the loom-permutation workflow, expanded
  loom_model tests, and this script stay wired. Hermetic: no cargo, no loom
  download — suitable for default Windows cargo test.

  Does not claim full tokio broadcast / daemon graph ports or blocking Miri.

.PARAMETER SelfCheck
  Explicit docs/path smoke (CI unit proof). Same checks as the default path.

.EXAMPLE
  pwsh ./scripts/loom-permutation-check.ps1 -SelfCheck
#>
[CmdletBinding()]
param(
    [switch]$SelfCheck
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$docPath = Join-Path $repoRoot "docs/ops/concurrency-safety.md"
$workflowPath = Join-Path $repoRoot ".github/workflows/loom-permutation.yml"
$loomModelPath = Join-Path $repoRoot "tests/loom_model.rs"
$loomPermTestPath = Join-Path $repoRoot "tests/loom_permutation.rs"
$cargoTomlPath = Join-Path $repoRoot "Cargo.toml"
$selfPath = Join-Path $repoRoot "scripts/loom-permutation-check.ps1"

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
        [string]$Context = "docs/ops/concurrency-safety.md"
    )
    $ok = $Doc.Contains($Needle)
    [void](Write-Check -Label $Label -Ok $ok)
    if (-not $ok) {
        throw "$Context missing required anchor: '$Needle'"
    }
}

Write-Host "Loom permutation checker smoke (C00 L7)"
if ($SelfCheck) {
    Write-Host "Mode: SelfCheck (docs + workflow + cfg anchors; no cargo / no loom)"
}

Assert-File -Path $docPath -Label "concurrency safety doc"
Assert-File -Path $workflowPath -Label "loom-permutation workflow"
Assert-File -Path $loomModelPath -Label "loom_model test"
Assert-File -Path $loomPermTestPath -Label "loom_permutation test wrapper"
Assert-File -Path $cargoTomlPath -Label "Cargo.toml"
Assert-File -Path $selfPath -Label "loom permutation check script"

$doc = Get-Content -LiteralPath $docPath -Raw
$workflow = Get-Content -LiteralPath $workflowPath -Raw
$loomModel = Get-Content -LiteralPath $loomModelPath -Raw
$cargoToml = Get-Content -LiteralPath $cargoTomlPath -Raw

Write-Host "Concurrency safety doc anchors (done vs unpaid):"
Test-DocContains -Doc $doc -Needle "Loom permutation checkers" `
    -Label "loom permutation section heading"
Test-DocContains -Doc $doc -Needle "scripts/loom-permutation-check.ps1" `
    -Label "permutation SelfCheck script reference"
Test-DocContains -Doc $doc -Needle "Loom permutation SelfCheck | **done**" `
    -Label "permutation SelfCheck gate marked done"
Test-DocContains -Doc $doc -Needle "Loom permutation suite CI | **done**" `
    -Label "permutation suite CI gate marked done"
Test-DocContains -Doc $doc -Needle "loom-permutation.yml" `
    -Label "loom-permutation workflow reference"
Test-DocContains -Doc $doc -Needle "Full tokio broadcast / daemon graph under loom | **unpaid**" `
    -Label "full daemon graph unpaid gate"
Test-DocContains -Doc $doc -Needle "Full loom / shuttle permutation checkers | **unpaid**" `
    -Label "shared loom/shuttle unpaid gate retained"
Test-DocContains -Doc $doc -Needle "bounded_try_send_respects_capacity" `
    -Label "sync_channel-style loom model reference"
Test-DocContains -Doc $doc -Needle "broadcast_epoch_fans_out_to_subscribers" `
    -Label "broadcast/SSE loom model reference"
Test-DocContains -Doc $doc -Needle "watcher_pipeline_bounded_enqueue_under_cancel" `
    -Label "watcher pipeline loom model reference"

Write-Host "Workflow blocking-gate anchors:"
if ($workflow -match 'continue-on-error:\s*true') {
    throw "loom-permutation.yml must not set continue-on-error (blocking permutation CI)."
}
[void](Write-Check -Label "workflow has no continue-on-error" -Ok $true)

if ($workflow -notmatch 'loom-permutation-check\.ps1') {
    throw "loom-permutation.yml must exercise scripts/loom-permutation-check.ps1."
}
[void](Write-Check -Label "workflow references loom-permutation-check.ps1" -Ok $true)

if ($workflow -notmatch 'cargo test loom') {
    throw "loom-permutation.yml must run cargo test loom under --cfg loom."
}
[void](Write-Check -Label "workflow runs cargo test loom" -Ok $true)

if ($workflow -notmatch '--cfg loom') {
    throw "loom-permutation.yml must pass RUSTFLAGS --cfg loom."
}
[void](Write-Check -Label "workflow sets --cfg loom" -Ok $true)

Write-Host "Cargo / test cfg anchors:"
if ($cargoToml -notmatch "target\.'cfg\(loom\)'\.dev-dependencies") {
    throw "Cargo.toml missing [target.'cfg(loom)'.dev-dependencies] loom entry."
}
[void](Write-Check -Label "Cargo.toml cfg(loom) dev-dependencies" -Ok $true)

if ($loomModel -notmatch 'bounded_try_send_respects_capacity') {
    throw "tests/loom_model.rs must include bounded try_send permutation."
}
[void](Write-Check -Label "loom_model bounded try_send test" -Ok $true)

if ($loomModel -notmatch 'broadcast_epoch_fans_out_to_subscribers') {
    throw "tests/loom_model.rs must include broadcast/SSE epoch permutation."
}
[void](Write-Check -Label "loom_model broadcast epoch test" -Ok $true)

if ($loomModel -notmatch 'watcher_pipeline_bounded_enqueue_under_cancel') {
    throw "tests/loom_model.rs must include watcher pipeline permutation."
}
[void](Write-Check -Label "loom_model watcher pipeline test" -Ok $true)

Write-Host "Loom permutation SelfCheck passed"
exit 0
