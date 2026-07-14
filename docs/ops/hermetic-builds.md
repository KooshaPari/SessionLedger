# Hermetic and offline build checks

SessionLedger's current offline evidence is scoped to `crates/sl-daemon`. The
check proves that Cargo can resolve the committed lockfile, fetch dependencies
up front, and then build the daemon without network access:

```powershell
pwsh ./scripts/hermetic-check.ps1
```

The script runs from `crates/sl-daemon`:

1. `cargo fetch --locked`
2. `cargo build --locked --offline`

If the offline build needs the network, Cargo fails before producing the daemon
binary and the script reports the failure as blocking. This catches missing
lockfile updates, dependency feature changes that were not fetched, and registry
access during the build phase.

## CI policy

`.github/workflows/hermetic.yml` runs this check on Ubuntu for pushes and pull
requests. It is a dependency-offline gate, not a claim that release builds are
fully hermetic across every host and target. The same workflow also runs
`scripts/repro-check.ps1 -PolicyOnly` so release packaging keeps exporting
`SOURCE_DATE_EPOCH` without a second compile matrix.

A second job rebuilds `sl-daemon` inside the digest-pinned builder image recorded
in [`hermetic-builder.json`](hermetic-builder.json). The image digest must match
the `container.image` reference in the workflow; bump both together when the
builder stage changes.

The optional root-package check can be run locally with:

```powershell
pwsh ./scripts/hermetic-check.ps1 -IncludeRootPackage
```

That mode builds `session-ledger` with `--package session-ledger` after a root
workspace fetch. CI keeps the required gate focused on `sl-daemon` because the
root workspace also contains desktop packaging dependencies that are governed by
separate release checks.

## What this does not prove

This policy does not meet SLSA Build Level 3. Remaining gaps include:

- no pinned immutable runner image or system package snapshot;
- no vendored dependency directory checked into the repository;
- no isolated rebuild on two independent builders;
- no proof that the linker, OS libraries, or archive metadata are deterministic;
- no protected release environment requiring maintainer approval for bypasses.

Treat this as stronger offline evidence for `sl-daemon` and as a prerequisite
for future hermetic release work, not as a SLSA L3 attestation.

## Environment isolation checklist (SLSA L3 gaps)

Use this checklist when closing C06 environment-isolation / OCI gaps. Items
marked **done** are already in-tree; the rest stay unpaid until operators wire
GitHub Environments and harden runners.

Machine-verify the checklist anchors and done-gate evidence paths (no cargo
build, no network):

```powershell
pwsh ./scripts/hermetic-isolation-check.ps1 -SelfCheck
```

`-SelfCheck` asserts this section stays present, done rows stay marked **done**,
unpaid L3 rows stay documented, `hermetic-builder.json` digests match
`hermetic.yml`, and `release.yml` keeps soft-fail `oci-image` plus the
verify-on-deploy pointer. It does **not** claim SLSA Build Level 3. Optional
soft CI runs the same SelfCheck from `hermetic.yml` (`continue-on-error: true`).

| Gate | Status | Evidence / next step |
|------|--------|----------------------|
| Offline `sl-daemon` fetch+build | **done** | `scripts/hermetic-check.ps1` + `hermetic.yml` |
| Digest-pinned builder image | **done** | `hermetic-builder.json` + container job |
| `SOURCE_DATE_EPOCH` release wiring | **done** | `repro-check.ps1 -PolicyOnly` |
| Best-effort GHCR build + keyless cosign + attest | **done** | `release.yml` `oci-image` (`continue-on-error: true`) |
| Verify-on-deploy (cosign / attestation) | **done (deploy-time)** | `scripts/oci-cosign-verify.ps1` + [distribution.md](distribution.md#verify-an-oci-image-cosign) |
| Protected GitHub Environment for releases | unpaid | Create `release` (or similar) Environment with required reviewers; bind `oci-image` / publish jobs with `environment:` |
| Make `oci-image` release-blocking | unpaid / deferred | Only after Environment + reliable `packages:write` / OIDC; today soft-fail preserves unsigned portable Releases |
| Immutable / ephemeral runners for release | unpaid | Pin self-hosted or hardened runners; avoid mutable `ubuntu-latest` as sole L3 claim |
| Vendored deps + two-builder rebuild | unpaid | Vendor or remote-cache proof; rebuild on a second independent builder |
| System package / linker snapshot | unpaid | Lock OS packages inside the builder image beyond the Rust toolchain pin |
| Isolation checklist SelfCheck | **done** | `scripts/hermetic-isolation-check.ps1 -SelfCheck` (+ soft CI job) |

**Policy:** Prefer deploy-time verify (`oci-cosign-verify.ps1`) over flipping
`oci-image` to hard-fail while GHCR permissions or Sigstore/OIDC can soft-fail.
Portable archives + `SHA256SUMS` remain the supported path when OCI provenance
is missing.

## Builder pin

[`hermetic-builder.json`](hermetic-builder.json) records the MSRV, immutable
`rust:1.87-slim` digest, and offline target path. `scripts/hermetic-check.ps1`
asserts the host `rustc` meets the pinned MSRV before running the offline build.
