# ADR-0071 — Type errors with the lint, declarations by Oxc

**Status**: Accepted (2026-10-17, decided by the Product Owner during F19's run; EN-091). Partly supersedes the consequences of ADR-0010 (removed since, ADR-0101; its linter first restated by ADR-0075): `tsc` leaves `npm run check`, and `tsconfig.build.json` no longer emits the declarations.

## Context

- `tsc` had two jobs here: the type check of every folder (`npm run typecheck`, a step of `check:all`) and the library's declarations (`tsc -p tsconfig.build.json`, in `npm run build`).
- The Product Owner asked that a type error never stop the agent half-way through an idea, but come listed with the lint findings, and that any Oxc tool able to take over one of `tsc`'s jobs do so (2026-10-17), in line with ADR-0069.
- Oxc has no type checker of its own (`REF-OXC-README`). Oxlint's type-aware mode runs `tsgolint` on typescript-go, TypeScript 7's compiler, and `--type-check` adds the compiler's diagnostics to the lint (`REF-OXLINT-CLI`, `REF-OXLINT-TYPE-AWARE`). Oxc's isolated declarations write a module's `.d.ts` from that module alone, for code that meets TypeScript's `isolatedDeclarations` rule (`REF-OXC-ISOLATED-DECLARATIONS`, `REF-TS-ISOLATED-DECLARATIONS`).
- Tried on this repository (EN-091): oxlint's type check reported the same errors as `tsc --noEmit` on the tree and on planted ones; the isolated declarations rule asked for two annotations in `src/`, both on exported constants computed from other constants, none on a function or a local; Oxc's declarations equal `tsc`'s 589 files in meaning, in 0.06 s against 0.63 s.

## Decision

1. **Type errors come with the lint.** Every oxlint pass — `npm run lint`, `npm run fix`, the pre-commit hook — runs with `--type-check`; `tsc --noEmit` leaves `check` and `check:all`. `npm run typecheck` stays as a command of its own.
2. **The library's declarations are Oxc's.** `tools/declarations` writes `dist/types/` from each module of `src/` alone; `tsconfig.build.json` is removed.
3. **The library's exports carry their types.** An exported function has its return type written (already the rule); an exported constant whose value is not a plain literal has its type written. No local constant, no variable needs one. The published API is then an explicit interface, which also makes a breaking change visible in the diff of the export itself (Product Owner, 2026-10-17).
4. `tsc` (TypeScript 7) stays a dev package for what Oxc does not cover: the editors, `npm run typecheck`, and `tsgolint` brings its own typescript-go.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Type errors in oxlint, declarations by Oxc (chosen) | one pass lists every finding, type errors included; declarations ten times faster, without a compiler run; the API typed where it is declared | `--type-check` is marked experimental: a regression would show as a missed error, guarded by a test that plants one; two annotations to write |
| `tsc` for both, as before | the reference compiler | a separate step that stops at the first type error, apart from the lint; 0.6 s more per build |
| `isolatedDeclarations` in `tsconfig.json` for every folder | the rule shown by the editors everywhere | 87 errors outside `src/` (64 in `eslint/`, which leaves at EN-085), on files that publish no declarations |

## Consequences

- `tests/type-check-with-lint.test.ts` plants a type error and expects oxlint to report it at its place; `tests/declarations.test.ts` runs `tools/declarations` on `src/`, so that an export missing its type fails the tests, not only the CI's build.
- A new export of `src/` computed from other values is written with its type, as in `export const DEGREES_PER_HALF_TURN: number = DEGREES_PER_TURN / 2`.
- The other decisions of ADR-0010 stay, restated by ADR-0075: oxlint and ESLint carry what they can, `tsconfig.json` only what they cannot.

## References

ADR-0069, ADR-0075, ADR-0101 (ADR-0010, removed), `REF-OXC-README`, `REF-OXLINT-CLI`, `REF-OXLINT-TYPE-AWARE`, `REF-TSGOLINT`, `REF-OXC-ISOLATED-DECLARATIONS`, `REF-TS-ISOLATED-DECLARATIONS`, [EN-091](../backlog/stories/E17-F19-EN-091-configuration-files-reviewed-for-speed.md)
