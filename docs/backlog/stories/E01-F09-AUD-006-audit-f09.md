---
id: AUD-006
epic: E01
feature: F09
title: Audit F09
status: ready
points: 2
---

# E01 · F09 · AUD-006 — Audit F09

Full check of F09 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F09, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F09 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-006.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [ ] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, next feature, light evolvability check; findings table (1.5 h)

## Findings

Independent review by the `auditor` (A–E), mutations, properties and breadth checks by the agent, 2026-10-11.

| Check                             | Result                             | Evidence and action                                                                                                                                                                 |
| --------------------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A1 — stretch re-derived           | correct                            | SVG 2 §8.2 (`align = none`, steps 5–9) and CSS Transforms §3 re-read; `stretchToSize` is `o + w · (p − o) / n`                                                                      |
| A2 — expected values              | correct                            | every check row and test value recomputed: 10 × 4, 4 × 8, 29 × 1, 3 × 2 to 100 × 50, the playground rectangle and its width 0                                                       |
| A3 — very large sizes             | bound stated                       | the far edge is exact while `w · n` stays below 2⁵³; beyond it can be off by a unit (accepted, as for F02); stated in the derivation                                                |
| A3 — many points                  | **fixed**                          | `boundingBox` spread its points into `Math.max`, which throws past about 100 000 points; now a fold; test with 200 000 points                                                       |
| A3 — −0, negative stored points   | fine                               | the output writes `0`; negatives covered by the property                                                                                                                            |
| A4 — mutations (agent)            | 10 of 10 caught                    | axis swaps, a wrong unit square, reversed radii, size swap, each validity clause, the playground's validity                                                                         |
| A5 — properties                   | added                              | round trip to the stored size; order of x and y kept (a rectangle stays one); the rectangle tool valid exactly when the rectangle is                                                |
| B — sources                       | re-opened                          | `REF-SVG2-COORDS` (its "used for" now names F09), `REF-CSS-TRANSFORMS`; link check run on this branch                                                                               |
| C — provenance                    | own                                | short functions written from the derivation                                                                                                                                         |
| D — duplicates                    | kept on purpose                    | `isValidRectangle` and `rectangleContour` stay as wrappers anchoring the F01 tests; `fitInBox` shares the scale, not the translation (wording fixed)                                |
| E — F09 against the paused F08    | **question for the Product Owner** | F09 stores integer points and an integer size, while F08 decided that a converted polygon keeps its fractional vertices and does not move: Q27 opened, to settle before F08 resumes |
| E — docs                          | fixed                              | note 0008 key order, guide pages F01 and F03, `read-shape.ts` TSDoc                                                                                                                 |
| E4 — SP-005 status on this branch | expected                           | `done` on the paused F08 branch, not yet on `main`                                                                                                                                  |
| F — quality gates                 | green                              | `check:all`, coverage 100 %, 422 tests                                                                                                                                              |

Tests per criterion: F09.AC1 2, AC2 5, AC3 3, AC4 3; the F01 and F03 rectangle tests also run through the free shape.

## G. Next feature

- F08 (node editing), paused: settle Q27 first (stored point type, the size of a converted polygon, the frame of the typed x, y), then rewrite EN-011 to US-016 on the free shape of F09.
- F06 (polygon stretched to fill its box): reuse `stretchToSize` rather than a third box map.

## H. Light evolvability check

| Question            | Answer                                                                                              |
| ------------------- | --------------------------------------------------------------------------------------------------- |
| Size                | 82 modules in `src/`; 422 tests; `model` to split into subfolders before F08 (improvement log)      |
| Tools               | StrykerJS still blocked; 10 mutations by hand                                                       |
| Fitness functions   | the F01 and F03 tests as a regression net for the model change                                      |
| Patterns            | two box maps on purpose; `boundingBox` now safe for large inputs                                    |
| Agent configuration | pitfalls added at CHK-006                                                                           |
| Process             | a feature inserted mid-run worked; a paused branch must be watched for decisions that diverge (Q27) |
