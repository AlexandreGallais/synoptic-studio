# TypeScript

## Version

TypeScript **7** (`typescript` 7.0.2, the native compiler, EN-071) is the only TypeScript (ADR-0075): it checks the types through oxlint's `--type-check` in `npm run lint` (ADR-0071), and alone as `npm run typecheck`. TypeScript 6 left with typescript-eslint and TypeDoc (EN-085).

## Split with the linters (ADR-0075)

What a linter can check lives in the linters (oxlint, the project's linter); `tsconfig.json` keeps only what belongs to the type system:

| Option | Why in TypeScript |
| --- | --- |
| `strict` | `any` inference, `null`, `this`, variance: out of a linter's reach |
| `noUncheckedIndexedAccess` | `T[number]` becomes `T \| undefined`: a type, not syntax |
| `exactOptionalPropertyTypes` | `x?: T` does not accept an explicit `undefined` |
| `noImplicitOverride`, `noPropertyAccessFromIndexSignature` | type semantics |
| `noImplicitReturns` | replaces `consistent-return`, which conflicts with `unicorn/no-useless-undefined` |
| `verbatimModuleSyntax`, `isolatedModules`, `noUncheckedSideEffectImports` | module emission and resolution |

Left to the linters (off in `tsconfig.json`): `noUnusedLocals`, `noUnusedParameters`, `noFallthroughCasesInSwitch`.

## Files

- `tsconfig.json`: type checking only (`noEmit`) of `src/`, `playground/`, `tests/`, `scripts/`, `.claude/hooks/`, `docs/.vitepress/` and root files.
- No `tsconfig.build.json`: `tools/declarations` writes the declarations `dist/types/` from `src/` (tests excluded), each module alone, on Oxc's isolated declarations (EN-091, ADR-0071). An exported constant whose value is not a plain literal therefore has its type written (`export const DEGREES_PER_HALF_TURN: number = DEGREES_PER_TURN / 2`), as an exported function has its return type.

## Typed rules

The type-aware rules run in oxlint on `tsgolint` (`--type-check`, EN-074): `strict-boolean-expressions`, `no-unnecessary-condition`, `switch-exhaustiveness-check`, `no-floating-promises`… know the types. A file outside `tsconfig.json` cannot be checked: add it to `include`. Oxc has no type checker: the rules that asked ESLint for a type run in the project's linter, reading the written code ([lint](./lint.md#the-project-s-linter-tools-lint)).
