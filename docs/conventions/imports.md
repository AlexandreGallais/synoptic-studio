# Imports

## `index.ts` in every folder (ADR-0012)

- Every code folder holds an `index.ts` (`local/folder-has-index`).
- An `index.ts` contains **only** `export * from "./module";`, one line per module and subfolder, sorted (`local/barrel-exports`). **Autofixed**: create a file, then run `npm run fix`.
- Exceptions:
  - tests (`*.test.ts`) are never exported;
  - `playground/index.ts` is the entry point, not a barrel; every subfolder of the playground has one;
  - `tests/index.ts` gathers the shared test fixtures, not a barrel of the library.

## Canonical path (`local/canonical-import-path`, autofixed)

Import **the child of the common ancestor** that contains the target:

| From (in `src/`) | Target (in `src/`) | Written | Fixed to |
| --- | --- | --- | --- |
| `model/free-shape/free-shape-radii.ts` | `model/free-shape/free-shape.ts` | `./free-shape.ts` | `./free-shape` |
| `model/free-shape/free-shape-radii.ts` | `model/placement/turned-corners.ts` | `../placement/turned-corners` | `../placement` |
| `model/rectangle/rectangle-contour.ts` | `geometry/index.ts` | `../../geometry/index` | `../../geometry` |
| `geometry/placement/contour-bounding-box.ts` | `geometry/corner-radius/has-length.ts` | `../corner-radius/has-length` | `../corner-radius` |
| `geometry/layout-box/fit-in-box.ts` | `geometry/layout-box/size.ts` | `./size` | _(already canonical)_ |
| `geometry/layout-box/fit-in-box.ts` | `geometry/index.ts` | `..` | **error**: own layer's barrel = cycle |

Never a `.ts` extension, never `/index`.

## Values and types on separate lines

A type-only import always uses its own `import type` statement (`@typescript-eslint/consistent-type-imports` with `separate-type-imports`, `import-x/consistent-type-specifier-style: prefer-top-level`). Both are autofixed: an import used only as a type becomes `import type`, and a mixed import is split:

```ts
import { ESLint } from "eslint";

import type { Linter } from "eslint";
```

Two statements from the same module are allowed when one is `import type` (`import-x/no-duplicates` keeps them apart); two value imports from the same module are merged.

## Order (oxfmt's `sortImports`, written by `npm run format`)

Groups separated by a blank line: Node modules (`node:…`) → packages → parents (`../`) → siblings (`./`) → `import type`, its siblings, then its parents, then its modules and packages, without blank lines. Alphabetical within each group, character by character, capitals first; members sorted (oxlint's `sort-imports`). oxfmt writes the order since EN-095, and `npm run format:check` refuses a file out of it; `import-x/order` accepted parents and siblings in any order among the `import type` statements, oxfmt orders them.

## Forbidden

- Cycles (`import-x/no-cycle`), self-import, duplicate imports.
- `import * as` (`import-x/no-namespace`), `export default` (except root configuration files), dynamic `import()`.
- Undeclared package, or `devDependencies` used from `src/` (`import-x/no-extraneous-dependencies`): **zero runtime dependency**.
- Node modules (`node:fs`…) outside tooling (`import-x/no-nodejs-modules`).
- Forbidden layer (`import-x/no-restricted-paths`, table `LAYERS` of `tools/lint/src/settings.rs`).
- Unused imports and variables: imports **removed automatically** (oxlint's `no-unused-vars`).
