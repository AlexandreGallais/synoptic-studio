# Functions

## One function = one named concept

- The name is a standard mathematical, geometric or business term: `dot`, `cross`, `orientation`, `filletSetback`, `regularPolygonVertices`, `canConnect`.
- A high-level function **reads like the formula**: a sequence of calls to named functions. Reading the names alone tells "this serves that, which serves that".
- Top-level functions are `function` **declarations** (`func-style`): hoisted, so a module reads **top-down**, from the formula to its steps (`no-use-before-define` allows functions).
- Arrow functions are reserved for inline callbacks.

## A function never modifies its arguments

It takes values and **returns** a result. Enforced by:

- `no-param-reassign` (properties included) — everywhere;
- `functional/immutable-data`, `functional/prefer-immutable-types` (`ReadonlyDeep` parameters), `functional/type-declaration-immutability` — in the core;
- model types are `readonly`: the compiler rejects any mutation, even in the shell.

Exception by nature: a `procedure` receiving a DOM element may call its methods (`append`, `setAttribute`). See ADR-0014.

## Explicit inputs (ADR-0016)

In `src/`: **no default parameter, no optional parameter**. Defaults belong to the model (a symbol's parameters declare them), not to function signatures. A behavior variant is a distinct named function, or an explicit field of a named parameter object.

## Kind `@kind` (ADR-0011)

Every top-level function carries **exactly one** `@kind`. A function mixing two kinds must be split.

| `@kind`     | Definition                                              | Side effects |
| ----------- | ------------------------------------------------------- | ------------ |
| `math`      | pure numeric computation (scalars, vectors)             | no           |
| `geometry`  | operation with a geometric meaning on geometric objects | no           |
| `domain`    | business rule (symbol, port, parameter, pipe…)          | no           |
| `format`    | conversion between representations (model ↔ SVG, JSON)  | no           |
| `procedure` | action with a side effect (DOM, state, events)          | yes          |

Why a JSDoc tag and not a decorator: a TypeScript decorator only applies to classes and their members, never to a free function (`REF-TS-DECORATORS`). The tag plays the same role, and ESLint reads it.

## Limits per kind (`local/kind-limits`)

Values: `eslint/settings/kinds.ts`.

| `@kind`     | Code lines | Statements | Complexity | Depth |
| ----------- | ---------- | ---------- | ---------- | ----- |
| `math`      | 10         | 4          | 3          | 1     |
| `geometry`  | 20         | 8          | 5          | 2     |
| `domain`    | 20         | 8          | 5          | 2     |
| `format`    | 20         | 8          | 4          | 1     |
| `procedure` | 40         | 15         | 3          | 1     |

- Lines: lines holding code (blank and comment lines excluded), signature included.
- Complexity: McCabe, decision points + 1 (`if`, `?:`, `&&`, `||`, `??`, `case`, loops, `catch`), **callbacks included**.
- Every kind: 3 parameters at most (`@typescript-eslint/max-params`) — beyond, one named object.

## Long procedures: Composed Method

A business procedure may be long (40 lines), never complicated (complexity 3, depth 1). It is written as a **list of named steps at the same level of abstraction** (`REF-BECK-SBPP`):

```ts
/**
 * Places a pipe between two ports and records the command for undo.
 *
 * @kind procedure
 * ...
 */
export function addPipe(editor: Editor, from: PortReference, to: PortReference): void {
  const route = orthogonalRoute(editor.view, from, to);
  const pipe = pipeFromRoute(route);
  const command = addPipeCommand(pipe);

  executeCommand(editor, command);
  renderView(editor.canvas, editor.view);
}
```

`orthogonalRoute`, `pipeFromRoute`, `addPipeCommand` are pure `domain` functions; `executeCommand` and `renderView` are `procedure`s. _(Illustrative example: these functions do not exist yet.)_

Every decision (choosing a route, validating) lives in a tested pure function; the procedure only chains. If a procedure exceeds its limits after two splits, **stop and propose a split** to the Product Owner (`AGENTS.md`, Stop and ask when). There is deliberately no annotation lifting the limit.

## Forbidden

- `any`, `!` (non-null assertion), `as` (except `as const`): an exception requires `// eslint-disable-next-line rule -- justification`.
- Classes, `this`, `enum` (use a union of literals), `for…in`, `get`/`set` accessors, dynamic `import()`.
- A spread into the value a `reduce` callback returns (`[...acc, x]`, `{ ...acc, k: v }`): it copies the accumulator at every step, in quadratic time (EN-061, `REF-OXLINT-ACCUMULATING-SPREAD`); build the result with `map` or `flatMap`, or, outside `src/`, a loop over a local array — never by mutating the accumulator (ADR-0014). `no-restricted-syntax` reports any spread into that literal, the accumulator's or not.
- Magic numbers: only `-1`, `0`, `1`, `2` are allowed inline; everything else is a named, documented constant.
