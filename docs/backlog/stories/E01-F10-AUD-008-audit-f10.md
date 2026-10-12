---
id: AUD-008
epic: E01
feature: F10
title: "Audit F10: round shape"
status: ready
points: 1
---

# E01 · F10 · AUD-008 — Audit F10: round shape

Full check of F10 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F10, when audited, then every formula is re-derived, every expected value recomputed by hand, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F10 is re-verified online and dated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-008.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mathematics, mutation spot-checks, missing properties (1 h)
- [x] T2 — B to H. Sources, provenance, duplicates, consistency, quality gates, research list for the next feature, light evolvability check; findings table (1 h)

## Findings (2026-10-12)

Independent review by the `auditor` subagent (sections A to E, on a copy of the committed tree), mutation spot-checks by the agent (A4). Each story of F10 had its own review before (pull requests #134, #135, #136, #138).

| Check                                    | Result                                  | Evidence                                                                                                                                                                                                              | Action                                                                                              |
| ---------------------------------------- | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| A1, A2 — derivation and expected values  | correct                                 | `DERIV-round-shape` re-derived; every value of the tests recomputed; every size from 0 × 0 to 300 × 300 converted and compared piece by piece: no mismatch                                                            | —                                                                                                   |
| A4 — one mutation per function (6)       | all caught                              | `roundShapeCorners` radius (6 tests), `roundShapeAsFreeShape` ceil → floor (1), `isValidRoundShape` height (1), `shapeKind` round (3), `isFreeShape` (2), `shapeAsFreeShape` free (1); `ceil` → `round` is equivalent | —                                                                                                   |
| A5 — `roundShapeAsFreeShape`             | no property                             | three examples only                                                                                                                                                                                                   | added: a property over every size up to 300                                                         |
| A3 — 0 × 0                               | untested, "a line" in the domain        | it draws nothing                                                                                                                                                                                                      | test added; domain and TSDoc say it                                                                 |
| A3 — very large sizes                    | pieces of 2e-9 from a side of about 9e6 | the derivation states the limit                                                                                                                                                                                       | no action: disclosed, as for any rectangle at its maximal radius                                    |
| B — sources                              | quotes verbatim                         | 2 URLs re-opened on 2026-10-12                                                                                                                                                                                        | —                                                                                                   |
| C — provenance                           | written from the derivation             | one-line compositions of project functions                                                                                                                                                                            | —                                                                                                   |
| D1 — the conversion rule written twice   | two forms, the button and the writer    | `show-point-buttons`, `write-converted-shape`                                                                                                                                                                         | fixed: `isConvertible`                                                                              |
| E3 — `CLAUDE.md` state line              | F04 said not yet merged                 | released in 0.13.0                                                                                                                                                                                                    | fixed                                                                                               |
| E1 — kinds told by `anchor`              | against ADR-0036's one function         | `create-box-elements`                                                                                                                                                                                                 | fixed: `shapeKind`; `shapeAsFreeShape` keeps the test for TypeScript's narrowing, said in its TSDoc |
| E1 — playground TSDoc                    | named the polygon only                  | five files                                                                                                                                                                                                            | fixed                                                                                               |
| E1 — a derived radius shown as requested | `ShapeReading.radii`                    | the points list of a round shape                                                                                                                                                                                      | documented in `ShapeReading` and `interaction.md` §3                                                |
| E1, E4 — domain and epic                 | the round shape missing                 | `interaction.md` §3, E01 "In scope"                                                                                                                                                                                   | fixed                                                                                               |
| B2 — weekly link check                   | last run before the new references      | `links.yml` 2026-10-11                                                                                                                                                                                                | no action: it runs weekly on `main`, after the feature merge                                        |
| F — quality gates                        | green                                   | `check:all`, 657 tests, coverage 100 % of `src/`                                                                                                                                                                      | —                                                                                                   |

## G. Next feature

F05 (text), next in E01's order; the sources and questions of AUD-007 § G still stand (SVG `text`, `text-anchor`, the baseline, its box without font files, a turned text, fonts available offline).

## H. Light evolvability check

| Question            | Answer                                                                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 112 modules in `src/` (model 57, geometry 30, math 15, io 6, render 4), 657 tests; `model`'s subfolders are overdue (improvement log) |
| Tools               | StrykerJS still blocked; 6 mutations by hand, all caught                                                                              |
| Fitness functions   | `shapeKind` and `isConvertible` are the rules of the three kinds: a fourth kind changes them only                                     |
| Patterns            | the round shape reuses the rectangle tool's functions; no new geometry                                                                |
| Agent configuration | state line updated                                                                                                                    |
| Process             | a small feature still takes six pull requests; the Product Owner's requests on speed are in the improvement log                       |
