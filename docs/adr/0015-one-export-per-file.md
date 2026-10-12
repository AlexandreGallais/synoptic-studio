# ADR-0015 — One export per file, named like the file

**Status**: Superseded by ADR-0019

## Context

The Product Owner asked for one function per file, the file name being the export name, so that the tree of files reads like the list of concepts. Open point raised by the Product Owner: should a type live next to the function that uses it, or in its own file?

## Sources

- Angular style guide: focus a source file on a single concept; match the file name to the TypeScript identifier inside (`REF-ANGULAR-STYLE`).
- Airbnb style guide 23.6–23.7: a base file name exactly matches its default export; a function's file name is identical to the function's name, in camelCase (`REF-AIRBNB-STYLE`).
- Google TypeScript style guide: named exports only, minimize the exported API surface of modules (`REF-GOOGLE-TS-STYLE`).

## Decision

Enforced by `local/one-export-per-file` on `src/` and `playground/` (barrels, entry points and tests excepted):

1. A module exports **exactly one value** (function or constant), with `export <declaration>`. No `export { … }` list, no re-export (re-exports belong to barrels).
2. The file is named **exactly** like that export: `filletSetback.ts`, `SVG_NAMESPACE.ts`. `unicorn/filename-case` is off there (folders stay kebab-case).
3. **Types**: a type may be exported next to the value **only if it appears in the value's declaration** (parameter, return or annotation type): it is the function's contract, intrinsically tied to it. Any other type lives in its own file, named like it (`Point.ts`), which then exports that type only.
4. Non-exported helpers may stay in the file only if they serve that single export; a helper used by two modules becomes its own module.

### Why types go with their function, under that condition

A type that exists only to describe one function's input or output has no meaning without it: separating them adds a file and an import with no gain, and keeping them together prevents the type from drifting away from its only user. As soon as a type is shared (another function's signature uses it), it is a concept of its own and gets its own file — the rule detects the first case structurally; the second is a review point.

## Consequences

- Many small files; navigation by file name (`Go to file`) equals navigation by concept.
- Test files mirror their module: `filletSetback.test.ts`.
- The documentation site (TypeDoc) lists one page per exported symbol, matching the files.

## References

`REF-ANGULAR-STYLE`, `REF-AIRBNB-STYLE`, `REF-GOOGLE-TS-STYLE`, ADR-0012
