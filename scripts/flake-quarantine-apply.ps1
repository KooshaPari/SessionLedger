#Requires -Version 7.0
<#
.SYNOPSIS
Emit cargo test skip arguments for quarantined flakes.

.DESCRIPTION
Reads docs/ops/flakes-records.json and prints one `--skip <test_name>` pair per
quarantined entry whose `test` field ends with a Rust test function name.
Downstream CI steps can append the output to `cargo test` invocations.
#>
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
$recordsPath = Join-Path $repoRoot 'docs/ops/flakes-records.json'
$recordsRaw = Get-Content -Raw -Path $recordsPath

if ($recordsRaw.Trim() -eq '[]') {
    return
}

$records = $recordsRaw | ConvertFrom-Json
if ($null -eq $records) {
    return
}

foreach ($entry in $records) {
    if ($entry.status -ne 'quarantined') {
        continue
    }

    $test = [string]$entry.test
    if ($test -match '(?:^|\s)([A-Za-z_][A-Za-z0-9_]*)\s*$') {
        Write-Output '--skip'
        Write-Output $Matches[1]
        Write-Host "flake quarantine: skipping '$($Matches[1])' from $($entry.id)" 1>&2
    } else {
        throw "quarantined flake '$($entry.id)' test field must end with a Rust test name: $test"
    }
}
