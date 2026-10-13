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
src/geometry/corner-radius/
  index.ts              export * from "./fillet-setback"; …
  fillet-setback.ts     export function filletSetback(…): number
  fillet-setback.test.ts
src/math/vector/
  point.ts              export type Point = …
```

## Names

- Every file and folder: `kebab-case` (`unicorn/filename-case`); a module file is the kebab-case of its export.
- Tests: `<module>.test.ts`, next to the module.
- Repository-wide tests in `tests/`: `backlog`, `references`, `barrels`, `diagrams`, `package`, `fitness`, `folders`, and the fixtures shared by the tests of `src/` (`geometry-fixtures`, imported from `tests`).
- The actions shared by the playground's tests: `playground/user-actions/`.

## Folders

Every code folder has an `index.ts` barrel (see [imports.md](./imports.md)). Folders are named after the **business domains** of `docs/domain/`, the same name in every layer where the domain has code — never a kind of code, "utils" or "helpers" (ADR-0043, `id-denylist`).

A code folder holds **modules or domain folders, never both**: beside folders sits only the barrel (and the playground's entry). `local/folder-layout` reports a file beside folders at once, in the editor.

| Folder          | Domains                                                                                                                                                                                                                                                                        |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `src/math/`     | `vector/` (the point, the vector and its arithmetic), `angle/` (degrees in a turn, the direction of an angle), `bisection/`, `precision/` (`EPSILON`)                                                                                                                          |
| `src/geometry/` | `contour/` (arcs, segments, corners, `cyclicItem`), `corner-radius/` (fillets, clamp, turning angles), `regular-polygon/` (regular and stretched), `layout-box/` (alignment, anchor, fit, stretch, size), `placement/` (straight box, turn)                                    |
| `src/model/`    | one folder per shape, its type inside (`rectangle/`, `regular-polygon/`, `free-shape/`), then `corner-radius/`, `placement/`, `selection/`                                                                                                                                     |
| `src/io/`       | modules only: the writing of path data, one domain                                                                                                                                                                                                                             |
| `src/render/`   | modules only: the SVG elements, one domain                                                                                                                                                                                                                                     |
| `playground/`   | one folder per panel (`toolbar/`, `shape/`, `points/`, `position/`, `drawing/`, `guided-test/`), the steps of the guided tests (`guided-steps/`), the fields every panel reads and writes (`page/`), the wiring of the panels (`mount/`), the tests' actions (`user-actions/`) |

A shape's type lives in its folder (`free-shape/free-shape.ts`); folders may use each other's types, their value imports go one way (`import-x/no-cycle`). A test of the whole page sits in the folder of the panel it exercises. Why: [ADR-0043](../adr/0043-folders-by-domain.md).

### Folders past 15 modules

15 modules per folder, tests and barrel not counted, is a signal (ADR-0043): a folder past it asks whether a domain is missing; it is split along that domain, or listed here with the reason none is. `local/folder-layout` fails on a folder past 15 not listed in `eslint/settings/folders.ts`; `tests/folders.test.ts` keeps that list and this table equal.

| Folder                    | Modules | Why it stays whole                                                                                                                          |
| ------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `playground/user-actions` | 23      | one theme, what a user does to the page; splitting it into reading, typing and clicking would split by kind of code, which ADR-0043 refuses |
