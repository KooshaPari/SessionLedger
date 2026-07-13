[CmdletBinding()]
param(
    [string]$ProjectRoot = (Split-Path -Parent $PSScriptRoot),
    [switch]$WindowsInstallLifecycle
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

function Assert-Not-Path {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (Test-Path -LiteralPath $Path) {
        throw "Expected $Description to be absent at '$Path'."
    }
    Write-Output "ok: $Description absent"
}

function Join-RepoPath {
    param([Parameter(Mandatory = $true)][string[]]$Parts)
    $path = $ProjectRoot
    foreach ($part in $Parts) {
        $path = Join-Path $path $part
    }
    return $path
}

function Get-ViewerVersion {
    $metadata = cargo metadata --manifest-path (Join-RepoPath @("Cargo.toml")) --no-deps --format-version 1 |
        ConvertFrom-Json
    $viewer = $metadata.packages | Where-Object { $_.name -eq "sl-viewer" } | Select-Object -First 1
    if (-not $viewer) {
        throw "Could not determine the sl-viewer version from cargo metadata."
    }
    return $viewer.version
}

function New-ViewerStubExe {
    param(
        [Parameter(Mandatory = $true)][string]$OutputPath,
        [Parameter(Mandatory = $true)][string]$Version,
        [Parameter(Mandatory = $true)][string]$WorkRoot
    )

    $rustSource = Join-Path $WorkRoot "viewer_stub.rs"
    @"
fn main() {
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--version" | "-V" => {
                println!("sl-viewer $Version");
                return;
            }
            "--help" | "-h" => {
                println!("SessionLedger viewer stub for installer lifecycle smoke");
                return;
            }
            _ => {}
        }
    }
}
"@ | Set-Content -LiteralPath $rustSource -Encoding utf8

    & rustc $rustSource -o $OutputPath
    if ($LASTEXITCODE -ne 0) {
        throw "rustc failed to build the viewer stub executable."
    }
}

function Invoke-WindowsInstallLifecycleSmoke {
    if (-not $IsWindows -and $env:OS -ne "Windows_NT") {
        throw "Windows install lifecycle smoke requires a Windows host."
    }

    $version = Get-ViewerVersion
    $workRoot = Join-Path $env:TEMP "sl-installer-lifecycle-smoke-$([Guid]::NewGuid().ToString('N'))"
    $distDir = Join-Path $workRoot "dist"
    $extractRoot = Join-Path $workRoot "extracted"
    $stubExe = Join-Path $workRoot "sl-viewer-stub.exe"
    $installDir = Join-Path $workRoot "installed"
    $shortcutPath = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\SessionLedger.lnk"
    $uninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\SessionLedger"
    $packageScript = Join-RepoPath @("scripts", "package-windows.ps1")

    try {
        New-Item -ItemType Directory -Force -Path $workRoot | Out-Null
        New-ViewerStubExe -OutputPath $stubExe -Version $version -WorkRoot $workRoot
        Write-Output "ok: generated unsigned sl-viewer stub for packaging smoke"

        & $packageScript -SkipBuild -BinaryPath $stubExe -Version $version -DistDir $distDir
        if ($LASTEXITCODE -ne 0) {
            throw "package-windows.ps1 failed with exit code $LASTEXITCODE."
        }

        $archive = Get-ChildItem -LiteralPath $distDir -Filter *.zip -File | Select-Object -First 1
        if (-not $archive) {
            throw "Windows package archive was not produced."
        }

        Expand-Archive -LiteralPath $archive.FullName -DestinationPath $extractRoot
        $layoutDir = Get-ChildItem -LiteralPath $extractRoot -Directory | Select-Object -First 1
        if (-not $layoutDir) {
            throw "Extracted Windows package layout directory was not found."
        }

        $installPs1 = Join-Path $layoutDir.FullName "Install.ps1"
        $uninstallPs1 = Join-Path $layoutDir.FullName "Uninstall.ps1"
        Assert-Path $installPs1 "extracted Install.ps1"
        Assert-Path $uninstallPs1 "extracted Uninstall.ps1"

        Push-Location $layoutDir.FullName
        try {
            & $installPs1 -InstallDir $installDir
            if ($LASTEXITCODE -ne 0) {
                throw "Install.ps1 failed with exit code $LASTEXITCODE."
            }
        }
        finally {
            Pop-Location
        }

        $installedBinary = Join-Path $installDir "sl-viewer.exe"
        Assert-Path $installedBinary "installed sl-viewer.exe"
        if (-not (Test-Path -LiteralPath $shortcutPath -PathType Leaf)) {
            throw "Start Menu shortcut was not created at '$shortcutPath'."
        }
        if (-not (Test-Path -LiteralPath $uninstallKey)) {
            throw "Windows uninstall registry entry was not created."
        }
        Write-Output "ok: Install.ps1 registered shortcut and uninstall metadata"

        $versionOutput = & $installedBinary --version
        if ($LASTEXITCODE -ne 0) {
            throw "Installed sl-viewer.exe --version failed with exit code $LASTEXITCODE."
        }
        $expectedVersionLine = "sl-viewer $version"
        if ($versionOutput -notcontains $expectedVersionLine) {
            throw "Installed binary version mismatch. Expected '$expectedVersionLine', got '$versionOutput'."
        }
        Write-Output "ok: installed binary --version returned '$expectedVersionLine'"

        & $uninstallPs1 -InstallDir $installDir
        if ($LASTEXITCODE -ne 0) {
            throw "Uninstall.ps1 failed with exit code $LASTEXITCODE."
        }

        Start-Sleep -Seconds 3
        Assert-Not-Path $installDir "installed application directory after uninstall"
        Assert-Not-Path $shortcutPath "Start Menu shortcut after uninstall"
        Assert-Not-Path $uninstallKey "Windows uninstall registry entry after uninstall"
        Write-Output "ok: Windows unsigned install lifecycle smoke passed"
    }
    finally {
        Remove-Item -LiteralPath $workRoot -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $shortcutPath -Force -ErrorAction SilentlyContinue
        Remove-Item -Path $uninstallKey -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $installDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

$productWxs = Join-RepoPath @("packaging", "windows", "Product.wxs")
$installPs1 = Join-RepoPath @("packaging", "windows", "Install.ps1")
$uninstallPs1 = Join-RepoPath @("packaging", "windows", "Uninstall.ps1")
$appImageScript = Join-RepoPath @("packaging", "linux", "package-appimage.sh")
$distributionDoc = Join-RepoPath @("docs", "ops", "distribution.md")
$packagingReadme = Join-RepoPath @("packaging", "README.md")
$daemonContainerfile = Join-RepoPath @("crates", "sl-daemon", "Containerfile")

Write-Output "Installer lifecycle smoke: scaffold assertions."
Write-Output "Unsigned Windows install/uninstall execution is optional and does not prove platform signing."
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

Assert-Contains $daemonContainerfile 'HEALTHCHECK' "daemon Containerfile defines OCI HEALTHCHECK"
Assert-Contains $daemonContainerfile '/healthz' "daemon Containerfile probes /healthz"

Write-Output "ok: installer lifecycle scaffold smoke passed"

if ($WindowsInstallLifecycle) {
    Invoke-WindowsInstallLifecycleSmoke
}
