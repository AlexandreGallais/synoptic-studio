---
id: VAL-025
epic: E17
feature: F24
title: Validate F24
status: done
points: 1
---

# E17 · F24 · VAL-025 — Validate F24

F24 is a tooling feature: the agent validates it and opens its pull request into `develop` without waiting (ADR-0072); its proof is the CI of `develop` → `main` (#370) shown job by job.

## Acceptance criteria

- Given the CI of #370 after F24's merge into `develop`, when it ends, then each job is a check of its own and the run's times are written in the demo page.

## Tasks

- [x] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-025.T1`.

## Result (2026-10-18)

Validated by the agent (ADR-0072), lightly as the Product Owner chose, the proof being #370's own CI: F24's tagged tests pass (3 Rust `f24_ac…`, 6 TypeScript `[F24.ACn]`); the demo page `docs/guide/f24-the-ci-in-parallel-jobs.md`. No checklist line: nothing is drawn, the checks of #370 are what to look at. Pending until the next review: the five checks seen on #370, and criterion 3's warm time, measured when #370 runs on `develop`'s cache.
