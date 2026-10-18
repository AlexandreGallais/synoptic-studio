---
id: F22
epic: E17
title: One story at a time, the release by a button, its CI green
status: in-progress
---

# E17 · F22 — One story at a time, the release by a button, its CI green

**Benefit hypothesis**: the Product Owner merges a reviewed feature into `main` with a green CI, publishes 0.3.0 by one button, its notes the features' texts, without a release branch nor a commit of the version; a run works one story at a time and spends less of the week's usage.

Decided by the Product Owner on 2026-10-18 at E17's retrospective, then changed the same day during F22's run by [ADR-0086](../../adr/0086-one-story-at-a-time-into-develop-the-release-by-a-button.md), which supersedes ADR-0083 and adds EN-122 ([RET-004](../stories/E17-RET-004-retrospective-e17.md), P1 and P2; [ADR-0083](../../adr/0083-the-release-made-by-the-merge-into-main-from-a-tag.md)). A tooling feature: validated by the agent (ADR-0072). Its research was done by RET-004 (`REF-SEMANTIC-RELEASE-FAQ`, `REF-NPM-VERSION`, `REF-GH-WORKFLOW-DISPATCH`); four stories, so no checkpoint (ADR-0084). E17 closes at the publication of 0.3.0.

## Refinement (2026-10-18)

- The CI first: the three tests that fail on GitHub's runner and pass on the Mac hold on both, without weakening what they prove (`[F19.AC6]` under `GITHUB_ACTIONS`, `[F19.AC9]` when a crate download fails, `[F02.AC1]` on a slower machine).
- Then the release: option B of RET-004's P2, as ADR-0083 decides; 0.3.0 is its first release.

## Refinement (2026-10-18, during the run)

- One agent at a time, one story at a time; a feature branch per feature from `develop`; every merge a merge commit, story → feature and feature → `develop`.
- A review at each return of the Product Owner: a feature under way is finished first, then a `REV` from `develop` holds the review, the naming pass, the tool fixes, the method's changes and the docs, after which `develop` → `main` is opened; the push deploys the site only. Epics lose their closing (EN-123).
- The release is a button of GitHub's Actions tab: the version from the last tag, no commit; its notes the features' texts; `CHANGELOG.md` deleted. F22 works this way from EN-122 on.

## Acceptance criteria

1. The CI of a pull request into `main` passes on GitHub's runner with the tests of `[F19.AC6]`, `[F19.AC9]` and `[F02.AC1]` still proving their criteria; a failure of `cargo metadata` is reported with its message, not as a JSON error (EN-116).
2. The button of `release.yml` on `main` makes the tag `vx.y.z` on `main`'s head and the GitHub release, its notes the bodies of the pull requests into `main` merged since the last tag, checked by `release check`, with no release branch and no commit; a push to `main` deploys the site only; `package.json` holds `0.0.0-development` and `CHANGELOG.md` is gone (EN-117).
3. A release with no commit of the library nor the playground since the last tag is a patch with the notice of `docs/tooling/releases.md`; a release with no commit since the last tag does nothing (EN-117).
4. A story starts from its feature branch and merges into it by a merge commit, by GitHub's auto-merge in a run, and a feature into `develop` by a merge commit; a review asked mid-feature waits for that feature's end, then a `REV` from `develop` precedes `develop` → `main`; the run's rules describe one agent at a time (EN-122).
5. An epic has no closing: a `REV` at each return of the Product Owner holds the review, the naming of the changed code, the tool fixes and the method; a refinement `SP` opens an epic from `develop`, and a feature when questions remain; a feature over 4 stories is split (EN-123).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-122](../stories/E17-F22-EN-122-one-story-at-a-time-into-develop.md) | EN | One story at a time into develop | done |
| [EN-123](../stories/E17-F22-EN-123-epics-without-a-closing.md) | EN | Epics without a closing, a review at each return | done |
| [EN-116](../stories/E17-F22-EN-116-ci-green-on-github-runners.md) | EN | The CI's tests green on GitHub's runner | done |
| [EN-117](../stories/E17-F22-EN-117-release-from-the-merge-into-main.md) | EN | The release by a button, from the tags | done |
| [AUD-024](../stories/E17-F22-AUD-024-audit-f22.md) | AUD | Audit F22 | done |
| [VAL-023](../stories/E17-F22-VAL-023-validate-f22.md) | VAL | Validate F22 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-116 | AUD-024, VAL-023 |
| 2 | EN-117 | AUD-024, VAL-023 |
| 3 | EN-117 | AUD-024, VAL-023 |
| 4 | EN-122 | AUD-024, VAL-023 |
| 5 | EN-123 | AUD-024, VAL-023 |
