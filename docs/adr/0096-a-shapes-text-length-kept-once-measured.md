# ADR-0096 — A shape's text length kept once measured, in the core

**Status**: Accepted (AUD-042, 2026-10-23), the audit's speed finding; an exception to ADR-0014's "no mutation" for one memo, registered as ADR-0066 asks.

## Context

- The history's size (`historySize`, ADR-0095, decision 4) counts, for each shape a past or future state holds and the present does not, the length of its JSON text. It is measured once or twice by `boundedHistory` at each step and once more by the toolbar, so that each measure writes again the text of every shape the history holds.
- AUD-042 measured a step at 20 000 moves of one rectangle and at 80 000, about the 80 380 the default 100 MB holds: 59 ms and 387 ms a step, 30 ms and 129 ms a click that took none, each measure writing 20 000 or 80 000 texts of 614 characters.
- A shape is never changed once made (ADR-0014): its text, so its length, is the same at every measure; only the shapes a step adds are new.
- The functional core forbids mutation and module state (ADR-0014): `functional/immutable-data` refuses a `WeakMap`'s `set`. ECMAScript has `WeakMap` (ADR-0025, decision 1): its keys are objects, held weakly, so that an entry goes with its shape.

## Decision

1. **One memo in the core**: `shapeTextLength` keeps the length of each shape's JSON text in a module's `WeakMap`, `TEXT_LENGTHS`, written once for each shape, then read; `historySize` counts each shape by it. Its one write is a `lint-disable-next-line functional/immutable-data, functional/no-expression-statements` with its reason, registered in `docs/conventions/exceptions.md` (ADR-0066).
2. **Its condition**: nothing outside `shapeTextLength` reads the memo, and a shape measured is never changed, so that the function gives what `JSON.stringify(shape).length` gives at every call; the history's shapes are objects (`T extends WeakKey`).
3. Keeping the history's size from step to step, updated by each step rather than measured again, is left to a story of its own (F41).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| A `WeakMap` by shape, in the core (chosen) | each text written once in the life of its shape; the same result; an entry goes with its shape; ECMAScript alone | module state in the core, one exception to the lint; a shape changed in place would be measured wrong, which ADR-0014's readonly types already prevent |
| The length passed in by the shell, measured there | the core left pure | every function of the history takes one more parameter, a function, where the core takes plain data (ADR-0025, decision 2) |
| The size kept in the history, updated at each step | no measure of the whole history at a step | the update rules of a step, an undo, a redo and a bound to derive and prove against `historySize`: a story of its own |
| Nothing | no exception | a step 387 ms at 80 000 moves, the default size's |

## Consequences

- `historySize`, `boundedHistory` and their helpers take shapes that are objects (`T extends WeakKey`): the playground's `KeptShape` and the tests' shapes are.
- A transcription to another language (ADR-0025) keeps the memo or drops it: `shapeTextLength` is the length of the shape's text either way.
- The measures of a step before and after are in AUD-042's story file.

## References

ADR-0014, ADR-0025, ADR-0066, ADR-0095, AUD-042
