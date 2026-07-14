# Reproducible build checks

SessionLedger checks whether two clean release builds of the same binary produce
the same SHA-256 digest. This is reproducibility evidence, not a claim that the
build is fully hermetic or SLSA Build Level 3.

## Quick check

Requirements:

- the repository's Rust toolchain and committed `Cargo.lock`;
- PowerShell 7 (`pwsh` on Linux/macOS or `pwsh.exe` on Windows);
- Git, so the script can derive a stable timestamp from the current commit.

From the repository root, run:

```powershell
pwsh ./scripts/repro-check.ps1
```

The script:

1. derives `SOURCE_DATE_EPOCH` from the current commit timestamp;
2. disables Cargo incremental compilation;
3. builds `sl-daemon` twice with `--release --locked` into separate temporary
   target directories;
4. hashes each release binary with SHA-256 and fails if the digests differ; and
5. removes both build directories.

To reproduce an earlier result, pass its epoch explicitly:

```powershell
pwsh ./scripts/repro-check.ps1 -SourceDateEpoch 1783900000
```

CI runs this check on Linux as a small, blocking pull-request smoke test. For a
release candidate, run it on every release target and record the two hashes,
commit, Rust version, target triple, and host image beside the release evidence.
The script accepts `-ManifestPath` and `-BinaryName` for another Rust binary.

## Interpretation and limits

A matching digest demonstrates that two isolated target directories on one host
produced the same binary. It does not isolate the build from the network, pin the
runner image or system linker, vendor dependencies, compare independent hosts,
or prove that archives have deterministic metadata. `SOURCE_DATE_EPOCH` only
affects tools that honor it.

Windows is best-effort because the MSVC linker, PDB generation, SDK, and host
toolchain may add nondeterministic data. By default, the script reports a
Windows mismatch as a warning; pass `-Strict` to make it blocking. For a
mismatch, preserve both binaries, compare PE sections and PDB behavior, and
record the runner and toolchain versions. Linux CI is always strict.

For stronger hermetic evidence, start with the `sl-daemon` offline dependency
gate in [`hermetic-builds.md`](hermetic-builds.md), then use an immutable builder
image, vendor and build dependencies offline, pin the exact Rust toolchain and
system packages, and run the comparison on two fresh workers. Compare unpacked
binaries separately from ZIP or tar metadata.

## Provenance enforcement path

The release workflow requires GitHub build provenance for assets published from
the canonical `KooshaPari/SessionLedger` repository. The attestation step is
skipped outside that repository so forks are not broken by unavailable OIDC
permissions. Checksum signing remains best-effort because Sigstore availability
is outside the build's control.

This is stronger provenance evidence, but it is not SLSA Build Level 3. To move
closer to that bar:

1. make release publication depend on successful per-platform attestations in
   each matrix build job, with a second aggregate attestation over collected
   Release assets;
2. verify each attestation's repository, workflow, commit, and subject digest
   before publishing; and
3. protect the release environment so bypassing the required workflow needs
   explicit maintainer approval.

Consumers must verify `SHA256SUMS`, its Sigstore bundle when present, and GitHub
attestations as described in [`distribution.md`](distribution.md).

## SLSA materials metadata (partial L3)

SessionLedger does not claim full SLSA Build Level 3. The release workflow does
require GitHub build provenance that binds each published archive **subject**
(name + SHA-256 digest) to **materials** describing the source inputs GitHub
Actions consumed (notably the tagged repository checkout). This is partial
material-metadata evidence: it documents what was built and from which commit, but
it does not prove hermetic isolation, reusable-workflow signing, or complete
dependency closure.

### Contract and fixture

The blocking contract is enforced by
[`scripts/provenance-contract-check.ps1`](../../scripts/provenance-contract-check.ps1)
and [`.github/workflows/provenance-contract.yml`](../../.github/workflows/provenance-contract.yml).
The script asserts that:

1. each matrix `build` job and the aggregate `release` job call
   `attest-build-provenance` with an explicit `subject-path` binding; and
2. [`docs/ops/fixtures/slsa-materials-contract.sample.json`](fixtures/slsa-materials-contract.sample.json)
   remains a valid in-toto statement containing both `subject` and materials
   (`predicate.materials` or `predicate.buildDefinition.resolvedDependencies`).

The fixture uses placeholder digests only; it documents the fields consumers and
maintainers should expect from canonical Release attestations without requiring
Sigstore certificates in CI.

### What to verify on a Release

After downloading an archive from a canonical Release:

1. confirm the archive SHA-256 matches `SHA256SUMS`;
2. run `gh attestation verify <archive> --repo KooshaPari/SessionLedger` and
   confirm the attestation subject digest matches the archive; and
3. inspect the attestation predicate for materials linking the build to
   `KooshaPari/SessionLedger` at the Release tag commit (SLSA v1
   `buildDefinition.resolvedDependencies`, or v0.2 `materials` when present).

Cross-check `session-ledger.cdx.json` when you need component-level dependency
metadata; the CycloneDX SBOM complements but does not replace provenance
materials on the archive subject.

Remaining gaps toward full SLSA-L3 include reusable-workflow provenance,
environment isolation proofs, and mandatory SBOM-to-subject attestations for
every matrix artifact. Platform code-signing remains deferred per
[`0003-platform-code-signing.md`](../adr/0003-platform-code-signing.md).
