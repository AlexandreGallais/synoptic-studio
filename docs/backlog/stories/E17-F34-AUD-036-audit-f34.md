---
id: AUD-036
epic: E17
feature: F34
title: Audit F34
status: done
points: 1
---

# E17 · F34 · AUD-036 — Audit F34

Audit F34 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F34's criteria, when the audit ends, then each is traced to a test titled `[F34.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-036.T1`.

## Findings

The `auditor` alone reviewed F34 read-only (ADR-0086): both auto-merge workflows merge only what the old one did, never into `main`, one run per pull request; twelve mutations of the workflows each failed a `[F34.AC1]` test; `main`'s required checks are still reported. The main thread fixed the docs.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | The run skill named `story-automerge.yml` for every merge, a `REV` into `develop` now merged by `feature-automerge.yml` | fixed: the workflow of its base named in step 8 and its "Not merged" line |
| 2 | The spike skill named `story-automerge.yml` for `develop` | fixed: `feature-automerge.yml` |
| 3 | `REF-ADOBE-LIVE-SHAPES` refuses automated clients (HTTP 403), its two sentences no longer checkable | the row says so, with its date; another source is a choice for the Product Owner at the next review |
| 4 | E17's table said F34's questions waited | fixed |
| 5 | Rebase merging is allowed in the repository's settings, against `ci.md`'s settings | for the Product Owner at the next review: uncheck it, or reword the line; the auto-merges always pass `--merge` |
| 6 | Whether "External pull requests" leaves the Actions tab once its file is gone is unchecked | read at the review after the next merge into `main` |
| 7 | CodeQL's one job departs from GitHub's one language per analysis | `ci.md` says so and why |
