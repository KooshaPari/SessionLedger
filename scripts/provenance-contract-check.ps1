[CmdletBinding()]
param(
    [string]$Workflow = ".github/workflows/release.yml"
)

$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot

if ([System.IO.Path]::IsPathRooted($Workflow)) {
    $WorkflowPath = $Workflow
} else {
    $WorkflowPath = Join-Path $RepoRoot $Workflow
}

if (-not (Test-Path -LiteralPath $WorkflowPath -PathType Leaf)) {
    throw "Release workflow not found: $WorkflowPath"
}

$Content = Get-Content -LiteralPath $WorkflowPath -Raw
$Missing = [System.Collections.Generic.List[string]]::new()

if ($Content -notmatch 'attest-build-provenance@') {
    $Missing.Add("attest-build-provenance action")
}

if ($Content -notmatch '(?ms)build:\s*[\s\S]*?permissions:\s*[\s\S]*?id-token:\s*write') {
    $Missing.Add("build job id-token: write permission")
}

if ($Content -notmatch '(?ms)build:\s*[\s\S]*?permissions:\s*[\s\S]*?attestations:\s*write') {
    $Missing.Add("build job attestations: write permission")
}

if ($Content -notmatch '(?ms)build:\s*[\s\S]*?name:\s*attest platform artifact') {
    $Missing.Add("per-matrix attest platform artifact step")
}

if ($Content -notmatch '(?ms)release:\s*[\s\S]*?attest Release assets') {
    $Missing.Add("release job attest Release assets step")
}

if ($Content -notmatch 'Build provenance failed for KooshaPari/SessionLedger') {
    $Missing.Add("blocking release provenance failure message")
}

if ($Missing.Count -gt 0) {
    Write-Error ("Provenance contract check failed. Missing: {0}" -f ($Missing -join ", "))
    exit 1
}

Write-Host "Provenance contract check passed for $WorkflowPath"
