# ADR-0014 — Functional core, imperative shell

**Status**: Accepted

## Context

Product Owner rule: a function never modifies its arguments; it takes values and returns a result. Yet rendering and interaction handle the DOM, which is mutable by nature.

## Decision

Split into two zones (`REF-BERNHARDT-FCIS`):

| Zone                 | Layers                                       | Rules                                                                                                                                                                                                                                                         |
| -------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Functional core**  | `math`, `geometry`, `model`, `routing`, `io` | no mutation (`functional/immutable-data`), no `let`, no loop (`map`/`reduce`), no expression statement, no `throw`/`try`, `ReadonlyDeep` parameters and types, no access to the DOM or the host (`document`, `window`, `console`, `Math.random`, `Date.now`…) |
| **Imperative shell** | `render`, `interaction`, `playground`        | statement rules relaxed (loops, `if`, DOM calls); kept: `no-param-reassign` with properties, no `let`, no class                                                                                                                                               |

Everywhere: no classes (`functional/no-classes`), no `this`; model data are `readonly` types — the compiler therefore prevents any mutation of the model, even from the shell.

## Consequences

- The core is tested without a browser nor test doubles (the point of the split).
- A `procedure` receiving a DOM element may call its methods (`append`, `setAttribute`): that is its role. It never reassigns a property of a parameter.
- The relaxation is centralized in `eslint/scopes/dom-layers.ts`.

## References

`REF-BERNHARDT-FCIS`, ADR-0005, ADR-0011
