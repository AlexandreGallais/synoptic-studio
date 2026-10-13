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

Layout of ADR-0042: folders named by business theme, never by kind of code.

| Folder          | Themes                                                                                                                                                                                                                                                                                  |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/math/`     | `vector/` (the `Vector` type and its arithmetic); at the root the point, the angle in turns, the bisection, `EPSILON`                                                                                                                                                                   |
| `src/geometry/` | `rounded-corners/` (fillets, clamp, turning angles), `box/` (bounding boxes, fit, stretch, turn), `polygon/` (regular and stretched polygons); at the root the types every folder needs (`Arc`, `Segment`, `ContourPiece`, `Corner`, `Size`…) and `cyclicItem`                          |
| `src/model/`    | one folder per shape, its type inside (`rectangle/`, `regular-polygon/`, `free-shape/`), and per concern of the editor (`radii/`, `placement/`, `selection/`)                                                                                                                           |
| `playground/`   | one folder per panel (`toolbar/`, `shape/`, `points/`, `position/`, `drawing/`, `guided-test/`), the steps of the guided test (`guided-steps/`), the page's own reading and writing (`page/`), the tests' actions (`user-actions/`); at the root the entry, the mounting and the update |

A shape's type lives in its folder (`free-shape/free-shape.ts`): a file and a folder of the same name cannot sit side by side, an import of `./free-shape` reaching the file first. Folders may use each other's types; their value imports go one way (`import-x/no-cycle`).

### Folders past 15 modules

15 modules per folder, tests and barrel not counted, is a signal (ADR-0042): a folder past it is split along a theme that makes sense, or listed here with the reason none does. `tests/folders.test.ts` fails on a folder past 15 that is not listed.

| Folder                    | Modules | Why it stays whole                                                                                                                          |
| ------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `playground/user-actions` | 20      | one theme, what a user does to the page; splitting it into reading, typing and clicking would split by kind of code, which ADR-0042 refuses |
