# Windows MSI scaffold

`packaging/windows/Product.wxs` is a WiX v4 starting point for a per-user MSI.
It installs `sl-viewer.exe` below `%LOCALAPPDATA%\Programs\SessionLedger` and
adds a Start Menu shortcut. It is not wired into release CI and no MSI is
currently published.

## Developer build

On Windows, first create the tested application layout:

```powershell
.\scripts\package-windows.ps1
```

Install WiX v4 (`dotnet tool install --global wix`), then point WiX at the
versioned layout emitted under `packaging\dist`:

```powershell
$version = "0.1.0"
$source = Resolve-Path "packaging\dist\sl-viewer-v$version-x86_64-pc-windows-msvc"
wix build .\packaging\windows\Product.wxs `
  -d "Version=$version" `
  -d "SourceDir=$source" `
  -o ".\packaging\dist\SessionLedger-$version-x64.msi"
```

This command is a documented scaffold, not a supported release target. Before
publishing an MSI, add clean-Windows install, upgrade, repair, and uninstall
tests and validate the WiX source against the pinned WiX toolchain.

## Signing is deferred

Neither `sl-viewer.exe` nor the MSI is Authenticode-signed. Production release
work must sign both files with `signtool`, timestamp signatures, and establish
certificate custody before enabling MSI publication. Until then, installer
artifacts are for internal evaluation only and may trigger SmartScreen.

Release archives remain covered by the existing checksum, keyless cosign, and
GitHub attestation path documented in
[`docs/ops/distribution.md`](../docs/ops/distribution.md#release-integrity-signing-cosign).
That supply-chain evidence does not replace Authenticode.
