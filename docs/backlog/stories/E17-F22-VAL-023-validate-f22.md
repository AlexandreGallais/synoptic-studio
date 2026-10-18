---
id: VAL-023
epic: E17
feature: F22
title: Validate F22
status: ready
points: 1
---

# E17 · F22 · VAL-023 — Validate F22

F22 is a tooling feature: the agent validates it on its demo page and checklist, and opens its pull request into `develop` without waiting (ADR-0072); then the Product Owner merges `develop` into `main` and publishes 0.3.0 by the button, the first release made by ADR-0086's flow, and E17 closes.

Two halves of F22's criteria no test of the repository can prove, left to this validation by AUD-024 (F-05): AC1's CI run on GitHub's runner, the green CI of `develop` → `main` its evidence; and AC4's rules on the docs, the run skill, the worker brief, `project-rules`, `AGENTS.md` and `docs/tooling/git-workflow.md` read again and shown to describe one agent at a time and a merge commit for every merge.

## Acceptance criteria

- Given the demo page, when it is read, then each criterion of F22 is shown with its evidence.
- Given the pull request `develop` → `main` of 0.3.0, when its CI ends, then it is green.
- Given the feature's criteria, when the validation ends, then the feature's pull request into `develop` is opened with the label `autonomous`.

## Tasks

One task = one commit, referenced as `VAL-023.Tn`.

- [ ] T1 — Each criterion of F22 proven again, the checklist lines run, the demo page written (1 h)
