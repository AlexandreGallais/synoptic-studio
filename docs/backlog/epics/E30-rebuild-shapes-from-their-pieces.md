---
id: E30
title: Rebuild shapes from their pieces
status: draft
---

# E30 — Rebuild shapes from their pieces

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: shapes rebuilt from their pieces. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E03; needs E29.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need a shape made of the regions and edges of others |
| the solution | the shape builder and operations on edges |
| is a | Inkscape's shape builder (`REF-INKSCAPE-SHAPEBUILDER`), non-destructive |
| that | keeps a region as inside A and outside B, recomputed from its operands |
| unlike | regions cut once and flattened |
| our solution | regions kept or merged by a click, contours cut, joined and connected, the result edited afterwards |
| Business outcomes | a shape built from the regions of three others stays editable |
| Leading indicators | features accepted |
| Non-functional requirements | exact on segments and arcs, as E29 |
| In scope | the shape builder, operations on edges (`shapes.md` §5) |
| Out of scope | the six operations (E29) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The shape builder | the regions built from the operands' edges; a region kept or merged, stored as inside A and outside B; the result edited afterwards (3) | to refine |
| Operations on edges | the contour cut between two points; two points joined, an edge deleted; a connection drawn over the shape, the shapes rebuilt from the pieces (3) | to refine |

Size: 2 features, 6 stories, 20 + 25 × 6 + (45 to 75) × 2 = 260 to 320 active minutes, 4.3 to 5.3 hours (note 0034, §1).

## Taken from the old epics

- E03.4, E03.5: the shape builder, operations on edges.

## Open points

- The curved operands of E29's first open point apply here too.
