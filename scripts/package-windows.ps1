[CmdletBinding()]
param(
    [string]$Target = "x86_64-pc-windows-msvc",
    [string]$Version,
    [string]$DistDir,
    [string]$BinaryPath,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$ProjectRoot = Split-Path -Parent $PSScriptRoot

$metadata = $null
if (-not $Version -or -not $BinaryPath) {
    $metadata = cargo metadata --manifest-path (Join-Path $ProjectRoot "Cargo.toml") --no-deps --format-version 1 |
        ConvertFrom-Json
}

if (-not $Version) {
    $viewer = $metadata.packages | Where-Object { $_.name -eq "sl-viewer" } | Select-Object -First 1
    if (-not $viewer) {
        throw "Could not determine the sl-viewer version from cargo metadata."
    }
    $Version = $viewer.version
}

if (-not $DistDir) {
    $DistDir = Join-Path $ProjectRoot "packaging\dist"
}

if (-not $SkipBuild) {
    & cargo build --manifest-path (Join-Path $ProjectRoot "Cargo.toml") `
        -p sl-viewer --release --locked --target $Target
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build failed with exit code $LASTEXITCODE."
    }
}

if (-not $BinaryPath) {
    $BinaryPath = Join-Path $metadata.target_directory "$Target\release\sl-viewer.exe"
}
if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw "Viewer binary not found at '$BinaryPath'. Build it first or pass -BinaryPath."
}

$tagVersion = if ($Version.StartsWith("v")) { $Version } else { "v$Version" }
$packageName = "sl-viewer-$tagVersion-$Target"
$layoutDir = Join-Path $DistDir $packageName
$archivePath = Join-Path $DistDir "$packageName.zip"

New-Item -ItemType Directory -Force -Path $DistDir | Out-Null
Remove-Item -Recurse -Force -ErrorAction SilentlyContinue -LiteralPath $layoutDir
Remove-Item -Force -ErrorAction SilentlyContinue -LiteralPath $archivePath
New-Item -ItemType Directory -Path $layoutDir | Out-Null

Copy-Item -LiteralPath $BinaryPath -Destination (Join-Path $layoutDir "sl-viewer.exe")
Copy-Item -LiteralPath (Join-Path $ProjectRoot "LICENSE-MIT") -Destination $layoutDir
Copy-Item -LiteralPath (Join-Path $ProjectRoot "LICENSE-APACHE") -Destination $layoutDir

@"
SessionLedger viewer $tagVersion

Run sl-viewer.exe from this directory.

This is a portable, unsigned build. It is not an MSI installer and does not
register an uninstaller or automatic updates. Windows SmartScreen may warn
before first launch. See docs/ops/distribution.md in the source repository.
"@ | Set-Content -LiteralPath (Join-Path $layoutDir "README.txt") -Encoding utf8

Compress-Archive -LiteralPath $layoutDir -DestinationPath $archivePath -CompressionLevel Optimal
Write-Output "Windows package: $archivePath"
