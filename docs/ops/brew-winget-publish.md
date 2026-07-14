# Publishing Homebrew + winget (from in-repo templates)

SessionLedger keeps **templates** under `packaging/homebrew/` and
`packaging/winget/`. Those files are **not** a live Homebrew tap and are
**not** published to `microsoft/winget-pkgs` until you deliberately open
external PRs after a tagged GitHub Release.

Supported user install today remains GitHub Release archives plus
[`scripts/install.sh`](../../scripts/install.sh) /
[`scripts/install.ps1`](../../scripts/install.ps1). See
[`packaging/channels.md`](../../packaging/channels.md).

## Prerequisites

1. A tagged `v*` GitHub Release with portable `sl-viewer` archives and a
   top-level `SHA256SUMS` asset (from `.github/workflows/release.yml`).
2. Digests filled into the in-repo templates via
   [`scripts/fill-packaging-checksums.ps1`](../../scripts/fill-packaging-checksums.ps1).

## Fill digests from SHA256SUMS

Download `SHA256SUMS` from the Release, then:

```powershell
pwsh ./scripts/fill-packaging-checksums.ps1 `
  -Sha256Sums ./SHA256SUMS `
  -Version v0.1.0
```

Or let the script fetch the asset:

```powershell
pwsh ./scripts/fill-packaging-checksums.ps1 `
  -DownloadFromRelease `
  -Version v0.1.0
```

Dry-run without writing:

```powershell
pwsh ./scripts/fill-packaging-checksums.ps1 `
  -Sha256Sums ./SHA256SUMS `
  -Version v0.1.0 `
  -WhatIf
```

The script updates:

| Path | Fields |
|------|--------|
| `packaging/homebrew/sessionledger.rb` | per-arch `url` / `sha256` for macOS arm/intel + Linux x86_64 |
| `packaging/winget/KooshaPari.SessionLedger.installer.yaml` | `InstallerUrl`, `InstallerSha256`, nested path, `PackageVersion` |
| `packaging/winget/KooshaPari.SessionLedger.yaml` | `PackageVersion` |
| `packaging/winget/KooshaPari.SessionLedger.locale.en-US.yaml` | `PackageVersion`, `ReleaseNotesUrl` |

Commit the filled templates in this repo if you want the hashes checked in,
or keep them local only until the external publish PRs land.

## Homebrew: publish a tap (or core PR)

**Status:** templates only. There is no claim that
`brew install koosha/sessionledger/sessionledger` works until a tap exists.

Suggested personal/org tap flow:

1. Create a repository named `homebrew-sessionledger` (Homebrew expects the
   `homebrew-` prefix for `brew tap koosha/sessionledger`).
2. Copy the filled formula to `Formula/sessionledger.rb` in that tap repo.
3. Push the tap, then locally:

   ```bash
   brew tap koosha/sessionledger
   brew install koosha/sessionledger/sessionledger
   brew test sessionledger
   ```

4. Optional later: open a `Homebrew/homebrew-core` PR only when the project
   meets core acceptance (stable versioned releases, livehomepage, etc.).
   Prefer a personal/org tap first.

Until the tap is public, continue recommending the curl install script or
Cargo for the daemon.

## winget: open a microsoft/winget-pkgs PR

**Status:** templates only. `winget install KooshaPari.SessionLedger` will
not resolve until Microsoft merges a manifests PR.

1. Fork [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs).
2. Copy the three filled YAML files into:

   ```text
   manifests/k/KooshaPari/SessionLedger/<PackageVersion>/
     KooshaPari.SessionLedger.yaml
     KooshaPari.SessionLedger.installer.yaml
     KooshaPari.SessionLedger.locale.en-US.yaml
   ```

3. Validate locally (Windows):

   ```powershell
   winget validate --manifest manifests/k/KooshaPari/SessionLedger/<PackageVersion>
   winget install --manifest manifests/k/KooshaPari/SessionLedger/<PackageVersion>
   ```

4. Open a PR against `microsoft/winget-pkgs` following their contribution
   guidelines. Link the GitHub Release and note that the installer is a
   portable ZIP (`InstallerType: zip` + nested portable).

## What this does *not* do

- Does not create or host a Homebrew tap.
- Does not submit winget-pkgs automatically.
- Does not Authenticode-sign the Windows ZIP (ADR 0003).
- Does not bottle `sl-daemon`; the formula caveats still point at Cargo.

After external publish lands, update [`packaging/channels.md`](../../packaging/channels.md)
channel status from “Manifests in-repo…” to Active and document the real
`brew tap` / `winget install` commands.
