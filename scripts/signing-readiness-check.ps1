<#
.SYNOPSIS
  Machine-check the platform signing readiness checklist (C11 L112 deferred evidence).

.DESCRIPTION
  Verifies that docs/adr/0003-platform-code-signing.md and
  docs/ops/signing-readiness.md document the deferred signing posture, and that
  .github/workflows/release.yml retains unsigned MSI/PKG packaging and smoke
  anchors. Does not access signing certificates, notarization credentials, or
  external secret stores.

  -SelfCheck: docs + release.yml anchor smoke only (no cargo / no network).

.PARAMETER SelfCheck
  Explicit docs/workflow smoke (CI unit proof). Same checks as the default path.

.EXAMPLE
  pwsh ./scripts/signing-readiness-check.ps1 -SelfCheck
#>
[CmdletBinding()]
param(
    [switch]$SelfCheck
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$adrPath = Join-Path $repoRoot "docs/adr/0003-platform-code-signing.md"
$readinessPath = Join-Path $repoRoot "docs/ops/signing-readiness.md"
$releasePath = Join-Path $repoRoot ".github/workflows/release.yml"
$distributionPath = Join-Path $repoRoot "docs/ops/distribution.md"
$selfPath = Join-Path $repoRoot "scripts/signing-readiness-check.ps1"

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
        throw "Missing required anchor: '$Needle' ($Label)"
    }
}

Write-Host "Platform signing readiness check (C11 L112 deferred evidence)"
if ($SelfCheck) {
    Write-Host "Mode: SelfCheck (ADR + release.yml unsigned anchors; no secrets / no network)"
}

Assert-File -Path $adrPath -Label "ADR 0003"
Assert-File -Path $readinessPath -Label "signing readiness checklist"
Assert-File -Path $releasePath -Label "release workflow"
Assert-File -Path $distributionPath -Label "distribution guide"
Assert-File -Path $selfPath -Label "signing readiness check script"

$adr = Get-Content -LiteralPath $adrPath -Raw
$readiness = Get-Content -LiteralPath $readinessPath -Raw
$release = Get-Content -LiteralPath $releasePath -Raw
$distribution = Get-Content -LiteralPath $distributionPath -Raw

Write-Host "ADR 0003 anchors:"
Test-DocContains -Doc $adr -Needle "Platform code-signing and notarization remain deferred" `
    -Label "ADR deferral title"
Test-DocContains -Doc $adr -Needle "SHA256SUMS" `
    -Label "ADR portable SHA256SUMS trust path"
Test-DocContains -Doc $adr -Needle "Authenticode" `
    -Label "ADR Authenticode deferral"
Test-DocContains -Doc $adr -Needle "notarization" `
    -Label "ADR notarization deferral"
Test-DocContains -Doc $adr -Needle "## Reconsider when" `
    -Label "ADR reconsider triggers"
Test-DocContains -Doc $adr -Needle "signing-readiness.md" `
    -Label "ADR cross-link to signing readiness checklist"

Write-Host "Signing readiness checklist anchors:"
Test-DocContains -Doc $readiness -Needle "Platform signing readiness checklist" `
    -Label "checklist heading"
Test-DocContains -Doc $readiness -Needle "Current unsigned state" `
    -Label "unsigned current state section"
Test-DocContains -Doc $readiness -Needle "SessionLedger-<ver>-x64.msi" `
    -Label "unsigned MSI artifact naming"
Test-DocContains -Doc $readiness -Needle "SessionLedger-<ver>-<arch>.pkg" `
    -Label "unsigned PKG artifact naming"
Test-DocContains -Doc $readiness -Needle "signing-readiness-check.ps1" `
    -Label "SelfCheck script reference"
Test-DocContains -Doc $readiness -Needle "Signing readiness SelfCheck | **done**" `
    -Label "SelfCheck gate marked done"
Test-DocContains -Doc $readiness -Needle "Maintainer-held Apple Developer ID certificate" `
    -Label "unpaid Apple credential gate"
Test-DocContains -Doc $readiness -Needle "Maintainer-held Windows Authenticode certificate" `
    -Label "unpaid Windows credential gate"
Test-DocContains -Doc $readiness -Needle "Do **not** add placeholder secret names" `
    -Label "no fake secrets policy"

Write-Host "release.yml unsigned path anchors:"
Test-DocContains -Doc $release -Needle "Platform signing / notarization: DEFERRED" `
    -Label "release.yml deferral header comment"
Test-DocContains -Doc $release -Needle "package Windows MSI (unsigned)" `
    -Label "unsigned Windows MSI packaging step"
Test-DocContains -Doc $release -Needle "package macOS app + PKG (unsigned)" `
    -Label "unsigned macOS PKG packaging step"
Test-DocContains -Doc $release -Needle "package-msi.ps1" `
    -Label "package-msi.ps1 invocation"
Test-DocContains -Doc $release -Needle "smoke-windows:" `
    -Label "smoke-windows job"
Test-DocContains -Doc $release -Needle "smoke-macos-pkg:" `
    -Label "smoke-macos-pkg job"
Test-DocContains -Doc $release -Needle "SessionLedger-*-x64.msi" `
    -Label "MSI silent install smoke anchor"
Test-DocContains -Doc $release -Needle "ADR 0003" `
    -Label "ADR 0003 reference in release workflow"

if ($release -notmatch '(?m)^\s*smoke-windows:\s*$') {
    throw "release.yml missing smoke-windows job definition."
}
if ($release -notmatch '(?m)^\s*smoke-macos-pkg:\s*$') {
    throw "release.yml missing smoke-macos-pkg job definition."
}
[void](Write-Check -Label "release.yml smoke job definitions present" -Ok $true)

Write-Host "Distribution cross-links:"
Test-DocContains -Doc $distribution -Needle "Platform code-signing & notarization — DEFERRED" `
    -Label "distribution deferral section"
Test-DocContains -Doc $distribution -Needle "0003-platform-code-signing.md" `
    -Label "distribution ADR 0003 link"

$summary = @"
## Platform signing readiness SelfCheck

SelfCheck passed: ADR 0003 deferral + ``docs/ops/signing-readiness.md`` unsigned
state checklist + ``release.yml`` unsigned MSI/PKG packaging and smoke anchors.

Platform Authenticode / notarization credentials remain **unpaid** — no CI
secret claims were made.
"@

if ($env:GITHUB_STEP_SUMMARY) {
    $summary | Out-File -FilePath $env:GITHUB_STEP_SUMMARY -Append -Encoding utf8
}

Write-Host "Signing readiness SelfCheck passed (deferred platform signing; unsigned MSI/PKG anchors OK)."
exit 0
