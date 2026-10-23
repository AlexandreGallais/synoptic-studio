---
id: AUD-039
epic: E02
feature: F37
title: Audit F37
status: ready
points: 1
---

# E02 · F37 · AUD-039 — Audit F37

Audit F37 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F37's criteria, when the audit ends, then each is traced to a test titled `[F37.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-039.T1`.

## Findings

The `auditor` alone reviewed F37 read-only (ADR-0086): no error in Delete, Shift + click nor the rectangle's drag, one rounding error in the rectangle's touching, one question for the Product Owner and eight smaller findings, each reproduced before it was fixed (its scripts run against the library's sources); the mutations found three more (11 to 13). F37's three criteria are traced to the `[F37.AC1]` tests of `drawing-without-shapes.test.ts` and `select-shapes.test.ts`, the `[F37.AC2]` tests of `shape-selection-after-click.test.ts` and `select-shapes.test.ts`, the `[F37.AC3]` tests of `shapes-touched-by-box.test.ts`, `shapes-enclosed-by-box.test.ts`, `shape-selection-after-box.test.ts` and `select-in-rectangle.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | Touching was exact while enclosing allows `EPSILON`: the Polygon tool's square, 4 corners of radius 10, 100 × 100 at (0, 0), its contour box's left at x ≈ 1.1 × 10⁻¹⁵, was enclosed by `[0, 100]²` but not touched by `[−10, 0] × [40, 60]` along its side, against `interaction.md` §2 (Its outline counts) and Q53; 20 polygons of 4 and 10 corners failed so, the stretched decagon's bottom at y ≈ 100 − 1.4 × 10⁻¹⁴ among them | fixed: `widenedBox(box, margin)` in `src/geometry/placement/`, the rectangle widened by `EPSILON` before `isContourTouchingBox` and the flat branch (`isPickAreaTouched`); `isBoxWithin` reads its `EPSILON` through it, the same floats; `DERIV-box-overlap` step 10 and three checks; the square and the decagon in `shapes-touched-by-box.test.ts`, each branch's rounding error in `is-pick-area-touched.test.ts`, both seen failing with the margin 0; the 20 polygons now touched |
| 2 | A shape with area whose points lie on one line (a free shape (0, 0), (50, 50), (100, 100)) is touched by `[45, 55]²` and enclosed by `[0, 100]²`, while a click never picks it (`interaction.md` §2, A shape without area, last sentence) | a question for the Product Owner, below; the code kept as it is |
| 3 | `isBoxWithin`'s `EPSILON` was tested on `minX` only | fixed: maxX, minY and maxY each past by 10⁻¹² held within, each seen failing with its `EPSILON` removed |
| 4 | `isOnArc` was tested at the ends of the positive arc only | fixed: `isOnArc(BACK, BACK.start)` and `(BACK, BACK.end)` true; `<= 0` made `< 0` fails both arcs' ends |
| 5 | "VAL-038", dropped on 2026-10-23, still named as where the default keys are confirmed: `interaction.md` §1 (The default keys), Q50, F37's guide page and F37's feature plan | fixed but the feature plan: "for the review's summary" (AUD-039); the feature file is the main thread's, below |
| 6 | `interaction.md` §1's row "select by a rectangle" still said what it takes is open (SP-027); F37's benefit hypothesis still names Ctrl + click and the panel's common values, which went to E13 and F38 | fixed but the hypothesis: the row says it touches by the painted area, encloses with Alt and adds with Shift (Product Owner, 2026-10-22); the hypothesis is the main thread's, below |
| 7 | Q54 gave Alt and Shift "held at the release" to the Product Owner's answer, while the release is US-069's choice | fixed: Q54 says both are read at the release (agent, US-069), `interaction.md` §2 and the guide page too |
| 8 | `REF-WIKI-INTERVAL` and `REF-SUNDAY-INCLUSION`, read again on 2026-10-23, kept older status dates | fixed: `[verified] 2026-10-23` |
| 9 | `REF-FIGMA-SELECT-LAYERS` named Ctrl + click kept for picking inside a group without quoting it | fixed: the "Deep select" sentences, read through the library |
| 10 | The chord's signed side σ ((E − S) × (P − S)) written in both `isOnArc` and `isInCircularSegment`; `readOutlinePathData` computed a box's width and height beside `boxSize` (ADR-0049) | fixed: `arcChordSide(arc, point)` in `src/geometry/painted-area/`, the same products and a negation, which changes no comparison (`golden:check`); `boxSize` in `readOutlinePathData` |
| 11 | Mutation: `is-on-arc.ts:21`, `<= 0` → `<= 1`, survived: no point of the circle off the arc lay close to the chord's line | fixed: `fails for a point of the circle just past an end`, the fillet of radius 1 about (1, 1) and (0.2, 1.6), its side 0.8 |
| 12 | Mutation: `move-selection-drag.ts:26`, `width + height > 0` → `> 1`, survived: no drag left the press's point by one unit on one axis | fixed: `draws the rectangle once the pointer has left the press's point by one unit`, (−10, −10) to (−9, −10) |
| 13 | Mutation: `create-selection-elements.ts:24`, `pieces.length > 0` → `> 1`, survived | no action: a contour has a piece per side, at least 3, and a text never measured none, so both keep the same areas |

Mutations (A4), `npm run mutate` against each file's own test file: the 14 modules of `box-overlap/`, `isBoxWithin`, `isPointInBox`, `widenedBox`, `arcChordSide`, `isInCircularSegment`, `isBetween`, `drawingWithoutShapes`, the 4 new modules of `model/picking/` and the 2 of `model/selection/`, 48 caught and 1 survivor (finding 11); the selection's 8 playground modules against `select-shapes.test.ts`, 15 caught and 1 survivor (finding 13); the rectangle's 8 against `select-in-rectangle.test.ts`, 17 caught and 1 survivor (finding 12). By hand, the new tests of findings 1, 3, 4, 10, 11 and 12, each seen failing with its fix undone, and each criterion once: `drawingWithoutShapes` removing none (`[F37.AC1]`, 12 tests fail, the page's Delete among them); `shapeSelectionAfterClick` reading no Shift (`[F37.AC2]`, 11 fail, the page's Shift + click among them); `shapesTouchedByBox` taking the shapes it does not touch (`[F37.AC3]`, 15 fail, the page's rectangle among them); the page's rectangle reading no Alt (1 fails) and no Shift (2 fail).

Properties (ADR-0022): `widenedBox` and `arcChordSide` get theirs; the other functions of F37 had theirs.

Breadth (step 7): `check:all` passed, and `golden:check` after finding 10; the weekly link check of `main` failed on 2026-10-19 with five pages answering 403 to the bot (Adobe's live shapes, four npm package pages), none of F37's; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F37 did.

Next feature (G): F38 shows the position and size of several shapes, Mixed where they differ, and sets them by a value typed, relative with a sign or fixed. Its stories need how Figma, Inkscape and Penpot show a value several layers do not share (Figma's "Mixed"), whether a relative value is typed with its sign alone or as an expression (`+50`, `100 + 50`), how a fixed size applies to a turned shape and to a text that fits its content, and whether X and Y of several shapes are their common box's or each shape's; none of those pages is in `docs/references.md` yet.

Evolvability (H):

- Size: `src/geometry/painted-area/` reaches 15 modules with `arcChordSide`, at the signal of `files.md` and not past it; `box-overlap/` and `placement/` hold 14, `playground/selection/` 11 and `playground/selection-drag/` 6. The next module of `painted-area/` asks for a split, its distances (`segmentDistance`, `polygonDistance`) a candidate domain.
- Tools: no "considered for later" row is triggered by F37.
- Fitness functions: finding 1 is a rule written twice, `EPSILON` on enclosing and none on touching; a property drawing a polygon's rounding errors against both would have held it, nothing new to automate beyond the tests added.
- Patterns: the duplicates were a chord's side (finding 10) and a box's size (finding 10); the naming pass at each review catches them.
- Agent configuration: nothing.
- Process: VAL-038 dropped mid-feature left its name in four pages (finding 5); a feature whose VAL is dropped gets "for the review's summary" in the same commit.

## Question for the Product Owner

Asked on 2026-10-23, finding 2. A shape whose box has an area but whose points lie on one line, a free shape (0, 0), (50, 50), (100, 100) whose box is 100 × 100, paints nothing and is never picked by a click (`interaction.md` §2, A shape without area, its last sentence). The selection rectangle reads its contour as drawn: the rectangle from (45, 45) to (55, 55) touches it, crossing its line, and the one from (0, 0) to (100, 100) encloses it, its box within the rectangle. The code is kept as it is until the answer.

- **A**: keep it: a rectangle touches it by its line and encloses it by its box, as the code does, though a click never picks it (the agent's choice, EN-144). Example: the rectangle from (45, 45) to (55, 55) selects it.
- **B**: never touched nor enclosed, as a click never picks it; only Ctrl + A selects it. Example: the rectangle from (45, 45) to (55, 55), or from (0, 0) to (100, 100), selects nothing; Ctrl + A then Delete removes it.

## For the main thread

- The feature file is the main thread's (ADR-0065): F37's feature plan still names VAL-038 in its "Verified by" column (finding 5), "AUD-039" alone there; F37's benefit hypothesis still names Ctrl + click and "their common values shown in the panel" (finding 6), which went to E13 and F38: "several shapes are selected at once, by Shift + click or a rectangle, and removed by a key", for instance.
- For the done commit's footer: the rule docs changed in this task, `interaction.md` §1 and §2, the Q rows Q50, Q53 and Q54 of `docs/domain/README.md`, and F37's guide page; VAL-038 is dropped, its decisions going to the review's summary.

### Incidents

#### 2026-10-23 — `mutate`, for the audit's mutation spot-checks

- Used: `npm run mutate -- <27 files> -- npx vitest run {test}` in the background, before the audit's new modules were committed.
- Failed: the copy holds tracked files only (`git stash create`), so the two new modules, untracked, were missing and every file was skipped as "its related tests fail"; `{test}`, unquoted in the background call, reached the tool dropped, so each skip ran the whole suite; the run, stopped, left its temporary worktree registered; `git add -N` on the new files made `git stash create` fail ("not uptodate").
- Instead: the run stopped and its worktree removed (`git worktree remove --force`), the new files staged by `git add`, then `npm run mutate` again with `'{test}'` quoted.
