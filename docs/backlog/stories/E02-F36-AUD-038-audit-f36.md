---
id: AUD-038
epic: E02
feature: F36
title: Audit F36
status: done
points: 1
---

# E02 · F36 · AUD-038 — Audit F36

Audit F36 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F36's criteria, when the audit ends, then each is traced to a test titled `[F36.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-038.T1`.

## Findings

The `auditor` alone reviewed F36 read-only (ADR-0086): no correctness error in the hit test (`shapeAtPoint` and the painted area), the dragged box nor the split, and nine findings, each reproduced before it was fixed. F36's three criteria are traced to the `[F36.AC1]` tests of `dragged-box.test.ts` and `drag-shape.test.ts`, the `[F36.AC2]` tests of `shape-at-point.test.ts` and `hover.test.ts`, the `[F36.AC3]` tests of `split-drawing.test.ts`, `select-shape.test.ts` and `load-picked-shape.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | `pieceSamples` cut a cubic by the length of its control polygon, which bounds its average chord, not its longest: (0, 0), (200, 0), (100, 100), (100, 100) at a step of 2 gave 171 steps and a chord of 3.478 near t = 0, where the speed is 600 | fixed: B′ is the quadratic Bézier of the weights 3(Pᵢ₊₁ − Pᵢ), inside their hull (`REF-POMAX-BEZIER`), so n = max(⌈3 · max(\|P₀P₁\|, \|P₁P₂\|, \|P₂P₃\|) / s⌉, 1) bounds every chord by `s`; the example (300 steps, the longest chord 1.990) and the property "cuts a cubic into chords of at most `step`", both seen failing with a factor 2 |
| 2 | `REF-SVG2-PAINT`'s row and the TSDoc of `segmentBand` and `contourBand` did not say where the half width comes from: §13.5.7's sentence on a dash says "within distance stroke-width", where half of it is meant | fixed: the half width is stroke-width / 2, the radius of the round join's sector "of diameter stroke-width", written in the row's "Used for" and both `halfWidth` parameters |
| 3 | `REF-POMAX-BEZIER`, read again on 2026-10-22, kept its status date of 2026-10-14; `DERIV-painted-area` cites `REF-SVG2-FILL-RULE`, `REF-MATHWORLD-PERP-DOT`, `REF-MATHWORLD-BISECTION` and `REF-WIKI-QUADRATIC` without being in their "Used for" | fixed: `[verified] 2026-10-22`, with the hull's sentence read for finding 1; the four rows name the derivation |
| 4 | `DERIV-painted-area` step 3 takes every fillet under half a turn from `DERIV-fillet-arc` alone, while a fillet beside a curved side comes from `DERIV-fillet-beside-curve` | fixed: step 3 and the sources cite its step 8, "by less than π while (i) holds" |
| 5 | The guide page and its row in `docs/guide/README.md` kept the feature's first title, "Draw, pick and remove shapes", Delete having moved to F37 | fixed: "F36 — Draw and pick shapes"; the row names the hover and the click too |
| 6 | EN-143's tasks T1 and T2 were not ticked, the story done | fixed |
| 7 | Screen pixels turned into user units by `/ VIEW_SCALE` written three times (`show-hover.ts`, `select-shape-at.ts`, `create-outline-element.ts`, ADR-0049) | fixed: `screenToUser(pixels, scale)` in `src/model/drawing/`, beside `PICK_TOLERANCE`, a `domain` function of the model since the rule is `interaction.md` §2's; the playground passes `VIEW_SCALE` |
| 8 | A cubic's turning parameters on an axis, `quadraticRoots(derivativeCoefficients(…))` kept between 0 and 1, written in both `cubic-y-breaks.ts` and `placement/cubic-extremes.ts` | fixed: `cubicAxisTurns(weights)` in `src/geometry/placement/`, the same operations in the same order, so that no float changes (`golden:check`); its examples and a property |
| 9 | The test helper `readModel`, its assertion and its exceptions row copied in six test files (five of F36's count, and `text-box-fields.test.ts`) | fixed: `readModel` in `playground/user-actions/`, one exceptions row; `readShapeModel` beside it narrows to a shape by `isDrawnText`, with no assertion; `playground/user-actions` is at 33 modules in `files.md` |
| 10 | Mutation: `segment-distance.ts:24`, `projected <= 0` → `<= 1`, survived: no example had a foot just past the start | fixed: `measures to the foot when it falls just past the start`, (0, 0) to (2, 1) and (1, −1), 3 / √5 |
| 11 | Mutation: `is-in-circular-segment.ts:22`, `<` → `<=` on the radius, survived: no point exactly on the arc | fixed: `fails on the arc itself: (8, 4), 20 from the centre exactly` |
| 12 | Mutation: `monotone-crossing.ts:39`, `>` → `>=`, survived: no crossing exactly at the point | fixed: `counts 0 for a crossing at the point itself, not strictly right of it`, a straight cubic side on x = 0 |
| 13 | Mutation by hand: `cubic-level-parameter.ts:27`, `< 0` → `<= 0`, survived (the 86 tests of `painted-area/`) | no action: the predicate changes only where y(t) equals the level, the root itself, y being monotone and not constant on the part (`isLevelCrossed` refuses a flat one), so the bisection's bracket closes on the same value within one unit in the last place, as `.claude/rules/library.md` says of a bisection; measure zero |
| 14 | Mutation: `contour-band.ts:33`, the chord's end `index + 1` → `index − 1`, survived: the same rectangles, turned the other way, the first one from the piece's start to its end; only the count of the bands was checked | fixed: the arc's example checks each band's first side against its chord |
| 15 | Mutation: `load-picked-geometry.ts:28`, `common !== undefined` negated or flipped, survived: the redraw after a pick writes the radius field from `data-radii` (`writeRadiusField`), so writing the common radius before it changed nothing | fixed: the write removed, the TSDoc names the redraw |

Mutations (A4), `npm run mutate`: the 18 modules of `contour-band/` and `painted-area/`, `cubicAxisTurns`, the 7 new modules of `model/drawing/` and `draggedBox` against their own test files, 79 caught and 2 survivors, caught once findings 10 and 14 were fixed; the hover's 6 modules against `hover.test.ts` (10 caught), the selection's 6 against `select-shape.test.ts` and `load-picked-shape.test.ts` (8 caught, finding 15), the drag's 7 against `drag-shape.test.ts` (16 caught). By hand, the three boundaries findings 11 to 13 name, the first two seen surviving with their new test left out, and each criterion once: `draggedBox` laying every side from the press (`[F36.AC1]`, 9 tests fail, the playground's drag among them); `shapeAtPoint` taking the first shape rather than the last (`[F36.AC2]`, 2 fail, the hover among them); `splitDrawing` putting every other shape below (`[F36.AC3]`, 6 fail, the page's selection among them).

Properties (ADR-0022): `pieceSamples` gets the bound of its cubic chords, `cubicAxisTurns` its own (at most two values, inside (0, 1), the derivative 0 there); the other functions of F36 had theirs.

Next feature (G): F37 selects several shapes, by Shift + click, Ctrl + click and a rectangle, and removes them by Delete or Backspace outside a field: its spike needs how Figma, Inkscape and Penpot add to a selection and drag a selection rectangle (whether it takes the shapes it touches or only those inside), how their panel shows values the shapes do not share, and which keys remove; `REF-INKSCAPE-SELECTOR` covers only the frame of the selection.

Evolvability (H):

- Size: `playground/user-actions` grew to 33 modules (finding 9), listed in `files.md` as one theme; nothing else past the signal.
- Tools: no "considered for later" row is triggered by F36.
- Fitness functions: a property for each sampling bound (finding 1) is what caught the cubic's chord; nothing new to automate.
- Patterns: the two duplicates were a named calculation (finding 7) and a shared formula (finding 8); the test helper (finding 9) shows a test file's helper copied by the next story, which the naming pass at each review catches.
- Agent configuration: nothing.
- Process: F36's worktrees were made by hand (see the incident below).
