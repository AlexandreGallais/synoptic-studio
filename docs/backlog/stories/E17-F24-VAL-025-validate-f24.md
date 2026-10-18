---
id: VAL-025
epic: E17
feature: F24
title: Validate F24
status: ready
points: 1
---

# E17 · F24 · VAL-025 — Validate F24

F24 is a tooling feature: the agent validates it and opens its pull request into `develop` without waiting (ADR-0072); its proof is the CI of `develop` → `main` (#370) shown job by job.

## Acceptance criteria

- Given the CI of #370 after F24's merge into `develop`, when it ends, then each job is a check of its own and the run's times are written in the demo page.

## Tasks

- [ ] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-025.T1`.
