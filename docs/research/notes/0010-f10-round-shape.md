# 0010 — Sources of F10: round shape

Spike SP-008, 2026-10-12. Claude Code's own research; every source below was read.

## The capsule (EN-016)

- A rectangle capped by two half circles is a stadium (also discorectangle, obround) (`REF-MATHWORLD-STADIUM`). The project calls it a capsule in plain words.
- Rounding every corner of the box by half its smaller side gives it exactly: on the short sides the two setbacks fill the side, on the long sides a straight part remains, and no radius is reduced (`DERIV-round-shape` step 1, from `DERIV-local-radius-clamp`). SVG does not draw a capsule from a `rect`: it caps `rx` at half the width and `ry` at half the height separately (`REF-SVG2-SHAPES` §10.2), so a 120 × 80 `rect` with a large radius has elliptical corners. Figma's rounded rectangle at its maximal radius was not read; the derivation does not need it.
- No new geometry: the round shape's corners go through `roundedContour`, as every shape.

## Conversion (EN-016, US-020)

- Integer radii of half the smaller side rounded up draw the same shape: an odd side is reduced to exactly its half by the clamp (`DERIV-round-shape` step 2; in floating point `40.50000000000001`: tests use a tolerance). The free shape is the rectangle tool's for the box, valid at a size of 0.

## Model (EN-016) — ADR-0036

- A shape of its own: `kind: "round"`, width, height, angle, position; one function tells the three shapes apart. The playground's tests of `"anchor" in model` take a shape without anchor for a free shape: US-020 replaces them (`read-edited-free-shape`, `show-point-fields`, `show-delete-button`, `show-point-buttons`, `mount-playground`, `create-box-elements`, `listen-to-point-fields`, `shape-reading`).

## Open

Nothing for the Product Owner.
