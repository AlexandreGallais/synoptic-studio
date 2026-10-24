---
id: E21
title: Resize shapes by their box
status: draft
---

# E21 — Resize shapes by their box

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the box's handles and its fixed point. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E18 and the helps of REV-008; needs E02.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to resize shapes by the mouse without losing exact numbers |
| the solution | the box's eight handles and the 3 × 3 reference point |
| is a | the editors' resize tool |
| that | every handle dragged gives integer inputs, as typed numbers do |
| unlike | Width and Height typed, the box resized from its top-left only |
| our solution | Inkscape's box handles with the opposite side fixed, Shift keeping the proportions, x and y read at a chosen point of the box; every help drawn in the layer above the drawing |
| Business outcomes | a shape, or several, resized from any side or corner in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | integer inputs after every drag (ADR-0003); the helps' strokes kept off the drawing's SVG (ADR-0099) |
| In scope | the helps moved to the layer above (`interaction.md` §7), the eight handles (`shapes.md` §3), the reference point of x and y |
| Out of scope | turning (E22); dragged points (E23); a stroke's width kept when stretched (E31); snapping (E26) |

## Features

Proposed at SP-033 (2026-10-25) as drafts, the helps first (the agent's placement): a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The helps in the layer above | the box frames drawn as strokes in the layer above, not in the drawing's SVG; the points' and the curves' handles moved there too, with their halo (ADR-0099, decision 2) (2) | to refine |
| The box's eight handles | the handles drawn and dragged, the opposite side fixed, on integers; a regular polygon and a round shape stretched as their own rules say; Shift on a corner keeping the proportions (3) | to refine |
| The reference point of x and y | the 3 × 3 point shown and chosen; the x and y of the box of several shapes shown at it (2) | to refine |

Size: 3 features, 7 stories, 20 + 25 × 7 + (45 to 75) × 3 = 330 to 420 active minutes, 5.5 to 7 hours (note 0034, §1).

## Taken from the old epics

- E18.1, E18.10, E18.12 (its stretch part), E18.13: resize by the box, the reference point, a shape stretched from a fixed point.
- E18.3 (the straight box's handles), E18.14: the box of several shapes' position shown (SP-032).
- D.7: the helps still drawn in the drawing's SVG (REV-008).

## Open points

None known beyond the refinement's own questions.
