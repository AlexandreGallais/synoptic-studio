---
id: VAL-024
epic: E02
feature: F23
title: Validate F23
status: draft
points: 1
---

# E02 · F23 · VAL-024 — Validate F23

F23 is a tooling feature: the agent validates it on its demo page and checklist, and opens its pull request into `develop` without waiting (ADR-0072).

## Acceptance criteria

- Given the demo page, when it is read, then each criterion of F23 is shown with its evidence and each card's fix.
- Given the feature's criteria, when the validation ends, then the feature's pull request into `develop` is opened with the label `autonomous`.

## Tasks

One task = one commit, referenced as `VAL-024.Tn`.

- [ ] T1 — Each criterion of F23 proven again, the checklist lines run, the demo page written (1 h)
