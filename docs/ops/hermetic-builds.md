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
fully hermetic across every host and target.

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
