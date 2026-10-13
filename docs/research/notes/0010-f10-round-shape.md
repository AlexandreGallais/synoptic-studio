# 0010 — Sources of F10: round shape

Spike SP-008, 2026-10-12. Claude Code's own research; every source below was read.

## The capsule (EN-016)

- A rectangle capped by two half circles is a stadium (also discorectangle, obround) (`REF-MATHWORLD-STADIUM`). The project calls it a capsule in plain words.
- Rounding every corner of the box by half its smaller side gives it exactly: on the short sides the two setbacks fill the side, on the long sides a straight part remains, and no radius is reduced (`DERIV-round-shape` step 1, from `DERIV-local-radius-clamp`). SVG does not draw a capsule from a `rect`: it caps `rx` at half the width and `ry` at half the height separately (`REF-SVG2-SHAPES` §10.2), so a 120 × 80 `rect` with a large radius has elliptical corners. Figma's rounded rectangle at its maximal radius was not read; the derivation does not need it.
- No new geometry: the round shape's corners go through `roundedContour`, as every shape.

## Conversion (EN-016, US-020)

- Integer radii of half the smaller side rounded up draw the same shape: an odd side is reduced to exactly its half by the clamp (`DERIV-round-shape` step 2; in floating point `40.50000000000001`: tests use a tolerance). The free shape is the rectangle tool's for the box, valid at a size of 0.

## Model (EN-016)

- A shape of its own (draft ADR-0036, accepted on the feature branch, never on `main`): `kind: "round"`, width, height, angle, position; one function tells the three shapes apart. The playground's tests of `"anchor" in model` take a shape without anchor for a free shape: US-020 replaces them (`read-edited-free-shape`, `show-point-fields`, `show-delete-button`, `show-point-buttons`, `mount-playground`, `create-box-elements`, `listen-to-point-fields`, `shape-reading`).

## Archive: F10 cancelled in E01 (2026-10-12)

F10 was built up to its audit, then cancelled at its validation (VAL-008), and moved to E14 by the Product Owner. None of its code reached `main`. It is kept for E14:

- **The code**: pull request #139 (AUD-008) holds the feature branch at its audit, readable as `refs/pull/139/head` (commit `f55a2dc`): code, tests, draft ADR-0036, stories. The Git tag `archive/f10-round-shape`, with the validation branch's guided test steps 58 to 64 and demo page, was deleted when the history restarted at 0.1.0; the audited state is also kept as a Git bundle in the private records (ADR-0041). The pull requests into the feature branch keep each story's diff: SP-008 (#134), EN-016 (#135), CHK-008 (#136), US-020 (#138), AUD-008 (#139).
- **The model** (`src/model/`): `RoundShape` (`kind: "round"`, width, height, angle, position), `isValidRoundShape`, `roundShapeCorners` (the box rounded by half its smaller side), `roundShapeAsFreeShape` (the rectangle tool's free shape, radii rounded up), `shapeKind` and `isFreeShape` (three kinds of shape told apart), `shapeAsFreeShape` and `isConvertible` (one rule for converting a polygon or a round shape).
- **The playground**: a "Round" choice; `read-shape` split by kind (`read-polygon-shape`, `read-rectangle-shape`, `read-round-shape`); the radius fields hidden for a round shape; the tests of `"anchor" in model` replaced by `isFreeShape`, which E14 can reuse for any new kind of shape.
- **Lessons from the reviews**: a derived radius must not leak into the next shape's fields; a degenerate shape (width 0) must stay convertible; a rule used in two places is written once.

### Validation (VAL-008, 2026-10-12)

- Steps 58 to 64: no other remark.
- Product Owner decision: a round shape in a long box is an ellipse, as in Figma and Inkscape, not a capsule — a capsule is what a rectangle with a very large radius gives; this replaces the refinement's choice of the capsule.
- A converted round shape should also offer points on the curve, at its top, right, bottom and left, with handles as in Figma, besides the 4 corners of its box.
- Ellipses and curves with handles are outside ADR-0001 (Q4), and the subject is too large for E01: the round shape, the ellipse and Bézier curves with handles make epic E14; text (F05) makes epic E15; E01 ends with F06.

## Open

For E14's first spike: the questions listed in the epic.
