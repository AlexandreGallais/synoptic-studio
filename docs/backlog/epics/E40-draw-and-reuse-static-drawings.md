---
id: E40
title: Draw and reuse static drawings
status: draft
---

# E40 — Draw and reuse static drawings

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: free drawings reused. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E10; needs E27 and E38.

| Field | Content |
| --- | --- |
| For | business users |
| who | need a static drawing (a tank's outline, a building) reused across views |
| the solution | the drawing tools and the drawing library |
| is a | a group saved as a reusable drawing |
| that | draws with a Figma-like pen (Q12, ADR-0018) |
| unlike | a drawing copied view by view |
| our solution | a group becomes a drawing, kept in a library shared between projects |
| Business outcomes | a business user draws, groups and saves a drawing, then reuses it in another view and project |
| Leading indicators | features accepted |
| Non-functional requirements | none beyond the shapes' |
| In scope | the pen, a group saved as a drawing, the drawing library (`shapes.md` §8) |
| Out of scope | parameters and animations |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The pen | a Figma-like pen placing points; a point dragged pulling its handles; a drawing's tools kept few (3) | to refine |
| A group saved as a drawing | a selected group saved as a drawing; placed in a view (2) | to refine |
| The drawing library | the library of drawings; a drawing reused in another view and another project (2) | to refine |

Size: 3 features, 7 stories, 20 + 25 × 7 + (45 to 75) × 3 = 330 to 420 active minutes, 5.5 to 7 hours (note 0034, §1).

## Taken from the old epics

- E10.1 to E10.3: the pen, a group saved as a drawing, the drawing library.

## Open points

None known beyond the refinement's own questions.
