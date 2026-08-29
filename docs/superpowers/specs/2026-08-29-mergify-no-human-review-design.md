# Mergify no-human-review policy

## Goal

Allow SessionLedger pull requests to merge automatically after the configured
machine checks pass, without requiring a human approval. Pull requests remain
the only integration path; direct pushes to `main` remain protected.

## Scope

Change only `.mergify.yml`:

1. Rename the merge rule to describe CI-gated auto-merge rather than approval.
2. Remove `#approved-reviews-by>=1` from that merge rule.
3. Remove the same approval predicate from the `ready-to-merge` label rule.

## Retained safeguards

The policy continues to require the configured CI successes, no merge
conflicts, an open PR, squash merging, current commit-message formatting, and
the existing reviewer request / labeling / large-PR rules. GitHub `main`
protection continues to require PRs and its required `ci / lint` and `ci /
test` checks; its approval count is intentionally zero.

## Validation

1. Parse `.mergify.yml` as YAML.
2. Assert neither merge-related rule contains an approval predicate.
3. Confirm the retained CI and conflict predicates remain present.
4. Open a PR and let normal hosted checks and Mergify evaluate it. No paid
   review or runner is used.

## Rollback

Reintroduce the two approval predicates in a follow-up PR and set GitHub's
required approval count back to one if human review becomes required again.
