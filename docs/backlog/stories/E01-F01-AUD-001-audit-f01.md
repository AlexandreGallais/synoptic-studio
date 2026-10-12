---
id: AUD-001
epic: E01
feature: F01
title: "Audit F01: rectangle with corner radius"
status: done
points: 3
---

# E01 · F01 · AUD-001 — Audit F01: rectangle with corner radius

Full check of F01 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F01, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F01 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Notes

- F01 functions merged before ADR-0022 have no properties yet: `subtract`, `dot`, `perpDot`, `turningAngle`, `turningAngles`, `cyclicItem`, `filletSetback`, `edgeLengths`, `edgeFactor`, `edgeFactors`, `vertexFactors`, `cornerSetbacks` (also through general polygons, not only rectangles), `formatSvgNumber`, `contourToPathData`, `rectangleContour`, `isValidRectangle`. T1 adds them where an invariant can be stated.
- Found while preparing the audit (2026-10-08): the F01 story table showed EN-001, US-001, EN-002, US-002 as `draft` although done; fixed, and `backlog.test.ts` now checks parents' tables.

## Tasks

One task = one commit, referenced as `AUD-001.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (3 h)
- [x] T2 — B. Sources re-verified (1 h)
- [x] T3 — C, D, E. Provenance, duplicates, consistency (2 h)
- [x] T4 — F, G. Quality gates and research list for the next feature; findings table (1 h)

## Findings

Independent review by the `auditor` subagent (sections A–E) on 2026-10-09, plus the agent's own mutation spot-checks; each finding reproduced before being recorded.

| Check                                      | Result                                                                                                                                                              | Evidence                                                              | Action                                                                                    |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| A1 — criteria traced to tests              | AC2 (value) and AC3 tagged; AC4 untagged; AC1 and the AC2 display live in the untested playground                                                                   | `grep "\[F01\.AC"`                                                    | AC4 tagged here; AC1 and AC2 display: playground test in VAL-001 T1                       |
| A2 — missing properties                    | eight test files without property                                                                                                                                   | `grep -c test.prop`                                                   | added: `formatSvgNumber`, `rectangleContour`, `filletArc`, plus nine older functions (T1) |
| A3 — `hasLength` boundary                  | no test file; chain tolerance 1000 × looser than stated                                                                                                             | —                                                                     | test file at the EPSILON boundary; tolerance = `EPSILON`                                  |
| A4 — mutation spot-checks                  | 22 of 24 mutants caught; 2 equivalent in JavaScript (`cyclicItem` without `+ n`: `.at` accepts −n…−1; `effectiveCornerRadius` max instead of min: four equal radii) | scratch script, one mutation per function                             | `hasLength` `>=`→`>` and truncation in `formatSvgNumber` now caught                       |
| A — end to end                             | AC3 and AC4 numbers exact on the real code; chain gaps ≤ 1.4e-14                                                                                                    | jiti on `rectangleCorners → roundedContour → contourPiecesToPathData` | —                                                                                         |
| B — sources                                | all sources cited by `src/` re-opened and accurate; 86 bibliography links answer                                                                                    | lychee; auditor                                                       | —                                                                                         |
| B1 — derivation steps                      | components of `a⊥` attributed to the perp-dot page; scalar multiplication uncited                                                                                   | page re-read                                                          | cites fixed (T2)                                                                          |
| B2 — undated rows                          | `REF-FIGMA-CR` and eight others `[verified]` without date                                                                                                           | `grep`                                                                | Figma dated; others logged                                                                |
| C — provenance                             | nothing copied; names follow the derivations                                                                                                                        | auditor                                                               | —                                                                                         |
| D1 — two path-data writers                 | `contourToPathData` only the oracle of a test; constants twice                                                                                                      | `grep`                                                                | improvement log                                                                           |
| D2 — corner triple twice, τ computed twice | `turning-angles.ts`, `rounded-contour.ts`                                                                                                                           | —                                                                     | improvement log                                                                           |
| E1 — start of the rounded path             | stated only in a derivation                                                                                                                                         | —                                                                     | `shapes.md` Q11 section (T3)                                                              |
| E2 — Q17                                   | not in `shapes.md`                                                                                                                                                  | —                                                                     | added (T3)                                                                                |
| E3 — rounding of ties                      | `toFixed` rounds ties away from zero, other languages to even                                                                                                       | `node -e` 0.015625 → 0.01563                                          | stated in TSDoc (ADR-0025)                                                                |
| E4 — feature plan                          | out of date                                                                                                                                                         | —                                                                     | updated (T3)                                                                              |
| E5 — concave limit                         | promise only in a derivation                                                                                                                                        | —                                                                     | `shapes.md` known limit (T3)                                                              |
| F — quality gates                          | `check:all` green, 252 tests, 100 % coverage; no `.skip` / `.only`; no `eslint-disable` in `src/` or `playground/`                                                  | output                                                                | —                                                                                         |

## G — For the next feature (F02, regular polygon)

- Sources: vertices of a regular polygon (cos / sin of the central angle), `DERIV-regular-polygon-fit` to re-read and cite; the clamp and fillets of F01 apply unchanged (angles other than π/2 for the first time).
- Questions for the Product Owner: n ≥ 3 bounds (upper limit?), orientation (flat base confirmed in the derivation), radius on polygons with acute angles (n = 3).
- Q16 (spike) stays unreachable for regular polygons.

## H — Light evolvability

| Question            | Answer                                                                                                                       |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Size                | 41 modules in `src/`; `check:all` about 25 s                                                                                 |
| Tools               | StrykerJS still blocked (issue 6210); `knip` trigger reached (logged); playground smoke test worth making permanent (logged) |
| Fitness functions   | `[F01.ACn]` traceability works; a test that every exported `math` / `geometry` function has a property is logged             |
| Patterns            | D1, D2 logged                                                                                                                |
| Agent configuration | the per-story auditor found at least one real gap in every story; keep it                                                    |
| Process             | autonomous run: 7 stories, 0 human intervention until VAL                                                                    |
