---
id: AUD-040
epic: E02
feature: F38
title: Audit F38
status: done
points: 1
---

# E02 · F38 · AUD-040 — Audit F38

Audit F38 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F38's criteria, when the audit ends, then each is traced to a test titled `[F38.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-040.T1`.

## Findings

The `auditor` alone reviewed F38 read-only (ADR-0086), its probes run against the library's sources and the page in happy-dom: one error in the panel of one shape, a rule the Product Owner settled on 2026-10-23 for a text's X and Y, and three duplicates or couplings, each reproduced before it was fixed; the mutations found one test missing (6) and five equivalent mutations (7, 8). F38's two criteria are traced to the `[F38.AC1]` tests of `common-panel-values.test.ts`, `shape-panel-values.test.ts`, `text-panel-values.test.ts` and `write-several-values.test.ts`, and the `[F38.AC2]` tests of `parse-panel-value.test.ts`, `panel-value-after-edit.test.ts`, `shape-after-panel-edit.test.ts`, `text-after-panel-edit.test.ts` and `write-several-values.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | A shape 0 × 0 alone showed X `Infinity`: its contour has no piece, and `readStraightStart` and `showShape` took the box of no point, its minima at `+∞`; 50 typed in X then moved it by 50 − ∞ and stored x `−Infinity`, the shape refused for good, a width typed afterwards refused too (reproduced: 0 × 0 at (10, 20) showed X `Infinity`, then 50 in X stored `−Infinity`). The dashed box around several shapes left such a shape out the same way, its filter keeping only contours with a piece (AUD-039's finding 13 assumed a contour always has 3) | fixed: `shapeStraightBox(corners)`, the box of the contour as drawn, or of the corners when it has no piece, used by `shapePanelValues`, `readStraightStart` (now after the validity check) and `showShape`; it sits in `src/geometry/curve-contour/`, not `placement/`, where it would close an import cycle through `curve-fillet` (`import/no-cycle`); `pickAreaBox(area)` in `src/model/picking/`, used by `isPickAreaEnclosed` and the dashed box, which now holds a shape 0 × 0 by its point; `DERIV-arc-bounding-box` step 4 and a check; `move-point-shape.test.ts` (0 × 0 at (10, 20) shows X 10, Y 20, and 50 in X puts it at x 50) and a 0 × 0 shape in the dashed box of `select-shapes.test.ts`, the four tests seen failing with the fix undone |
| 2 | One text alone showed a turned text by its turned top-left corner (US-054, provisional), while F38 shows it among several by its straight box: ADR-0057's box 100 × 20 at (47, 61) turned by 23° read X 54.88207 alone and 47.06745 among several | fixed by the Product Owner's decision of 2026-10-23: X and Y of a shape or a text are the top-left of its straight box, one text included, superseding US-054's rule; X and Y typed move the text by the difference (`straightBoxShift`), a growing turned text still keeping its turned top-left corner (ADR-0057); `readStraightStart` in `playground/text-placement/`, used by `writeTextPosition`, `readTextPosition` and `loadPickedText`, `choosePlacement` rebuilt on it; no accepted ADR states the turned-corner display (ADR-0057 decides the corner kept, which stands), so no ADR is superseded; `shapes.md` (Text, Scripts and turning), `interaction.md` §2, Q57, F38's guide page, Figma's help on a rotated layer written there as a fact (`REF-FIGMA-ADJUST`, its quote added, read again on 2026-10-23); the box 100 × 20 turned by 23° shows X 47.06745, Y 42.2584 alone, picked and among several (`write-several-values.test.ts`); the tests of US-038, US-054, US-055 and the guided steps of F05 and F14 computed again by hand |
| 3 | `commonValue` of `common-panel-values.ts` and `commonRadius` wrote the same rule twice, one within `EPSILON`, one exact | fixed: `commonValue(values, tolerance)` in `src/math/precision/`, `commonPanelValues` within `EPSILON`, `commonRadius` within 0, which keeps `===`'s results for valid radii (`NaN` common to none, `−0` equal to `0`) |
| 4 | A text's box side was capped at the interface's `MAX_SIZE`, equal to `TEXT_RANGES.boxSide.last` by chance | fixed: `textAfterPanelEdit` caps a box's side at the smaller of the two; capped at 100 000 for an interface capping at 200 000, at 500 for one capping at 500 |
| 5 | The shift "the value typed minus where the straight box starts" was written in `shapeAfterPanelEdit`, `textAfterPanelEdit` and `readTypedShift` (ADR-0049) | fixed: `straightBoxShift(start, target)` in `src/model/placement/`, used by the three and by the text's `choosePlacement` |
| 6 | Mutation: `apply-several-value.ts:39`, `block === undefined` → `!==`, survived: no test of the page typed X or Y for a text drawn, a size leaving a text fitting its content before its block is read | fixed: `moves a text drawn by 50 in X, a text never measured left as it is`, the rectangle and the text at x 50 |
| 7 | Mutations returning `undefined` where `undefined` is returned (`parse-panel-value.ts:23`, `common-value.ts:21`, `listen-to-position.ts:25`) survived | no action: the code is the same |
| 8 | Mutations of a count: `shape-straight-box.ts:22` and `create-selection-elements.ts:25`, `> 0` → `> 1`; `write-several-values.ts:112`, `values.length > 0` → `> 1` | no action: a closed contour with length has at least two pieces, a side out and one back, and a shape or a text drawn four corners or more; with one value left, every value is common and no field reads Mixed |

Mutations (A4), `npm run mutate` against each file's own test file: the 9 modules of `model/panel-values/`, `parsePanelValue`, `shapeStraightBox`, `pickAreaBox`, `commonValue`, `commonRadius` and `straightBoxShift`, 32 caught and 3 survivors (findings 7 and 8); the panel's 7 playground modules against `write-several-values.test.ts`, 11 caught and 2 survivors (findings 6 and 8); the text's placement, 4 modules against the tests of US-038 and US-054, 10 caught; `listenToPosition` and `showShape` against `move-point-shape.test.ts` and `turn-shape.test.ts`, 3 caught and 4 survivors, one a finding 7, three in F01's effective radius and JSON indent, outside F38 (the radius's condition negated fails 63 of its related tests; the other two were not run against theirs); the dashed box against `select-shapes.test.ts`, 2 caught and 1 survivor (finding 8). `load-picked-text.ts` writing the turned corner again leaves its tests passing: the redraw that follows writes X and Y from `writeTextPosition`, so only their agreement with the text kept matters. By hand, each criterion once: `commonPanelValues` within an infinite tolerance (`[F38.AC1]`, 12 tests fail, the page's Mixed among them); `panelValueAfterEdit` ignoring the sign (`[F38.AC2]`, 16 fail, the page's `+50` and `-20` among them); `textAfterPanelEdit` setting a text fitting its content (`[F38.AC2]`, 3 fail, the page's among them); and the new code: `chooseAxisTarget` taking every field as typed (13 fail), `chooseBasePosition` never keeping the turned corner (the tests of US-038 fail), the filter of the dashed box on pieces again (1 fails), `shapeStraightBox` without its fallback (4 fail), `readEditedShape` with its test of finding 6 (1 fails).

Properties (ADR-0022): `shapeStraightBox`, `commonValue` and `straightBoxShift` get theirs; the other functions of F38 had theirs or are `domain` functions of one rule each.

Breadth (step 7): `check:all` passed, and `golden:check` with the push, `commonRadius` and the straight box of a shape with a contour returning the same floats; the weekly link check was not read again for this audit; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F38 did, but `npm run mutate`, which copies only the tracked files, skips every file whose tests import a module not yet added to Git (worked around by `git add` before running it).

Next feature (G): E02 goes on with moving several shapes by the mouse and the keys on integers, undo and redo, and a view that zooms and pans. Its stories need how Figma, Inkscape and Penpot move a selection by a drag and by the arrow keys (`REF-FIGMA-ADJUST` holds the nudges of 1 and 10), where a dragged turned shape lands on integers (the x and y of its straight box, ADR-0035), what a history step holds (one per value typed, per drag, per key held) and how a view zooms about the pointer; none of the history and zoom pages is in `docs/references.md` yet.

Evolvability (H):

- Size: `playground/selection/` holds 15 modules, at the signal of `files.md`: its next module asks for a split, the panel of several shapes (`*-several-*`) a candidate domain; `src/geometry/curve-contour/` 11, `src/model/picking/` 12, `src/model/placement/` 9.
- Tools: no "considered for later" row is triggered by F38.
- Fitness functions: finding 1 is an empty box read as a number; a property of `boundingBox`'s callers drawing a shape 0 × 0 would have held it, the new properties now draw one.
- Patterns: the duplicates were a common value (finding 3) and a shift (finding 5); the naming pass at each review catches them.
- Agent configuration: nothing.
- Process: a provisional rule (US-054) left beside a feature showing the same value another way (finding 2); a feature that shows a value an older feature shows lists that feature's rule at its refinement.

## For the main thread

- The feature file F17 is the main thread's (ADR-0065): its criterion 12 says "X and Y of a turned text show its turned top-left corner", superseded on 2026-10-23 (finding 2); its tests keep the tag `[F17.AC12]` and now prove the straight box: "X and Y of a turned text show the top-left of its straight box, updated when it turns; a value typed moves its straight box there, its angle and size kept", for instance.
- `docs/guide/e15-place-text.md` step 9 ("X and Y show its turned top-left corner") and `f17-text-corrections.md` record validations as they were run, and are left as they are.
- For the done commit's footer: the rule docs changed in this task, `shapes.md` (§1 Rotation of shapes; Text, Scripts and turning), `interaction.md` §2, the Q row Q57 of `docs/domain/README.md`, and F38's guide page; VAL-039 is dropped, its decisions going to the review's summary.
