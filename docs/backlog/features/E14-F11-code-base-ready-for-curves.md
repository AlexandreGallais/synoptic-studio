---
id: F11
epic: E14
title: A code base ready for curves
status: in-progress
---

# E14 · F11 — A code base ready for curves

**Benefit hypothesis**: the debt E01 left — repeated test helpers, two path writers, a folder of 50 model files, functions without a property test — is paid before curves add a second kind of point, so that E14 builds on one structure instead of two.

Written at RET-001 (2026-10-13), at the Product Owner's request: plan the clean-up and the folder structure before the next epic. Its stories are drafts; the Product Owner sets them `ready`.

## Refinement

Decided with the Product Owner on 2026-10-13:

| Question                                        | Decision                                                                                                                                              |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| The older path writer, `contourToPathData`      | removed from the public API: one writer in the library; its tests compare with written-out path data, e.g. `M0 0 L100 0 L100 50 L0 50 Z`              |
| Most files per folder                           | at the end of SP-010: folders by business theme, 15 modules as a signal rather than a rule to split blindly, the playground split by panel (ADR-0042) |
| Validation of a feature that changes no drawing | a demo page of figures before and after (largest folder, functions without a property test, repeated helpers, `check:all` green), no guided test      |
| A place for curves in the contour pieces        | out of scope: it comes after the ADR that supersedes ADR-0001, in the next feature of E14                                                             |

## Acceptance criteria

1. The test helpers repeated across files (typing in a field, reading a text, mounting the playground, building a rectangle, the shoelace area) live once, in shared test modules.
2. The library has one writer of path data, `contourPiecesToPathData`; `contourToPathData` is removed from the public API (a breaking change: the next version is 0.2.0), and `SEPARATOR` and `CLOSE_PATH` are declared once.
3. A rounded contour computes each corner's triple and turning angle once.
4. The folders of `src/` and `playground/` follow the layout of ADR-0042, written in `docs/conventions/files.md`: folders by business theme, a shape's type in its folder; a folder past 15 modules (tests not counted) is listed there with its reason, checked by a test.
5. Every function of `src/math/` and `src/geometry/` has a property test: the list of `tests/fitness.test.ts` is empty.
6. Behavior is unchanged: every existing test passes, with the same values.

## Stories

In delivery order.

| ID                                                         | Type       | Title                                                     | Status |
| ---------------------------------------------------------- | ---------- | --------------------------------------------------------- | ------ |
| [SP-010](../stories/E14-F11-SP-010-research-f11.md)        | Spike      | Research for F11: folder layouts and test placement       | done   |
| [EN-020](../stories/E14-F11-EN-020-property-tests.md)      | Enabler    | Property tests for the nine functions without one         | done   |
| [EN-021](../stories/E14-F11-EN-021-shared-test-helpers.md) | Enabler    | Shared test helpers, one path writer, one pass per corner | done   |
| [CHK-010](../stories/E14-F11-CHK-010-checkpoint-f11.md)    | Checkpoint | Checkpoint in the middle of F11                           | done   |
| [EN-022](../stories/E14-F11-EN-022-folder-layout.md)       | Enabler    | Folders of the library and the playground                 | done   |
| [AUD-010](../stories/E14-F11-AUD-010-audit-f11.md)         | Audit      | Audit F11                                                 | ready  |
| [VAL-010](../stories/E14-F11-VAL-010-validate-f11.md)      | Validation | Validate F11                                              | draft  |

## Feature plan

| Acceptance criterion | Realized by    | Verified by             |
| -------------------- | -------------- | ----------------------- |
| F11.AC1              | EN-021         | AUD-010                 |
| F11.AC2              | EN-021         | AUD-010                 |
| F11.AC3              | EN-021         | AUD-010                 |
| F11.AC4              | SP-010, EN-022 | AUD-010                 |
| F11.AC5              | EN-020         | `tests/fitness.test.ts` |
| F11.AC6              | every story    | `npm run check:all`     |
