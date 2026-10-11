# 0007 — Sources of F08: node editing

Spike SP-005, 2026-10-11. Claude Code's own research; every source below was read.

## Free shape (EN-011)

- Inkscape tells a **shape** (rectangle, ellipse, star, spiral, edited by its own handles) from a **path** (edited node by node); « Path ‣ Object to path » turns the first into the second, and « a path can never be converted back into an object » (`REF-INKSCAPE-OBJECT-TO-PATH`). The free shape of F08 is that path: a regular polygon becomes one by a button, never back (Product Owner, F08).
- Its model: the vertices in contour order (Q11) and one radius per vertex (ADR-0029). The vertices of a converted polygon keep their fractional coordinates, so that the shape does not move; a vertex typed by the user is an integer (ADR-0031, Product Owner).
- Width and height are its bounding box (Product Owner), as `boundingBox` already computes for the fit of F02 (`DERIV-regular-polygon-fit` step 3).
- A rectangle needs no conversion (Product Owner): it becomes a free shape of its 4 vertices and its radii when a vertex moves or is deleted.

## Adding and deleting (US-015, US-016)

- Inkscape inserts a node with a button once several nodes are selected, and deletes the selected nodes with the button or the `Del` key (`REF-INKSCAPE-NODE-OPS`); a node is inserted « right in the middle between two other nodes »; the page does not say what the path becomes after a deletion.
- The new point is the midpoint of the edge, ((x₁ + x₂)/2, (y₁ + y₂)/2) (`REF-MATHWORLD-MIDPOINT`): on the edge, so the edge does not change; fractional when the sum is odd, which ADR-0031 allows. Its radius is 0, as a new vertex of F03. The last and the first vertices are neighbours too (the contour is cyclic, Q11).
- Deleting a vertex removes it and its radius from the cyclic list: its two neighbours become consecutive, so they are joined by a straight edge and the shape stays closed. At least 3 vertices remain (Product Owner).

## Concave contours

Moving a vertex can make the contour concave. `DERIV-local-radius-clamp` notes that an arc may then touch a non-adjacent edge: a known limit, to detect, not to correct. Not in F08's criteria; recorded in the improvement log for the audit to follow.

## Coordinates

Typed x and y are safe integers, negative allowed (Product Owner, 2026-10-11). Each vertex keeps its number wherever it moves (Product Owner). Adding a point splits an edge, so the rounding of a neighbouring corner may shrink: accepted by the Product Owner, as the point was asked for.

## Open

- Deleting a vertex joins its neighbours: confirmed by the Product Owner at VAL-005 (Q28).
