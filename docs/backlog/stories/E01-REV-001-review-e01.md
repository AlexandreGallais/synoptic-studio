---
id: REV-001
epic: E01
title: Review E01 with the Product Owner
status: ready
points: 3
---

# E01 · REV-001 — Review E01 with the Product Owner

Show everything E01 delivered, in plain language, and test it together (ADR-0023, `/review` skill).

## Acceptance criteria

- Given the validated features of E01, when the review is prepared, then an epic report in `docs/guide/` explains each feature, its terms and its limits, with pictures and a user test crossing all features.
- Given the user test, when the Product Owner runs it in the playground, then each step's result is recorded, with their feedback quoted.
- Given the feedback, when the review ends, then every misunderstanding or change is a backlog item or a domain update, and the epic's closure criteria are checked.

## Tasks

One task = one commit, referenced as `REV-001.Tn`.

- [ ] T1 — Re-run the guided tests of every feature of E01 on `main` (1 h)
- [ ] T2 — Write the epic report and its user test (3 h)
- [ ] T3 — Session with the Product Owner; record feedback and backlog changes (1 h)
