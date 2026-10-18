---
id: F28
epic: E17
title: One workflow per purpose
status: in-progress
---

# E17 · F28 — One workflow per purpose

**Benefit hypothesis**: the Product Owner opens GitHub's Actions tab and reads one workflow per purpose, each named for what it does and started by one condition at its top, the site's deployment apart from the release.

Decided by the Product Owner on 2026-10-18, after 0.3.0's merge into `main`: the workflows mix purposes (`release.yml` deploys the site, saves the Rust cache and releases, each job under its own condition), and their use is unclear (`CodeQL`, `CI`, `External pull requests`, `Story auto-merge`). A workflow file holds one purpose, and its trigger is its only condition; a workflow that does half of another's work is a file of its own. A tooling feature, validated by the agent (ADR-0072). Its spike SP-024 writes its stories and the other tooling features decided the same day.

## Acceptance criteria

1. Each workflow of `.github/workflows/` holds one purpose, named for it, its trigger the only condition that chooses whether it runs; the site's deployment and the release are two workflows (written by SP-024).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-024](../stories/E17-F28-SP-024-the-tooling-features-of-2026-10-18.md) | SP | The tooling features of 2026-10-18 | ready |
| [AUD-030](../stories/E17-F28-AUD-030-audit-f28.md) | AUD | Audit F28 | ready |
| [VAL-029](../stories/E17-F28-VAL-029-validate-f28.md) | VAL | Validate F28 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | SP-024 | AUD-030, VAL-029 |
