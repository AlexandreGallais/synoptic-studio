---
id: E48
title: Check each release
status: draft
---

# E48 — Check each release

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the suites and budgets of a release. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E12; needs E45.

| Field | Content |
| --- | --- |
| For | the Product Owner and the users |
| who | need each release to keep what the last one did |
| the solution | end-to-end and regression suites, a size budget, the public API report |
| is a | the release's checks |
| that | run on each release |
| unlike | a release checked by hand |
| our solution | each editor's end-to-end suite, a regression suite, a size budget and the API report |
| Business outcomes | a release refused when a suite fails or a budget is passed |
| Leading indicators | features accepted |
| Non-functional requirements | no runtime dependency |
| In scope | regression and end-to-end tests (ADR-0020), the size budget, the public API report |
| Out of scope | npm publication (never, Q40) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| End-to-end and regression suites | the end-to-end suite of each editor; the regression suite; both on each release (3) | to refine |
| Budgets and the public API | a size budget; the public API report (2) | to refine |

Size: 2 features, 5 stories, 20 + 25 × 5 + (45 to 75) × 2 = 235 to 295 active minutes, 3.9 to 4.9 hours (note 0034, §1).

## Taken from the old epics

- E12.6 to E12.8: the regression and end-to-end suites, the size budget, the public API report.

## Open points

None known beyond the refinement's own questions.
