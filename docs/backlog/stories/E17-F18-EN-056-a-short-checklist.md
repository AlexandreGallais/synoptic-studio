---
id: EN-056
epic: E17
feature: F18
title: A short checklist, prepared by `val:prepare`
status: ready
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

## What changed

- **T1.** `readFeaturesOverLimit` names each feature of a checklist holding more than `CHECKLIST_LINE_LIMIT` lines, 12, with its count (`F05: 14 lines`); `checklist-line-limit.test.ts` runs it over every checklist the playground opens. The checklist leaves the drawing's stage: it is a column of the page, 24rem wide, between the drawing and the options, and the drawing takes the room left (the page is a row of columns, a hidden one giving its room back). Tested in happy-dom: the panel is outside the stage, right after it, in the flow, neither growing nor shrinking. Mutation-checked: the panel set back to `position: absolute` fails the place test; `>=` for `>` fails the count tests.

## What the Product Owner looks at

`npm run dev`, then `http://localhost:5173/?checklist=E15`: the checklist is a column right of the drawing, left of the options; no line covers the drawing, the toolbar or the guided test; the drawing narrows by the column's width. Without `?checklist=`, the drawing takes the whole room again.
