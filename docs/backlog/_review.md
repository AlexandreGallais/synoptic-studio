---
id: REV-000
epic: E00
title: Review E00 with the Product Owner
status: draft
points: 3
---

# E00 · REV-000 — Review E00 with the Product Owner

Show everything E00 delivered, in plain language, and test it together (ADR-0023, `/review` skill).

## Acceptance criteria

- Given the validated features of E00, when the review is prepared, then an epic report in `docs/guide/` explains each feature, its terms and its limits, with pictures and a user test crossing all features.
- Given the user test, when the Product Owner runs it in the playground, then each step's result is recorded, with their remarks in neutral words.
- Given the feedback, when the review ends, then every misunderstanding or change is a backlog item or a domain update, and the epic's closure criteria are checked.

## Tasks

One task = one commit, referenced as `REV-000.Tn`.

- [ ] T1 — Re-run the guided tests of every feature of E00 on `develop` (1 h)
- [ ] T2 — Write the epic report and its user test (3 h)
- [ ] T3 — Session with the Product Owner; record feedback and backlog changes (1 h)
