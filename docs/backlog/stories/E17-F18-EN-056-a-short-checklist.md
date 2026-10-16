---
id: EN-056
epic: E17
feature: F18
title: A short checklist, prepared by `val:prepare`
status: draft
points: 2
---

# E17 · F18 · EN-056 — A short checklist, prepared by `val:prepare`

Keep an epic's checklist short and tried, its panel beside the drawing, and prepare it with the tool (rows 17 and 28 of RET-003; I65, I66, I72, F5; F18.AC12).

## Acceptance criteria

- Given an epic's checklist, when a feature has more than 12 lines, then a test fails and names the feature.
- Given the checklist page, when it opens, then its panel sits to the right of the drawing and never over it.
- Given `npm run val:prepare -- <feature>`, when it runs, then it writes the feature's checklist lines and the address `?checklist=<epic>`, and no guided-steps file; `playground/guided-steps/epic-15/` is removed if the checklist replaces it.

## Tasks

One task = one commit, referenced as `EN-056.Tn`.

- [ ] T1 — The line count test and the panel's place (1 h)
- [ ] T2 — `val:prepare` writes checklist lines (1 h)
