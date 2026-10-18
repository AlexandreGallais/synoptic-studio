---
id: F22
epic: E17
title: The release made by the merge into main, its CI green
status: draft
---

# E17 · F22 — The release made by the merge into main, its CI green

**Benefit hypothesis**: the Product Owner publishes 0.3.0 by one merge of `develop` into `main` whose CI is green, and the release appears with its notes, without a release branch nor a commit of the version.

Decided by the Product Owner on 2026-10-18 at E17's retrospective ([RET-004](../stories/E17-RET-004-retrospective-e17.md), P1 and P2; [ADR-0083](../../adr/0083-the-release-made-by-the-merge-into-main-from-a-tag.md)). A tooling feature: validated by the agent (ADR-0072). Its research was done by RET-004 (`REF-SEMANTIC-RELEASE-FAQ`, `REF-NPM-VERSION`, `REF-GH-WORKFLOW-DISPATCH`); two stories, so no checkpoint (ADR-0084). E17 closes at the publication of 0.3.0.

## Refinement (2026-10-18)

- The CI first: the three tests that fail on GitHub's runner and pass on the Mac hold on both, without weakening what they prove (`[F19.AC6]` under `GITHUB_ACTIONS`, `[F19.AC9]` when a crate download fails, `[F02.AC1]` on a slower machine).
- Then the release: option B of RET-004's P2, as ADR-0083 decides; 0.3.0 is its first release.

## Acceptance criteria

1. The CI of a pull request `develop` → `main` passes on GitHub's runner with the tests of `[F19.AC6]`, `[F19.AC9]` and `[F02.AC1]` still proving their criteria; a failure of `cargo metadata` is reported with its message, not as a JSON error (EN-116).
2. A merge of `develop` into `main` makes the tag `vx.y.z` on the merge commit and the GitHub release, its notes the pull request's body checked by `release check`, with no release branch and no commit; `package.json` holds `0.0.0-development` (EN-117).
3. A release with no commit of the library nor the playground since the last tag is a patch with the notice of `docs/tooling/releases.md`; a release already published does nothing (EN-117).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-116](../stories/E17-F22-EN-116-ci-green-on-github-runners.md) | EN | The CI's tests green on GitHub's runner | draft |
| [EN-117](../stories/E17-F22-EN-117-release-from-the-merge-into-main.md) | EN | The release made by the merge into main, from the tags | draft |
| [AUD-024](../stories/E17-F22-AUD-024-audit-f22.md) | AUD | Audit F22 | draft |
| [VAL-023](../stories/E17-F22-VAL-023-validate-f22.md) | VAL | Validate F22 | draft |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-116 | AUD-024, VAL-023 |
| 2 | EN-117 | AUD-024, VAL-023 |
| 3 | EN-117 | AUD-024, VAL-023 |
