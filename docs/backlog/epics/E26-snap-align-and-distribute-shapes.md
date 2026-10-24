---
id: E26
title: Snap, align and distribute shapes
status: draft
---

# E26 — Snap, align and distribute shapes

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: shapes placed against each other. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E19, kept whole; needs E21, E22 and E23.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need shapes placed exactly against each other, without computing positions |
| the solution | snapping, alignment guides, align and distribute |
| is a | the editors' placement aids |
| that | Inkscape-like snapping, its tolerance in screen pixels (`REF-INKSCAPE-SNAP`) |
| unlike | approximate mouse placement |
| our solution | a moved point stops on another object's corners, edges, centers and points, on a guide, or at an existing gap; shapes are aligned and evenly spaced by a button |
| Business outcomes | shapes are snapped, aligned and evenly spaced in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | the tolerance in screen pixels, the same at every zoom (`interaction.md` §4) |
| In scope | the snapping targets and rules of `interaction.md` §4, align and distribute of §5 |
| Out of scope | snapping to ports (E34); the shape builder (E30) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Snap to objects | the targets, the boxes' corners, edges, centers and midpoints, and the points; the tolerance in screen pixels; a mark where it snapped (3) | to refine |
| Guides and equal gaps | temporary guides when a point lines up; a stop at an existing gap; an object winning over a guide, even farther (3) | to refine |
| Align and distribute | the six alignments; equal gaps across and down, on the boxes in the drawing's space (2) | to refine |

Size: 3 features, 8 stories, 20 + 25 × 8 + (45 to 75) × 3 = 355 to 445 active minutes, 5.9 to 7.4 hours (note 0034, §1).

## Taken from the old epics

- E19.1 to E19.3: snap to objects, guides and equal gaps, align and distribute, unchanged.

## Open points

None known beyond the refinement's own questions.
