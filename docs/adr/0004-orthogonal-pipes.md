# ADR-0004 — Orthogonal pipes only

**Status**: Accepted

## Context

Pipes must stay readable. Whether 45° routes were allowed was left open by the Product Owner.

## Decision

- Pipes made exclusively of horizontal and vertical segments.
- Ports declare an exit direction (N, E, S, W).
- Diagonals excluded in v1.

## Consequences

- Consistent with process diagram conventions (horizontal and vertical lines).
- Orthogonal routing algorithms from the literature apply directly.
- Diagonals can be reintroduced by a later ADR.

## References

`REF-PID-LECTURE`, `REF-WYBROW-2009`, `REF-MARRIOTT-2014`
