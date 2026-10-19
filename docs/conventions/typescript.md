# TypeScript

## Version

TypeScript 7 (`typescript` 7.0.2, the native compiler) is the only TypeScript (ADR-0075). It checks the types inside `npm run lint`, through oxlint's type check, `typeCheck` in the `options` of `.oxlintrc.json` (ADR-0071, EN-121), and alone with `npx tsc --noEmit`.

## What TypeScript checks, what the linters check (ADR-0075)

Each check lives in one tool. What a linter can check is a lint rule. `tsconfig.json` keeps only what belongs to the type system:

| Option | Why in TypeScript |
| --- | --- |
| `strict` | `any` inference, `null`, `this`, variance: out of a linter's reach |
| `noUncheckedIndexedAccess` | `T[number]` becomes `T \| undefined`: a type, not syntax |
| `exactOptionalPropertyTypes` | `x?: T` does not accept an explicit `undefined` |
| `noImplicitOverride`, `noPropertyAccessFromIndexSignature` | type semantics |
| `noImplicitReturns` | replaces `consistent-return`, which conflicts with `unicorn/no-useless-undefined` |
| `verbatimModuleSyntax`, `isolatedModules`, `noUncheckedSideEffectImports` | module emission and resolution |

The linters handle `noUnusedLocals`, `noUnusedParameters` and `noFallthroughCasesInSwitch`, so these options are off in `tsconfig.json`.

## Files

- `tsconfig.json` only checks types (`noEmit`). It covers `src/`, `playground/`, `tests/`, `scripts/`, `docs/.vitepress/` and the root files. A file outside its `include` is not type-checked, and the type-aware rules cannot read it: add it to `include`.
- The declarations in `dist/types/` are written by `tools/declarations`, one module at a time, from `src/` without its tests (Oxc's isolated declarations, ADR-0071). Each module must therefore state its own types: an exported function writes its return type, and an exported constant whose value is not a plain literal writes its type (`export const DEGREES_PER_HALF_TURN: number = DEGREES_PER_TURN / 2`).

## Rules that need types

The type-aware rules (`strict-boolean-expressions`, `no-unnecessary-condition`, `switch-exhaustiveness-check`, `no-floating-promises`…) run in oxlint on `tsgolint`. Oxc has no type checker, so a rule of the project's linter that needs a type reads the written code instead ([lint](./lint.md#the-linter-of-the-project)).
