---
id: AUD-010
epic: E14
feature: F11
title: Audit F11
status: done
points: 2
---

# E14 · F11 · AUD-010 — Audit F11

Check that F11 changed the structure and nothing else, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the code before and after F11, when audited, then no test value changed and no behavior moved.
- Given the project, when audited, then no duplicate helper, writer or fixture remains.

## Tasks

One task = one commit, referenced as `AUD-010.Tn`.

- [x] T1 — Independent review by the `auditor`, `verifier` and `doc-editor` agents; test and helper fixes (2 h)
- [x] T2 — Documentation fixes (1 h)
- [x] T3 — Findings table, next feature, light evolvability check (0.5 h)

## Findings

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
| A1 — `cornerTriples`, `turningAngles`, `cornerSetbacks`, `clampedRadii`, `filletArc` re-derived | correct | DERIV-turning-angle, DERIV-local-radius-clamp, DERIV-fillet-arc re-derived before reading the code; bodies identical to before F11, the angle now passed in | — |
| A2 — expected values | correct | 3-4-5 triangle clamped to its incircle (r′ = 1), L shape's concave fillet (60, 50) → (50, 60), centre (60, 60), sweep 0 | — |
| A4 — mutations, by hand | 5 of 6 caught | `cornerTriples` next + 2, `turningAngles` swapped, `filletArc` `>= 0`, writer keeping the last segment, `cornerSetbacks` angle + 1: caught; `clampedRadii` factor index + 1 survived its own file | case 1000 and 10 on a square of 100 added (r′ = 99.00990, 0.99010): caught |
| A4 — mutations, verifier | 6 survivors in the checks | `folders` and `barrels` tests without recursion, `barrels` blind to the playground, fixtures `expectPoints` decimals, `integerPoint` negative half, `integerTriangle` spread, `freeShapeOf` box, the page's script tag | non-vacuity floors and nested names in `folders` and `barrels`; `barrels` reads the playground; fixture tests; `loadPage` test; fitness filters `index.ts` by basename — all killed again |
| AC6 — no test value changed | holds | scripted comparison of every matcher between `c3a2637` and the feature head: 957 → 1028 expectations, only the removed `contourToPathData` cases gone, no decimals changed | — |
| D1, AC1 — duplicates | 3 left at first | 17 raw `?.value` and 6 `?.textContent` reads in playground tests, the anchor grid chosen twice, `openPage` and `openServedPage` with one body, a local `expectPoint`, one `[F01.AC3]` case in two files | routed through `readValue`, `readText`, new `chooseRadio`, `readCheckedRadio`; `loadPage` shared; `expectPoints`; the duplicate case kept in `effective-radii` only |
| D1 — `selectElement` and `selectPageElement` | kept | the tests' actions do not lean on the code they check | written in `testing.md` |
| AC2 — one writer | holds in `src/` | `SEPARATOR`, `CLOSE_PATH` once; the playground still writes a `Z` by hand for its boxes and handles (`drawing/create-straight-box-element.ts`, `points/create-handle-elements.ts`), before F11 | no action: outside the library's writer; shown in VAL-010's figures |
| E3 — conventions | 11 gaps | `files.md` list of tests and example tree, `imports.md` table of non-existent paths and barrels, `testing.md` long paragraph, `agent.md` incident history, note 0012 retained choices, F11 and EN-022 wording | rewritten (doc-editor's table) |
| E1 — ADR-0042 rule 5 | contradiction | math had 17 modules, not under 15; EN-022 split it into `vector/` by rule 3 | recorded in EN-022; the accepted ADR left as it is — a question for the Product Owner |
| E1 — `playground/page/` | open | a theme (the page itself) or a folder by kind of code, which ADR-0042 refuses | question for the Product Owner, in EN-022 |
| F — quality gates | green | `check:all` at the push; public API diff: − `contourToPathData`, + `clampedRadii`, + `cornerTriples`; commit marked breaking, next version 0.2.0 | — |

## Next feature (G)

F10, the round shape, opens the curves of E14. Its spike needs:

- an ADR superseding ADR-0001 (segments and circular arcs only) and ADR-0018 (Bézier curves kept for static drawings): which curves enter symbols — cubic Bézier, elliptical arcs, or both;
- the SVG 2 path commands `C`, `S` and `A` with their parameters (`REF-SVG2-PATHS` holds the arc; the cubic to read);
- the ellipse filling its box, and its conversion to a free shape: 4 box corners (as in E01) or 4 points on the curve with handles (both wanted, research note 0010);
- the handle modes of Figma (independent, mirrored angle, mirrored angle and length) and Inkscape's smooth nodes;
- strokes and dashes on curves: the offset of a Bézier curve or an ellipse is not a curve of the same kind (exact or approximated, with which tolerance);
- the point type of a free shape with two kinds of points: corners rounded by arcs and points on the curve with handles (`ContourPiece` gets a third kind).

## Light evolvability check (H)

| Question | Answer |
| --- | --- |
| Size | 117 modules in `src/` in 15 folders, none past 15; 77 in the playground in 8 folders, plus 23 test actions in `user-actions/` (listed); 838 tests, 816 before F11 |
| Tools | StrykerJS still blocked (issue 6210 open); the agents' mutations by hand found 12 survivors across F11, all killed |
| Fitness functions | property per function in its own file; folder size; barrels of `src` and the playground; each check now fails on an empty scan |
| Patterns | `playground/page/` to be confirmed as a theme; the playground writes some path data by hand (boxes, handles) |
| Agent configuration | the unused-imports rule reads JSDoc words as uses (improvement log); worktrees install their hooks with `npm ci` |
| Process | every story of the run was reviewed by two to three agents; their findings changed an accepted ADR's rule before merge and added 22 tests |
