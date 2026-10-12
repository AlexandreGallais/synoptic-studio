# ADR-0002 — Everything is a path, except text

**Status**: Accepted

## Context

Handling shapes uniformly (booleans, aligned strokes, node editing) requires a single representation. Native SVG primitives (`rect`, `circle`…) and the native `stroke` cannot align the outline inside or outside.

## Decision

- Every shape is rendered as a `<path>`.
- The stroke is computed as a path (offset), not through the `stroke` property.
- Exception: text stays a native `<text>`.

## Consequences

- A single geometric model for every operation.
- `stroke-alignment` being only an unimplemented W3C draft, it is achieved by computation.
- Text does not take part in booleans.

## References

`REF-SVG-STROKES`, `REF-SVGWG-957`, `REF-SVG2-PAINT`
