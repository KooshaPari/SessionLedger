[CmdletBinding()]
param(
    [string]$Ref = "main",
    [int]$Count = 30,
    [switch]$Strict,
    [switch]$BranchProtectionChecklist,
    [string]$Repo = "KooshaPari/SessionLedger"
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $RepoRoot
try {
    function Get-CommitSignatureKind {
        param([string]$Sha)
        $Raw = (& git cat-file -p $Sha 2>$null | Out-String)
        if ([string]::IsNullOrWhiteSpace($Raw)) {
            return "missing"
        }
        if ($Raw -notmatch '(?ms)^gpgsig ') {
            return "unsigned"
        }
        if ($Raw -match 'BEGIN SSH SIGNATURE') {
            return "ssh"
        }
        if ($Raw -match 'BEGIN PGP SIGNATURE') {
            return "gpg"
        }
        return "malformed"
    }

    function Test-CommitVerifier {
        param([string]$Sha)
        $Output = (& git verify-commit $Sha 2>&1 | Out-String).Trim()
        if ($LASTEXITCODE -eq 0) {
            return @{ Ok = $true; Detail = $Output }
        }
        if ($Output -match 'No signature') {
            return @{ Ok = $false; Detail = "no signature" }
        }
        if ($Output -match 'gpg:\s+Signature made|Good signature|not certified|Can''t check signature|no public key') {
            return @{ Ok = $true; Detail = $Output }
        }
        return @{ Ok = $false; Detail = $Output }
    }

    function Write-ChecklistItem {
        param([string]$Text, [bool]$Done = $false)
        $Mark = if ($Done) { "[x]" } else { "[ ]" }
        Write-Host "$Mark $Text"
    }

    function Invoke-BranchProtectionChecklist {
        $Items = @(
            "Require signed commits on branch main",
            "Require a pull request before merging (recommended)",
            "Do not allow bypassing the above settings (recommended)"
        )

        Write-Host "Branch protection checklist for $Repo (main):"
        foreach ($Item in $Items) {
            Write-ChecklistItem $Item
        }

        $Gh = Get-Command gh -ErrorAction SilentlyContinue
        if (-not $Gh) {
            Write-Host ""
            Write-Host "WARN: gh CLI not found; branch protection is not machine-verifiable in OSS."
            Write-Host "See docs/ops/commit-signing.md for manual Settings -> Branches steps."
            return 0
        }

        try {
            $Json = & gh api "repos/$Repo/branches/main/protection" 2>&1
            if ($LASTEXITCODE -ne 0) {
                throw ($Json | Out-String)
            }
            $Protection = $Json | ConvertFrom-Json
            $Signed = [bool]$Protection.required_signatures
            Write-Host ""
            Write-ChecklistItem "Require signed commits on branch main" $Signed
            if (-not $Signed) {
                Write-Host "WARN: GitHub API reports required_signatures=false (or unset)."
                Write-Host "Enable 'Require signed commits' in repository branch protection."
                Write-Host "Exiting soft (docs-only) because OSS CI cannot enforce admin settings."
                return 0
            }
            Write-Host "Branch protection reports required_signatures=true."
            return 0
        }
        catch {
            Write-Host ""
            Write-Host "WARN: Could not query branch protection via gh api (admin scope required)."
            Write-Host $_.Exception.Message
            Write-Host "Exiting soft (docs-only). See docs/ops/commit-signing.md."
            return 0
        }
    }

    if ($BranchProtectionChecklist) {
        exit (Invoke-BranchProtectionChecklist)
    }

    & git rev-parse --verify "$Ref^{commit}" *> $null
    if ($LASTEXITCODE -ne 0) {
        throw "Ref not found: $Ref"
    }

    $Tip = (& git rev-parse "$Ref^{commit}").Trim()
    $Shas = @(& git rev-list -n $Count $Ref)
    if ($Shas.Count -eq 0) {
        throw "No commits found for ref $Ref"
    }

    $Stats = @{
        gpg       = 0
        ssh       = 0
        unsigned  = 0
        malformed = 0
    }
    $Problems = [System.Collections.Generic.List[string]]::new()

    foreach ($Sha in $Shas) {
        $Kind = Get-CommitSignatureKind $Sha
        if ($Stats.ContainsKey($Kind)) {
            $Stats[$Kind]++
        }
        else {
            $Stats.malformed++
        }

        if ($Kind -eq "malformed") {
            $Problems.Add("$Sha has gpgsig block but unrecognized signature format")
            continue
        }
        if ($Kind -in @("gpg", "ssh") -and $Sha -eq $Tip) {
            $Verify = Test-CommitVerifier $Sha
            if (-not $Verify.Ok) {
                $Problems.Add("$Sha ($Kind) verify-commit failed: $($Verify.Detail)")
            }
        }
        if ($Strict -and $Kind -eq "unsigned") {
            $Subject = (& git show -s --format=%s $Sha).Trim()
            $Problems.Add("$Sha unsigned in strict window: $Subject")
        }
    }

    $TipKind = Get-CommitSignatureKind $Tip
    Write-Host "Commit signing report for $Ref (tip $Tip, last $($Shas.Count) commits)"
    Write-Host "  gpg:       $($Stats.gpg)"
    Write-Host "  ssh:       $($Stats.ssh)"
    Write-Host "  unsigned:  $($Stats.unsigned)"
    Write-Host "  malformed: $($Stats.malformed)"
    Write-Host "  tip:       $TipKind"

    if ($TipKind -notin @("gpg", "ssh")) {
        $Problems.Add("Tip commit $Tip on $Ref is not GPG/SSH signed ($TipKind)")
    }

    if ($Problems.Count -gt 0) {
        Write-Host ""
        Write-Host "Findings:"
        foreach ($Problem in $Problems) {
            Write-Host "  - $Problem"
        }
        exit 1
    }

    Write-Host ""
    Write-Host "Commit signing check passed for $Ref."
    exit 0
}
finally {
    Pop-Location
}
