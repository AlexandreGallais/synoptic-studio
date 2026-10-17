# Naming

## Casing (`@typescript-eslint/naming-convention`)

Run by the project's linter (`tools/lint`, EN-076), which reads a name's type from its written annotation, or else its initializer: a module constant holding a function is an arrow or a function expression, or has its function type written (`const items: () => readonly Item[] = builtOnce(…)`); a boolean is `boolean`, `true` or `false` written, or a comparison, a negation or `Boolean(…)`. A type the code does not tell, such as a call's result, takes the next row.

| Element | Form | Example |
| --- | --- | --- |
| function, local variable, parameter | `camelCase` | `filletSetback`, `interiorAngle` |
| module constant (top-level `const`) | `UPPER_CASE` | `SVG_DECIMALS`, `CLOSE_PATH` |
| type, alias | `PascalCase` | `Point`, `ViewBox` |
| boolean (variable, parameter) | prefix + `PascalCase` | `isClosed`, `hasPorts`, `canConnect` |

Boolean prefixes: `is`, `has`, `can`, `should`, `are`, `was`, `will`, `did`, `does`.

## Length (`id-length`)

2 to 40 characters. Single letters are reserved for standard mathematical variables: `a b c d h i j k n p q r s t u v w x y z`.

## Denied names (`id-denylist`)

`tmp`, `temp`, `info`, `obj`, `val`, `res`, `ret`, `stuff`, `thing`, `util(s)`, `helper(s)`, `manager`, `foo`, `bar`, `baz`: they name no concept.

## Per kind (`local/kind-naming`)

| `@kind` | Rule | Good | Rejected |
| --- | --- | --- | --- |
| `procedure` | starts with a **verb** from `tools/lint/src/settings.rs` | `renderShape`, `mountPlayground` | `shapeRenderer` |
| `format` | reads as a **conversion**: `xToY`, `toX`, `fromX`, `parseX`, `formatX`, `serializeX`… | `polygonToPathData` | `svgPath` |
| `math`, `geometry`, `domain` | **names the concept**, without vague verb (`get`, `compute`, `calculate`, `calc`, `make`, `do`, `handle`, `process`, `perform`) | `dot`, `filletSetback` | `computeDot`, `getSetback` |
| all | returns a boolean ⇔ starts with a predicate word (`is`, `has`, `can`, `should`, `are`, `contains`, `includes`, `intersects`, `equals`) | `canConnect` | `connectable(): boolean`, `isPort(): Port` |

To add a procedure verb: insert it (sorted) in `PROCEDURE_VERBS` of `tools/lint/src/settings.rs` rather than bending a name.

## Every calculation named (ADR-0049)

The names above say how a function is written; this rule says **what** gets a function. An operation, or a run of steps, that a field names — mathematics, geometry, physics, technical drafting, industrial practice, the laboratory — is a call to a function of that name, with its source; a process is a `procedure` named after what it does.

| Written inline | Named | Field |
| --- | --- | --- |
| `x·cos θ − y·sin θ`, `x·sin θ + y·cos θ` | `rotate` | geometry (F13) |
| `atan2(cross, dot)` | `signedAngle` | geometry (F13) |
| `((a % n) + n) % n` | `modulo` | arithmetic (F13) |
| `(i · b) / n` for `i = 0 … n` | `regularPartition` | analysis (F13) |
| reading which buttons are greyed, field by field | `readDisabledState` | the page (F13) |
| `start + size · i`, a record in a font table | `elementAddress` | computing (NAM-001) |
| `⌊n / s⌋`, the whole records that fit | `integerQuotient` | arithmetic (NAM-001) |
| `⌊(H − f) / λ⌋ + 1`, the lines a box holds | `progressionTermsUpTo` | sequences (NAM-001) |
| `\|m − n\| ≤ t` | `isWithinTolerance` | metrology (NAM-001) |

Bare arithmetic stays only where no field names it, or where naming it would change a floating-point result: then a comment says why. Each story's review looks for unnamed calculations; a `NAM` story at the end of each epic goes over the whole code twice ([agile process](../process/agile.md)).

## Files

The file is named like its export (see [files.md](./files.md)).
