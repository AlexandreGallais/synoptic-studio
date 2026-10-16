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

- [x] T1 — The line count test and the panel's place (1 h)
- [x] T2 — `val:prepare` writes checklist lines (1 h)

## What changed

- **T1.** `readFeaturesOverLimit` names each feature of a checklist holding more than `CHECKLIST_LINE_LIMIT` lines, 12, with its count (`F05: 14 lines`); `checklist-line-limit.test.ts` runs it over every checklist the playground opens. The checklist leaves the drawing's stage: it is a column of the page, 24rem wide, between the drawing and the options, and the drawing takes the room left (the page is a row of columns, a hidden one giving its room back). Tested in happy-dom: the panel is outside the stage, right after it, in the flow, neither growing nor shrinking. Mutation-checked: the panel set back to `position: absolute` fails the place test; `>=` for `>` fails the count tests.

- **T2.** `npm run val:prepare -- <feature>` is a std-only Rust crate, `tools/val-prepare/`, checked by `check:rust`. From the feature's numbered acceptance criteria it writes `playground/checklists/epic-<n>-<feature>-lines.ts`, one line per criterion (what to try: the criterion, tagged `F18.AC12`; what to see: a line to write), adds them to the epic's checklist, made if missing, registers it in `checklists.ts` and the folder's index, and prints `http://localhost:5173/?checklist=<epic>`. It writes no guided-steps file; it refuses a feature without numbered criteria (F19 and F20 today) or whose lines already exist, and warns above 12 criteria. Tried on this repository: F18 then F19 in E17, `npm run fix` and `tsc` clean, the 12-line test then failing on `F18: 20 lines` as it should; the files removed afterwards. Mutation-checked: the warning at 12 and the address built from the feature fail the tests.
- `playground/guided-steps/epic-15/` is kept: `?guide=epic` shows its steps (`guided-steps/lists/guided-steps.ts` imports F05's, F14's, F15's and F16's), and the Product Owner's decision of 2026-10-15 keeps the guided steps written up to E15 there.

## What the Product Owner looks at

`npm run dev`, then `http://localhost:5173/?checklist=E15`: the checklist is a column right of the drawing, left of the options; no line covers the drawing, the toolbar or the guided test; the drawing narrows by the column's width. Without `?checklist=`, the drawing takes the whole room again.

## For the main thread

- **Checklist edit needed before this branch can pass** (shared file): E15's F05 holds 14 lines, so `checklist-line-limit.test.ts` fails on `F05: 14 lines` until `playground/checklists/epic-15-text-lines.ts` merges "Weight 100…" with "Italic on, then off", and "Size 48, then 16" with "Size 0, then back to 32" (agreed with the main thread, applied on the feature branch).
- `playground/guided-steps/epic-15/` is **not** removed: `?guide=epic` uses it (Product Owner, 2026-10-15). The criterion's "removed if the checklist replaces it" does not apply.
- `val:prepare` writes the checklist files (`epic-*`, `checklists.ts`, `index.ts`): it is meant for the main thread at a `VAL`, the shared-file rule refusing it on a worker's branch.
- F18's own `VAL`: its 20 criteria give 20 draft lines; they must be merged to 12 at most, which the tool says and the test enforces.
- Technical choices made by the worker: the page is now a row of flex columns (drawing, checklist, options, pipeline) instead of a grid; a hidden column gives its room to the drawing, which replaces the `body:has(#output[hidden])` rule. The checklist column is 24rem wide.

### Journal

- EN-056: the checklist became a column beside the drawing, a test caps each feature at 12 lines (E15's F05 cut from 14 to 12), and `val:prepare` (Rust) drafts a feature's lines from its criteria; the guided steps of E15 stay for `?guide=epic`.
