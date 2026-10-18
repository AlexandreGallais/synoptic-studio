---
id: F03
epic: E01
title: Per-vertex corner radius
status: done
---

# E01 · F03 — Per-vertex corner radius

**Benefit hypothesis**: the designer rounds each corner of a shape on its own, e.g. a rectangle with a single rounded corner.

**Product Owner's wish (VAL-001, 2026-10-09)**: click a vertex to see its configuration and change its corner radius alone, as in Figma. The geometry of F01 already works per vertex (`Corner`); F01 and F02 give the same radius to every corner.

Refinement started on 2026-10-11 (started with `/run`).

## Decided in refinement (Product Owner, 2026-10-11)

- **Split**: F03 is the corner radius per vertex only; moving, adding and deleting vertices is [F08](./E01-F08-node-editing.md), right after.
- **An explicit points mode, not a double-click**, as Inkscape's node editing; the target users are less expert than Figma's, so a power-user gesture is never the only way. A vertex is hard to hit: its handle must be easy to click.
- **Playground**: a minimal tool bar with two large buttons, Shape and Points; in Points mode each vertex has a large handle, clicked to select it. The full tool menu (drawing, selection, points…) comes with the editor applications.

- **The radius of one corner**: in Points mode, the same Radius field applies only to the selected vertices, as in Inkscape. No "independent corners" switch. In Shape mode the field applies to every vertex, and shows **Mixed** when the radii differ.
- **Points of the selected shape only**, never those of the others. The first vertex of the path data is red, the second green, to read the direction.
- **A list of the vertices**, in the order of the path data: number, x, y, radius; a row clicked selects its vertex. Clicking the vertex on the drawing stays possible.
- **Several vertices**: in the list, Shift + click is a range and Ctrl + click (Cmd) adds or removes; on the drawing, Shift + click or Ctrl + click adds or removes, no range.
- **Number of corners changed**: radii kept by vertex number; new vertices at 0.
- **Not everything in the playground**: the Product Owner asked that every rule be written down for later, not built at once. The rules are in `docs/domain/interaction.md` §3 for the editor applications; moving a vertex is F08 (typed), dragging it E02.

## Acceptance criteria

1. Each vertex of a rectangle or a regular polygon stores its own requested corner radius, clamped locally (ADR-0007); a shape whose vertices all have the same radius is drawn exactly as in F01 and F02.
2. In Shape mode, the radius field gives one radius to every vertex, and shows "Mixed" when the radii differ.
3. Changing the number of corners of a polygon keeps the radii by vertex number; new vertices have radius 0.
4. In the playground, a tool bar with two large buttons, Shape and Points; in Points mode, the vertices of the shape have large handles, vertex 1 of the contour red and vertex 2 green, and a list shows them in contour order with x, y and radius.
5. In Points mode, vertices are selected by their handle or their list row (Shift and Ctrl / Cmd as in `docs/domain/interaction.md` §3), and the radius field applies to the selected vertices only.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-004](../stories/E01-F03-SP-004-research-f03.md) | Spike | Research for F03 | done |
| [EN-010](../stories/E01-F03-EN-010-radius-per-vertex.md) | Enabler | One requested radius per vertex in the model | done |
| [US-011](../stories/E01-F03-US-011-radius-field-mixed.md) | User story | Radius field for every corner, Mixed | done |
| [CHK-004](../stories/E01-F03-CHK-004-checkpoint-f03.md) | Checkpoint | Checkpoint in the middle of F03 | done |
| [US-012](../stories/E01-F03-US-012-points-mode.md) | User story | Points mode: handles, first and second marked | done |
| [US-013](../stories/E01-F03-US-013-points-list-selection.md) | User story | List of points, selection, radius per vertex | done |
| [AUD-004](../stories/E01-F03-AUD-004-audit-f03.md) | Audit | Audit F03: per-vertex corner radius | done |
| [VAL-004](../stories/E01-F03-VAL-004-validate-radius-per-vertex.md) | Validation | Validate F03 on the reference cases | done |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 — a requested radius per vertex, clamped | SP-004, EN-010 | `[F03.AC1]` tests, VAL-004 |
| 2 — radius field for every vertex, Mixed | US-011 | `[F03.AC2]` tests, card, VAL-004 |
| 3 — radii kept by vertex number | US-011 | `[F03.AC3]` tests, VAL-004 |
| 4 — tool bar, handles, first and second, list | US-012, US-013 | `[F03.AC4]` tests, cards, VAL-004 |
| 5 — selection, radius on the selection | US-013 | `[F03.AC5]` tests, card, VAL-004 |
