<#
.SYNOPSIS
  Machine-check the C08 L75 cross-language OKF fixture parity SSOT.

.DESCRIPTION
  Verifies docs/ops/cross-language-parity.md documents the Python / TypeScript /
  Go parity matrix, language-tag rule, and Harbor N/A boundary, and that each
  matrix fixture exists with a matching source_id language tag. Hermetic: no
  daemon, no network, no cargo.

.PARAMETER SelfCheck
  Explicit docs/fixture smoke (CI unit proof). Same checks as the default path.

.EXAMPLE
  pwsh ./scripts/cross-language-parity-check.ps1 -SelfCheck
#>
[CmdletBinding()]
param(
    [switch]$SelfCheck
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$docPath = Join-Path $repoRoot "docs/ops/cross-language-parity.md"
$evalScopePath = Join-Path $repoRoot "docs/EVAL_SCOPE.md"
$manifestPath = Join-Path $repoRoot "docs/ops/eval-manifest.json"
$checkScript = Join-Path $repoRoot "scripts/cross-language-parity-check.ps1"
$rustWrapper = Join-Path $repoRoot "tests/cross_language_parity.rs"
$workflowPath = Join-Path $repoRoot ".github/workflows/eval-compression.yml"
$fixturesDir = Join-Path $repoRoot "docs/reference/conformance/fixtures"

$parityRows = @(
    @{ Language = "Python"; Tag = "python"; Fixture = "cursor-python-029.okf.json" }
    @{ Language = "TypeScript"; Tag = "typescript"; Fixture = "codex-typescript-023.okf.json" }
    @{ Language = "Go"; Tag = "go"; Fixture = "forge-go-module-026.okf.json" }
)

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
        [Parameter(Mandatory = $true)][string]$Label
    )
    $ok = $Doc.Contains($Needle)
    [void](Write-Check -Label $Label -Ok $ok)
    if (-not $ok) {
        throw "docs/ops/cross-language-parity.md missing required anchor: '$Needle'"
    }
}

Write-Host "Cross-language parity checklist check (C08 L75)"
if ($SelfCheck) {
    Write-Host "Mode: SelfCheck (docs + fixture tags; no cargo / no network)"
}

Assert-File -Path $docPath -Label "cross-language parity doc"
Assert-File -Path $checkScript -Label "cross-language parity check script"
Assert-File -Path $rustWrapper -Label "cross-language parity rust SelfCheck wrapper"
Assert-File -Path $evalScopePath -Label "EVAL_SCOPE.md"
Assert-File -Path $manifestPath -Label "eval-manifest.json"
Assert-File -Path $workflowPath -Label "eval-compression workflow"

$doc = Get-Content -LiteralPath $docPath -Raw
$evalScope = Get-Content -LiteralPath $evalScopePath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$workflow = Get-Content -LiteralPath $workflowPath -Raw

Write-Host "Doc anchors:"
Test-DocContains -Doc $doc -Needle "# Cross-language OKF fixture parity (SSOT)" `
    -Label "SSOT heading"
Test-DocContains -Doc $doc -Needle "## Parity matrix" `
    -Label "parity matrix section"
Test-DocContains -Doc $doc -Needle "### Fixture language-tag rule" `
    -Label "language-tag rule"
Test-DocContains -Doc $doc -Needle "cursor-python-029.okf.json" `
    -Label "Python fixture row"
Test-DocContains -Doc $doc -Needle "codex-typescript-023.okf.json" `
    -Label "TypeScript fixture row"
Test-DocContains -Doc $doc -Needle "forge-go-module-026.okf.json" `
    -Label "Go fixture row"
Test-DocContains -Doc $doc -Needle "Harbor / Portage / Terminal-Bench" `
    -Label "Harbor N/A non-goal"
Test-DocContains -Doc $doc -Needle "scripts/cross-language-parity-check.ps1" `
    -Label "SelfCheck script reference"
Test-DocContains -Doc $doc -Needle "-SelfCheck" `
    -Label "SelfCheck invocation"
Test-DocContains -Doc $doc -Needle "Cross-language parity SelfCheck | **done**" `
    -Label "SelfCheck gate row"
Test-DocContains -Doc $doc -Needle "tests/cross_language_parity.rs" `
    -Label "rust wrapper reference"

Write-Host "Cross-links:"
$evalOk = $evalScope.Contains("cross-language-parity.md")
[void](Write-Check -Label "EVAL_SCOPE -> cross-language-parity" -Ok $evalOk)
if (-not $evalOk) {
    throw "docs/EVAL_SCOPE.md missing link to cross-language-parity.md"
}

Write-Host "Parity fixtures + language tags:"
foreach ($row in $parityRows) {
    $fixturePath = Join-Path $fixturesDir $row.Fixture
    Assert-File -Path $fixturePath -Label "$($row.Language) fixture $($row.Fixture)"

    $stem = $row.Fixture -replace '\.okf\.json$', ''
    $raw = Get-Content -LiteralPath $fixturePath -Raw
    $hasSourceId = $raw.Contains("`"source_id`": `"$stem`"") -or `
        $raw.Contains("`"source_id`":`"$stem`"")
    if (-not $hasSourceId) {
        throw "Fixture $($row.Fixture) missing source_id '$stem'."
    }

    if ($stem -notlike "*$($row.Tag)*") {
        throw "Fixture stem '$stem' missing language tag '$($row.Tag)'."
    }
    if (-not $manifest.Contains($row.Fixture)) {
        throw "eval-manifest.json missing fixture anchor '$($row.Fixture)'."
    }

    Write-Host "  [PASS] $($row.Language): $($row.Fixture) (tag=$($row.Tag))"
}

Write-Host "Workflow anchors:"
$wfOk = $workflow.Contains("cross-language-parity-check.ps1")
[void](Write-Check -Label "eval-compression.yml SelfCheck step" -Ok $wfOk)
if (-not $wfOk) {
    throw ".github/workflows/eval-compression.yml missing cross-language-parity-check.ps1"
}

Write-Host "Cross-language parity SelfCheck passed (C08 L75 SSOT anchors + fixture language tags present)."
