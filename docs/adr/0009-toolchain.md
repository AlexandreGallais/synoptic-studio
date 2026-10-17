# ADR-0009 — Toolchain: TypeScript 6, Vite, Vitest, ESLint, Prettier, VitePress

**Status**: Superseded by ADR-0075 (2026-10-17): the toolchain moved to Oxc and TypeScript 7; its rules on dependencies stay, restated there.

## Context

The Product Owner asked for a TypeScript library built with Vite, a web server to try the interface, the strictest possible ESLint, Prettier, a Markdown documentation site, Git conventions enforced by tools, and up-to-date, vulnerability-free dependencies. Zero runtime dependency (project rule, now in `AGENTS.md`).

## Decision

- All dependencies are `devDependencies`, pinned to an exact version (`--save-exact`). Each one is justified in [dependencies.md](../tooling/dependencies.md); a test fails when a package is missing from that page.
- **TypeScript 6.0**: TypeScript 7 (native compiler) is out, but `typescript-eslint` requires `typescript <6.1`, and typed lint rules depend on it.
- **VitePress 2 (alpha)** instead of VitePress 1.6 (stable): 1.6 bundles Vite 5 and esbuild ≤0.24, flagged by `npm audit` (GHSA-67mh-4wv8-2f99); 2.0 uses Vite 8, the project's own version.
- **No `eslint-plugin-prettier`** (ADR-0013).
- `deepmerge-ts` is forced to `^8` through `overrides` (advisory GHSA-ggr8-5vv4-36mx in the version pulled by `eslint-plugin-functional`).
- Freshness and safety are enforced: `npm run deps:audit` (any severity) in `check` and before `build`; `npm run deps:outdated` (npm-check-updates, latest versions, TypeScript limited to patches) in CI and before `git push`.

## Consequences

- Upgrading to TypeScript 7: when `typescript-eslint` supports it (new ADR).
- Every plugin upgrade may add rules: the audit test (`eslint/config.test.ts`) fails until they are decided.
- Moving VitePress to a stable 2.x as soon as it is released.

## References

`REF-ESLINT-TS-CONFIG`, `REF-PRETTIER-LINTERS`, [dependencies.md](../tooling/dependencies.md)
