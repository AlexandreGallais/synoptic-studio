# ADR-0066 — A register of exceptions, and a byte buffer written once

**Status**: Accepted (Product Owner, 2026-10-16, at RET-003).

## Context

- AUD-019 found that reading Apple Color Emoji.ttc (192 MB) ends Node.js with "invalid array length": `collectionFaceToFile` spreads table bytes into plain arrays, as the functional lint asks (no mutation of a value once built).
- A buffer written once (a `Uint8Array` of the final size, each part set at its place) needs a mutation the functional lint forbids, so an `eslint-disable` with its justification.
- The places where the code departs from a rule (a lint rule disabled, an `as`, a `!`) are justified one by one beside the code, but no page lists them, and nothing checks that each one has its reason.

## Decision

1. **A byte buffer written once is allowed** for a pure optimisation, justified: the buffer is built inside one function, written once at known places, and returned; nothing outside the function sees it change. The function keeps its tests and its `@see`.
2. **A register of exceptions**, `docs/conventions/exceptions.md`: one row per place the code departs from a rule — where (file, function), which rule, why. A justification is mandatory for each row.
3. **A check keeps the register and the code in step**: every `eslint-disable` (and every other exception the register lists) has its row, and every row points to code that still holds it.

## Reasons

| Option                                          | For                                                                  | Against                                     |
| ----------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------- |
| One exception per function, registered (chosen) | the rule holds everywhere else; each exception visible and justified | one more page and one check                 |
| A lint scope off for `src/io/`                  | simple                                                               | every reader of `io` loses the rule, unseen |
| Plain arrays kept                               | no exception                                                         | large collections crash the editor          |

## Consequences

- EN-057 writes the register, its check, and the byte buffer of `collectionFaceToFile`.
- A new exception is added with its row in the same change.

## References

ADR-0014, ADR-0016, `docs/conventions/README.md`, AUD-019
