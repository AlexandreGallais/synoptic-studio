---
id: EN-059
epic: E17
feature: F18
title: The done commit checks the docs a rule touches
status: ready
points: 1
---

# E17 · F18 · EN-059 — The done commit checks the docs a rule touches

Keep the Q rows, the decisions to confirm and the guide pages up to date when a story changes a rule (row 30 of RET-003; I43, I49; F18.AC16).

## Acceptance criteria

- Given a story that changes a rule in `docs/domain/`, when its done commit is made, then a check asks for the Q row of `docs/domain/README.md`, the VAL's decisions to confirm and the guide page to be in the same change, or a line saying none is affected.

## Tasks

One task = one commit, referenced as `EN-059.Tn`.

- [ ] T1 — The check in `story:finish` or a hook; its test (2 h)
