# ADR-0046 — Cubic Bézier sides in symbols, from points with handles

**Status**: Accepted (Product Owner, 2026-10-12, Q4: curves with handles are needed in symbols; F12 refinement and SP-011, 2026-10-13). Supersedes ADR-0001's "No Bézier curve" for symbols; its other points stand.

## Context

ADR-0001 limited symbols to segments and circular arcs, an arc only as a fillet, for exact strokes, intersections and lengths. The Product Owner decided that symbols need curves with handles (a propeller, a converted round shape), with Figma's three handle modes (Q4, F12). SP-011 read how Figma models them ([research note 0013](../research/notes/0013-f12-curves-with-handles.md)).

## Decision

1. A side of a free shape is a **cubic Bézier** when a handle is on at either end: from point A through A + A.after and B + B.before to point B (`REF-SVG2-PATHS`, `REF-FIGMA-VECTOR-NETWORK`). Otherwise it stays a segment, its corners filleted by arcs (ADR-0007).
2. Handles are vectors from their point, stored exactly (ADR-0031, ADR-0034), typed as integers in one of the methods of Q32; a point keeps its radius while in handles mode.
3. Arcs stay fillets only; quadratic curves stay out; elliptical arcs are decided with the round shape (F10).
4. Strokes, intersections and dash lengths on curves are approximated, not exact (`REF-POMAX-BEZIER`): their method and tolerance are decided with E03 and E04, not now.

## Consequences

- `ContourPiece` gains a cubic piece; the path writer a `C` command; the straight box a derivation for the curve's extremes.
- ADR-0002's exact strokes hold for segments and arcs only; E04 needs an ADR for curves.
- A corner in radius mode next to a curved side stays sharp until a rule is chosen (note 0013).
- A shape's box is the box of its curve, not of its points (Product Owner, 2026-10-13): the straight box, fit and stretch use the curve's extremes.

## References

`REF-SVG2-PATHS`, `REF-FIGMA-VECTOR-NETWORK`, `REF-FIGMA-HANDLE-MIRRORING`, `REF-POMAX-BEZIER`, ADR-0001, ADR-0007, ADR-0018
