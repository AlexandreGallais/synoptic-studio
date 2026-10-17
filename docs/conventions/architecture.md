# Architecture

## Pipeline

Typed numbers (integers) → **model** (integers) → evaluated **geometry** (floats, segments + arcs) → **SVG** (`d`). Details: ADR-0005.

## Layers

Imports only go to the listed layers. Source table: `eslint/settings/layers.ts` (enforced by `import-x/no-restricted-paths` and `local/kind-in-layer`).

| Layer | Role | May import | DOM | `@kind` |
| --- | --- | --- | --- | --- |
| `src/math/` | scalars, vectors, tolerances, robust predicates | — | no | `math` |
| `src/geometry/` | segments, arcs, intersections, corner radius, offset, booleans | math | no | `math`, `geometry` |
| `src/model/` | shapes, symbols, ports, configurations, business types, views, pipes | math, geometry | no | `domain` |
| `src/routing/` | orthogonal pipe routing | math, geometry, model | no | `geometry`, `domain` |
| `src/io/` | SVG / JSON serialization | math, geometry, model | no | `format` |
| `src/render/` | model → SVG elements | all but interaction | yes | `format`, `procedure` |
| `src/interaction/` | tools, hit-testing, snapping, commands | every `src/` layer | yes | `domain`, `procedure` |
| `playground/` | demonstration application | **public API `src/index.ts` only** | yes | `procedure` |

A layer exists on disk only from its first module on (an empty folder has no meaningful barrel). Layers present after E01: `math`, `geometry`, `model`, `io`, `render`. The pure decisions of an interaction (which vertices a click selects, F03) live in `model`; `interaction` will hold the effects (events, hit-testing).

## Functional core, imperative shell (ADR-0014)

- **Core** (`math` → `io`): pure functions. No mutation, no `let`, no imperative loop, no `throw`, no access to `document`, `window`, `console`, `Math.random`, `Date.now`…
- **Shell** (`render`, `interaction`, `playground`): effects allowed (DOM, events), but never reassigning a property of a parameter, nor mutating the model (its types are `readonly`).

In practice: every decision is taken in the core; the shell only applies it.

## Portability (ADR-0025)

The core runs in any ECMAScript engine (browser, Node.js, server, worker) and can be transcribed into another language function by function:

- ECMAScript only — no DOM, Node.js or host API (lint: `eslint/scopes/pure-layers.ts`);
- plain read-only data in and out; no class, closure returned or exception;
- the TSDoc states the mathematical intent where JavaScript differs from other languages (`%` sign, `.at` with a negative index, `-0`, `NaN`);
- `number` is IEEE 754 binary64; `EPSILON` and `SVG_DECIMALS` are part of the specification.
