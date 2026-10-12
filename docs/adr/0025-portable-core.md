# ADR-0025 — A portable core: any JavaScript engine, transcribable to another language

**Status**: Accepted

## Context

The Product Owner wants the library's mathematics to be its own (never delegated to the SVG or DOM APIs), runnable in any JavaScript engine — a browser, Node.js or another server runtime, so that heavy computation can move to a server — and simple to transcribe into another language (for instance Rust compiled to WebAssembly) if speed requires it. ADR-0005 and ADR-0014 already keep the DOM out of the core; host APIs of Node.js and language-specific behaviors were not addressed.

## Decision

1. The functional core (`math`, `geometry`, `model`, `routing`, `io`) uses **ECMAScript only**: no DOM, no Node.js API, no host API (`fetch`, `structuredClone`, `Intl`, timers, `console`). Enforced by `no-restricted-globals` in `eslint/scopes/pure-layers.ts`; its tests already run in Node.js without a DOM.
2. **Plain data in, plain data out**: read-only records and arrays of numbers, strings and booleans; no class, no prototype, no closure returned, no exception (already ADR-0014 and the lint).
3. **Mathematical intent over JavaScript behavior**: the TSDoc formula and the derivation state the mathematics; where JavaScript semantics differ from other languages (remainder sign of `%`, negative indexes of `Array.prototype.at`, `-0`, `NaN` propagation), the TSDoc says which result is meant, so that a transcription reproduces the intent.
4. **Numbers are IEEE 754 binary64** (`number`), the `f64` of most languages; `EPSILON` and `SVG_DECIMALS` are part of the specification, not of the implementation.
5. One function = one formula with its source: the library can be transcribed function by function, with its tests as the acceptance suite.

## Consequences

- The shell (`render`, `interaction`, `playground`) stays the only place tied to a browser.
- A server or worker can run the core unchanged.
- Existing functions are checked against point 3 by AUD-001 (e.g. `cyclicVertex` relies on `%` and `.at` with negative indexes).

## References

ADR-0003, ADR-0005, ADR-0014
