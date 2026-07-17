<#
.SYNOPSIS
  Machine-check the C06 environment-isolation checklist in hermetic-builds.md.

.DESCRIPTION
  Verifies that docs/ops/hermetic-builds.md documents the SLSA L3 environment-
  isolation checklist and that in-tree "done" evidence paths exist (offline
  hermetic gate, builder pin, SOURCE_DATE_EPOCH policy, OCI soft-fail +
  verify-on-deploy script). Unpaid L3 gates are reported but do not fail
  SelfCheck — they remain operator/maintainer work.

  -SelfCheck (default when no other mode is requested): docs + path + pin
  consistency only — no cargo build, no network, no gh API. Safe for soft CI.

  Without -SelfCheck the script still runs the same checks (there is no live
  rebuild here; use scripts/hermetic-check.ps1 for offline cargo proof).

.PARAMETER SelfCheck
  Explicit docs/pin smoke (CI unit proof). Same checks as the default path.

.PARAMETER Strict
  Also fail when unpaid checklist gates are still marked unpaid in the doc
  (not recommended for CI until Environments + hardened runners land).

.EXAMPLE
  pwsh ./scripts/hermetic-isolation-check.ps1 -SelfCheck
#>
[CmdletBinding()]
param(
    [switch]$SelfCheck,

    [switch]$Strict
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$docPath = Join-Path $repoRoot "docs/ops/hermetic-builds.md"
$builderPinPath = Join-Path $repoRoot "docs/ops/hermetic-builder.json"
$hermeticWorkflow = Join-Path $repoRoot ".github/workflows/hermetic.yml"
$builderContainerfile = Join-Path $repoRoot "ci/hermetic-builder/Containerfile"
$builderWorkflow = Join-Path $repoRoot ".github/workflows/hermetic-builder.yml"
$releaseWorkflow = Join-Path $repoRoot ".github/workflows/release.yml"
$hermeticCheck = Join-Path $repoRoot "scripts/hermetic-check.ps1"
$reproCheck = Join-Path $repoRoot "scripts/repro-check.ps1"
$ociVerify = Join-Path $repoRoot "scripts/oci-cosign-verify.ps1"
$isolationCheck = Join-Path $repoRoot "scripts/hermetic-isolation-check.ps1"

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
        throw "docs/ops/hermetic-builds.md missing required anchor: '$Needle'"
    }
}

Write-Host "Hermetic isolation checklist check (C06 L3 gaps)"
if ($SelfCheck) {
    Write-Host "Mode: SelfCheck (docs + evidence paths; no cargo / no network)"
}

Assert-File -Path $docPath -Label "hermetic builds doc"
Assert-File -Path $isolationCheck -Label "hermetic isolation check script"
Assert-File -Path $hermeticCheck -Label "hermetic offline check script"
Assert-File -Path $builderPinPath -Label "hermetic builder pin"
Assert-File -Path $hermeticWorkflow -Label "hermetic workflow"
Assert-File -Path $builderContainerfile -Label "repository hermetic builder Containerfile"
Assert-File -Path $builderWorkflow -Label "hermetic builder publish workflow"
Assert-File -Path $reproCheck -Label "repro-check script"
Assert-File -Path $ociVerify -Label "oci cosign verify script"
Assert-File -Path $releaseWorkflow -Label "release workflow"

$doc = Get-Content -LiteralPath $docPath -Raw

Write-Host "Doc anchors:"
Test-DocContains -Doc $doc -Needle "Environment isolation checklist (SLSA L3 gaps)" `
    -Label "checklist heading"
Test-DocContains -Doc $doc -Needle "scripts/hermetic-isolation-check.ps1" `
    -Label "isolation check script reference"
Test-DocContains -Doc $doc -Needle "-SelfCheck" `
    -Label "SelfCheck invocation"
Test-DocContains -Doc $doc -Needle "Offline ``sl-daemon`` fetch+build" `
    -Label "offline sl-daemon gate row"
Test-DocContains -Doc $doc -Needle "Repository-maintained digest-pinned builder image" `
    -Label "builder pin gate row"
Test-DocContains -Doc $doc -Needle "Repository-maintained builder image" `
    -Label "repository builder section"
Test-DocContains -Doc $doc -Needle "Git + CA roots" `
    -Label "builder checkout prerequisites"
Test-DocContains -Doc $doc -Needle "SOURCE_DATE_EPOCH" `
    -Label "SOURCE_DATE_EPOCH gate row"
Test-DocContains -Doc $doc -Needle "Verify-on-deploy" `
    -Label "verify-on-deploy gate row"
Test-DocContains -Doc $doc -Needle "Protected GitHub Environment for releases" `
    -Label "protected Environment unpaid row"
Test-DocContains -Doc $doc -Needle "Immutable / ephemeral runners for release" `
    -Label "immutable runners unpaid row"
Test-DocContains -Doc $doc -Needle "Vendored deps + two-builder rebuild" `
    -Label "vendored/two-builder unpaid row"
Test-DocContains -Doc $doc -Needle "System package / linker snapshot" `
    -Label "system package snapshot unpaid row"
Test-DocContains -Doc $doc -Needle "Isolation checklist SelfCheck" `
    -Label "SelfCheck gate row"

# Done-gate evidence must stay marked done in the table.
$doneNeedles = @(
    @{ Needle = "Offline ``sl-daemon`` fetch+build | **done**"; Label = "offline gate marked done" },
    @{ Needle = "Repository-maintained digest-pinned builder image | **done**"; Label = "builder pin marked done" },
    @{ Needle = "``SOURCE_DATE_EPOCH`` release wiring | **done**"; Label = "SOURCE_DATE_EPOCH marked done" },
    @{ Needle = "GHCR build + keyless cosign + attest + release verify | **done**"; Label = "OCI release-blocking gate marked done" },
    @{ Needle = "Verify-on-deploy (cosign / attestation) | **done (deploy-time)**"; Label = "verify-on-deploy marked done" },
    @{ Needle = "Isolation checklist SelfCheck | **done**"; Label = "SelfCheck gate marked done" }
)
Write-Host "Done-gate status marks:"
foreach ($item in $doneNeedles) {
    Test-DocContains -Doc $doc -Needle $item.Needle -Label $item.Label
}

$unpaidNeedles = @(
    "Protected GitHub Environment for releases | unpaid",
    "Immutable / ephemeral runners for release | unpaid",
    "Vendored deps + two-builder rebuild | unpaid",
    "System package / linker snapshot | unpaid"
)
Write-Host "Unpaid L3 gap rows (documented, not yet closed):"
$unpaidStillOpen = 0
foreach ($needle in $unpaidNeedles) {
    $present = $doc.Contains($needle)
    if ($present) {
        Write-Host "  [OPEN] $needle"
        $unpaidStillOpen++
    }
    else {
        # Row may have flipped to done — treat as closed for reporting.
        Write-Host "  [CLOSED-or-renamed] expected unpaid row missing: $needle"
    }
}

Write-Host "Builder pin + workflow consistency:"
$pin = Get-Content -LiteralPath $builderPinPath -Raw | ConvertFrom-Json
if ([string]::IsNullOrWhiteSpace($pin.builder_image_digest)) {
    throw "hermetic-builder.json missing builder_image_digest."
}
if ($pin.builder_image_digest -notmatch '^sha256:[0-9a-f]{64}$') {
    throw "hermetic-builder.json builder_image_digest is not a sha256 digest."
}
if ([string]::IsNullOrWhiteSpace($pin.msrv)) {
    throw "hermetic-builder.json missing msrv."
}
if ([string]::IsNullOrWhiteSpace($pin.verify_command)) {
    throw "hermetic-builder.json missing verify_command."
}
[string[]]$requiredPinFields = @(
    "builder_definition",
    "builder_publish_workflow",
    "upstream_rust_image",
    "upstream_rust_image_digest"
)
foreach ($field in $requiredPinFields) {
    if ([string]::IsNullOrWhiteSpace([string]$pin.$field)) {
        throw "hermetic-builder.json missing $field."
    }
}
if ($pin.builder_image -notmatch '^ghcr\.io/kooshapari/sessionledger-hermetic-builder$') {
    throw "hermetic-builder.json must pin the repository-maintained GHCR builder image."
}
if ($pin.upstream_rust_image_digest -notmatch '^sha256:[0-9a-f]{64}$') {
    throw "hermetic-builder.json upstream_rust_image_digest is not a sha256 digest."
}
[string[]]$requiredTools = @($pin.required_tools)
if ($requiredTools -notcontains "git" -or $requiredTools -notcontains "ca-certificates") {
    throw "hermetic-builder.json must declare git and ca-certificates as builder prerequisites."
}
[void](Write-Check -Label "builder pin JSON parses (MSRV=$($pin.msrv))" -Ok $true)

$wf = Get-Content -LiteralPath $hermeticWorkflow -Raw
if ($wf -notmatch [regex]::Escape($pin.builder_image_digest)) {
    throw "hermetic.yml container image digest does not match hermetic-builder.json ($($pin.builder_image_digest))."
}
[void](Write-Check -Label "hermetic.yml pins matching builder digest" -Ok $true)

if ($wf -notmatch [regex]::Escape($pin.builder_image)) {
    throw "hermetic.yml does not use the repository-maintained builder image $($pin.builder_image)."
}
foreach ($needle in @("packages: read", "git --version", "ca-certificates.crt")) {
    if (-not $wf.Contains($needle)) {
        throw "hermetic.yml missing builder prerequisite proof: $needle"
    }
}
[void](Write-Check -Label "hermetic.yml proves Git + CA-root checkout prerequisites" -Ok $true)

$builderContainer = Get-Content -LiteralPath $builderContainerfile -Raw
foreach ($needle in @($pin.upstream_rust_image_digest, "ca-certificates git", "git --version", "ca-certificates.crt")) {
    if (-not $builderContainer.Contains($needle)) {
        throw "repository builder Containerfile missing required provenance/prerequisite anchor: $needle"
    }
}
[void](Write-Check -Label "repository builder pins Rust base and installs Git + CA roots" -Ok $true)

$builderPublish = Get-Content -LiteralPath $builderWorkflow -Raw
foreach ($needle in @("packages: write", "docker/build-push-action", "steps.build.outputs.digest", "ci/hermetic-builder/Containerfile")) {
    if (-not $builderPublish.Contains($needle)) {
        throw "hermetic builder publish workflow missing required anchor: $needle"
    }
}
[void](Write-Check -Label "builder workflow publishes immutable GHCR digest" -Ok $true)

if ($wf -notmatch 'hermetic-check\.ps1') {
    throw "hermetic.yml does not invoke scripts/hermetic-check.ps1."
}
[void](Write-Check -Label "hermetic.yml runs hermetic-check.ps1" -Ok $true)

$rel = Get-Content -LiteralPath $releaseWorkflow -Raw
if ($rel -notmatch '(?m)^\s*oci-image:\s*$') {
    throw "release.yml missing oci-image job."
}
if ($rel -notmatch 'detect OCI release gate') {
    throw "release.yml missing detect OCI release gate step for canonical blocking / fork skip."
}
if ($rel -notmatch 'oci-cosign-verify\.ps1') {
    throw "release.yml should reference scripts/oci-cosign-verify.ps1 for deploy-time verify guidance."
}
if ($rel -notmatch 'needs:.*oci-image') {
    throw "release.yml release job should depend on oci-image for blocking publication."
}
[void](Write-Check -Label "release.yml oci-image blocking gate + verify-on-deploy pointer" -Ok $true)

$summary = @"
## Hermetic isolation checklist

SelfCheck passed: ``docs/ops/hermetic-builds.md`` checklist anchors + done-gate
evidence paths + builder digest pin consistency.

Unpaid L3 rows still documented as open: $unpaidStillOpen
(Environment protection, immutable runners, vendor/two-builder,
system package snapshot). Soft CI only — full SLSA Build L3 remains unpaid.
"@

if ($env:GITHUB_STEP_SUMMARY) {
    $summary | Out-File -FilePath $env:GITHUB_STEP_SUMMARY -Append -Encoding utf8
}

if ($Strict -and $unpaidStillOpen -gt 0) {
    Write-Host "Strict mode: $unpaidStillOpen unpaid L3 checklist row(s) remain."
    exit 1
}

Write-Host "Hermetic isolation SelfCheck passed ($unpaidStillOpen unpaid L3 gap row(s) still documented)."
exit 0
