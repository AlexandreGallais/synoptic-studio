# ADR-0033 — A free shape of corners, fractions until edited, points typed in the SVG's coordinates

**Status**: Accepted (Product Owner, 2026-10-11, Q27) — supersedes ADR-0029 for free shapes (radii in a list of their own) and ADR-0032 on integer points and size; completes ADR-0031; partly superseded by ADR-0035 (a free shape is drawn at its position, not at the top-left of its stored points)

## Context

F09 stored a free shape as its points in their own box, its width and height, and its radii in a second list (ADR-0029, ADR-0032), all integers. At the validation of F09 the Product Owner found it confusing to keep the radii apart from their points. F08 decided that converting a regular polygon does not move it (ADR-0031), and Q27 settled the rest: fractions are accepted until the user edits a point, which then lands on integers, and the x and y typed are the SVG's own coordinates, not the shape's inner box.

## Decision

- A free shape stores **corners**, each a point and its requested radius (`Corner`, the type the geometry already rounds), its width and its height. The radius stays an integer ≥ 0 (ADR-0003, ADR-0029).
- Its points and its size are **finite numbers**: a regular polygon converted keeps its exact points and the size of the box around them (a hexagon in 100 × 100 is 100 × 86.60254), so it draws the same. What the user types is an integer.
- A point is typed in the **coordinates of the SVG**: the shape first keeps its corners as drawn (its own box becomes the drawn one, at scale 1), the typed point replaces the old one, and the width and height become the box around the points. Stretching by width and height then works as in F09, without loss.
- A regular polygon stays a shape of its own, with its list of radii (ADR-0029), until it is converted.

## Consequences

- `FreeShape` holds `corners`, `width`, `height`; the rectangle tool stores the unit square with its radii as corners.
- A point typed leaves the other points where they are drawn, fractions included; only the coordinates typed become integers.
- A free shape whose points all lie on one line has no box to stretch: it is refused (ADR-0032).

## References

ADR-0003, ADR-0029, ADR-0031, ADR-0032, `DERIV-free-shape-stretch`
