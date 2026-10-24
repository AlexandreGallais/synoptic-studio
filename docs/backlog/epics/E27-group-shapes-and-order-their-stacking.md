---
id: E27
title: Group shapes and order their stacking
status: draft
---

# E27 — Group shapes and order their stacking

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the shape tree. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E13 and a card met on the way; needs E02; before E29, whose booleans are nodes of the tree (ADR-0006).

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need shapes held together and drawn in the order they choose |
| the solution | groups in the model, the stacking order and duplicates |
| is a | the shape tree of the editors |
| that | a group is a nested `g`, its children kept as typed |
| unlike | the flat drawing of F35, the last drawn on top |
| our solution | groups made and undone by the usual keys, a child picked inside its group by Ctrl + click (Q50), shapes brought forward or back, duplicated with their offset remembered |
| Business outcomes | a symbol of grouped shapes, reordered and duplicated, in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | derived positions never written back into the model (ADR-0003); integer inputs only |
| In scope | the shape tree (groups, children, stacking order, `shapes.md` §7, `interaction.md` §6), group and ungroup, the order, duplicates (`interaction.md` §1, For later) |
| Out of scope | the layout box (E28); the layers panel (E44) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The shape tree in the model | groups and children in the model, the flat drawing migrated; a group drawn as a nested `g`; the stacking order kept in the tree (3) | to refine |
| Group and ungroup | grouping and ungrouping by the keys and buttons; a child picked inside its group; a group moved, deleted and resized proportionally (3) | to refine |
| Order and duplicates | brought forward, backward, to the front, to the back; a shape duplicated, its first move's offset remembered for the next copies (2) | to refine |

Size: 3 features, 8 stories, 20 + 25 × 8 + (45 to 75) × 3 = 355 to 445 active minutes, 5.9 to 7.4 hours (note 0034, §1).

## Taken from the old epics

- E13.1, E13.8: the shape tree in the model, a group resized proportionally (SP-032).
- D.1: duplicating a shape, its first move's offset remembered.

## Open points

None known beyond the refinement's own questions.
