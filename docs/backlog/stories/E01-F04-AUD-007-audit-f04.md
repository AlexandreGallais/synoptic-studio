---
id: AUD-007
epic: E01
feature: F04
title: "Audit F04: free rotation of shapes"
status: ready
points: 2
---

# E01 · F04 · AUD-007 — Audit F04: free rotation of shapes

Full check of F04 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F04, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F04 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-007.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [x] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature, light evolvability check; findings table (1.5 h)

## Findings (2026-10-12)

Independent review by the `auditor` subagent (sections A to E, on a copy of the committed tree), mutation spot-checks by the agent (A4). Each story of F04 had its own review before (pull requests #122 to #127).

| Check                                             | Result                                               | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                            | Action                                                              |
| ------------------------------------------------- | ---------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| A1, A2 — derivations and expected values          | correct                                              | `DERIV-turned-shape` steps 1–5 and `DERIV-arc-bounding-box` steps 1–4 re-derived; every value of the feature's tests and test cards recomputed                                                                                                                                                                                                                                                                                                      | —                                                                   |
| A4 — one mutation per function (14)               | all caught                                           | `degreesInTurn` modulo (7 tests), `turnDirection` sign of sin (4), `turnAbout` sign (4), `arcAxisPoints` direction (1), `contourBoundingBox` arc filter (3), `boxToViewBox` margin (4), `turnedCorners` center (11), `isShapeAngle` bound (5), `isShapePlacement` y (1), `positionKeepingPoint` sign (10), `positionAfterResize` corner (2), `freeShapeKeptInPlace` local y (4), `freeShapeContour` move (12), `regularPolygonContour` position (1) | —                                                                   |
| A3 — `degreesInTurn`                              | a dead `+ 0`, a false comment                        | the inner sum lies in (0, 720): never a negative zero (probe over ±1e5)                                                                                                                                                                                                                                                                                                                                                                             | fixed: removed, the TSDoc says why                                  |
| A3 — `positionAfterResize` at 0°                  | early return unobservable                            | the general path gives the same position (100 000 random cases)                                                                                                                                                                                                                                                                                                                                                                                     | no action: kept, nothing computed at 0° as for every straight shape |
| A5 — properties                                   | missing for four functions; turned edits only at 90° | `positionKeepingPoint`, `arcAxisPoints`, `boxToViewBox`; midpoint, deletion and conversion of a turned shape                                                                                                                                                                                                                                                                                                                                        | added: properties at any angle                                      |
| B — sources                                       | reachable, quotes verbatim                           | 9 URLs re-opened on 2026-10-12                                                                                                                                                                                                                                                                                                                                                                                                                      | —                                                                   |
| C — provenance                                    | written from the derivations                         | no library pattern, no dependency added                                                                                                                                                                                                                                                                                                                                                                                                             | —                                                                   |
| D1 — duplicates                                   | box center computed three times; 360 declared twice  | `turnedCorners`, `positionAfterResize`, `freeShapeWithPoint`; `degreesInTurn`, `isShapeAngle`                                                                                                                                                                                                                                                                                                                                                       | fixed: `boxCenter`, `DEGREES_PER_TURN`                              |
| E1 — a width emptied, then retyped                | the turned shape jumped                              | a refused draw dropped the turned box kept                                                                                                                                                                                                                                                                                                                                                                                                          | fixed: only a refused angle drops it; test                          |
| E1 — ADR-0034                                     | changed by ADR-0035 without saying so                | a coordinate typed on a turned shape is an integer in the drawing only                                                                                                                                                                                                                                                                                                                                                                              | fixed: status lines, index, domain (Q27, Q28, `shapes.md`)          |
| E1 — TSDoc saying "as drawn" for straight corners | contradicted `turnedCorners`                         | `freeShapeCorners`, `freeShapeAroundCorners`, rectangle and polygon corners, midpoint, deletion                                                                                                                                                                                                                                                                                                                                                     | fixed                                                               |
| E1 — snapping to the integer grid                 | to reconcile with a fractional stored position       | `interaction.md` §4                                                                                                                                                                                                                                                                                                                                                                                                                                 | noted there: the straight box's x and y are snapped                 |
| F — quality gates                                 | green                                                | `check:all`, 627 tests, coverage 100 % of `src/`                                                                                                                                                                                                                                                                                                                                                                                                    | —                                                                   |

## G. Next feature

F05 (text), next in E01's order, needs for its spike:

- SVG 2 text: `<text>`, `x`, `y`, `text-anchor` (start, middle, end), `font-family`, `font-size` in user units, `fill`; how `dominant-baseline` places the line (the baseline, not the top).
- Its straight box: without reading font files (ADR-0002), the box of a text cannot be computed by the core; a browser's `getBBox` measures it. How the editor shows it, and whether it is needed before E13's layout box.
- A turned text (F04): `transform="rotate(…)"` on `<text>`, the only primitive turned by SVG rather than computed; the angle and position of ADR-0035 for a text.
- Fonts available offline and the same in every browser (generic families, a font embedded in the library?): a Product Owner question.
- Integer size, color as for the shapes (E04 strokes and fills not there yet).

## H. Light evolvability check

| Question            | Answer                                                                                                                                 |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 103 modules in `src/` (model 46, geometry 30, math 15, io 6, render 5), 627 tests; the subfolders of `model` are due (improvement log) |
| Tools               | StrykerJS still blocked; 14 mutations by hand, all caught                                                                              |
| Fitness functions   | the playground coverage threshold of CHK-007 still to do                                                                               |
| Patterns            | `boxCenter` now the one center of a turn: the movable center of E02 changes one place                                                  |
| Agent configuration | pitfalls of CHK-007 still true; nothing new                                                                                            |
| Process             | the stories' reviews caught most findings; the audit found the gaps between stories (a width retyped, ADR-0034, "as drawn")            |
