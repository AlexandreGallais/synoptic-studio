---
id: AUD-002
epic: E01
feature: F02
title: "Audit F02: regular polygon with corner radius"
status: done
points: 3
---

# E01 · F02 · AUD-002 — Audit F02: regular polygon with corner radius

Full check of F02 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F02, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F02 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-002.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (3 h)
- [x] T2 — B. Sources re-verified (1 h)
- [x] T3 — C, D, E. Provenance, duplicates, consistency (2 h)
- [x] T4 — F, G. Quality gates and research list for the next feature; findings table (1 h)

## Findings

Independent review by the `auditor` (A–E), mutations and breadth checks by the agent, 2026-10-09.

| Check                                            | Result                           | Evidence                                                                                                                                | Action                                                                         |
| ------------------------------------------------ | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| A1 — derivation re-derived without the code      | correct                          | steps 1–6 and the five check rows recomputed (node)                                                                                     | —                                                                              |
| A1 — "within `EPSILON`" in step 4 and `fitInBox` | wrong for large boxes            | error about 1e-16 × the size: 3.7e-9 at 1e8, −0.5 written at the largest safe integer                                                   | wording fixed (relative error); size range asked: Q20                          |
| A2 — expected test values                        | correct                          | every value recomputed by hand or with node                                                                                             | —                                                                              |
| A3 — missing cases                               | pentagon, n = 12, rounded size 0 | no example test                                                                                                                         | tests added                                                                    |
| A3 — signed zeros                                | none                             | `unitRegularPolygon`, `fitInBox` from size 0 to the largest safe integer                                                                | —                                                                              |
| A4 — mutations (agent)                           | 10 of 10 caught                  | one per function: start index, `minX`, `left`, `>= 0`, integer check, `max: 13`, width/height swap, radius 0, extra cap, radius dropped | —                                                                              |
| A4 — mutations (auditor)                         | 16 of 17 caught                  | `effectiveCornerRadius` `min` → `max` survives                                                                                          | equivalent: the new property proves every corner has the same effective radius |
| A5 — properties                                  | present                          | all `geometry` and `domain` functions of F02; predicates by examples                                                                    | property "same effective radius" added                                         |
| B1 — sources re-opened                           | all read again                   | MathWorld ×4, OpenStax, Euclid I.8 and I.47, Inkscape, ISA InTech                                                                       | IEC, ANSI, ISO refuse robots (403): stated, link check excludes them           |
| B2 — link check                                  | green                            | lychee on the branch: 336 links, 0 error, 4 excluded                                                                                    | `.lycheeignore` rows with reason                                               |
| B3 — `@see` to `[unverified]`                    | none                             | lint; no `eslint-disable` added                                                                                                         | —                                                                              |
| C — provenance                                   | own                              | TSDoc formulas match bodies; index-based start and `0 − sin` guard are project-specific                                                 | —                                                                              |
| D — duplicates                                   | one left                         | `expectPoints` in two test files                                                                                                        | improvement log (shared fixtures)                                              |
| E1 — playground help                             | wrong for polygons               | "the curve starts that many pixels before the corner": true on a rectangle only                                                         | text fixed                                                                     |
| E1 — research note                               | contradiction                    | pre-research line said IEC was not read                                                                                                 | marked superseded                                                              |
| E4 — feature plan, testing convention            | incomplete                       | "Verified by" and the playground test's criteria                                                                                        | completed                                                                      |
| F — quality gates                                | green                            | `check:all`, coverage 100 %, `npm run build`, no `.skip` / `.only`                                                                      | —                                                                              |

## G. Next feature (F03, per-vertex radius and node editing)

Sources and questions for its spike:

- Per-vertex radius: `REF-FIGMA-CR` (verified) for the behavior; the clamp of F01 already works per vertex (`Corner`); the `Math.max`/`Math.min` mutant of `effectiveCornerRadius` stops being equivalent — the interface needs one effective radius per vertex.
- Node editing (select, move by integers, add on an edge, delete): Inkscape's node tool and Figma's vector editing as functional references, to read (no row yet); projecting a click onto an edge (closest point on a segment) needs a source.
- Concave contours: the known limit of the clamp (an arc touching a non-adjacent edge, `shapes.md` §2) becomes reachable — segment–arc and arc–arc intersection (`REF-SCHNEIDER-EBERLY`, `REF-BOURKE-CIRCLES`, both to verify).
- Business questions: can a regular polygon's vertices be edited (it would stop being regular: convert it to a free contour?); does a moved vertex keep its radius; what does deleting a vertex down to two do.
- Large boxes: the absolute `EPSILON` of `hasLength` leaves tiny segments on boxes of millions of units (improvement log) — a size range or a relative tolerance, for the Product Owner.

## H. Light evolvability check (2026-10-09)

| Question            | Answer                                                                                                                                                                                                                      |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 53 modules in `src/` (geometry 21, math 11, model 11, io 6, render 4); 306 tests in 4.8 s; `check:all` about 28 s; no layer close to a package split                                                                        |
| Tools               | StrykerJS still blocked (issue 6210 open): mutations by hand, 10 for 10 caught in this audit; `knip` waits for the retrospective; `deps:tools` added at the Product Owner's request                                         |
| Fitness functions   | every new `geometry` function has a property test; still a candidate automatic check (improvement log)                                                                                                                      |
| Patterns            | duplicated size rule removed (`isModelSize`); test helpers (`expectPoints`, signed area) logged; `regularPolygonCorners` mirrors `rectangleCorners` — fine for two shapes, a shared "corners of a shape" when a third comes |
| Agent configuration | `CLAUDE.md` + rules 185 lines; one library rule added (visibility in the playground); `/run` orders the review before `done`                                                                                                |
| Process             | 6 stories merged by the run; every review changed its story; one real bug caught only by review (hidden field), which happy-dom could not see before the head was loaded — real-browser tests remain an E12 item            |
