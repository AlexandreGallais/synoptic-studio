# 0008 — Sources of F09: stretchable free shape

Spike SP-006, 2026-10-11. Claude Code's own research; every source below was read. (Note 0007, for F08, is on the paused branch `feature/f08-node-editing`.)

## Stretch (EN-012)

- CSS Transforms builds a transform about an origin: translate by the origin, apply the transform, translate back (`REF-CSS-TRANSFORMS` §3); `scale(sx, sy)` scales each axis by its own factor (§9.1). With the top-left of the box as origin and `sx = w / nₓ`, `sy = h / n_y`, the drawn box has exactly the asked width and height and its top-left does not move (`DERIV-free-shape-stretch` steps 1–2).
- Without loss: the stored points never change; the drawing depends on them and on the size only (step 3). A width of 0, then back, gives the same shape.
- A box of no width or no height in the stored points (all points on one line) cannot be stretched: every point is drawn on that line. The rectangle tool never stores such a box.

## The rectangle (EN-012, US-017)

- The rectangle tool stores the unit square, top-left first, clockwise (Q11): stretched to `w × h`, it draws `(0, 0), (w, 0), (w, h), (0, h)`, exactly the vertices of F01 (`1 · w` and `0 · w` are exact in floating point) — step 4.
- `Rectangle` becomes the rectangle tool's input; the stored shape is the free shape (ADR-0032). Its radii are those of ADR-0029, one per point.

## Open

- Where a free shape sits: F09 keeps the rectangle at the origin, as before. Placing shapes (x, y of a shape) comes with the shape tree (E13) or the editor (E02).
