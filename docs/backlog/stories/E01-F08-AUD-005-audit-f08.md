---
id: AUD-005
epic: E01
feature: F08
title: "Audit F08: node editing"
status: done
points: 2
---

# E01 · F08 · AUD-005 — Audit F08: node editing

Full check of F08 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F08, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F08 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-005.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [x] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature, light evolvability check; findings table (1.5 h)

## Findings (2026-10-11)

Independent review by the `auditor` subagent (sections A to E) and mutation spot-checks by the agent (A4).

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
| A3 — a point typed that leaves no width or height | lost the free shape kept, showed the rectangle's message | happy-dom probe: converted hexagon, three points deleted, two x typed 0 → no rows | fixed: `writeStoredFreeShape` keeps only a valid shape; test "keeps the free shape when a point typed would leave it no width" |
| A3 — a point added on a shape of width 0 | dropped silently, a radius reset | probe: step 1, width 0, points 1 and 2, add → radius 10 became 0 | fixed: the button is greyed when width or height is 0; test |
| A4 — `freeShapeWithPoint` dropping the moved point's radius | survived the `src` tests | mutation `{ ...corner, point }` → `{ point, radius: 0 }` | fixed: test "\[F08.AC5\] moves a point with its radius" and a property; now caught (2 tests) |
| A4 — `areFreeShapePoints` without the finiteness clause | survived | mutation: clause removed, 372 tests pass | fixed: an `Infinity` point case; now caught |
| A4 — `stretchToSize` multiplying inside the division | survived, equivalent | `w · ((p − o) / n)` keeps the exact edges; the harmful order, dividing first, fails 2 tests of F09 | no action: equivalent at the edges |
| A4 — eleven other mutations of the feature | caught | `unitRegularPolygon` sign of 2j (19 tests), `areFreeShapePoints` y span (2), `isValidFreeShape` size > 0 (2), `freeShapeAroundCorners` height (10), `freeShapeContour` size (17), `freeShapeCorners` radii order (9), `freeShapeRadii` zero (13), `freeShapeWithPoint` index (3), `regularPolygonAsFreeShape` width (1), `rectangleAsFreeShape` height (10), and in the stories `midpoint`, `edgeOfSelection`, `freeShapeWithMidpoint`, `freeShapeWithoutPoints`, `deletionOutcome` | — |
| A5 — properties | missing for three functions | `freeShapeWithPoint`, `deletionOutcome`, `freeShapeAroundCorners` | added |
| A1 — `DERIV-free-shape-stretch` step 2 | presented the identity at its own size as given by the formula | probe: `o + w·(p − o)/w` differs from `p` by an ulp for 7 coordinates of scaled unit polygons | fixed: the derivation says the code does not compute that axis |
| B — sources | all still say what is cited | 13 references re-fetched by the auditor | dates set to 2026-10-11 |
| C — provenance | no finding | compositions of 1 to 10 lines written from the derivations | — |
| D — duplicates | radii rebuilt in the playground; two "exactly one selected" readers | `write-stored-free-shape.ts`, `listen-to-point-fields.ts`, `show-point-fields.ts` | first fixed (`freeShapeRadii`); second left for the playground state module (improvement log) |
| E1 — integer point after a move: whole point (ADR-0031, Q27) or coordinate typed (ADR-0033, US-014) | contradiction | the converted hexagon's point 1 typed x 80 keeps y 93.30127 | Product Owner decision at VAL-005: open question Q28 (a) |
| E1 — fractional points and sizes after a stretch, a middle point or a deletion, further than ADR-0003 says | contradiction | hexagon with three points deleted: height 43.30127 | Product Owner decision at VAL-005: Q28 (b); ADR index row of 0003 aligned with its status line |
| E1 — "all points on one line are refused" | wrong for slanted lines | probe: (0, 0), (1, 1), (2, 2) valid | TSDoc fixed; ADR-0033 left as accepted, Q28 (c) |
| E1 — stale TSDoc of `regularPolygonContour`, of `isValidFreeShape`, of `freeShapeWithMidpoint` and `deletionOutcome` limits | stale or silent | start at the topmost vertex; fractions "only as a conversion leaves them" | fixed |
| E2 — open points only in the feature file and `interaction.md` | not with the open questions | deleting joins neighbours; crossing edges; cap of typed coordinates | gathered in Q28 |
| F — quality gates | green | `npm run check:all` | — |

## G. Next feature

F04, free rotation of shapes (draft): the rotation of a point about a center (a verified source), the bounding box of a rotated contour of segments and arcs, how an integer angle in degrees stays exact for quarter turns, how the box of F07 turns with its content (Product Owner's rule), and how a rotation composes with a free shape's points and its stretch (ADR-0033).

## H. Light evolvability check

| Question | Answer |
| --- | --- |
| Size | 91 modules in `src/` (model 42); 51 playground modules; about 500 tests; `model` subfolders stay for the E01 retrospective |
| Tools | StrykerJS still blocked: 23 mutations by hand in this feature, 2 gaps found that way |
| Fitness functions | a stored playground value is checked on writing and reading (`isValidFreeShape`) |
| Patterns | the playground state module (improvement log) would remove the "exactly one selected" twins and the guards spread over writers |
| Agent configuration | the auditor found what the story reviews missed at the seams between stories (an edit leaving a flat shape): keep a feature-wide probe in every audit |
| Process | four Product Owner decisions gathered in one open question (Q28) for the validation, rather than asked one by one during the run |
