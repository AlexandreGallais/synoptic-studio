# ADR-0092 — A barrel re-exports a types-only module as types

**Status**: Accepted (the agent, 2026-10-19, AUD-033, from the audit's finding F7; to be told to the Product Owner). Supersedes, in ADR-0012's decision 1, "containing only `export * from "./x";`" for a module or sub-folder holding only types: its line is `export type * from "./x";`. The rest of decision 1 and ADR-0012's other decisions stand.

## Context

- ADR-0012's decision 1 gives a barrel `export * from "./x";` lines only, one per module or sub-folder, sorted.
- The type rules enable oxlint's `typescript/consistent-type-exports` ([rules](../conventions/rules.md)), which reports an `export *` of a module holding only types, "All exports in the declaration are only used as types", and asks for `export type`: the `type` keyword says "that the export exists only in the type system, not at runtime", which "allows transpilers to drop exports without knowing the types of the dependencies" (`REF-TSESLINT-TYPE-EXPORTS`). The project compiles with `verbatimModuleSyntax` and `isolatedModules` ([TypeScript](../conventions/typescript.md)), which keep a re-export as it is written.
- So the barrels already hold both forms: 82 lines `export type * from "./x";` in 37 barrels of `src/` and in 11 of `playground/` on 2026-10-19, each for a module of types (`point`, `vector`, `font-tables`…). `local/barrel-exports` rewrote such a line into `export *` until EN-138 made it keep the line as written; the decision that the barrels obey was never changed (AUD-033, F7).

## Decision

1. **A module or sub-folder holding only types is re-exported by `export type * from "./x";`**, any other by `export * from "./x";`, one line each, sorted by module as before (`local/barrel-exports`, autofixed). A barrel holds no other line.
2. **`typescript/consistent-type-exports` decides which form a line takes**: `local/barrel-exports` keeps a line written `export type *` and writes `export *` for a module it adds; `npm run fix` then gives the first form to a module of types (EN-138).

## Reasons

- The two rules then agree: a barrel the autofix writes passes the type rule, and the type rule's form passes the barrel rule.
- A types-only module has nothing at runtime: `export type *` says so where it is read, and lets a transpiler drop the line without reading the module.

| Option | For | Against |
| --- | --- | --- |
| `export type *` for a types-only module (chosen) | the barrels as they are; both rules agree | two forms of a barrel line |
| `export *` only (ADR-0012 as written) | one form | fails `typescript/consistent-type-exports` on 48 barrels; the rule turned off for barrels, against the lint-first rules |
| no barrel line for a types-only module | one form | breaks ADR-0012's import of the folder, never the file |

## Consequences

- [imports](../conventions/imports.md) and [lint](../conventions/lint.md) give a barrel's two forms and point here; ADR-0012's status in the [index](./) names this decision.
- A module that gains its first value keeps its `export type *` line, which `barrels.test.ts` then fails (`.claude/rules/library.md`): the line is written `export *` by hand.

## References

ADR-0012, `REF-TSESLINT-TYPE-EXPORTS`, EN-138, AUD-033
