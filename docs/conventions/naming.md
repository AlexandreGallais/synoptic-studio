# Naming

Names are what a reader searches for and reads instead of a body, so a name tells its role and its concept without a look at the type or the code.

## Casing (`@typescript-eslint/naming-convention`)

| Element | Form | Example |
| --- | --- | --- |
| function, local variable, parameter | `camelCase` | `filletSetback`, `interiorAngle` |
| module constant (top-level `const`) | `UPPER_CASE` | `SVG_DECIMALS`, `CLOSE_PATH` |
| type, alias | `PascalCase` | `Point`, `ViewBox` |
| boolean (variable, parameter) | prefix + `PascalCase` | `isClosed`, `hasPorts`, `canConnect` |

- Boolean prefixes: `is`, `has`, `can`, `should`, `are`, `was`, `will`, `did`, `does`.
- The project's linter has no type checker, so it reads a name's type from what is written. A module constant holds a function when its value is an arrow or a function expression, or when its function type is written (`const items: () => readonly Item[] = builtOnce(…)`). A name is a boolean when it is typed `boolean` or its value is `true`, `false`, a comparison, a negation or `Boolean(…)`. When the type cannot be read, as with a call's result, the next row of the table applies: write the type to get the row meant.

## Length and vague names

- A name has 2 to 40 characters (`id-length`). Single letters are kept for the standard variables of a formula: `a b c d h i j k n p q r s t u v w x y z`.
- A name that says no concept is refused (`id-denylist`): `tmp`, `temp`, `info`, `obj`, `val`, `res`, `ret`, `stuff`, `thing`, `util(s)`, `helper(s)`, `manager`, `foo`, `bar`, `baz`.

## Names by kind (`local/kind-naming`)

| `@kind` | The name… | Good | Refused |
| --- | --- | --- | --- |
| `procedure` | starts with a verb of `PROCEDURE_VERBS` | `renderShape`, `mountPlayground` | `shapeRenderer` |
| `format` | reads as a conversion: `xToY`, `toX`, `fromX`, `parseX`, `formatX`, `serializeX`… | `polygonToPathData` | `svgPath` |
| `math`, `geometry`, `domain` | names the concept, without a vague verb (`get`, `compute`, `calculate`, `calc`, `make`, `do`, `handle`, `process`, `perform`) | `dot`, `filletSetback` | `computeDot`, `getSetback` |
| all | starts with a predicate word (`is`, `has`, `can`, `should`, `are`, `contains`, `includes`, `intersects`, `equals`) exactly when it returns a boolean | `canConnect` | `connectable(): boolean`, `isPort(): Port` |

When a procedure needs a verb the list lacks, add it, sorted, to `PROCEDURE_VERBS` in `tools/lint/src/settings.rs`. Never bend the name to fit the list.

## Every calculation named (ADR-0049)

The rules above say how a function is named. This one says what gets a function: an operation, or a run of steps, that a field names is a call to a function of that name, with its source. The fields include mathematics, geometry, physics, technical drafting, industrial practice and the laboratory. A process is a `procedure` named after what it does. A named calculation is written, tested and sourced once, and the code that uses it reads as its field does.

| Written inline | Named | Field |
| --- | --- | --- |
| `x·cos θ − y·sin θ`, `x·sin θ + y·cos θ` | `rotate` | geometry |
| `atan2(cross, dot)` | `signedAngle` | geometry |
| `((a % n) + n) % n` | `modulo` | arithmetic |
| `(i · b) / n` for `i = 0 … n` | `regularPartition` | analysis |
| `start + size · i`, a record in a font table | `elementAddress` | computing |
| `⌊n / s⌋`, the whole records that fit | `integerQuotient` | arithmetic |
| `⌊(H − f) / λ⌋ + 1`, the lines a box holds | `progressionTermsUpTo` | sequences |
| `\|m − n\| ≤ t` | `isWithinTolerance` | metrology |
| reading which buttons are greyed, field by field | `readDisabledState` | the page |

Bare arithmetic stays only where no field names the operation, or where naming it would change a floating-point result. In both cases, a comment says why. The review of each story and the `NAM` story of each epic look for calculations left unnamed ([agile process](../process/agile.md)).
