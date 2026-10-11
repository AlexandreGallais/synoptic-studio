---
id: F03
epic: E01
title: Per-vertex corner radius
status: draft
---

# E01 · F03 — Per-vertex corner radius

**Benefit hypothesis**: the designer rounds each corner of a shape on its own, e.g. a rectangle with a single rounded corner.

**Product Owner's wish (VAL-001, 2026-10-09)**: click a vertex to see its configuration and change its corner radius alone, as in Figma. The geometry of F01 already works per vertex (`Corner`); F01 and F02 give the same radius to every corner.

Refinement started on 2026-10-11 (started with `/run`).

## Decided in refinement (Product Owner, 2026-10-11)

- **Split**: F03 is the corner radius per vertex only; moving, adding and deleting vertices is [F08](./E01-F08-node-editing.md), right after.
- **An explicit points mode, not a double-click**: « dans Inkscape, pour modifier les points, il y a une option de modification de points \[…\] au lieu de double clic »; the target users are « pas autant expert que celui de Figma », so a power-user gesture is never the only way. A vertex is hard to hit: its handle must be easy to click.
- **Playground**: a minimal tool bar with two large buttons, Shape and Points; in Points mode each vertex has a large handle, clicked to select it. The full tool menu (drawing, selection, points…) comes with the editor applications.

## Open in refinement

- How the radius is set in Points mode: the Radius field applied to the selected vertices (proposed, Inkscape), or a "independent corners" switch with one field per corner (Figma). The Product Owner leaned to the first, to be confirmed.

## Acceptance criteria

1. Each vertex has its own corner radius, clamped locally (ADR-0007).

## Stories

To be written in refinement.
