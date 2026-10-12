# 0010 — Sources of F10: round shape

Spike SP-008, 2026-10-12. Claude Code's own research; every source below was read.

## The capsule (EN-016)

- A rectangle capped by two half circles is a stadium (also discorectangle, obround) (`REF-MATHWORLD-STADIUM`). The project calls it a capsule in plain words.
- Rounding every corner of the box by half its smaller side gives it exactly: on the short sides the two setbacks fill the side, on the long sides a straight part remains, and no radius is reduced (`DERIV-round-shape` step 1, from `DERIV-local-radius-clamp`). SVG caps a `rect` radius at half its side in the same spirit (`REF-SVG2-SHAPES` §10.2), a functional reference only: the project draws a `path` (ADR-0002).
- No new geometry: the round shape's corners go through `roundedContour`, as every shape.

## Conversion (EN-016, US-020)

- Integer radii of half the smaller side rounded up draw the same shape: an odd side is reduced to exactly its half by the clamp (`DERIV-round-shape` step 2).

## Model (EN-016) — ADR-0036

- A shape of its own: width, height, angle, position; told apart from the free shape (`corners`) and the polygon (`anchor`) by what it stores.

## Open

Nothing for the Product Owner.
