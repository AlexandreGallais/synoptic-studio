---
id: AUD-013
epic: E14
feature: F10
title: Audit F10
status: done
points: 1
---

# E14 · F10 · AUD-013 — Audit F10

Independent audit of F10 (ADR-0020): criteria traced, formulas re-derived, expected values recomputed, sources re-read, duplicates and leftovers looked for.

## Acceptance criteria

- Given F10's criteria, when the audit runs, then each is traced to code and tests.
- Given its findings, when the audit ends, then each correctness finding is fixed and the rest recorded.

## Tasks

One task = one commit, referenced as `AUD-013.Tn`.

- [x] T1 — Run the auditor and fix its findings (2 h)

## Findings

Auditor, on the whole feature (`develop...feature/f10-round-shape`), 2026-10-14: no correctness defect; every formula re-derived, every expected value recomputed, every new source re-read.

| Check | Result | Action |
| --- | --- | --- |
| A, mathematics | κ, the middle on the circle, the gap 1.00027253, the box, the turned corners: all match | none |
| Criteria | AC1 to AC5 tagged in tests; AC6 by `check:all` and the readers left unchanged | none |
| E, agent rules | `AGENTS.md` and `shapes.md` allowed only typed handles, while the round shape derives its own | fixed: ADR-0048 cited in both |
| E, feature file | E01's capsule and conversion still read as decided | fixed: marked superseded |
| B, claims without source | "as Figma's ellipse converted", "of common tools": not read | fixed: removed or attributed to the Product Owner |
| B, reference rows | `REF-WIKI-BEZIER` and `REF-POMAX-BEZIER` used for ADR-0048, dates not updated | fixed |
| E, glossary | no "Round shape" row | fixed |
| D, shape kinds | told apart three ways (`isFreeShape`, `"anchor" in`, `"kind" in`) | F13 pass 2 |
| D, per-axis product | `stretchedToEllipse` beside `stretchedHandles` | F13 pass 2 |
| D, test helper | `isDisabled` in two playground tests | F13 pass 2 |
| H, evolvability | small stories; the kinds are not a tagged union, the cause of the three ways above | F13 pass 2 |
