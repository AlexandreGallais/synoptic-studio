---
id: F08
epic: E01
title: Node editing
status: draft
---

# E01 · F08 — Node editing

**Benefit hypothesis**: the designer reshapes any contour vertex by vertex.

Split from F03 (Product Owner, 2026-10-11).

Refined with the Product Owner on 2026-10-11 (started with `/run`). Since F09 the rectangle is a free shape from the start and width and height are stored (ADR-0032): the decisions below that say otherwise are to revisit with Q27 when F08 resumes, together with the Product Owner's remark of VAL-006: store each point with its radius, as a corner (« mettre les radis ailleurs, je trouve ça trop chelou »). **Paused after SP-005** (2026-10-11): F09 (stretchable free shape) comes first; the stories below go back to `draft`, to be rewritten on F09's free shape, then set `ready` again with the Product Owner.

## Decided in refinement (Product Owner, 2026-10-11)

- **A regular polygon becomes a free shape by a dedicated button**, « un bouton dédié pour convertir en forme libre »: it loses its number of corners and its anchor. Converting **does not move it**: « Le but, c'est que je le convertis en forme libre et il le bouge pas ». Its vertices keep their exact, fractional coordinates and its radii.
- **A rectangle needs no conversion**: its points are moved directly, as it loses nothing, « le rectangle, en soi, on ne perd aucune option »; it then becomes a free shape of 4 points with its radii, and putting the point back gives the same drawing.
- **A point moved lands on integers**: « si je commence à bouger des points, là, je suis limité aux entiers ». Only the points moved; the others keep their coordinates.
- **Width and height of a free shape are its bounding box**: « La largeur et la hauteur, c'est la b-box de la forme. Donc, si tu déplaces les points, elle va bouger. »
- **Moving a point**: by typing its x and y in the list of points. Dragging it on the drawing belongs to the editor applications (E02), « les points drag and drop, c'est plutôt côté IHM ».
- **Adding a point**: a button « Ajouter un point au milieu », active when two neighbouring points are selected; the new point is exactly in the middle, fractions allowed, so that the edge does not change: « il faut que le point vienne vraiment au milieu. S'il y a des décimales, ce n'est pas grave »; it has no rounding.
- **Deleting points**: a Delete button and the Delete key, on a free shape or a rectangle; a shape keeps at least 3 points, the deletion is refused below with a message. A regular polygon is converted first: deleting one of its points is refused with a message saying to convert it.
- **Several points selected move together** (for the editor applications, with dragging).
- **Not in F08**, written for later: dragging points, stretching a shape by its width and height from a fixed point chosen in the 3 × 3 grid (as Inkscape), the warning shown when converting with a « do not show again » box (E02, editor applications, « tout ce qui est truc du UX, pour moi, ça doit aller plus tard dans une epic »); cutting, joining or deleting edges and adding connections between points (E03).

## Open in refinement

- Deleting a point of a free shape joins its two neighbours with a straight edge, so that the shape stays closed (a symbol's shape is a closed contour, `shapes.md` §1). To confirm with the Product Owner at the validation: « je n'ai pas envie que, si on supprime un point, les autres points se rattachent » was said about the regular polygon.

## Acceptance criteria

1. A regular polygon is turned into a free shape by a button: the same vertices, the same radii, the same drawing; it no longer has a number of corners nor an anchor.
2. A point of a rectangle or of a free shape is moved by typing its x and y, integers; a rectangle whose point moves becomes a free shape of 4 points with its radii; width and height show the bounding box.
3. With two neighbouring points selected, a button adds a point exactly in the middle of their edge, without rounding, the edge unchanged.
4. Selected points of a rectangle or a free shape are deleted by a button or the Delete key, their neighbours joined; at least 3 points remain, otherwise the deletion is refused with a message; on a regular polygon it is refused with a message to convert it first.

## Stories

In delivery order.

| ID                                                             | Type       | Title                                       | Status |
| -------------------------------------------------------------- | ---------- | ------------------------------------------- | ------ |
| [SP-005](../stories/E01-F08-SP-005-research-f08.md)            | Spike      | Research for F08                            | ready  |
| [EN-011](../stories/E01-F08-EN-011-free-shape.md)              | Enabler    | A free shape in the model                   | draft  |
| [US-014](../stories/E01-F08-US-014-convert-and-move.md)        | User story | Convert a polygon, move a point by its x, y | draft  |
| [CHK-005](../stories/E01-F08-CHK-005-checkpoint-f08.md)        | Checkpoint | Checkpoint in the middle of F08             | draft  |
| [US-015](../stories/E01-F08-US-015-add-middle-point.md)        | User story | Add a point in the middle of an edge        | draft  |
| [US-016](../stories/E01-F08-US-016-delete-points.md)           | User story | Delete points                               | draft  |
| [AUD-005](../stories/E01-F08-AUD-005-audit-f08.md)             | Audit      | Audit F08: node editing                     | draft  |
| [VAL-005](../stories/E01-F08-VAL-005-validate-node-editing.md) | Validation | Validate F08 on the reference cases         | draft  |

## Feature plan

| Acceptance criterion                       | Realized by    | Verified by                      |
| ------------------------------------------ | -------------- | -------------------------------- |
| 1 — polygon converted by a button, unmoved | EN-011, US-014 | `[F08.AC1]` tests, card, VAL-005 |
| 2 — a point moved by its x, y, on integers | EN-011, US-014 | `[F08.AC2]` tests, card, VAL-005 |
| 3 — a point added in the middle of an edge | US-015         | `[F08.AC3]` tests, card, VAL-005 |
| 4 — points deleted, 3 at least             | EN-011, US-016 | `[F08.AC4]` tests, card, VAL-005 |
