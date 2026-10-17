---
id: F19
epic: E17
title: Checks on Oxc, TypeScript 7 and Rust
status: draft
---

# E17 · F19 — Checks on Oxc, TypeScript 7 and Rust

**Benefit hypothesis**: with oxlint, oxfmt, TypeScript 7 and a Rust linter of the project in place of ESLint, Prettier and TypeScript 6, `check:all` runs in 70 to 90 s most of the time, with every current rule kept or its loss stated.

Opened by its spike ([SP-021](../stories/E17-F19-SP-021-research-f19.md), [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md)); to refine with the Product Owner ([E17](../epics/E17-fast-tooling-with-oxc-and-rust.md)). The agent tools, the library server and the clean-up of E17 moved to [F21](./E17-F21-agent-tools-in-rust.md) on 2026-10-17.

## Decisions of the Product Owner (2026-10-17)

Recorded as understood, with the research behind each in note 0023, section "The Product Owner's answers".

- **`check:all`**: option B — Oxc instead of ESLint and Prettier, the docs build only when the docs change; about 70 to 90 s most of the time.
- **Scope**: the library stays TypeScript, run as JavaScript. Every tool around it is written in Rust or Go — the project's own in Rust, third-party ones rewritten natively (TypeScript 7, `tsgolint`, Oxc); no JavaScript tool of the project's own. Vite, Vitest and VitePress stay.
- **Split**: this feature holds the checks; the agent tools in Rust, the library server and the clean-up of E17 form F21.
- **Rules that need types**: `prefer-destructuring` native, `no-unsafe-enum-assignment` dropped because `enum` is forbidden; `naming-convention` and the five type-aware functional rules rewritten without types in their strict form — `readonly` or `Readonly<…>` written in place, nothing trusted from a type defined elsewhere; nothing kept in ESLint.
- **The project's rules**: in Rust, in a linter of the project on Oxc's crates (`oxc_parser`, `oxc_ast`, `oxc_semantic`), not as JS plugins (this supersedes the JS-plugin decision taken earlier the same day).
- **TypeScript**: TypeScript 7 only; TypeScript 6 removed. TypeDoc, which needs TypeScript 6's API, is replaced.
- **Rust**: a coverage floor and an advisory check of the crates.
- **Platforms**: the library's tests never depend on macOS. The tools stay on the Product Owner's Mac: written with Rust's portable standard calls where one exists, a goal and not a blocker, with one cheap check and no three-system CI.

## Acceptance criteria

1. `npm run lint` runs oxlint 1.87.0 or later (each version at least 7 days old) with its native rules and `tsgolint`'s type-aware rules, then the project's Rust linter, and reports nothing on the tree; every rule of `eslint/` is carried, rewritten, or dropped with its reason in the ADR.
2. `@typescript-eslint/prefer-destructuring` is carried by the native `eslint/prefer-destructuring` with the same options; `no-unsafe-enum-assignment` is dropped because `enum` is forbidden; `naming-convention` and the five type-aware functional rules are rewritten without types in their strict form, each with tests of what it catches and of a type defined elsewhere that it refuses.
3. The 10 project rules, `functional/no-return-void`, `no-restricted-syntax` (its 7 selectors), `require-atomic-updates` and `consistent-this` run in the project's Rust linter on the `oxc` crates (0.153.0 or later, at least 7 days old, each crate in `docs/tooling/dependencies.md`), each with its tests carried over from `eslint/`.
4. The 21 Markdown rules run in the same Rust linter, with the findings ESLint's Markdown plugin gave on the same pages.
5. `npm run format:check` runs oxfmt on the same targets as Prettier, Markdown included.
6. TypeScript 7.0.2 or later is the only TypeScript: the type check reports what TypeScript 6 reported and `build` writes the same declarations; no package of TypeScript 6 is left.
7. The API pages of the documentation site (one per exported function, type and constant, the index and the sidebar) come from a Rust generator on Oxc instead of TypeDoc, and its checks hold: every export documented, every link resolved, nothing internal published.
8. `check:all` runs the docs build only when `docs/`, `src/` or the playground changed, and ends in 70 to 90 s on a code story on the Product Owner's machine, lane times printed as now.
9. A `rust` lane runs `cargo fmt --check`, Clippy with warnings denied, the tests, a coverage floor (`cargo-llvm-cov`) and `cargo deny check advisories` (offline in the pre-push).
10. The Rust tools use Rust's portable standard calls where one exists (`File::lock` instead of `flock`), and the `rust` lane checks them for Windows without linking.
11. The formatter prints Markdown tables without padding and keeps paragraphs as written (moved from F18, ADR-0068).

## Questions for the refinement

- The rules that run today as JS plugins of third-party packages (246 of JSDoc, Unicorn, Import and typescript-eslint not yet ported to oxlint, 7 of `unused-imports` and `@stylistic`, 12 of `eslint-plugin-functional`): under the decision that every tool is Rust or Go, each is rewritten in the Rust linter, dropped with its reason, or waits for oxlint to port it — a choice per plugin.

## Stories to write

In delivery order; numbers are given at the refinement.

| #   | Type    | Title                                                                                                                                        | Criteria |
| --- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| 1   | Enabler | The Cargo workspace at the root, its toolchain file, warnings denied, the `rust` lane in `check:all`; the F18 crates moved in                | 9        |
| 2   | Enabler | The Rust checks: a coverage floor with `cargo-llvm-cov`, `cargo deny check advisories`, their rows in `docs/tooling/dependencies.md`         | 9        |
| 3   | Enabler | Portable Rust tools: `heavy-lock` on `File::lock`, `mutate`'s `kill` and the Unix symlinks replaced by std calls, `cargo check` for Windows  | 10       |
| 4   | Enabler | TypeScript 7 alone: `tsc` 7, TypeScript 6 and TypeDoc removed, the exit code read as 1                                                       | 6        |
| 5   | Enabler | The API pages from a Rust generator on Oxc, with TypeDoc's three checks                                                                      | 7        |
| 6   | Enabler | oxfmt instead of Prettier: configuration, scripts, lint-staged, the 3 union types                                                            | 5, 11    |
| 7   | Enabler | oxlint with its native and type-aware rules, the type check in it, the scopes as `overrides`, the 110 new findings decided                   | 1, 2     |
| 8   | Enabler | The project's Rust linter on Oxc: the 10 project rules, `no-return-void`, `no-restricted-syntax` and the two core rules, their tests         | 3        |
| 9   | Enabler | `naming-convention` and the five type-aware functional rules in the Rust linter, strict form                                                 | 2        |
| 10  | Enabler | The 21 Markdown rules in the Rust linter                                                                                                     | 4        |
| 11  | Enabler | The docs build only when the docs change; ESLint and Prettier out of `check:all`; ADR on the Oxc toolchain superseding ADR-0009 and ADR-0013 | 8        |

## Not moved, and why

- **Vitest, Vite and VitePress**: kept by the Product Owner's decision. They run with TypeScript 7 alone (tried on 2026-10-17: they transform with Oxc, not `tsc`). Vite 8.3.4 already bundles with Rolldown 1.2.
- **`default-font-metrics.ts`**: it runs the library to write `inter-metrics.json`; a Rust copy would duplicate the font reader (F21 holds it).
- **secretlint, knip, npm-check-updates, `npm audit`, commitlint, lint-staged, husky**: third-party tools of 0.7 to 2.6 s each beside the test lane, no measured need; the husky hooks stay shell scripts calling the binaries. A Rust or Go replacement for each is a later question.

## Stories

In delivery order; the other stories are written after the spike.

| ID                                                      | Type       | Title                                | Status |
| ------------------------------------------------------- | ---------- | ------------------------------------ | ------ |
| [SP-021](../stories/E17-F19-SP-021-research-f19.md)     | Spike      | Research for F19, Oxc and Rust tools | ready  |
| [CHK-020](../stories/E17-F19-CHK-020-checkpoint-f19.md) | Checkpoint | Checkpoint in the middle of F19      | draft  |
| [AUD-021](../stories/E17-F19-AUD-021-audit-f19.md)      | Audit      | Audit F19                            | draft  |
| [VAL-020](../stories/E17-F19-VAL-020-validate-f19.md)   | Validation | Validate F19                         | draft  |
