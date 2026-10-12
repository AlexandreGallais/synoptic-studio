# Naming

## Casing (`@typescript-eslint/naming-convention`)

| Element                             | Form                  | Example                              |
| ----------------------------------- | --------------------- | ------------------------------------ |
| function, local variable, parameter | `camelCase`           | `filletSetback`, `interiorAngle`     |
| module constant (top-level `const`) | `UPPER_CASE`          | `SVG_DECIMALS`, `CLOSE_PATH`         |
| type, alias                         | `PascalCase`          | `Point`, `ViewBox`                   |
| boolean (variable, parameter)       | prefix + `PascalCase` | `isClosed`, `hasPorts`, `canConnect` |

Boolean prefixes: `is`, `has`, `can`, `should`, `are`, `was`, `will`, `did`, `does`.

## Length (`id-length`)

2 to 40 characters. Single letters are reserved for standard mathematical variables: `a b c d h i j k n p q r s t u v w x y z`.

## Denied names (`id-denylist`)

`tmp`, `temp`, `info`, `obj`, `val`, `res`, `ret`, `stuff`, `thing`, `util(s)`, `helper(s)`, `manager`, `foo`, `bar`, `baz`: they name no concept.

## Per kind (`local/kind-naming`)

| `@kind`                      | Rule                                                                                                                                   | Good                             | Rejected                                   |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- | ------------------------------------------ |
| `procedure`                  | starts with a **verb** from `eslint/settings/verbs.ts`                                                                                 | `renderShape`, `mountPlayground` | `shapeRenderer`                            |
| `format`                     | reads as a **conversion**: `xToY`, `toX`, `fromX`, `parseX`, `formatX`, `serializeX`…                                                  | `polygonToPathData`              | `svgPath`                                  |
| `math`, `geometry`, `domain` | **names the concept**, without vague verb (`get`, `compute`, `calculate`, `calc`, `make`, `do`, `handle`, `process`, `perform`)        | `dot`, `filletSetback`           | `computeDot`, `getSetback`                 |
| all                          | returns a boolean ⇔ starts with a predicate word (`is`, `has`, `can`, `should`, `are`, `contains`, `includes`, `intersects`, `equals`) | `canConnect`                     | `connectable(): boolean`, `isPort(): Port` |

To add a procedure verb: insert it (sorted) in `eslint/settings/verbs.ts` rather than bending a name.

## Files

The file is named like its export (see [files.md](./files.md)).
