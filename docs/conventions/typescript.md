# TypeScript

## Version

TypeScript **6.0** (pinned). TypeScript 7 exists, but `typescript-eslint` does not support it yet (ADR-0009). `npm run deps:outdated` only proposes TypeScript patch upgrades (`.ncurc.cjs`).

## Split with ESLint (ADR-0010)

What ESLint can check lives in ESLint; `tsconfig.json` keeps only what belongs to the type system:

| Option | Why in TypeScript |
| --- | --- |
| `strict` | `any` inference, `null`, `this`, variance: out of ESLint's reach |
| `noUncheckedIndexedAccess` | `T[number]` becomes `T \| undefined`: a type, not syntax |
| `exactOptionalPropertyTypes` | `x?: T` does not accept an explicit `undefined` |
| `noImplicitOverride`, `noPropertyAccessFromIndexSignature` | type semantics |
| `noImplicitReturns` | replaces `consistent-return`, which conflicts with `unicorn/no-useless-undefined` |
| `verbatimModuleSyntax`, `isolatedModules`, `noUncheckedSideEffectImports` | module emission and resolution |

Left to ESLint (off in `tsconfig.json`): `noUnusedLocals`, `noUnusedParameters`, `noFallthroughCasesInSwitch`.

## Files

- `tsconfig.json`: type checking only (`noEmit`) of `src/`, `playground/`, `eslint/`, `tests/`, `scripts/`, `.claude/hooks/`, `docs/.vitepress/` and root files.
- No `tsconfig.build.json`: `tools/declarations` writes the declarations `dist/types/` from `src/` (tests excluded), each module alone, on Oxc's isolated declarations (EN-091, ADR-0071). An exported constant whose value is not a plain literal therefore has its type written (`export const DEGREES_PER_HALF_TURN: number = DEGREES_PER_TURN / 2`), as an exported function has its return type.

## Typed rules

The type-aware rules run in oxlint on `tsgolint` (`--type-check`, EN-074): `strict-boolean-expressions`, `no-unnecessary-condition`, `switch-exhaustiveness-check`, `no-floating-promises`… know the types. A file outside `tsconfig.json` cannot be checked: add it to `include`. ESLint reads no type since EN-076: the rules that asked for one run in the project's linter, reading the written code ([lint](./lint.md#the-project-s-linter-tools-lint)).
