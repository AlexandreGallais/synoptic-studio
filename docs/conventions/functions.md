# Functions

## One business context per function

A function holds one business context, named by a term of its field. When a calculation needs two others that a field names, each of them is a function of its own, and the calculation reads as their composition (ADR-0049). The test is the name: if the parts cannot be named by their field, the function is one context.

- Name a function with the field's standard term: `dot`, `cross`, `orientation`, `filletSetback`, `regularPolygonVertices`, `canConnect`.
- A high-level function reads like its formula, as a sequence of calls to named functions. The names alone tell what serves what.
- Write top-level functions as `function` declarations (`func-style`). They are hoisted, so a module reads top-down, from the formula to its steps (`no-use-before-define` allows functions). Use arrow functions only for inline callbacks.

The size limits below are guardrails that flag a function holding more than one context. They never decide alone:

- **Over a limit**: split the function where a field names the parts.
- **When the only split gives invented halves** (`placeA`, `placeB`, `placeRest`) of a context that cannot be divided, the limit is wrong for that kind. Stop and propose a new limit to the Product Owner, with the function as evidence. A new limit is an ADR superseding ADR-0011's values.

There is deliberately no annotation lifting a limit. EN-093 measured the limits and kept them ([rules](./rules.md#function-limits-adr-0011-measured)).

## Kind `@kind` (ADR-0011)

Every top-level function carries exactly one `@kind`. A function that mixes two kinds is split. The project's linter reads the tag, which is a JSDoc tag because TypeScript decorators apply only to classes, never to a free function (`REF-TS-DECORATORS`).

| `@kind` | Definition | Side effects |
| --- | --- | --- |
| `math` | pure numeric computation (scalars, vectors) | no |
| `geometry` | operation with a geometric meaning on geometric objects | no |
| `domain` | business rule (symbol, port, parameter, pipe…) | no |
| `format` | conversion between representations (model ↔ SVG, JSON) | no |
| `procedure` | action with a side effect (DOM, state, events) | yes |

## Limits per kind

Checked by `local/kind-limits`.

The values are in `tools/lint/src/settings.rs`. The loosest of them are the global ceilings of `.oxlintrc.json` (`complexity`, `max-lines-per-function`…), for code without a `@kind`.

| `@kind` | Code lines | Statements | Complexity | Depth |
| --- | --- | --- | --- | --- |
| `math` | 10 | 4 | 3 | 1 |
| `geometry` | 20 | 8 | 5 | 2 |
| `domain` | 20 | 8 | 5 | 2 |
| `format` | 20 | 8 | 4 | 1 |
| `procedure` | 40 | 15 | 3 | 1 |

- **Lines** count the lines holding code, the signature included. Blank and comment lines do not count.
- **Complexity** is McCabe's: decision points plus 1 (`if`, `?:`, `&&`, `||`, `??`, `case`, loops, `catch`), callbacks included.
- **Parameters**: 3 at most for every kind (`max-params`). Beyond that, pass one named object.

## Long procedures: Composed Method

A procedure may be long but never branched: 40 lines, but a complexity of 3 and a depth of 1. Write it as a list of named steps at the same level of abstraction (`REF-BECK-SBPP`). Every decision, such as choosing a route or validating, lives in a tested pure function, and the procedure only chains them:

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

Here `orthogonalRoute`, `pipeFromRoute` and `addPipeCommand` are pure `domain` functions, while `executeCommand` and `renderView` are procedures. _(Illustrative example: these functions do not exist yet.)_

## Inputs and outputs

- **A function never modifies its arguments**: it takes values and returns a result (ADR-0014). `no-param-reassign` (properties included) checks it everywhere. In the core, `functional/immutable-data`, `functional/prefer-immutable-types` and `functional/type-declaration-immutability` also apply: a parameter's type is written deeply read-only (`readonly`, `Readonly<…>`; [lint](./lint.md#the-deeply-read-only-types)). Model types are `readonly`, so the compiler refuses a mutation even in the shell. A `procedure` that receives a DOM element may still call its methods (`append`, `setAttribute`), since changing the DOM is its job.
- **Explicit inputs** (ADR-0016): no default and no optional parameter in `src/`. A caller must see every value a function uses. Defaults belong to the model, where a symbol's parameters declare them. A variant of a behavior is a separate named function, or an explicit field of a named parameter object.
- **No magic number**: only `-1`, `0`, `1` and `2` appear inline. Any other number is a named, documented constant, so its meaning and source are written once.

## Forbidden

| Construct | Why | Instead |
| --- | --- | --- |
| `any`, `!`, `as` (but `as const`) | the type can no longer be trusted | an exception needs `// oxlint-disable-next-line rule -- reason` and its row in the [register](./exceptions.md) |
| classes, `this`, get and set accessors | the model is plain data, and functions over it are named one per file (ADR-0025) | functions over plain types |
| `enum` | it emits hidden JavaScript, Node.js's type stripping refuses it (`REF-NODE-TS`), and a union transcribes directly into a Rust `enum` (ADR-0025) | a union of literals |
| `for…in`, dynamic `import()` | inherited keys; a load the import graph cannot see | `Object.entries`, a static import |
| a spread of a fold's accumulator (`[...acc, x]`, `{ ...acc, k: v }`) | it copies the accumulator at each step, in quadratic time (`REF-OXLINT-ACCUMULATING-SPREAD`, `oxc/no-accumulating-spread`) | `map` or `flatMap`; outside `src/`, a loop over a local array; never a mutated accumulator |
