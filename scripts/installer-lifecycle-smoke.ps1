[CmdletBinding()]
param(
    [string]$ProjectRoot = (Split-Path -Parent $PSScriptRoot)
)

$ErrorActionPreference = "Stop"

function Assert-Path {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description at '$Path'."
    }
    Write-Output "ok: $Description"
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Select-String -LiteralPath $Path -Pattern $Pattern -Quiet)) {
        throw "Missing $Description in '$Path'."
    }
    Write-Output "ok: $Description"
}

function Join-RepoPath {
    param([Parameter(Mandatory = $true)][string[]]$Parts)
    $path = $ProjectRoot
    foreach ($part in $Parts) {
        $path = Join-Path $path $part
    }
    return $path
}

$productWxs = Join-RepoPath @("packaging", "windows", "Product.wxs")
$installPs1 = Join-RepoPath @("packaging", "windows", "Install.ps1")
$uninstallPs1 = Join-RepoPath @("packaging", "windows", "Uninstall.ps1")
$appImageScript = Join-RepoPath @("packaging", "linux", "package-appimage.sh")
$distributionDoc = Join-RepoPath @("docs", "ops", "distribution.md")
$packagingReadme = Join-RepoPath @("packaging", "README.md")

Write-Output "Installer lifecycle smoke: scaffold assertions only."
Write-Output "This does not build an MSI, run appimagetool, modify the host, or prove platform signing."
Write-Output "Full clean-host MSI install/uninstall evidence still requires Windows, WiX v4, and signed release policy."

Assert-Path $productWxs "WiX Product.wxs scaffold"
Assert-Path $installPs1 "Windows Install.ps1 scaffold"
Assert-Path $uninstallPs1 "Windows Uninstall.ps1 scaffold"
Assert-Path $appImageScript "Linux AppImage scaffold"

Assert-Contains $productWxs 'Scope="perUser"' "WiX per-user install scope"
Assert-Contains $productWxs 'sl-viewer\.exe' "WiX viewer executable payload"
Assert-Contains $productWxs 'StartMenuShortcut' "WiX Start Menu shortcut"
Assert-Contains $productWxs 'RemoveFolder.*On="uninstall"' "WiX uninstall removes install folder"

Assert-Contains $installPs1 'Programs\\SessionLedger' "Windows install path below LocalAppData"
Assert-Contains $installPs1 'SessionLedger\.lnk' "Windows Start Menu shortcut path"
Assert-Contains $installPs1 'UninstallString' "Windows Installed Apps uninstall registration"

Assert-Contains $uninstallPs1 'SessionLedger\.lnk' "Windows uninstall removes Start Menu shortcut"
Assert-Contains $uninstallPs1 'CurrentVersion\\Uninstall\\SessionLedger' "Windows uninstall removes registry entry"
Assert-Contains $uninstallPs1 'User data was not removed' "Windows uninstall preserves user data by design"

Assert-Contains $appImageScript 'appimagetool' "AppImage tool requirement is explicit"
Assert-Contains $appImageScript 'SessionLedger\.AppDir' "AppImage AppDir layout"
Assert-Contains $appImageScript 'sessionledger\.desktop' "AppImage desktop entry"

Assert-Contains $distributionDoc 'WiX MSI remains a scaffold' "distribution guide states MSI scaffold status"
Assert-Contains $distributionDoc 'Uninstall / cleanliness' "distribution guide documents uninstall lifecycle"
Assert-Contains $distributionDoc 'User data' "distribution guide documents user-data retention"
Assert-Contains $packagingReadme 'No MSI, AppImage, or `\.deb` is a supported release target' "packaging README limits installer claims"

Write-Output "ok: installer lifecycle scaffold smoke passed"
