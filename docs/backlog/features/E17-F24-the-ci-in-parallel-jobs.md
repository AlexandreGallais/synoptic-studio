---
id: F24
epic: E17
title: The CI in parallel jobs, its tests shown as they run
status: in-progress
---

# E17 · F24 — The CI in parallel jobs, its tests shown as they run

**Benefit hypothesis**: the CI of a pull request into `main` shows one check per lane, each passing or failing on its own, the tests file by file with their failures annotated on the pull request, and the Rust lane reuses a cache, so that the first merge into `main` (0.3.0) sees a green CI without the 26 min the single job's Rust lane took without a cache.

Decided by the Product Owner on 2026-10-18 at REV-006, while the CI of #370 ran: on GitHub's runner the single job took 139 s for `lint`, 265 s for `docs:build`, 326 s for `test` and over 26 min for `check:rust` compiled without a cache (cancelled after 26 min 07 s), none visible until the end. A tooling feature, validated by the agent (ADR-0072), done before 0.3.0.

## Acceptance criteria

1. The CI of a pull request into `main` runs its lanes as parallel jobs, one job per lane of `check:all`, each its own check on the pull request: `Lint and types`, `TypeScript tests`, `Docs, playground and build`, `Format, secrets and dependencies`, `Rust`; a lane failing leaves the others' results, and the main protection names the checks it requires (EN-124).
2. The TypeScript tests report file by file as they run, and a failing test is annotated on the pull request at its line (EN-124).
3. The Rust lane restores a cache saved by a push to `develop` or `main`, so that a pull request into `main` compiles only what changed; its time with a warm cache is measured and written (EN-125).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-124](../stories/E17-F24-EN-124-the-ci-in-parallel-jobs.md) | EN | The CI in parallel jobs, its tests shown as they run | done |
| [EN-125](../stories/E17-F24-EN-125-the-rust-cache-from-develop.md) | EN | The Rust cache saved from develop | done |
| [AUD-026](../stories/E17-F24-AUD-026-audit-f24.md) | AUD | Audit F24 | done |
| [VAL-025](../stories/E17-F24-VAL-025-validate-f24.md) | VAL | Validate F24 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-124 | AUD-026, VAL-025 |
| 2 | EN-124 | AUD-026, VAL-025 |
| 3 | EN-125 | AUD-026, VAL-025 |
