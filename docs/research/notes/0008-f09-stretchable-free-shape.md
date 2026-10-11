# 0008 — Sources of F09: stretchable free shape

Spike SP-006, 2026-10-11. Claude Code's own research; every source below was read. (Note 0007, for F08, is on the paused branch `feature/f08-node-editing`.)

## Stretch (EN-012)

- SVG maps a box onto another with one scale per axis when `align` is `none` (`REF-SVG2-COORDS` §8.2), and CSS turns a transform about an origin into translate, transform, translate back (`REF-CSS-TRANSFORMS` §3). With the top-left of the box as origin, computed as `oₓ + w · (pₓ − oₓ) / nₓ` (multiplying first, so that the far edge is exactly `w`), the drawn box has the asked width and height and its top-left does not move (`DERIV-free-shape-stretch` steps 1–2). `fitInBox` is the same map with one scale; it stays apart (one scale, scale first, an anchored place, exact values fixed by F02 and F07 tests), and `stretchToSize` computes per axis, multiplying first, the top-left kept.
- Without loss: the stored points never change; the drawing depends on them and on the size only (step 3). A width of 0, then back, gives the same shape.
- Stored points spanning no width or no height (all on one line) cannot be stretched: such a free shape is refused (ADR-0032). The rectangle tool never stores one.

## The rectangle (EN-012, US-017)

- The rectangle tool stores the unit square, top-left first, clockwise (Q11): stretched to `w × h`, it draws `(0, 0), (w, 0), (w, h), (0, h)`, exactly the vertices of F01 (`1 · w` and `0 · w` are exact in floating point) — step 4.
- `Rectangle` becomes the rectangle tool's input; the stored shape is the free shape (ADR-0032). Its radii are those of ADR-0029, one per point.

## Migration (EN-012, US-017)

- Kept as the rectangle tool's input: `Rectangle` (`width`, `height`, `radii`), `isValidRectangle`, `RECTANGLE_CORNER_COUNT`.
- New: the free shape (stored points, width, height, radii), its validity (sizes and radii integers ≥ 0, one radius per point, at least 3 points, a stored box of positive width and height), its drawn points and corners, the rectangle tool turning a `Rectangle` into a free shape.
- `rectangleContour` and `rectangleCorners` become the free shape's drawn points and corners: the F01 and F03 tests stay as the regression proof of F09.AC1, run on the free shape made by the tool. The grey box of a polygon (F07) is drawn the same way.
- Playground: the rectangle is read as a `Rectangle`, turned into a free shape, shown as such in the model panel (its `width`, `height` and `radii` keys stay at the top, so the existing tests keep reading them); the list of points shows the drawn points.

## Open

- Where a free shape sits: F09 keeps the rectangle at the origin, as before. Typed points and whether stored points stay integers: decided with F08. Placing shapes (x, y of a shape) comes with the shape tree (E13) or the editor (E02).
