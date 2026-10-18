---
id: F28
epic: E17
title: One workflow per purpose
status: in-progress
---

# E17 · F28 — One workflow per purpose

**Benefit hypothesis**: the Product Owner opens GitHub's Actions tab and reads one workflow per purpose, each named for what it does and started by one condition at its top, the site's deployment apart from the release, and the CI of `develop` → `main` holding nothing its checks do not need.

Decided by the Product Owner on 2026-10-18, after 0.3.0's merge into `main`: the workflows mix purposes (`release.yml` deploys the site, saves the Rust cache and releases, each job under its own condition), and their use is unclear (`CodeQL`, `CI`, `External pull requests`, `Story auto-merge`). A workflow file holds one purpose, and its trigger is its only condition; a workflow that does half of another's work is a file of its own (ADR-0088). A tooling feature, validated by the agent (ADR-0072). Its spike SP-024 read each workflow's use and wrote these stories with the other tooling features decided the same day ([research note 0026](../../research/notes/0026-the-tooling-features-of-2026-10-18.md)).

## Acceptance criteria

1. Each workflow of `.github/workflows/` holds one purpose, named for it in its file name, its `name:` and its first comment; its trigger is the only condition that chooses whether it runs and which of its jobs run, no `if:` reading the event or a branch (ADR-0088, decision 2), `story-automerge.yml` left to F34 (EN-132, EN-133).
2. The site's deployment, on a push to `main`, and the release, by its button alone, are two workflows; the release refuses a branch other than `main` by its tool; the Rust caches are saved by a workflow of their own on a push to `develop` or `main`, the site restoring them (EN-132, AUD-030).
3. The CI of the pull requests into `main` runs only on them, and each of its steps is needed by a script of its job's lane; the time of each job before is written in `docs/tooling/ci.md` by EN-133, and the time after is measured at the next review's CI into `main` and written there by that review (EN-133, AUD-030).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-024](../stories/E17-F28-SP-024-the-tooling-features-of-2026-10-18.md) | SP | The tooling features of 2026-10-18 | done |
| [EN-132](../stories/E17-F28-EN-132-the-site-the-release-and-the-caches-apart.md) | EN | The site, the release and the Rust caches apart | done |
| [EN-133](../stories/E17-F28-EN-133-the-ci-into-main-its-checks-only.md) | EN | The CI into main, its checks only | done |
| [AUD-030](../stories/E17-F28-AUD-030-audit-f28.md) | AUD | Audit F28 | ready |
| [VAL-029](../stories/E17-F28-VAL-029-validate-f28.md) | VAL | Validate F28 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-132, EN-133 | AUD-030, VAL-029 |
| 2 | EN-132 | AUD-030, VAL-029 |
| 3 | EN-133 | AUD-030, VAL-029 |
