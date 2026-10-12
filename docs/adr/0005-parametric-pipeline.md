# ADR-0005 — Parametric pipeline: numbers → maths → SVG

**Status**: Accepted

## Context

The user types numbers (integers). A simple, understandable and cheap chain of transformations to the displayed SVG is needed.

## Decision

One-way chain in 4 stages:

| Stage         | Layer            | Content                                                   | Number type | `@kind`               |
| ------------- | ---------------- | --------------------------------------------------------- | ----------- | --------------------- |
| 1. Input      | `interaction/`   | interface values                                          | integers    | `procedure`           |
| 2. Model      | `model/`         | declarative description (corners, sizes, radii, operands) | integers    | `domain`              |
| 3. Evaluation | `geometry/`      | computed geometry: segments + arcs                        | floats      | `geometry`            |
| 4. Output     | `io/`, `render/` | `d` string of the `<path>`, then DOM                      | text        | `format`, `procedure` |

Rules:

- The SVG is **an output**, never a source of truth. The DOM is never read back to know the geometry.
- Hit-testing, snapping, intersections: computed on stage 3, not on the DOM.
- Recomputed on demand, only for modified shapes (memoization by identifier + version).

## Consequences

- Negligible computation cost for symbols of a few dozen vertices.
- `math/` and `geometry/` layers testable without a browser.
- Changing the output format (Canvas, PDF) only affects stage 4.

## References

ADR-0002, ADR-0003
