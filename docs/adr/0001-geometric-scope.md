# ADR-0001 — Geometric scope: segments and circular arcs

**Status**: Accepted — scope narrowed to symbols by ADR-0018; its "No Bézier curve" superseded for symbols by ADR-0046

## Context

Synoptic symbols are schematic. The Product Owner asked for shapes described by numbers rather than drawn by hand, with a corner radius that behaves as in Figma.

## Decision

- Primitives: line segment and circular arc.
- An arc only appears as the fillet of a vertex (corner radius).
- No Bézier curve (quadratic or cubic).

## Consequences

- Closed-form intersections: segment/segment, segment/arc, arc/arc.
- Exact offset (the offset of an arc is a concentric arc) → strokes computed exactly.
- Exact lengths → exact dash patterns.
- No Bend tool, no handle mirroring, no corner smoothing.
- No ellipse without elliptical arcs (open question Q4).

## References

`REF-GG-FILLET`, `REF-SVG2-PATHS`
