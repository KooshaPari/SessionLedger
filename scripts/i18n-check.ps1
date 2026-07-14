<#
.SYNOPSIS
  Machine-check the C01 L16 i18n scaffold anchors.

.DESCRIPTION
  Verifies locales/en.json (English catalog), src/i18n.rs lookup helper,
  docs/ops/i18n.md Phase-0 decision + future hooks, and the hermetic
  SelfCheck wiring. No fluent/gettext/ICU runtime; no network; no cargo.

.PARAMETER SelfCheck
  Explicit docs/path smoke (CI unit proof). Same checks as the default path.

.EXAMPLE
  pwsh ./scripts/i18n-check.ps1 -SelfCheck
#>
[CmdletBinding()]
param(
    [switch]$SelfCheck
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$catalogPath = Join-Path $repoRoot "locales/en.json"
$helperPath = Join-Path $repoRoot "src/i18n.rs"
$docPath = Join-Path $repoRoot "docs/ops/i18n.md"
$checkScript = Join-Path $repoRoot "scripts/i18n-check.ps1"
$rustWrapper = Join-Path $repoRoot "tests/i18n.rs"

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

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Doc,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Label,
        [string]$Context = "document"
    )
    $ok = $Doc.Contains($Needle)
    [void](Write-Check -Label $Label -Ok $ok)
    if (-not $ok) {
        throw "$Context missing required anchor: '$Needle'"
    }
}

Write-Host "i18n scaffold check (C01 L16)"
if ($SelfCheck) {
    Write-Host "Mode: SelfCheck (catalog + helper + docs; no cargo / no network / no fluent)"
}

Assert-File -Path $catalogPath -Label "English locale catalog"
Assert-File -Path $helperPath -Label "i18n Rust helper"
Assert-File -Path $docPath -Label "i18n ops doc"
Assert-File -Path $checkScript -Label "i18n check script"
Assert-File -Path $rustWrapper -Label "i18n rust SelfCheck wrapper"

$catalog = Get-Content -LiteralPath $catalogPath -Raw
$helper = Get-Content -LiteralPath $helperPath -Raw
$doc = Get-Content -LiteralPath $docPath -Raw

Write-Host "Catalog anchors:"
Assert-Contains -Doc $catalog -Needle '"locale": "en"' `
    -Label "catalog locale en" -Context "locales/en.json"
Assert-Contains -Doc $catalog -Needle '"messages"' `
    -Label "messages object" -Context "locales/en.json"
Assert-Contains -Doc $catalog -Needle '"cli.app_about"' `
    -Label "cli.app_about key" -Context "locales/en.json"
Assert-Contains -Doc $catalog -Needle '"viewer.app_title"' `
    -Label "viewer.app_title key" -Context "locales/en.json"
Assert-Contains -Doc $catalog -Needle '"viewer.empty_state"' `
    -Label "viewer.empty_state key" -Context "locales/en.json"
Assert-Contains -Doc $catalog -Needle "SessionLedger" `
    -Label "SessionLedger brand string" -Context "locales/en.json"

# Parse JSON to ensure the catalog is valid.
try {
    $parsed = $catalog | ConvertFrom-Json
    $msgCount = @($parsed.messages.PSObject.Properties).Count
    $ok = $msgCount -ge 5
    [void](Write-Check -Label "catalog parses with >=5 messages ($msgCount)" -Ok $ok)
    if (-not $ok) {
        throw "locales/en.json must contain at least 5 message keys (found $msgCount)."
    }
} catch {
    throw "locales/en.json failed JSON parse: $_"
}

Write-Host "Helper anchors:"
Assert-Contains -Doc $helper -Label "DEFAULT_LOCALE" -Needle "DEFAULT_LOCALE" `
    -Context "src/i18n.rs"
Assert-Contains -Doc $helper -Label "include_str locales/en.json" `
    -Needle 'include_str!("../locales/en.json")' -Context "src/i18n.rs"
Assert-Contains -Doc $helper -Label "pub fn t" -Needle "pub fn t<" `
    -Context "src/i18n.rs"
Assert-Contains -Doc $helper -Label "pub fn try_catalog" -Needle "pub fn try_catalog(" `
    -Context "src/i18n.rs"
Assert-Contains -Doc $helper -Label "en_catalog" -Needle "pub fn en_catalog(" `
    -Context "src/i18n.rs"
Assert-Contains -Doc $helper -Label "C01 L16 marker" -Needle "C01 L16" `
    -Context "src/i18n.rs"

Write-Host "Doc anchors:"
Assert-Contains -Doc $doc -Needle "## Phase-0 decision" `
    -Label "Phase-0 decision heading" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "English-only" `
    -Label "English-only posture" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "locales/en.json" `
    -Label "catalog path reference" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "src/i18n.rs" `
    -Label "helper path reference" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "## Future hooks" `
    -Label "future hooks heading" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "Fluent" `
    -Label "Fluent future mention" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "try_catalog" `
    -Label "try_catalog future hook" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "scripts/i18n-check.ps1" `
    -Label "SelfCheck script reference" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "-SelfCheck" `
    -Label "SelfCheck invocation" -Context "docs/ops/i18n.md"
Assert-Contains -Doc $doc -Needle "i18n SelfCheck | **done**" `
    -Label "SelfCheck gate marked done" -Context "docs/ops/i18n.md"

Write-Host "i18n SelfCheck passed (C01 L16 English catalog + lookup helper; no fluent runtime)."
