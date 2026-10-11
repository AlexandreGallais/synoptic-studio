---
id: F04
epic: E01
title: Free rotation of shapes
status: draft
---

# E01 · F04 — Free rotation of shapes

**Benefit hypothesis**: the designer tilts a shape by any integer angle (e.g. 45°).

## Acceptance criteria

1. A shape rotates by an integer angle in degrees; the angle is stored, the rotated geometry derived.
2. Rotation composes with corner radius.

**Product Owner's wish (F02 refinement, 2026-10-09)**: a polygon always has a flat side at the bottom; turning it by any angle puts that side where wanted — e.g. the ISA diamond is a square turned by 45°, the IEC hexagon standing on a vertex a hexagon turned by 30° (research 0004). Rotation applies to polygons as to rectangles.

**Product Owner's rule (F07 refinement, 2026-10-10)**: the anchor of F07 is set inside the box first, then the box rotates with its content: a triangle whose base touches the bottom, turned by 90°, has that base on the left (« c'est la boîte du polygone qu'on fait tourner »).

**Product Owner's wishes (VAL-005 of F08, 2026-10-11)**, for the refinement of F04 and the editor applications:

- A **rotation mode** entered by a button, as Points mode is, showing the **center of rotation**, which can be moved anywhere.
- The center can be put on a point of the shape, on the center of the shape, and later on a point or the center of another shape: moving the pointer near one highlights it, to be clicked (snapping, `interaction.md` §4).
- Buttons turning by 90° (as in Inkscape), and an angle typed in a field (as in Figma), the angle always shown.
- Later, for animation: a property may turn a part by an angle from its value, about a chosen center or point (`symbols-and-views.md` §3).

## Stories

To be written in refinement, after F01.
