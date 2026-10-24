---
id: E04
title: Give shapes colors and exact strokes
status: dropped
---

# E04 — Give shapes colors and exact strokes

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E31](./E31-give-shapes-colors-and-strokes.md), the fill first, then the strokes.

Text moved to its own epic, E15, on 2026-10-12 (Product Owner). The shapes' fill and stroke colors joined it from E02 on 2026-10-19 (SP-025), as `interaction.md` §5 already placed them: a stroke's color comes with the stroke, and both reuse F16's color fields and picker.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need aligned outlines |
| the solution | computed strokes and text |
| is a | strokes computed as offset paths |
| that | inner, centered or outer strokes, exact joins, caps and dashes |
| unlike | the native SVG stroke |
| our solution | the outline is geometry, so it is exact |
| Business outcomes | aligned strokes with joins, caps and dashes |
| Leading indicators | stroke cases accepted |
| Non-functional requirements | exact offsets of arcs (ADR-0001) |
| In scope | fill color, stroke color, width, alignment, join, cap, dashes; strokes of the curves of E14 |
| Out of scope | text (E15) |
| Closure criteria | every property of `shapes.md` §6 demonstrated |

## Candidate features

| Feature | Status |
| --- | --- |
| A shape's fill color, with F16's fields and picker (from E02, SP-025) | to refine |
| Aligned stroke and its color; it decides whether the contour's band (`src/geometry/contour-band/`, `DERIV-band-within-box`), left without a caller by F44's US-086 and kept for it (SP-032, ADR-0099), serves a shape's stroke or goes | to refine |
| Joins, caps and dashes | to refine |
