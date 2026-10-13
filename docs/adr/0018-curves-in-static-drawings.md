# ADR-0018 — Bézier curves allowed in static drawings only

**Status**: Accepted — "only" superseded by ADR-0046: symbols have cubic sides from typed handles

## Context

ADR-0001 limits geometry to segments and circular arcs, so that symbols stay schematic and exact. Business users also need to draw free decorations in the View Editor (static drawings, `docs/domain/shapes.md` §8). The Product Owner asked for a Figma-like pen there: click to place points, drag a point to pull curve handles (Q12).

## Decision

- **Static drawings** may contain cubic Bézier curves, created with a Figma-like pen.
- **Symbols** keep ADR-0001 unchanged: segments and arcs only, no pen, no freehand.
- A drawing has no parameter and no animation; it never enters the symbol pipeline (corner radius, booleans, offsets of symbols).

## Consequences

- ADR-0001's scope is narrowed to symbols; its decision stays valid there.
- Drawings need their own path model (segments + cubic Béziers) and their own `d` output (`C` command); booleans, offsets and exact lengths on Béziers are out of scope until a later ADR.
- The CLAUDE.md prohibition "no Bézier" applies to symbols only.

## References

ADR-0001, `REF-SVG2-PATHS`
