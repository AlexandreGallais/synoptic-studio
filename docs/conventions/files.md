# Files

## One export per file (ADR-0019, `local/one-export-per-file`)

A module exports exactly one value, a function or a constant, and the file is named after it. With one export per file, a name is found by its file, a diff touches one concept, and an import says what it uses. The rule covers `src/` and `playground/`. Barrels, entry points and tests are exempt.

- Write `export <declaration>`. Never write an `export { … }` list, and never re-export: re-exporting is the barrel's job.
- Name the file in kebab-case after its export: `filletSetback` lives in `fillet-setback.ts`.
- A type may sit beside the value only when it appears in the value's signature (a parameter or return type), because it is part of the function's contract. Any other type gets its own file, named after it (`point.ts`).
- A private helper may stay in the file only while it serves the one export. Once a second module needs it, it becomes a module of its own.

```text
src/geometry/corner-radius/
  index.ts              export * from "./fillet-setback"; …
  fillet-setback.ts     export function filletSetback(…): number
  fillet-setback.test.ts
src/math/vector/
  point.ts              export type Point = …
```

## Names

- Every file and folder is named in kebab-case (`unicorn/filename-case`).
- A test is named `<module>.test.ts` and sits beside its module.
- Tests of the repository as a whole sit in `tests/`, for example `backlog`, `references`, `barrels`, `folders` and `fitness`. The fixtures shared by the tests of `src/` sit there too, imported from `tests`.
- The actions shared by the playground's tests sit in `playground/user-actions/`.

## Folders

Folders are named after the business domains of `docs/domain/`, and a domain keeps the same name in every layer where it has code: `src/geometry/corner-radius/` and `src/model/corner-radius/`. A folder is never named after a kind of code such as `utils` or `helpers` (ADR-0043, `id-denylist`): a domain name tells where a module belongs, a kind does not. To place a new module, list the layer's folders and pick its domain. A new domain gets its folder with its first module.

- Every code folder has an `index.ts` barrel ([imports](./imports.md)).
- A folder holds modules or domain folders, never both. Beside folders sits only the barrel, and the playground's entry. `local/folder-layout` reports a TypeScript file beside folders, and `tests/folders.test.ts` checks every other file.
- A shape's type lives in the shape's folder (`free-shape/free-shape.ts`). Folders may use each other's types, but their value imports go one way (`import-x/no-cycle`).
- A test of the whole page sits in the folder of the panel it exercises.

### Folders past 15 modules

Fifteen modules in a folder, tests and barrel not counted, is a signal that a domain is missing (ADR-0043). Split the folder along that domain. If no domain splits it, list it below with the reason. A split by kind of code (reading, writing, clicking) is not a domain, so it does not answer the signal. `local/folder-layout` fails on a folder past 15 that `tools/lint/src/settings.rs` does not list. A test of that file keeps its list equal to this table, and `tests/folders.test.ts` counts the folders on disk itself, so a folder missing here, or listed without a reason, fails (EN-061).

| Folder | Modules | Why it stays whole |
| --- | --- | --- |
| `playground/user-actions` | 33 | one theme, what a user does to the page; splitting it into reading, typing and clicking would split by kind of code, which ADR-0043 refuses |
| `playground/checklists` | 16 | one theme, the checklists the page opens, one file of lines per feature validated; `val-prepare` writes each beside the map of checklists it updates, so a folder per epic waits for that tool to write there (VAL-037) |
