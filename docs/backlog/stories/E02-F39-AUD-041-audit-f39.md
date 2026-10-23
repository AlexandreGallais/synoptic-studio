---
id: AUD-041
epic: E02
feature: F39
title: Audit F39
status: ready
points: 1
---

# E02 · F39 · AUD-041 — Audit F39

Audit F39 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F39's criteria, when the audit ends, then each is traced to a test titled `[F39.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-041.T1`.

## Findings

The `auditor` alone reviewed F39 read-only (ADR-0086) and found no wrong formula nor hand value; its findings were each reproduced before they were fixed, two of them settled by the Product Owner on 2026-10-23 (7, 8). F39's four criteria are traced to the `[F39.AC1]` test of `mount-playground.test.ts` (titled at this audit, finding 11), the `[F39.AC2]` tests of `landed-shift.test.ts`, `landed-shift-on-larger-axis.test.ts`, `offset-on-larger-axis.test.ts`, `shape-moved-by.test.ts`, `pick-areas-box.test.ts`, `shape-selection-after-press.test.ts` and `move-shapes.test.ts`, the `[F39.AC3]` tests of `shape-selection-after-press.test.ts` and `move-shapes.test.ts`, and the `[F39.AC4]` tests of `nudge-shift.test.ts` and `nudge-shapes.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | US-072's criterion "A and B selected, A pressed with Shift and dragged the same: both stay selected and move the same" had no test of Shift at the press only: `keeps A and B, A pressed with Shift among them` held Shift throughout, so that its move kept one axis | fixed: `[F39.AC3] keeps A and B, A pressed with Shift at the press only, both moved`, A at (41, 39.65064), B at (231, 39.65064); seen failing with a press with Shift taking A out of the selection, as Penpot's toggle would |
| 2 | Delete, Backspace, Ctrl + A and Escape pressed during a move were applied, then undone by the drag, which writes back at each move and at the release the drawing it kept at the press (`listenToSelectionKeys`, not guarded as the arrows are) | reproduced: B selected, A pressed and moved to (80.6, 59.4), Delete left one shape and none selected until the release put A back; fixed: no key of the selection while a press on a shape holds a move (`readPressedMove`, shared with the arrows' guard, in `playground/selection-keys/`, `playground/hover/` holding 14 modules); `interaction.md` §2 (Keys during a move), Q58, the guide page; `[F39.AC2] changes nothing on $key $init while A moves, A landed`, four cases, each seen failing with the guard removed |
| 3 | The keydown listeners pile up across a test file: each `openServedPage` mounts the page and adds its `document` listeners (`listenToSelectionKeys`, `listenToNudgeKeys`), never removed | reproduced: with the arrows' `event.defaultPrevented` check removed, `nudge-shapes.test.ts` failed 13 tests, a key nudging up to 26 times (A′'s X from 4.19873 to −255.80127 on Shift + Left), every earlier mount's listener nudging again; that check, which a slider's arrow needs too, keeps one nudge per key; the selection keys, applied again by each listener, give the same result. Not fixed here (ADR-0085): an incident card below |
| 4 | Penpot was cited for two behaviours its read code does not settle: a press on one of several moving them all, a press on an empty place inside the dashed box drawing a rectangle; and SP-028's option 5 (a) "as Penpot" for its second clause, A selected pressed with Shift staying selected, where Penpot's `d/toggle-selection` takes it out | read through the library: Penpot attaches `on-move-selected`, the move of the selection as it is, to the rectangle drawn around the selection, one shape or several, not to the shapes (`REF-PENPOT-VIEWPORT-SELECTION`, its row added, `REF-PENPOT-VIEWPORT-ACTIONS`'s status updated); which of it or a shape a press inside that rectangle reaches depends on the area the rectangle answers the pointer over, its fill `none`, not settled by the code read. Fixed: Penpot dropped from the two claims in note 0029 (§2, its table and its lists), `interaction.md` §2 (How the shapes selected move) and the guide page, Inkscape and Sketch still two agreeing sources; the note says Penpot's toggle takes out a shape selected pressed with Shift. SP-028, done, keeps its questions as asked; Q58 cites no editor |
| 5 | `nudgeShift` multiplied the direction by the step on each axis by hand, where `scale` names that calculation (ADR-0049) | fixed: `scale(direction, step)` |
| 6 | `drag-shape.test.ts` and `hover.test.ts` kept a local `firePointer`, a copy of `dispatchDrawingPointer` | fixed: both call `dispatchDrawingPointer`; the drawing reads no `buttons`, so a hover sent with the main button held changes nothing |
| 7 | With Shift held during a move, the axis Shift keeps still was landed too: A and B dragged across with Shift went by (31, −0.34936), A from y 20 to 19.65064 though dragged straight across | fixed by the Product Owner's decision of 2026-10-23: only the axis moved lands on integers, the axis kept still keeps its fraction, as Penpot leaves an axis kept by Shift unrounded (`REF-PENPOT-PIXEL-PRECISION`); `landedShiftOnLargerAxis(anchor, offset)` in `src/model/moving/`, used by `readMoveShift` at the release with Shift, its hand-computed tests and a property; A and B now go by (31, 0), A staying at y 20, B's straight box at (225.19873, −1.65064) (`[F39.AC2] keeps A and B still down with Shift held throughout, only across landed`); `interaction.md` §2 (The axis Shift keeps still), Q58, the guide page |
| 8 | A slider of the color picker (`#color-plane`, `#color-hue`, `#color-opacity`) was no field: Delete or Backspace there removed the shapes selected, Ctrl + A selected every shape | fixed by the Product Owner's decision of 2026-10-23: a slider counts as a field and keeps every key, `readKeyField` recognising `[role="slider"]`; `interaction.md` §1 and §2 (Ctrl + A and Escape, Delete, The arrows, how), Q51, Q52, Q59, the guide page; `select-shapes.test.ts`: Ctrl + A on `#color-hue`, Delete on `#color-hue` and Backspace on `#color-plane`, the three seen failing without the slider in the fields. The arrows' `event.defaultPrevented` check stays, for finding 3 |
| 9 | `ShapesMove`'s `anchor` said "landed on integers", while it is where the shapes start, the point a release lands | fixed: "at the press: the point a release lands on integers" |
| 10 | `[F02.AC5] keeps 100 000 and smaller values exactly`, F39's first criterion, had no limit of its own | fixed: a per-test timeout of 1 s, the test alone 10 ms (the median of 5 runs of its file), 100 times it; seen failing, "Test timed out in 1000ms", with the radius no longer typed 0 first |
| 11 | F39's first criterion was traced to no test titled `[F39.AC1]` | fixed: the test titled `[F02.AC5] [F39.AC1] keeps 100 000 and smaller values exactly` |
| 12 | Mutation: `start-selection-drag.ts:63`, `drag.isRectangle \|\| drag.move !== undefined` negated, survived: no test saw the canvas take the pointer for a move, happy-dom sending every event to the canvas anyway | fixed: `[F39.AC2] makes the canvas take the pointer pressed on A, so that the move goes on beyond it`, a spy on `setPointerCapture`; seen failing with the move left out of the condition |
| 13 | Mutations of a count: `pick-areas-box.ts:20` and `read-selection-move.ts:28`, `> 0` → `> 1` | no action: a shape drawn offers three corners or more, a text drawn four, a text never measured none, so that no area holds one corner alone |

Mutations (A4), `npm run mutate` against each file's related tests: the 5 modules of `src/model/moving/`, `shapeSelectionAfterPress` and `pickAreasBox`, 18 caught and 1 survivor (finding 13); the 14 playground modules of the move and the keys (`playground/selection-drag/`, `playground/selection-keys/`, `readStartedMove`), 31 caught and 2 survivors (findings 12 and 13). By hand, each criterion once: the test of F39.AC1 without its radius typed 0 first (`[F39.AC1]`, "Test timed out in 1000ms"); `landedShift` by `Math.floor` rather than `nearestInteger` (`[F39.AC2]`, 28 tests fail across `src/model/moving/`, `move-shapes.test.ts` and `nudge-shapes.test.ts`); `shapeSelectionAfterPress` selecting the shape pressed alone whatever Shift (`[F39.AC3]`, 3 fail); `FURTHER_STEP` 1 rather than 10 (`[F39.AC4]`, 11 fail). And the new code: `landedShiftOnLargerAxis` landing both axes (5 fail, finding 7); Shift at the press taking a shape selected out (5 fail, the test of finding 1 among them); the selection keys unguarded during a move (4 fail, finding 2); the sliders out of the fields (3 fail, finding 8); the arrows unguarded during a move (1 fails); the move left out of the pointer capture (1 fails, finding 12).

Properties (ADR-0022): `landedShiftOnLargerAxis` gets one, that it moves on one axis alone, landing the anchor there within half a unit; F39's other `domain` functions had theirs, `pickAreasBox` a rule of one composition (`pickAreaBox`, `boundingBox`), tested by examples.

Breadth (step 7): `check:all` passed with the push; the weekly link check was not read again for this audit; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F39 did.

Next feature (G): E02 goes on with undo and redo and a view that zooms and pans. Its stories need what one step of the history holds (Q7: a drag at its release, a value typed, a key held, which Penpot gathers into one change ended 1 s after the last key, `REF-PENPOT-TRANSFORMS`), how Figma, Inkscape and Penpot zoom about the pointer and pan, and the area shown, which SP-028's answer 8 asks for so that the helps are drawn only within it; none of the history and zoom pages is in `docs/references.md` yet.

Evolvability (H):

- Size: `playground/hover/` and `playground/selection/` hold 14 modules each, one under the signal of `files.md`; `src/model/picking/` and `src/model/selection/` 13; `src/model/moving/` 5, `playground/selection-keys/` 4.
- Tools: no "considered for later" row is triggered by F39.
- Fitness functions: finding 3 is a test harness that leaves listeners behind; a check that a test file's mounts remove what they add, or a mount that replaces its listeners, would have held it (the incident card).
- Patterns: a test helper copied in three files (finding 6), a calculation by hand where a named function exists (finding 5); the naming pass at each review catches them.
- Agent configuration: nothing.
- Process: a source cited for a behaviour its code read does not settle (finding 4); a spike's note that cites code says where a handler is attached before it cites the handler's effect.

## For the main thread

- For the done commit's footer: the rule docs changed in this task, `interaction.md` §1 and §2, the Q rows Q51, Q52, Q58 and Q59 of `docs/domain/README.md`, and F39's guide page; F39 has no `VAL` (VAL-040 dropped, a feature ending with its audit).
- The feature file F39 is the main thread's (ADR-0065): its criterion 2 may add "with Shift held, only the axis moved lands" (finding 7).

### Incidents

#### 2026-10-23 — `openServedPage`, mounting the playground once per test of a file

- Used: each test of a playground file mounts the page again by `openServedPage`, in the one `document` happy-dom keeps for the file.
- Failed: each mount adds its `keydown` listeners to `document` (`listenToSelectionKeys`, `listenToNudgeKeys`), never removed, so that a file's n-th test has n listeners for each key; with the arrows' `event.defaultPrevented` check removed, a key nudged up to 26 times in `nudge-shapes.test.ts` (AUD-041, finding 3).
- Instead: the check kept, which stops every listener after the first; the listeners left piling up, a key of the selection applied once per mount with the same result.
