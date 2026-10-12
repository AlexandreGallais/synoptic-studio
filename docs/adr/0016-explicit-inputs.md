# ADR-0016 — Explicit inputs: no default nor optional parameter in the library

**Status**: Accepted

## Context

The Product Owner asked whether a mathematics and geometry library should use default parameters, noting that the logic has none: every value comes from the model.

## Sources

- Google TypeScript style guide: default parameters are allowed but should be used **sparingly**; initializers must have no observable side effect (`REF-GOOGLE-TS-STYLE`).
- Airbnb style guide 7.7–7.9: prefer default parameter syntax over mutating arguments, avoid side effects in defaults, put them last (`REF-AIRBNB-STYLE`). These rules frame how to use defaults in application code; they do not argue for them.

Both guides target general-purpose code. Two project-specific facts weigh more:

- **Defaults belong to the model**: a symbol's parameters declare their default values (`docs/domain/symbols-and-views.md` §3). A default hidden in a function signature would be a second, invisible source of truth.
- **Pipeline** (ADR-0005): every value flows from the integer model to the geometry. A function receiving everything explicitly is fully described by its call, which keeps tests and hand-computed reference values honest.

## Decision

In `src/` (enforced by `no-restricted-syntax`, `eslint/scopes/modules.ts`):

- no default parameter, including inside destructured parameters;
- no optional parameter (`x?: T`).

A variant of behavior is either a distinct, named function, or an explicit field of a named parameter object (3 parameters at most, ADR-0011).

## Consequences

- Call sites are a little longer; every value is visible where it is used.
- The playground and the tooling are not concerned.

## References

`REF-GOOGLE-TS-STYLE`, `REF-AIRBNB-STYLE`, ADR-0005, ADR-0011
