---
id: VAL-001
epic: E01
feature: F01
title: Validate F01 on the reference cases
status: done
points: 2
---

# E01 · F01 · VAL-001 — Validate F01 on the reference cases

Replay every acceptance criterion of F01 in the playground and against the derivation's check table.

## Acceptance criteria

- Given the playground, when each reference case of F01 is entered, then the drawing and the effective radius match the expected values.
- Given the check table of `DERIV-local-radius-clamp`, when each case is run, then the results match.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F01 is `done`.

## Tasks

One task = one commit, referenced as `VAL-001.Tn`. This story is not merged automatically (ADR-0028).

- [x] T1 — One test per criterion `[F01.AC1]`…`[F01.AC4]`, including a test of the playground in happy-dom (1.5 h)
- [x] T2 — Guided test in the playground: the nine steps of the US-003 card, one at a time, with what to do and what to look at (Product Owner, 2026-10-09) (2 h)
- [x] T3 — Demo page in `docs/guide/` in plain language (fillet, setback, clamp explained) (1.5 h)
- [x] T4 — Product Owner runs the guided test and validates; feedback recorded; feature pull request into `main` (0.5 h)

## Validation (2026-10-09)

The Product Owner ran the nine steps of the guided test in the local playground: every step behaved as they meant, refused values included (step 9).

Compared with what each step meant to show: every step was understood as intended; no misunderstanding of the need, no unclear step. Remarks, none blocking F01:

| Remark                                                                                                                                        | Where it goes                                                      |
| --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Change the radius vertex by vertex, by clicking a vertex to see its configuration (Figma-like), e.g. a rectangle with a single rounded corner | F03 (per-vertex corner radius and node editing), noted in its file |
| Measure the drawing precisely with the browser's SVG API (bounding box, lengths) to help the tests                                            | improvement log, for the end-to-end tests of E12                   |
| Specialized expert agents (mathematics, code, agile…) with their own documentation, running as background tasks                               | improvement log, for the E01 retrospective                         |
