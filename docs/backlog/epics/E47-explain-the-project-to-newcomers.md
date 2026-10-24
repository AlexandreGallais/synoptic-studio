---
id: E47
title: Explain the project to newcomers
status: draft
---

# E47 — Explain the project to newcomers

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the project explained in a few pages. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E12; needs E45.

| Field | Content |
| --- | --- |
| For | newcomers: users, contributors, reviewers |
| who | need to understand and trust the product quickly |
| the solution | an overview, manuals and the traceability page |
| is a | the release's documents written for humans |
| that | summarize, and link to the detailed pages |
| unlike | a reader left with hundreds of detailed pages |
| our solution | every behavior sourced and tested, summarized in a few documents |
| Business outcomes | each editor has a user manual; an overview explains the project in a few pages |
| Leading indicators | features accepted |
| Non-functional requirements | documentation complete but not excessive; summaries link to the detailed pages |
| In scope | the overview, the architecture summary, the getting-started guide, the user manuals, the traceability page (ADR-0027) |
| Out of scope | npm publication (never, Q40); hosting a service |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The overview | the project in a few pages with its architecture summary; a getting-started guide (2) | to refine |
| User manuals | the Symbol Editor's; the Configurator's; the View Editor's (3) | to refine |
| The traceability page | the matrix of criteria, stories, tests and results generated (ADR-0027); linked from the overview (2) | to refine |

Size: 3 features, 7 stories, 20 + 25 × 7 + (45 to 75) × 3 = 330 to 420 active minutes, 5.5 to 7 hours (note 0034, §1).

## Taken from the old epics

- E12.1 to E12.4, E12.9: the overview, the architecture summary, the user manuals, the getting-started guide, the traceability page.
- Dropped: E12.5, npm publication, the library being never published on npm (Q40, 2026-10-17, `editions.md` §1).
- Not placed: E12.10, a playground asking its own test questions, kept as a row of `improvements.md` until a feature needs it (RET-004).

## Open points

None known beyond the refinement's own questions.
