---
id: F04
epic: E01
title: Free rotation of shapes
status: draft
---

# E01 · F04 — Free rotation of shapes

**Benefit hypothesis**: the designer tilts a shape by any integer angle (e.g. 45°).

Refined with the Product Owner on 2026-10-11 (started with `/run` after F08's merge).

## Wishes before the refinement

**Product Owner's wish (F02 refinement, 2026-10-09)**: a polygon always has a flat side at the bottom; turning it by any angle puts that side where wanted — e.g. the ISA diamond is a square turned by 45°, the IEC hexagon standing on a vertex a hexagon turned by 30° (research 0004). Rotation applies to polygons as to rectangles.

**Product Owner's rule (F07 refinement, 2026-10-10)**: the anchor of F07 is set inside the box first, then the box rotates with its content: a triangle whose base touches the bottom, turned by 90°, has that base on the left (« c'est la boîte du polygone qu'on fait tourner »).

**Product Owner's wishes (VAL-005 of F08, 2026-10-11)**: a rotation mode with a movable center, snapping to points; buttons turning by 90° (as in Inkscape) and an angle typed in a field (as in Figma), the angle always shown; later, animated rotation. The rotation mode is for E02 (below).

## Decided in refinement (Product Owner, 2026-10-11)

- **Center**: a shape turns about the **center of its box**, « par défaut, c'est au centre de la B-box ». Only shapes turn: « un point, ça ne sert à rien de le faire tourner ».
- **Direction**: a positive angle turns **clockwise** on screen, « angle positif, c'est dans le sens horaire ».
- **Range**: the angle is kept **from 0 to 359**: 370 becomes 10, −90 becomes 270, « c'est mieux de 0 à 359, franchement ».
- **Fields of a turned shape, as Figma**: width and height stay those of the shape before it turned; turning back to 0 gives the same shape. In Points mode, x and y are those of the turned point in the SVG (Q27). A turned polygon converted to a free shape keeps its angle.
- **Point numbers never change**: « on ne renumérote jamais, sauf avec les polygones \[…\] si j'enlève un, ça fait −1 » (already decided at F08: « mon premier point, c'est mon premier point, peu importe où il est »).
- **Two boxes**: the **turned box** of the shape, drawn as its selection, so that a turned square does not seem to grow; and the **straight box**, the smallest upright rectangle around the drawn shape, rounded corners included, as a browser computes it (`getBBox`), which grows at 45°. The straight box is drawn discreetly (dashed) with its x, y, width and height: « solution 1 me plaît mieux que les autres »; hiding it is an option for the editor applications.
- **Position**: every shape gets a position, the **x and y of its straight box**, typed to move it, as in Inkscape (« la vraie get bbox de l'SVG et qu'on puisse bouger X Y comme on veut »). Turning a square by 45° makes its x and y smaller, as its straight box grows about the same center. Without a position, a turned shape would jump when its width or a point changes, its center moving with its box.
- **Editing a turned shape**: changing the width keeps the shape's turned top-left corner in place, as Figma; a point typed leaves the others where they are (« le point bouge, mais le reste non »).

## Left to the agent

- The straight box is computed in F04: the view must contain a turned shape anyway (the Product Owner: « À toi de choisir ça »).
- The x and y shown are those of the top-left of the straight box; choosing another of its places (Inkscape's reference point) is for E02.

## Not in F04

For the editor applications (E02, « Ideas from the Product Owner »): the rotation mode and its movable center, dragging the handles of the turned box, the reference point of x and y, hiding the straight box. Animated rotation: E05.

## Acceptance criteria

1. Every shape has an angle, an integer number of degrees, clockwise on screen, kept from 0 to 359 (370 → 10, −90 → 270); it is drawn turned about the center of its box; the angle is stored, the turned geometry derived.
2. Turning keeps the shape: its corner radii, its anchor (the box turns with its content), its width and height and its point numbers are unchanged; back to 0, the drawing is the straight shape again.
3. Every shape has a position, the x and y of its straight box (the smallest upright rectangle around the drawn shape, rounded corners included), typed as integers to move it; turning does not move the center of its box.
4. On a turned shape, a width or height typed keeps its turned top-left corner in place; a point typed in the SVG's x and y leaves the other points where they are; a turned polygon converted to a free shape keeps its angle and its drawing.
5. The playground shows the angle in a field, buttons turning by −90° and +90°, the turned box as the selection, and the straight box dashed with its x, y, width and height.

## Stories

In delivery order.

| ID                                                              | Type       | Title                                   | Status |
| --------------------------------------------------------------- | ---------- | --------------------------------------- | ------ |
| [SP-007](../stories/E01-F04-SP-007-research-f04.md)             | Spike      | Research for F04                        | draft  |
| [EN-014](../stories/E01-F04-EN-014-angle-and-position.md)       | Enabler    | An angle and a position for every shape | draft  |
| [EN-015](../stories/E01-F04-EN-015-straight-box.md)             | Enabler    | The straight box of a drawn shape       | draft  |
| [US-018](../stories/E01-F04-US-018-turn-a-shape.md)             | User story | Turn and move a shape in the playground | draft  |
| [CHK-007](../stories/E01-F04-CHK-007-checkpoint-f04.md)         | Checkpoint | Checkpoint in the middle of F04         | draft  |
| [US-019](../stories/E01-F04-US-019-edit-a-turned-shape.md)      | User story | Edit a turned shape                     | draft  |
| [AUD-007](../stories/E01-F04-AUD-007-audit-f04.md)              | Audit      | Audit F04: free rotation of shapes      | draft  |
| [VAL-007](../stories/E01-F04-VAL-007-validate-free-rotation.md) | Validation | Validate F04 on the reference cases     | draft  |

## Feature plan

| Acceptance criterion                         | Realized by    | Verified by                      |
| -------------------------------------------- | -------------- | -------------------------------- |
| 1 — integer angle, clockwise, 0 to 359       | EN-014, US-018 | `[F04.AC1]` tests, card, VAL-007 |
| 2 — turning keeps the shape                  | EN-014         | `[F04.AC2]` tests, card, VAL-007 |
| 3 — position: x, y of the straight box       | EN-014, EN-015 | `[F04.AC3]` tests, card, VAL-007 |
| 4 — a turned shape edited without jumping    | US-019         | `[F04.AC4]` tests, card, VAL-007 |
| 5 — angle field, ±90° buttons, the two boxes | US-018         | `[F04.AC5]` tests, card, VAL-007 |
