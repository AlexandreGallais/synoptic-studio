# 0009 — Sources of F04: free rotation of shapes

Spike SP-007, 2026-10-11. Claude Code's own research; every source below was read.

## Angle and turning (EN-014)

- SVG's rotation about the origin is the matrix `[cos a, sin a, −sin a, cos a, 0, 0]`, and a rotation about a point is translate, rotate, translate back (`REF-SVG11-TRANSFORM` §7.4, §7.6). With y pointing down (`REF-SVG2-COORDS` §8.4) a positive angle appears clockwise, as CSS states for `rotate(90deg)` (`REF-CSS-TRANSFORMS` §9.1): the Product Owner's direction is SVG's own.
- The angle from 0 to 359: `%` keeps the dividend's sign; the modulo is `((n % d) + d) % d` (`REF-MDN-REMAINDER`). Stated in TSDoc (ADR-0025).
- Quarter turns are exact only if computed as quarter turns: `cos(π/2)` is `6.1e-17` in floating point. The angle is split into quarter turns and a rest (`DERIV-turned-shape` step 3).
- At 0° the straight shape is drawn without computing, so that every earlier test keeps its exact values.

## Position (EN-014) — ADR-0035

- A regular polygon has no position, a free shape only the top-left of its stored points. Every shape now stores an angle and a position, the top-left of its box before it turns; the x and y shown are those of the straight box, derived (the Product Owner's « vraie get bbox »). Storing the straight box's top-left instead would move a turned shape when its radius changes.
- Functional references: Inkscape draws the straight box dashed and turns about the center of the box, a movable cross (`REF-INKSCAPE-SELECTOR`); Figma's `rotation` runs from −180 to 180 about the top-left (`REF-FIGMA-ROTATION`) — the Product Owner chose 0 to 359 about the center. Figma's help on width and height of a rotated layer could not be read (pages not found); the rule « width and height stay those before turning » is the Product Owner's decision, attributed to them only.

## Straight box (EN-015)

- The bounding box is the tightest axis-aligned rectangle around the geometric shape, without stroke, what `getBBox` returns (`REF-SVG2-COORDS` §8.10).
- For a contour of segments and fillet arcs: the ends of every piece, plus the axis points `C ± r` of each arc that sweeps through them; a fillet sweeps less than π, so a perp dot product test decides (`DERIV-arc-bounding-box`).

## Editing a turned shape (US-019)

- Keeping a local point at its drawn place gives the new top-left in one formula (`DERIV-turned-shape` step 5): the turned top-left corner for a width typed, a point not moved for a point typed, the first point for a conversion.

## Playground (US-018)

- Angle field and ±90° buttons; the turned box as the selection; the straight box dashed with its x, y, width and height (Inkscape's dashed box, `REF-INKSCAPE-SELECTOR`). No new source needed for the fields: those of F01 to F08.

## Open

Nothing for the Product Owner. The rotation mode and its center are for E02.
