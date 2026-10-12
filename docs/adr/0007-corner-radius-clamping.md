# ADR-0007 — Corner radius clamping: local proportional reduction

**Status**: Accepted

## Context

When the radii requested at both ends of an edge exceed its length, they must be reduced. Product Owner requirements:

- two corners with the same radius on a too-short edge → each stops at the middle;
- a single corner pushed to the maximum on a square → the corner becomes an arc covering the whole side, and goes no further;
- never an "anti-corner": a rounding never eats the other edges;
- no error message: the value simply goes no further.

Comparison:

| Rule                | 2 corners at 100 on a 25 edge | 1 corner at 1000 on a 100 square | Effect on unrelated corners |
| ------------------- | ----------------------------- | -------------------------------- | --------------------------- |
| Figma (cap at half) | 12.5 each ✔                   | 50 → a flat part remains ✘       | none                        |
| CSS (global factor) | 12.5 each ✔                   | 100 ✔                            | **all reduced** ✘           |
| Local proportional  | 12.5 each ✔                   | 100 ✔                            | none ✔                      |

## Decision

**Local proportional reduction per edge**, derived from the CSS principle. Details and proof: `DERIV-local-radius-clamp`.

## Consequences

- Always-constructible result, whatever the requested value.
- Only the vertices adjacent to a conflicting edge are reduced.
- Non-maximal rule: a small margin may stay unused in rare cases. Accepted for simplicity.

## References

`REF-CSS-BR`, `REF-FIGMA-CR`, `DERIV-local-radius-clamp`
