# Platform signing readiness checklist

Operational companion to
[`docs/adr/0003-platform-code-signing.md`](../adr/0003-platform-code-signing.md).
Records the **current unsigned release state**, what CI would need before
Authenticode or Apple notarization can land, and machine-checkable anchors for
deferred evidence. This document does **not** claim that signing certificates
or notarization credentials exist in the repository or in GitHub Actions today.

## Current unsigned state (production channel)

| Artifact | Format | Release CI | Platform trust |
|----------|--------|------------|----------------|
| `sl-viewer` / `sl-daemon` portable archives | `.tar.gz` / `.zip` | **Shipped** on every `v*` tag | Unsigned binary; verify via `SHA256SUMS` + cosign + `gh attestation` |
| Windows installer | `SessionLedger-<ver>-x64.msi` (WiX v4) | **Shipped + smoke** (`smoke-windows` silent install) | Unsigned MSI; Authenticode **deferred** |
| macOS installer | `SessionLedger-<ver>-<arch>.pkg` (`productbuild`) | **Shipped + smoke** (`smoke-macos-pkg` expand) | Unsigned PKG; notarization **deferred** |
| macOS app bundle | `SessionLedger-<ver>-<arch>.app.tar.gz` | **Shipped** | Unsigned `.app`; codesign **deferred** |
| Linux installers | `.deb` / AppImage (best-effort) | Attached when packaging scripts succeed | Unsigned scaffolds |

Portable trust today (no platform certificates):

1. Download from the canonical GitHub Release for the intended `v*` tag.
2. Verify `SHA256SUMS` (and `SHA256SUMS.sigstore.json` when present).
3. Verify GitHub build provenance with `gh attestation verify` (blocking on
   canonical releases).
4. For internal testing only, accept Gatekeeper / SmartScreen friction documented
   in [`distribution.md`](distribution.md#platform-code-signing--notarization--deferred).

## Signing readiness checklist (future CI)

Revisit platform-native signing only when **all** rows below are satisfied.
Until then, record signing as **deferred / N/A** — not an open implementation
gap without credentials.

| Gate | Status | Evidence / prerequisite |
|------|--------|-------------------------|
| ADR 0003 deferral documented | **done** | [`0003-platform-code-signing.md`](../adr/0003-platform-code-signing.md) |
| Unsigned MSI/PKG build + Release smoke | **done** | [`.github/workflows/release.yml`](../../.github/workflows/release.yml) `package Windows MSI (unsigned)`, `package macOS app + PKG (unsigned)`, `smoke-windows`, `smoke-macos-pkg` |
| Unsigned portable clean-host smoke (PR CI) | **done** | [`ci.yml`](../../.github/workflows/ci.yml) `clean-host-smoke-windows` via `installer-lifecycle-smoke.ps1` |
| Checksum + cosign + GitHub attestation path | **done** | [`distribution.md`](distribution.md#release-integrity-signing-cosign) |
| Signing readiness SelfCheck | **done** | `scripts/signing-readiness-check.ps1 -SelfCheck` |
| Maintainer-held Apple Developer ID certificate in approved secret store | **unpaid** | No `codesign` / `notarytool` steps in CI |
| Maintainer-held Windows Authenticode certificate in approved secret store | **unpaid** | No `signtool` steps in CI |
| Signed clean-host install → launch → uninstall smoke (macOS + Windows) | **unpaid** | Authenticode / notarized evidence deferred per ADR 0003 |
| ADR 0001 auto-update requirements satisfied or explicitly out of scope | **unpaid** | Signature-mandatory updater remains out of scope |

### What CI would need (when credentials land)

These are **design prerequisites**, not secrets checked into the repo:

| Platform | CI inputs (maintainer-provided) | Workflow touchpoints |
|----------|--------------------------------|----------------------|
| **Windows** | Authenticode cert + password or DigiCert KeyLocker / Azure SignTool integration; optional timestamp server URL | Post-build step after `package-msi.ps1`; sign `sl-viewer.exe` before MSI packaging; extend `smoke-windows` to assert signature presence |
| **macOS** | Apple Developer ID Application cert; App Store Connect API key for `notarytool`; team ID | Post-build after `package-app.sh`; `notarytool submit` + stapling before PKG attach; extend `smoke-macos-pkg` for signed expand |
| **Both** | GitHub Environment with restricted secret access; named owner for clean-host validation | New protected Environment on `release.yml`; blocking smoke jobs before Release publish |

Do **not** add placeholder secret names or fake certificate values to the
repository. Document the integration points above; store credentials only in
the approved secret store when maintainers supply them.

## SelfCheck (machine proof)

Docs + workflow anchors only — no network, no certificates, no signing tools:

```powershell
pwsh ./scripts/signing-readiness-check.ps1 -SelfCheck
```

The script asserts:

- ADR 0003 deferral + portable trust model + reconsider triggers
- This checklist documents unsigned MSI/PKG state and unpaid credential gates
- `.github/workflows/release.yml` retains unsigned packaging step names and
  Release smoke job wiring

Soft CI may run the same SelfCheck with `continue-on-error: true` until a
dedicated workflow job is added.

## Related

- [`distribution.md`](distribution.md) — release matrix, clean-host evidence, cosign
- [`packaging/README.md`](../../packaging/README.md) — local MSI/PKG builds
- Issue [#66](https://github.com/KooshaPari/SessionLedger/issues/66) — signing + installers
- ADR 0001 — auto-update deferral until signature-verified replacement exists
