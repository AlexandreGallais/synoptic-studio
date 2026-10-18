# Architecture

## Pipeline

Data flows one way: typed integers → **model** (integers) → evaluated **geometry** (floats, segments and arcs) → **SVG** (`d`). Nothing is read back from the DOM (ADR-0005).

## Layers

A layer imports only the layers its row lists. This keeps each layer testable without the ones above it. The source of truth is `LAYERS` in `tools/lint/src/settings.rs`: imports are checked by `import-x/no-restricted-paths`, kinds by `local/kind-in-layer`.

| Layer | Role | May import | DOM | `@kind` |
| --- | --- | --- | --- | --- |
| `src/math/` | scalars, vectors, tolerances, robust predicates | — | no | `math` |
| `src/geometry/` | segments, arcs, intersections, corner radius, offset, booleans | math | no | `math`, `geometry` |
| `src/model/` | shapes, symbols, ports, configurations, business types, views, pipes | math, geometry | no | `domain` |
| `src/routing/` | orthogonal pipe routing | math, geometry, model | no | `geometry`, `domain` |
| `src/io/` | SVG and JSON serialization | math, geometry, model | no | `format` |
| `src/render/` | model → SVG elements | all but interaction | yes | `format`, `procedure` |
| `src/interaction/` | tools, hit-testing, snapping, commands | every `src/` layer | yes | `domain`, `procedure` |
| `playground/` | demonstration application | **public API `src/index.ts` only** | yes | `procedure` |

- A layer folder is created with its first module, never empty: an empty folder has no meaningful barrel.
- A decision belongs to the lowest layer that can take it. For example, which vertices a click selects is a pure decision, so it lives in `model`. `interaction` holds only the effects: events and hit-testing.

## Functional core, imperative shell (ADR-0014)

Every decision is taken in the core and the shell only applies it. A pure function is tested by its examples alone, so the logic stays where tests are cheap.

| Part | Layers | Allowed |
| --- | --- | --- |
| core | `math` to `io` | pure functions only: no mutation, `let`, loop or `throw`, and no host global (`document`, `window`, `console`, `Math.random`, `Date.now`) |
| shell | `render`, `interaction`, `playground` | effects such as the DOM and events, but never a parameter's property reassigned nor the model mutated (its types are `readonly`) |

## Portability (ADR-0025)

The core must run in any ECMAScript engine and be transcribable into another language function by function. To keep that possible:

- Use ECMAScript only, with no DOM, Node.js or host API. The pure layers' override of `.oxlintrc.json` (`no-restricted-globals`) checks this.
- Pass plain read-only data in and out. No class, no returned closure, no exception.
- State the mathematical intent in TSDoc wherever JavaScript differs from other languages: the sign of `%`, `.at` with a negative index, `-0`, `NaN`.
- `number` is IEEE 754 binary64, and `EPSILON` and `SVG_DECIMALS` are part of the specification.
