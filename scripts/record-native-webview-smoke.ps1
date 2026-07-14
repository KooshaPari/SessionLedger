<#
.SYNOPSIS
Record a native WebView accessibility smoke pass as machine-readable JSON.

.DESCRIPTION
Copies the sample fixture schema, fills build/git/host metadata, and writes an
evidence file maintainers can attach to audits. Does not launch the viewer or a
screen reader — run the checklists in docs/a11y/ first, then record the outcome.

.EXAMPLE
pwsh -NoProfile -File scripts/record-native-webview-smoke.ps1 `
  -Outcome pass `
  -ScreenReader NVDA `
  -OutPath docs/ops/fixtures/native-webview-smoke.local.json
#>
[CmdletBinding()]
param(
    [ValidateSet("pass", "fail", "partial")]
    [string]$Outcome = "pass",

    [ValidateSet("NVDA", "VoiceOver", "Narrator", "Orca", "none")]
    [string]$ScreenReader = "NVDA",

    [string]$ScreenReaderVersion = "",

    [string]$BuildId = "",

    [string]$OutPath = "",

    [string]$RepoRoot = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
    $RepoRoot = Split-Path -Parent $PSScriptRoot
}

$samplePath = Join-Path $RepoRoot "docs/ops/fixtures/native-webview-smoke.sample.json"
if (-not (Test-Path -LiteralPath $samplePath -PathType Leaf)) {
    throw "Missing sample fixture at '$samplePath'."
}

Push-Location $RepoRoot
try {
    $gitSha = (git rev-parse HEAD 2>$null)
    if (-not $gitSha) {
        $gitSha = "unknown"
    }
}
finally {
    Pop-Location
}

if ([string]::IsNullOrWhiteSpace($BuildId)) {
    $BuildId = "local-{0}" -f (Get-Date -Format "yyyyMMddHHmmss")
}

if ([string]::IsNullOrWhiteSpace($OutPath)) {
    $OutPath = Join-Path $RepoRoot "docs/ops/fixtures/native-webview-smoke.local.json"
}

$sample = Get-Content -LiteralPath $samplePath -Raw | ConvertFrom-Json
$sample.outcome = $Outcome
$sample.buildId = $BuildId
$sample.gitSha = "$gitSha"
$sample.generatedAt = (Get-Date).ToUniversalTime().ToString("o")
$sample.host.os = if ($IsWindows -or $env:OS -match "Windows") { "windows" }
    elseif ($IsMacOS) { "macos" }
    elseif ($IsLinux) { "linux" }
    else { "unknown" }
$sample.host.osVersion = [System.Environment]::OSVersion.Version.ToString()
$sample.host.screenReader = $ScreenReader
$sample.host.screenReaderVersion = $ScreenReaderVersion
$sample.host.ci = [bool]($env:CI -or $env:GITHUB_ACTIONS)

if ($Outcome -ne "pass") {
    foreach ($item in $sample.checklist) {
        if ($item.outcome -eq "pass") {
            $item.outcome = $Outcome
            $item.detail = "Marked $Outcome by recorder; edit per-item detail before filing."
        }
    }
}

$outDir = Split-Path -Parent $OutPath
if ($outDir -and -not (Test-Path -LiteralPath $outDir)) {
    New-Item -ItemType Directory -Path $outDir | Out-Null
}

$sample | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $OutPath -Encoding utf8
Write-Host "ok: native WebView smoke evidence written to $OutPath (outcome=$Outcome, sha=$gitSha)"
