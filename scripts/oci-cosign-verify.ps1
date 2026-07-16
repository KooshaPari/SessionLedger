<#
.SYNOPSIS
  Verify-on-deploy check for the best-effort GHCR sl-daemon OCI image.

.DESCRIPTION
  Resolves (or accepts) an image digest, then runs keyless cosign verify against
  the SessionLedger release.yml tag identity. Optionally runs
  `gh attestation verify` for GitHub OCI provenance.

  This script is for deploy-time trust checks. Release CI runs the same verify
  on the canonical repository after sign/attest. Forks skip the `oci-image` job
  when GHCR/OIDC credentials are unavailable.

  Exit codes:
    0 — cosign verify succeeded (and attestation when -RequireAttestation)
    1 — verification failed or required tools/digest missing
    2 — usage / parameter error

.PARAMETER Image
  Registry/repo reference without digest. Default: ghcr.io/kooshapari/sl-daemon

.PARAMETER Tag
  Release tag (for example v0.1.0). Used for certificate-identity and optional
  digest resolution when -Digest is omitted.

.PARAMETER Digest
  Image digest (sha256:…). Prefer this over tag-only pulls for deploy.

.PARAMETER CertificateIdentity
  Override the expected Fulcio certificate identity. Default binds to
  release.yml at refs/tags/<Tag>.

.PARAMETER CertificateOidcIssuer
  Expected OIDC issuer. Default: https://token.actions.githubusercontent.com

.PARAMETER Repo
  GitHub repo for `gh attestation verify`. Default: KooshaPari/SessionLedger

.PARAMETER SkipAttestation
  Skip the optional `gh attestation verify` step (cosign-only).

.PARAMETER RequireAttestation
  Fail if GitHub OCI attestation is missing or unverifiable. Implies attestation
  is attempted (overrides -SkipAttestation).

.PARAMETER AllowUnsigned
  When set, a missing signature prints a warning and exits 0. Use only for
  dry-runs; do not use for production verify-on-deploy.

.EXAMPLE
  pwsh ./scripts/oci-cosign-verify.ps1 -Tag v0.1.0 -Digest sha256:abc…

.EXAMPLE
  pwsh ./scripts/oci-cosign-verify.ps1 -Tag v0.1.0 -RequireAttestation
#>
[CmdletBinding()]
param(
    [string]$Image = "ghcr.io/kooshapari/sl-daemon",

    [Parameter(Mandatory = $true)]
    [string]$Tag,

    [string]$Digest = "",

    [string]$CertificateIdentity = "",

    [string]$CertificateOidcIssuer = "https://token.actions.githubusercontent.com",

    [string]$Repo = "KooshaPari/SessionLedger",

    [switch]$SkipAttestation,

    [switch]$RequireAttestation,

    [switch]$AllowUnsigned
)

$ErrorActionPreference = "Stop"

function Fail {
    param(
        [Parameter(Mandatory = $true)][int]$Code,
        [Parameter(Mandatory = $true)][string]$Message
    )
    Write-Host $Message -ForegroundColor Red
    exit $Code
}

function Write-UsageHint {
    Write-Host @"
Usage:
  pwsh ./scripts/oci-cosign-verify.ps1 -Tag <vX.Y.Z> [-Digest sha256:…] [-RequireAttestation]

Requires: cosign on PATH. Optional: crane (digest resolve), gh (attestation).
Portable Releases stay valid without OCI signatures; use those when verify fails.
"@
}

if ([string]::IsNullOrWhiteSpace($Tag)) {
    Write-UsageHint
    Fail -Code 2 -Message "Tag is required (for example v0.1.0)."
}

if ($Tag -notmatch '^v') {
    Write-Warning "Tag '$Tag' does not start with 'v'. Certificate identity still uses refs/tags/$Tag."
}

if ([string]::IsNullOrWhiteSpace($CertificateIdentity)) {
    $CertificateIdentity = "https://github.com/KooshaPari/SessionLedger/.github/workflows/release.yml@refs/tags/$Tag"
}

function Test-CommandAvailable {
    param([Parameter(Mandatory = $true)][string]$Name)
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

function Resolve-ImageDigest {
    param(
        [Parameter(Mandatory = $true)][string]$ImageRef,
        [Parameter(Mandatory = $true)][string]$ImageTag
    )

    $ref = "${ImageRef}:${ImageTag}"

    if (Test-CommandAvailable -Name "crane") {
        Write-Host "==> Resolving digest with crane: $ref"
        $resolved = (& crane digest $ref 2>&1 | Out-String).Trim()
        if ($LASTEXITCODE -eq 0 -and $resolved -match '^sha256:[0-9a-f]{64}$') {
            return $resolved
        }
        Write-Warning "crane digest failed for $ref (exit $LASTEXITCODE): $resolved"
    }

    if (Test-CommandAvailable -Name "cosign") {
        Write-Host "==> Resolving digest with cosign triangulate: $ref"
        $tri = (& cosign triangulate $ref 2>&1 | Out-String).Trim()
        if ($LASTEXITCODE -eq 0 -and $tri -match '@(sha256:[0-9a-f]{64})\s*$') {
            return $Matches[1]
        }
        # Some cosign versions print IMAGE@sha256:… on one line.
        if ($tri -match '(sha256:[0-9a-f]{64})') {
            return $Matches[1]
        }
        Write-Warning "cosign triangulate failed for $ref (exit $LASTEXITCODE): $tri"
    }

    return $null
}

$normalizedDigest = $Digest.Trim()
if (-not [string]::IsNullOrWhiteSpace($normalizedDigest)) {
    if ($normalizedDigest -notmatch '^sha256:[0-9a-f]{64}$') {
        Fail -Code 2 -Message "Digest must look like sha256:<64 hex chars>; got: $normalizedDigest"
    }
} else {
    $normalizedDigest = Resolve-ImageDigest -ImageRef $Image -ImageTag $Tag
    if ([string]::IsNullOrWhiteSpace($normalizedDigest)) {
        Fail -Code 1 -Message @"
Could not resolve image digest for ${Image}:${Tag}.
Pass -Digest sha256:… explicitly (from the Release Actions summary or crane digest),
or install crane/cosign with registry access.
"@
    }
    if ($normalizedDigest -notmatch '^sha256:[0-9a-f]{64}$') {
        Fail -Code 1 -Message "Resolved digest is invalid: $normalizedDigest"
    }
}

if (-not (Test-CommandAvailable -Name "cosign")) {
    Fail -Code 1 -Message "cosign is not on PATH. Install from https://docs.sigstore.dev/cosign/installation/"
}

$subject = "${Image}@${normalizedDigest}"
Write-Host "==> Verify-on-deploy subject: $subject"
Write-Host "    certificate-identity: $CertificateIdentity"
Write-Host "    certificate-oidc-issuer: $CertificateOidcIssuer"

$cosignArgs = @(
    "verify",
    "--certificate-identity", $CertificateIdentity,
    "--certificate-oidc-issuer", $CertificateOidcIssuer,
    $subject
)

Write-Host "==> cosign $($cosignArgs -join ' ')"
& cosign @cosignArgs
$cosignExit = $LASTEXITCODE

if ($cosignExit -ne 0) {
    $msg = @"
cosign verify failed for $subject (exit $cosignExit).
Treat OCI provenance as unavailable for this tag. Fall back to the portable
sl-daemon archive + SHA256SUMS path, or rebuild from crates/sl-daemon/Containerfile.
Unsigned portable Releases remain valid; do not treat a missing OCI signature as success.
"@
    if ($AllowUnsigned) {
        Write-Warning $msg
        Write-Warning "AllowUnsigned set: exiting 0 despite failed cosign verify (dry-run only)."
        exit 0
    }
    Fail -Code 1 -Message $msg
}

Write-Host "cosign verify: OK"

$runAttestation = $RequireAttestation -or (-not $SkipAttestation)
if ($runAttestation) {
    if (-not (Test-CommandAvailable -Name "gh")) {
        if ($RequireAttestation) {
            Fail -Code 1 -Message "gh is required for -RequireAttestation but is not on PATH."
        }
        Write-Warning "gh not on PATH; skipping OCI attestation verify. Pass -RequireAttestation to make this fatal."
    } else {
        $ociUri = "oci://${subject}"
        Write-Host "==> gh attestation verify $ociUri --repo $Repo"
        & gh attestation verify $ociUri --repo $Repo
        $attestExit = $LASTEXITCODE
        if ($attestExit -ne 0) {
            $attestMsg = "gh attestation verify failed for $ociUri (exit $attestExit)."
            if ($RequireAttestation) {
                Fail -Code 1 -Message "$attestMsg Cosign succeeded, but attestation is required."
            }
            Write-Warning "$attestMsg Cosign succeeded; attestation remains best-effort."
        } else {
            Write-Host "gh attestation verify: OK"
        }
    }
} else {
    Write-Host "Skipping gh attestation verify (-SkipAttestation)."
}

Write-Host "OCI verify-on-deploy passed for $subject"
exit 0
