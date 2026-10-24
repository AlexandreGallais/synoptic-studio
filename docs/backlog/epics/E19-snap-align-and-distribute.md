---
id: E19
title: Snap, align and distribute shapes
status: draft
---

# E19 — Snap, align and distribute shapes

A list of features grouped by theme, with no closing of its own (ADR-0086). Cut from E02 on 2026-10-19 (SP-025): the help to place shapes against each other, once E02 moves them and E18 resizes and turns them.

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
| Out of scope | snapping to ports (with the ports, E05); the shape builder (E03) |

## Features

Ordered by the agent at SP-025 (2026-10-19). A feature gets its id when it starts, its questions in its own short spike (ADR-0086).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Snap to objects | the targets: the boxes' corners, edges, centers and midpoints, the points; the tolerance in screen pixels; a mark where it snapped (3) | to refine |
| Guides and equal gaps | temporary horizontal and vertical guides when a point lines up with another object's; a stop where the gap to a neighbor equals an existing gap; an object winning over a guide, even farther (3) | to refine |
| Align and distribute | left, center, right, top, middle, bottom; equal horizontal and vertical gaps; on the boxes in the drawing's space, whatever the group (2) | to refine |
