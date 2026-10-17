# ADR-0010 — Strictness carried by ESLint first, TypeScript second

**Status**: Accepted, its consequences partly superseded by ADR-0071 (2026-10-17): `tsc` out of `npm run check`, the declarations written by Oxc.

## Context

The Product Owner asked that checks stop at lint rather than at the TypeScript build: ESLint carries every rule it can express, and TypeScript only the rules ESLint cannot.

## Decision

1. A rule ESLint can check **lives in ESLint**, even when a `tsconfig` option exists: `noUnusedLocals`, `noUnusedParameters`, `noFallthroughCasesInSwitch`, `allowUnreachableCode` are left to ESLint (`unused-imports/no-unused-vars`, `no-fallthrough`, `no-unreachable`).
2. Rules redundant with the compiler stay **enabled** in ESLint when they produce no false positive (`no-const-assign`, `no-dupe-keys`, `getter-return`…): lint alone is enough to block. Disabled exceptions: `no-undef` and `no-redeclare` (false positives on types, typescript-eslint FAQ).
3. `tsconfig.json` keeps only what ESLint cannot guarantee: `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitOverride`, `noImplicitReturns`, `noPropertyAccessFromIndexSignature`, `verbatimModuleSyntax`, `isolatedModules`, `noUncheckedSideEffectImports`.
4. **Every rule is `error`**, none is `warn`, and `--max-warnings=0`. A disabled rule is disabled explicitly, with its reason as a comment.
5. Every rule of every plugin is **decided once and only once** (enabled, configured or disabled); an audit test checks it.

## Consequences

- `npm run check` = Prettier + ESLint + `tsc` + Vitest + `npm audit`. `vite build` does not check types; `tsc -p tsconfig.build.json` emits the `.d.ts`.
- One-off exceptions use `// eslint-disable-next-line rule -- justification`. Unused directives are errors (`reportUnusedDisableDirectives`).

## References

ADR-0009, [typescript.md](../conventions/typescript.md), [lint.md](../conventions/lint.md)
