# Files

## One export per file (ADR-0019, `local/one-export-per-file`)

In `src/` and `playground/` (barrels, entry points and tests excepted):

- a module exports **exactly one value** — a function or a constant — with `export <declaration>`;
- the file is named the **kebab-case** of the export: `fillet-setback.ts`, `svg-namespace.ts`;
- no `export { … }` list, no re-export (re-exports are the barrel's job);
- a **type** may be exported next to the value only if it appears in the value's declaration (its parameter or return type): it is the function's contract;
- any other type lives in its own file, named like it in kebab-case (`point.ts`), exporting only that type;
- non-exported helpers may stay in the file only if they serve its single export; a helper shared by two modules becomes its own module.

```text
src/geometry/
  index.ts              export * from "./fillet-setback"; …
  fillet-setback.ts     export function filletSetback(…): Setback   (+ export type Setback)
  fillet-setback.test.ts
  point.ts              export type Point = …
```

## Names

- Every file and folder: `kebab-case` (`unicorn/filename-case`); a module file is the kebab-case of its export.
- Tests: `<module>.test.ts`, next to the module; repository-wide tests (backlog, bibliography, barrels, diagrams, dependencies, fitness) in `tests/`, with the fixtures shared by the tests of `src/` (`tests/geometry-fixtures.ts`, imported from `tests`); the actions shared by the playground's tests in `playground/user-actions/`.

## Folders

Every code folder has an `index.ts` barrel (see [imports.md](./imports.md)). A sub-folder groups modules of one sub-concept (`geometry/intersections/`), never "utils" or "helpers" (`id-denylist`).
