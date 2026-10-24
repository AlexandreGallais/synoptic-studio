---
id: E44
title: Edit the shape tree in a layers panel
status: draft
---

# E44 — Edit the shape tree in a layers panel

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the tree shown and edited as a list. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E13; needs E27.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to find, reorder, hide and lock shapes in a large symbol |
| the solution | a layers panel |
| is a | the shape tree shown as a list |
| that | reorders by a drag, renames, hides and locks |
| unlike | the tree reached only through the drawing |
| our solution | the tree of E27 listed beside the drawing |
| Business outcomes | a symbol's shapes reordered, renamed, hidden and locked from the panel |
| Leading indicators | features accepted |
| Non-functional requirements | none beyond the tree's |
| In scope | the layers panel (`interaction.md` §6) |
| Out of scope | the tree itself (E27) |

## Features

Proposed at SP-033 (2026-10-25) as a draft: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2). A single feature: the subject holds no more.

| Feature | Expected stories | Status |
| --- | --- | --- |
| The layers panel | the tree shown as a list and reordered by a drag; a layer renamed; hidden and locked (3) | to refine |

Size: 1 feature, 3 stories, 20 + 25 × 3 + (45 to 75) × 1 = 140 to 170 active minutes, 2.3 to 2.8 hours (note 0034, §1).

## Taken from the old epics

- E13.9: the layers panel, placed at the end of the project (`interaction.md` §6).

## Open points

None known beyond the refinement's own questions.
