---
id: AUD-004
epic: E01
feature: F03
title: "Audit F03: per-vertex corner radius"
status: done
points: 2
---

# E01 · F03 · AUD-004 — Audit F03: per-vertex corner radius

Full check of F03 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F03, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F03 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-004.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (2 h)
- [x] T2 — B to G. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature; findings table (1.5 h)

## Findings

Independent review by the `auditor` (A–E), mutations, properties and breadth checks by the agent, 2026-10-11.

| Check                                          | Result                                | Evidence and action                                                                                                                                                                                                                                                                            |
| ---------------------------------------------- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A1 — clamp with unequal radii re-derived       | correct                               | `DERIV-local-radius-clamp` steps 1–3 hold per vertex; the proof never uses equal radii; the four F03 rows recomputed (20/10, 30/20, 15/5, 20/7.5)                                                                                                                                              |
| A2 — expected values                           | correct                               | every F03 test value recomputed: arcs, hexagon vertices (25, 6.69873), (100, 50), effective 86.60254, handle path, row texts                                                                                                                                                                   |
| A3 — edge cases                                | correct                               | one huge radius beside zeros (one quarter circle over two edges), size 0 with radii (shown as requested, Q17), 12 corners, a selection past the vertex count (harmless)                                                                                                                        |
| A3 — radius typed in Points mode               | **defect, fixed**                     | 200 000 was kept uncapped (Q20, F02.AC5); capped before it goes to the selection; test added                                                                                                                                                                                                   |
| A4 — mutations (agent)                         | 14 of 14 caught, after one test fixed | every F03 model function mutated once; radii reversed on a square survived (vertex 1 and 4 arcs share a center at radius 50): the test now uses radius 20                                                                                                                                      |
| A5 — properties                                | added                                 | closed outline under any radii, flat base under any radii, `radiiWithSelection`, `commonRadius`, `cornersWithRadii`, handle clicks, range                                                                                                                                                      |
| B — sources                                    | all re-opened, one date updated       | `REF-FIGMA-CR`, `REF-WAI-APG-LISTBOX`, `REF-APPLE-SELECT`, `REF-INKSCAPE-NODE-TOOL`, `REF-SVG2-PATHS` (now dated 2026-10-11); link check green on the audit branch, after excluding GitLab merge request pages, which refuse automated clients (403; the merge request exists, GitLab API 200) |
| B — unsourced claim                            | fixed                                 | "as in Figma and Inkscape" in `selectionAfterHandleClick`: no source read says it; now cites the domain rule                                                                                                                                                                                   |
| C — provenance                                 | own                                   | generic code written from the domain rules; no dependency added                                                                                                                                                                                                                                |
| D — duplicates                                 | fixed, one logged                     | `cornersWithRadii` reuses `resizeRadii`; the handles use `formatCoordinatePair`; playground test helpers repeated in five files (improvement log)                                                                                                                                              |
| E1 — TSDoc of `selectionAfterToggle`           | fixed                                 | said Shift on the drawing moves the range start, against note 0006 and the code                                                                                                                                                                                                                |
| E1 — "editing the vertices" (`interaction.md`) | fixed                                 | now "moving, adding or deleting a vertex" turns a shape into a free contour; a radius per vertex keeps it what it is                                                                                                                                                                           |
| E1 — layout box and radii                      | business question                     | Q25 opened: one radius for a layout box, or one per corner (E13)                                                                                                                                                                                                                               |
| E3 — `testing.md`                              | fixed                                 | named only `mount-playground.test.ts`; the playground now has one test file per area                                                                                                                                                                                                           |
| E — consistency with ADR-0030, Q22–Q24         | consistent                            | local, free-V1 scope, no server, only `<path>` and no native stroke                                                                                                                                                                                                                            |
| F — quality gates                              | green                                 | `check:all`, coverage 100 %, 404 tests, no `.skip` / `.only`                                                                                                                                                                                                                                   |

Tests per criterion: F03.AC1 13, AC2 7, AC3 6, AC4 6, AC5 14.

## G. Next feature (F08, node editing)

Sources and questions for its spike:

- Moving a vertex by dragging and by typing x and y, in integers: pointer events and their coordinates in the SVG frame (`getScreenCTM`, the SVG 2 coordinate systems, `REF-SVG2-COORDS`).
- Adding a vertex on an edge and deleting one: where the new vertex goes (the point clicked, rounded to integers?), which radius it gets (0, as a new vertex of F03), how the contour order (Q11) is kept.
- The voluntary action that turns a rectangle or a regular polygon into a free contour: its name and place (a button in Points mode?), and what the model of a free contour stores (integer vertices, radii, no anchor).
- Concave contours: the known limit of `DERIV-local-radius-clamp` (an arc touching a non-adjacent edge) becomes reachable — detect, not correct.

## H. Light evolvability check

| Question            | Answer                                                                                                                                                                 |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 74 modules in `src/` (model 27, geometry 26, math 11, io 6, render 4); 404 tests; the playground grew to about 40 files, each within its limits                        |
| Tools               | StrykerJS still blocked (issue 6210): 14 mutations by hand; `knip` left to the E01 retrospective                                                                       |
| Fitness functions   | the pure-layer lint does not cover `src/interaction`; pure interaction decisions went to `model` (architecture doc) — revisit when the editor applications start       |
| Patterns            | playground test helpers repeated (improvement log); page state in DOM attributes (improvement log, CHK-004)                                                            |
| Agent configuration | the run picked up `main` before AUD-004 (new step 1, PR #83): no conflict but the improvement log's realigned table                                                    |
| Process             | 6 stories merged by the run (#79, #80, #81, #82, #84, #87); every review changed its story; three real defects caught by reviews (radius coming back, radii lost, cap) |
