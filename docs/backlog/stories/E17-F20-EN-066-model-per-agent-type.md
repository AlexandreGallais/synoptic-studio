---
id: EN-066
epic: E17
feature: F20
title: A model chosen per agent type
status: draft
points: 2
---

# E17 · F20 · EN-066 — A model chosen per agent type

Choose the model of each agent type: Sonnet for logs, backlog, merges and step texts; Opus for mathematics, audits and research. Decided at the refinement of 2026-10-16 as a measure from E17; each change is shown to the Product Owner before it is applied.

## Acceptance criteria

- Given each agent type, when its definition is read, then it names its model and the reason.
- Given a run, when the session report is read, then the tokens and cost per model are shown and compared with a run on a single model.

## Tasks

One task = one commit, referenced as `EN-066.Tn`.

- [ ] T1 — Set the model of each agent type, shown to the Product Owner first (1 h)
- [ ] T2 — Measure the cost per task over a run and record it (2 h)
