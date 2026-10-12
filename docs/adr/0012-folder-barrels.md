# ADR-0012 — One `index.ts` per folder, canonical import paths

**Status**: Accepted

## Context

The Product Owner asked for an `index.ts` in every folder, imports targeting **the folder and never a file** of another folder, and automatic rewriting on save to the shortest path (`folder/index.ts` → `folder`, `folder/file` → `folder`).

## Caveat

"Barrel files" have known costs in large codebases: worse tree-shaking, slower tools, import cycles (`REF-TKDODO-BARRELS`, `REF-ATLASSIAN-BARRELS`). They are acceptable here because:

- the library has no dependency and will stay small (a few hundred modules at most);
- `package.json` declares `"sideEffects": false`: the bundler can prune;
- cycles are forbidden and detected (`import-x/no-cycle`);
- importing an ancestor barrel (the classic cause of cycles) is forbidden by the custom rule.

## Decision

1. Every code folder has an `index.ts` (`local/folder-has-index`) containing only `export * from "./x";`, **one per module or sub-folder, sorted** (`local/barrel-exports`, autofixed).
2. Canonical path (`local/canonical-import-path`, autofixed): import **the child of the common ancestor** that contains the target.
   - same folder → the file: `./dot`;
   - other folder → the folder: `../geometry` (never `../geometry/index`, nor `../geometry/arc/filletSetback`);
   - file of an ancestor folder → the file: `../dot` (going through the ancestor's barrel would create a cycle);
   - never a `.ts` extension.
3. Importing the barrel of one's own folder or of an ancestor is an error (guaranteed cycle).
4. `import-x/no-useless-path-segments`, `import-x/extensions` and `import-x/no-internal-modules` are off: the custom rule is the only source of path fixes (no competing fixes).

## Consequences

- Creating a file → `npm run fix` (or saving the `index.ts`) updates the barrel.
- Tests (`*.test.ts`) are never exported by a barrel.
- `playground/index.ts` is the application entry point, not a barrel.

## References

`REF-TKDODO-BARRELS`, `REF-ATLASSIAN-BARRELS`, [imports.md](../conventions/imports.md)
