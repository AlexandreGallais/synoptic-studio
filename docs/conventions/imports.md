# Imports

## `index.ts` in every folder (ADR-0012)

- Every code folder holds an `index.ts` (`local/folder-has-index`).
- An `index.ts` contains **only** `export * from "./module";`, one line per module and sub-folder, sorted (`local/barrel-exports`). **Autofixed**: create a file, then run `npm run fix`.
- Exceptions: tests (`*.test.ts`) are never exported; `playground/index.ts` is an entry point, not a barrel.

## Canonical path (`local/canonical-import-path`, autofixed)

Import **the child of the common ancestor** that contains the target:

| From                         | Target                       | Written                  | Fixed to                               |
| ---------------------------- | ---------------------------- | ------------------------ | -------------------------------------- |
| `src/model/shape.ts`         | `src/model/port.ts`          | `./port.ts`              | `./port`                               |
| `src/model/shape.ts`         | `src/geometry/index.ts`      | `../geometry/index`      | `../geometry`                          |
| `src/model/shape.ts`         | `src/geometry/arc/fillet.ts` | `../geometry/arc/fillet` | `../geometry`                          |
| `src/geometry/vector.ts`     | `src/geometry/arc/fillet.ts` | `./arc/fillet`           | `./arc`                                |
| `src/geometry/arc/fillet.ts` | `src/geometry/vector.ts`     | `../vector`              | _(already canonical)_                  |
| `src/geometry/vector.ts`     | `src/geometry/index.ts`      | `.`                      | **error**: own folder's barrel = cycle |

Never a `.ts` extension, never `/index`.

## Values and types on separate lines

A type-only import always uses its own `import type` statement (`@typescript-eslint/consistent-type-imports` with `separate-type-imports`, `import-x/consistent-type-specifier-style: prefer-top-level`). Both are autofixed: an import used only as a type becomes `import type`, and a mixed import is split:

```ts
import { RuleTester } from "@typescript-eslint/rule-tester";

import type { TestCaseError } from "@typescript-eslint/rule-tester";
```

Two statements from the same module are allowed when one is `import type` (`import-x/no-duplicates` keeps them apart); two value imports from the same module are merged.

## Order (`import-x/order`, autofixed)

Groups separated by a blank line: Node modules (`node:…`) → packages → parents (`../`) → siblings (`./`) → `import type`. Alphabetical within each group; members sorted (`sort-imports`).

## Forbidden

- Cycles (`import-x/no-cycle`), self-import, duplicate imports.
- `import * as` (`import-x/no-namespace`), `export default` (except root configuration files), dynamic `import()`.
- Undeclared package, or `devDependencies` used from `src/` (`import-x/no-extraneous-dependencies`): **zero runtime dependency**.
- Node modules (`node:fs`…) outside tooling (`import-x/no-nodejs-modules`).
- Forbidden layer (`import-x/no-restricted-paths`, table `eslint/settings/layers.ts`).
- Unused imports and variables: **removed automatically** (`unused-imports/no-unused-imports`).
