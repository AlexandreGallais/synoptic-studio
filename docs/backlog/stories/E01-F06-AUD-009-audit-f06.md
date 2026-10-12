---
id: AUD-009
epic: E01
feature: F06
title: "Audit F06: polygon stretched to fill its box"
status: done
points: 2
---

# E01 · F06 · AUD-009 — Audit F06: polygon stretched to fill its box

Full check of F06 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F06, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F06 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-009.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [x] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature, light evolvability check; findings table (1.5 h)

## Findings (2026-10-12)

Independent review by the `auditor` subagent (sections A to E, on a copy of the committed tree, its own implementation of the derivation), quality gates and sections F to H by the agent. Each story of F06 had its own review before (pull requests #143, #144, #146, #147).

| Check                                         | Result                                                     | Evidence                                                                                                                                       | Action                                         |
| --------------------------------------------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| A1 — derivation re-derived                    | correct                                                    | steps 1–6 implemented again without the code: the 9 check rows, λ_lo and λ_hi for every `n` (pentagon 6.74030, hexagon 1/3), note 0011's table | —                                              |
| A2 — a test comment                           | wrong angle                                                | `φ₁ ≈ 8.7°`, not 9.2°, for the pentagon at `λ = e⁻³` (the conclusion, `NaN`, holds)                                                            | fixed                                          |
| A3 — very large sizes                         | correct, untested above 1000                               | boxes up to 100 000 and 1 × 100 000 checked by hand: finite, the four sides touched within 1e-9                                                | properties raised to 100 000                   |
| A4 — one mutation per function (14)           | caught                                                     | `bisection` halves swapped, `boxAspect` inverted, the guard halved, no mirror, the ratio inverted, no reversal, no y flip, the bracket at 1.8… | —                                              |
| A4 — three survivors                          | equivalent                                                 | `bisection` returning its low end (one ulp), `>=` as `>` in the guard (no case lands on `π/n` exactly), the low bracket halved (λ_lo ≥ 1/3)    | recorded                                       |
| A5 — properties                               | the even closure for every λ only tested at three values   | step 2 of the derivation                                                                                                                       | a property over `ln λ` in `[−3, 3]`            |
| B — sources                                   | reachable, quotes verbatim                                 | 8 sources re-opened (the two lecture PDFs decompressed by hand)                                                                                | three old `[verified]` dates set to 2026-10-12 |
| B — derivation header                         | `REF-SVG2-COORDS` cited in step 6, missing from the header | `docs/derivations/stretched-polygon.md`                                                                                                        | added                                          |
| C — provenance                                | written from the derivation                                | no library pattern, no dependency added                                                                                                        | —                                              |
| D — duplicates                                | none                                                       | `unitStretchedPolygon(n, 1)` and `unitRegularPolygon` both give a regular polygon, kept apart so that F02 is drawn bit for bit                 | —                                              |
| E — research note 0011                        | **sides turning** for side directions                      | the other equal-sided octagons                                                                                                                 | fixed                                          |
| E — code, TSDoc, derivation, domain, ADR-0037 | consistent                                                 | rule C, guard `π/n`, anchor without effect, radii per corner, conversion without moving; every criterion tagged in tests                       | —                                              |
| F — quality gates                             | green                                                      | `check:all`, 673 tests, coverage 100 % of `src/`, `npm run build`; no `.skip`; two justified casts in tests                                    | —                                              |

## G. Next steps

F06 is the last feature of E01: after VAL-009 come REV-001 and RET-001. The next epic, E14 (curves with handles, ellipses, round shape), will need at its first spike: the offset of a cubic Bézier curve and of an ellipse (no closed form: approximation and tolerance), Bézier intersections for booleans (E03) and arc length for dashes (E04), SVG's elliptical arc command, Figma's three handle modes and Inkscape's smooth nodes, and the round shape of F10 kept on the tag `archive/f10-round-shape` (research note 0010).

## H. Light evolvability check

| Question            | Answer                                                                                                                                      |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 116 modules in `src/`, 673 tests; F06 added 12 small files (one export each)                                                                |
| Tools               | the auditor's own implementation of each derivation keeps catching what one-per-story reviews miss (the pentagon's highest scale in EN-017) |
| Fitness functions   | the monotonic sweeps run in 0.3 s; proving them is in the improvement log                                                                   |
| Patterns            | a sound interval found by bisection, then a value inside it: reusable                                                                       |
| Agent configuration | nothing new beyond CHK-009's three pitfalls                                                                                                 |
| Process             | the Product Owner asked mid-run to compare rule C with rule B at the validation: the demo page shows both, generated outside the library    |
