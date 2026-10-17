# Imports

One way to import a thing, written by the autofix, so that an import is never a choice and a move touches the fewest lines.

## A barrel per folder (ADR-0012)

- Every code folder holds an `index.ts` (`local/folder-has-index`).
- A barrel holds only `export * from "./module";` lines, one per module and subfolder, sorted (`local/barrel-exports`). It is autofixed: create a file, then run `npm run fix`.
- Not barrels: tests are never exported, `playground/index.ts` is the playground's entry point (each of its subfolders has a barrel), and `tests/index.ts` gathers the shared test fixtures.

## Canonical path (`local/canonical-import-path`, autofixed)

Import the child of the common ancestor that contains the target: a file of the same folder, otherwise the folder. Never write a `.ts` extension nor `/index`.

| From (in `src/`) | Target (in `src/`) | Written | Fixed to |
| --- | --- | --- | --- |
| `model/free-shape/free-shape-radii.ts` | `model/free-shape/free-shape.ts` | `./free-shape.ts` | `./free-shape` |
| `model/free-shape/free-shape-radii.ts` | `model/placement/turned-corners.ts` | `../placement/turned-corners` | `../placement` |
| `model/rectangle/rectangle-contour.ts` | `geometry/index.ts` | `../../geometry/index` | `../../geometry` |
| `geometry/placement/contour-bounding-box.ts` | `geometry/corner-radius/has-length.ts` | `../corner-radius/has-length` | `../corner-radius` |
| `geometry/layout-box/fit-in-box.ts` | `geometry/layout-box/size.ts` | `./size` | _(already canonical)_ |
| `geometry/layout-box/fit-in-box.ts` | `geometry/index.ts` | `..` | **error**: own layer's barrel = cycle |

## Values and types on separate statements

A type-only import is its own `import type` statement (`@typescript-eslint/consistent-type-imports` with `separate-type-imports`, `import/consistent-type-specifier-style: prefer-top-level`). Both rules are autofixed: an import used only as a type becomes `import type`, and a mixed import is split. Two statements from one module are allowed only when one of them is `import type`. Two value imports from one module are merged (`import/no-duplicates`).

```ts
import { defineConfig } from "vite";

import type { UserConfig } from "vite";
```

## Order

The formatter writes the order (oxfmt's `sortImports`), and `npm run format:check` refuses a file out of it. Groups are separated by a blank line: Node.js modules (`node:…`), then packages, then parents (`../`), then siblings (`./`), then the `import type` statements. Names are sorted alphabetically within a group, capitals first, and members are sorted too (`sort-imports`).

## Forbidden

| Import | Why | Check |
| --- | --- | --- |
| a cycle, a self-import, a duplicate | a cycle is a load-order bug | `import/no-cycle`, `import/no-self-import`, `import/no-duplicates` |
| `import * as`, `export default` (but root configuration files), dynamic `import()` | one name per export, found by a search | `import/no-namespace`, `import/no-default-export` |
| a package not declared, or a `devDependencies` package in `src/` | the library has zero runtime dependency | `import-x/no-extraneous-dependencies` |
| a Node.js module (`node:fs`…) outside tooling | the core is portable (ADR-0025) | `import/no-nodejs-modules` |
| a layer the importer may not import | the layers ([architecture](./architecture.md)) | `import-x/no-restricted-paths` |
| an unused import or variable | dead code is read for nothing | `no-unused-vars`, imports removed by the autofix |
