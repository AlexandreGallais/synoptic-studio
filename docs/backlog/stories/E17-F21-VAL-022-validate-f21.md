---
id: VAL-022
epic: E17
feature: F21
title: Validate F21
status: ready
points: 1
---

# E17 · F21 · VAL-022 — Validate F21

F21 is a tooling feature: the agent validates it on its demo page and checklist, and opens its pull request into `develop` without waiting; the Product Owner may try the lines that concern them (ADR-0072).

## Acceptance criteria

- Given the demo page, when it is read, then each criterion of F21 is shown with its evidence: the tool, its test, and its time before and after.
- Given the checklist of E17, when the agent runs its F21 lines, then each passes and is ticked with its date.
- Given the release notes of 0.2.0 drafted by our tool beside release-please's (EN-112), when the page shows them, then the Product Owner can compare both.
- Given the feature's criteria, when the validation ends, then the feature's pull request into `develop` is opened with the label `autonomous`.

## Tasks

One task = one commit, referenced as `VAL-022.Tn`.

- [ ] T1 — Demo page and checklist lines; the criteria checked (1 h)
