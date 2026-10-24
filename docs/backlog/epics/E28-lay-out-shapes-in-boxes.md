---
id: E28
title: Lay out shapes in boxes
status: draft
---

# E28 — Lay out shapes in boxes

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: a box that places its children. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E13; needs E27.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need children placed in a box without computing their positions |
| the solution | a layout box, its anchor and its stacks (Q21) |
| is a | a rectangular box placing its children, as a Figma frame with a layout, made simple |
| that | places each child by a 3 × 3 anchor, or stacks them across or down |
| unlike | each child's position typed by hand |
| our solution | a child may override the box's anchor, fixed bottom left for example, and stay laid out by the box |
| Business outcomes | a box holding a polygon, a rectangle and a text places them by its anchor, one child overriding it, and the result stays editable |
| Leading indicators | features accepted |
| Non-functional requirements | derived positions never written back into the model (ADR-0003); integer inputs only |
| In scope | the layout box of `shapes.md` §3, the anchor per child, rows and columns with gaps, padding; the box's radii (Q25) |
| Out of scope | a box resizing to its children (Figma's hug); a non-rectangular box |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| A box placing its children by anchor | the box as a rectangle with its radii (Q25); its children placed by the 3 × 3 anchor, F07's reused; the name sourced (3) | to refine |
| A child's own anchor | a child overriding the anchor; leaving the stack completely (2) | to refine |
| Rows and columns | a stack across or down with its gap; padding; a child hidden leaving its place empty (3) | to refine |

Size: 3 features, 8 stories, 20 + 25 × 8 + (45 to 75) × 3 = 355 to 445 active minutes, 5.9 to 7.4 hours (note 0034, §1).

## Taken from the old epics

- E13.2 to E13.7, E13.10: the layout box, a child's anchor, stacks fixed once the symbol is used, F07's anchor reused, the name to source, Q25.

## Open points

- Q25, the layout box's corner radii: one for the whole box or one per corner, asked at the refinement.
