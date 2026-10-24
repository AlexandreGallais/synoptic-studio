---
id: E25
title: Keep the history fast
status: draft
---

# E25 — Keep the history fast

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the history's cost with many shapes. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E18; needs E02.

| Field | Content |
| --- | --- |
| For | symbol and view designers |
| who | work on drawings of up to about 1 000 shapes |
| the solution | a history whose steps cost what they change |
| is a | the history of F40, kept fast |
| that | a step on 1 000 shapes costs what one on one shape does |
| unlike | 9 ms a step on 1 000 shapes today (AUD-043) |
| our solution | a step, an undo and a redo changing only the shapes they change (ADR-0095) |
| Business outcomes | about 2 ms a step on 1 000 shapes |
| Leading indicators | the step's time measured |
| Non-functional requirements | the history's bounds of Q7 unchanged |
| In scope | the step's cost |
| Out of scope | the list of steps scrolled whole (E45, decided on 2026-10-23) |

## Features

Proposed at SP-033 (2026-10-25) as a draft: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2). A single feature of a single story: the list scrolled whole, its second story in the proposal, stays with the interface designed (the agent's placement, 2026-10-25).

| Feature | Expected stories | Status |
| --- | --- | --- |
| A history kept fast | a step, an undo and a redo changing only the shapes they change, a step on 1 000 shapes costing what one on one shape does (9 ms today, about 2 ms aimed, AUD-043) (1) | to refine |

Size: 1 feature, 1 story, 20 + 25 × 1 + (45 to 75) × 1 = 90 to 120 active minutes, 1.5 to 2 hours (note 0034, §1).

## Taken from the old epics

- E18.6: a history step kept cheap with 1 000 shapes (AUD-043).

## Open points

None known beyond the refinement's own questions.
