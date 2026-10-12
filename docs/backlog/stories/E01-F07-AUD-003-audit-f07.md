---
id: AUD-003
epic: E01
feature: F07
title: "Audit F07: polygon aligned in its box"
status: done
points: 2
---

# E01 · F07 · AUD-003 — Audit F07: polygon aligned in its box

Full check of F07 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F07, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F07 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-003.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [x] T2 — B to G. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature; findings table (1.5 h)

## Findings

Independent review by the `auditor` (A–E), mutations and breadth checks by the agent, 2026-10-11.

| Check                                        | Result                              | Evidence                                                                                                                                                                                         | Action                                                                                              |
| -------------------------------------------- | ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------- |
| A1 — step 5 re-derived from SVG 2            | correct                             | §8.2 steps 7, 11–14 and §8.7 re-read; `fitInBox` and `alignmentFactor` match                                                                                                                     | —                                                                                                   |
| A2 — expected values                         | correct                             | every F07 test value recomputed, incl. tangent points 17.32051 / 82.67949 and the apex fillet (center 20, top 10)                                                                                | —                                                                                                   |
| A3 — sweep n = 3…12, boxes 0…1000, 9 anchors | AC2 and AC3 hold as written         | anchored vertices written as the integer edge; filled axis written identically (0 differences)                                                                                                   | —                                                                                                   |
| A3 — very large boxes                        | step 4 bound understated            | 2⁵³ − 1 square, n = 4: reached size off by 2, not 0.5; anchored edge off by 1 at n = 5                                                                                                           | derivation and `shapes.md` (Q20) now say "a few units"; harmless below millions                     |
| A3 — polygon without `anchor`                | validation throws                   | `isAnchor(undefined)`: "Cannot read properties of undefined"                                                                                                                                     | already in the improvement log (validate untrusted data at the boundary, E11)                       |
| A4 — mutations (agent)                       | 8 of 8 caught                       | `fitInBox` axis swap, `mid` factor 0.25, `isAnchor` `true \|\|`, corner count `<`, `isValidRegularPolygon` without `isAnchor`, contour axes swapped, `readAnchor` axis, `writeAnchorValue` no-op | —                                                                                                   |
| A5 — properties                              | one missing                         | AC4 proven by two examples only                                                                                                                                                                  | property added: the flat base stays on the box for any n and radius (mutation-checked)              |
| B — sources                                  | all re-opened, dates right          | `REF-SVG2-COORDS`, `REF-FIGMA-ALIGN`, `REF-FIGMA-AUTO-LAYOUT`, `REF-WAI-APG-RADIO`; weekly link check green (2026-10-09)                                                                         | —                                                                                                   |
| C — provenance                               | own                                 | the SVG spec's formula, written from the derivation; no dependency changed                                                                                                                       | —                                                                                                   |
| D — duplicates                               | test helpers only                   | `expectPoints` ×3, `CENTER` ×4, `HEIGHT_3` ×2 in tests; `Size`, `ViewBox`, `BoundingBox` are distinct concepts                                                                                   | improvement log row of 2026-10-09 (shared fixtures) still open                                      |
| E1 — two meanings of "anchor"                | ambiguous                           | `shapes.md` Text said "anchor (start / middle / end)"                                                                                                                                            | now "text anchor"; the glossary row tells the anchor apart from the text anchor, Align and snapping |
| E1 — "default: center"                       | wording suggested a library default | the model has no default (ADR-0016); the playground starts on center                                                                                                                             | TSDoc reworded                                                                                      |
| E4 — feature plan                            | incomplete                          | US-010 added `[F07.AC1]`–`[F07.AC4]` playground tests not credited                                                                                                                               | "Verified by" counts them: AC1 6, AC2 8, AC3 4, AC4 3 + property, AC5 3                             |
| F — quality gates                            | green                               | `check:all`, coverage 100 %, no `.skip` / `.only`; the only `eslint-disable` lines are justified casts in tests                                                                                  | —                                                                                                   |

Before VAL-003: the F02 demo page still says "centered" (to link to the F07 page); no F07 guided steps yet.

## G. Next feature (F03, per-vertex radius and node editing)

Sources and questions for its spike, those of AUD-002 G still open, plus:

- Per-vertex radius on an anchored polygon: the anchor places the sharp polygon, so a per-vertex radius never moves it — the AC4 property generalizes to "any radii".
- Node editing of a regular polygon breaks its regularity: does an edited polygon keep its anchor (it would no longer be fitted), become a free contour, or is editing refused? Business question for the Product Owner.
- `isValidRegularPolygon` on untrusted data (a polygon without `anchor`): only matters once files are read (E11).

## H. Light evolvability check (2026-10-11)

| Question            | Answer                                                                                                                                                                |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 60 modules in `src/` (geometry 26, model 13, math 11, io 6, render 4), 7 more in `playground/`; 341 tests in 4.5 s; `check:all` about 28 s                            |
| Tools               | StrykerJS still blocked (issue 6210 open): 8 of 8 hand mutations caught here, every story review mutated too; `knip` trigger met, decision at the E01 retrospective   |
| Fitness functions   | the playground's size limit (`max-lines`) forced three extractions in US-010: the limit works; still no automatic check that every `geometry` function has a property |
| Patterns            | one list of alignments (`ALIGNMENTS` → `Alignment`); test helpers duplicated (`expectPoints`, `CENTER`) — shared fixtures remain a retrospective item                 |
| Agent configuration | `CLAUDE.md` + rules about 190 lines; four pitfalls added during F07 (casts, mutation restore, commit scopes, happy-dom `:checked`)                                    |
| Process             | 6 stories merged by the run; every review changed its story; the spike's review caught a false claim before code, US-010's review a vacuous test                      |
