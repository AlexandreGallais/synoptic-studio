---
id: E31
title: Give shapes colors and strokes
status: draft
---

# E31 — Give shapes colors and strokes

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: a shape's paint. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E04; needs F16 (done).

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need shapes filled and outlined exactly |
| the solution | the fill color and the aligned stroke as geometry (ADR-0002) |
| is a | the shapes' paint |
| that | draws a stroke as an outline computed by offset, never the browser's stroke |
| unlike | every shape the page's blue `#4a7bd0` until now |
| our solution | F16's fields and picker for the fill and the stroke; width, alignment, joins, caps and dashes exact on segments and arcs |
| Business outcomes | aligned strokes with joins, caps and dashes, and filled shapes, in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | exact offsets of arcs (ADR-0001); colors stored as ADR-0056 says |
| In scope | fill color, stroke color, width, alignment, join, cap, dashes (`shapes.md` §6, `interaction.md` §5 Colors); the strokes of E14's curves; the contour's band (`src/geometry/contour-band/`) used or removed |
| Out of scope | text (E15, E32) |

## Features

Proposed at SP-033 (2026-10-25) as drafts, the fill first (the agent's placement): a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| A shape's fill color | the fill in the model with ADR-0056's check; F16's fields and picker in the panel (2) | to refine |
| The aligned stroke and its color | width and alignment on segments and arcs, by offset (ADR-0002), the contour's band reused or removed (the card of SP-032); its color; its width kept when the shape is stretched (3) | to refine |
| Joins and caps | miter, round and bevel with a miter limit; caps on open contours; the cases of `shapes.md` §6 (3) | to refine |
| Dashes | a pattern of integers; exact lengths along arcs (2) | to refine |

Size: 4 features, 10 stories, 20 + 25 × 10 + (45 to 75) × 4 = 450 to 570 active minutes, 7.5 to 9.5 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E04.1: a shape's fill color, with F16's fields and picker.
- E04.2, E04.3, E04.5: the aligned stroke and its color, joins, caps and dashes, `src/geometry/contour-band/` and `DERIV-band-within-box` kept for this epic to decide (SP-032, AUD-046 finding 8).
- E04.4: the strokes of E14's curves, unsized.
- E18.13 (its stroke part): a stroke's width kept when the shape is stretched.

## Open points

- The strokes of curves (Q4, open): the offset of an arc is an arc (`shapes.md` §6), the offset of a cubic is not a cubic; an approximation and its tolerance need a source, perhaps an ADR beside ADR-0001 (note 0034, §5).
