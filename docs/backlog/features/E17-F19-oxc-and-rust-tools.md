---
id: F19
epic: E17
title: Checks and agent tools on Oxc and Rust
status: draft
---

# E17 · F19 — Checks and agent tools on Oxc and Rust

**Benefit hypothesis**: with oxlint, oxfmt, TypeScript 7 and Rust tools in place of ESLint, Prettier, TypeScript 6 and Node.js scripts, `check:all` runs in 70 to 90 s most of the time, and the agent's tools answer in milliseconds, with every current rule kept or its loss stated.

Opened by its spike ([SP-021](../stories/E17-F19-SP-021-research-f19.md), [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md)); to refine with the Product Owner ([E17](../epics/E17-fast-tooling-with-oxc-and-rust.md)).

## Decisions of the Product Owner (2026-10-17)

Recorded as understood, with the research behind each in note 0023, section "The Product Owner's answers".

- **`check:all`**: option B — Oxc instead of ESLint and Prettier, the docs build only when the docs change; about 70 to 90 s most of the time.
- **Scope**: the library stays TypeScript, run as JavaScript. Everything around it moves to faster technology where it is sound: Rust by default for the project's scripts, hooks, agent tools and the local MCP server; for third-party tools the fastest maintained option (Oxc, TypeScript 7). No ESLint nor Prettier use is left at the end of E17.
- **Rules that need types**: none dropped without a stated reason; types from an existing tool where one gives them, a rewrite without types otherwise.
- **The project's rules**: JS plugins, since they are oxlint's documented way and the measured cost (about 1 s for the 10 rules, beside the test lane) is not noticeable.
- **Rust**: a coverage floor and an advisory check of the crates.
- **Clean-up**: one story at the end of E17 removes every TypeScript or JavaScript file and dev package left unused, last before NAM-002.
- **Platforms**: the library's tests never depend on macOS. The tools stay on the Product Owner's Mac: written with Rust's portable standard calls where one exists, a goal and not a blocker, with one cheap check and no three-system CI.

## Acceptance criteria

1. `npm run lint` runs oxlint 1.87.0 or later (each version at least 7 days old) with its native rules, `tsgolint`'s type-aware rules and the JS plugins, and reports nothing on the tree; every rule of `eslint/` is carried, rewritten, or dropped with its reason in the ADR.
2. `@typescript-eslint/prefer-destructuring` is carried by the native `eslint/prefer-destructuring` with the same options; `no-unsafe-enum-assignment` is dropped because `enum` is forbidden; `naming-convention` and the five type-aware functional rules are rewritten without types, each with tests of what it still catches and of what it no longer proves.
3. The 10 project rules, `functional/no-return-void` and the plugins oxlint lacks run as JS plugins on oxlint, tested on its `RuleTester`; `no-restricted-syntax`, `require-atomic-updates` and `consistent-this` run through `oxlint-plugin-eslint`.
4. The 21 Markdown rules run in a Rust tool of the workspace, with the findings ESLint's Markdown plugin gave on the same pages.
5. `npm run format:check` runs oxfmt on the same targets as Prettier, Markdown included.
6. `tsc` is TypeScript 7.0.2 or later: the type check reports what TypeScript 6 reported and `build` writes the same declarations; TypeScript 6 stays only under the alias `@typescript/typescript6`, for TypeDoc.
7. `check:all` runs the docs build only when `docs/`, `src/` or the playground changed, and ends in 70 to 90 s on a code story on the Product Owner's machine, lane times printed as now.
8. A `rust` lane runs `cargo fmt --check`, Clippy with warnings denied, the tests, a coverage floor (`cargo-llvm-cov`) and `cargo deny check advisories` (offline in the pre-push).
9. Every script of `scripts/` and every hook of `.claude/hooks/` is a Rust binary of the workspace, its output unchanged, except `default-font-metrics.ts`, which runs the library itself.
10. The `library` MCP server is a Rust binary on `rmcp`, with the same tools, answers and store.
11. The Rust tools use Rust's portable standard calls where one exists (`File::lock` instead of `flock`), and the `rust` lane checks them for Windows without linking.
12. At the end of E17, no ESLint, Prettier or plugin package, configuration, TypeScript script or hook with a Rust twin, nor their tests and `docs/tooling/dependencies.md` rows, is left: knip and a search for their names find nothing; `dependencies` is still empty.
13. The formatter prints Markdown tables without padding and keeps paragraphs as written (moved from F18, ADR-0068).

## Questions for the refinement

- The five type-aware functional rules and `naming-convention`: rewritten without types (the decision's default; a type written elsewhere is trusted by its name), or kept in a 9 s ESLint pass with types until an Oxc tool gives types, which keeps ESLint and typescript-eslint.
- The split proposed below: F19 for the checks, a new feature for the tools in Rust.

## Stories to write

In delivery order; numbers are given at the refinement. Proposed split, F19 being large: stories 1 to 10 stay in F19 (the checks), stories 11 to 15 form a new feature of E17, "Agent tools and the library server in Rust", after F19 and before F20 or after it as the Product Owner orders.

| #   | Type    | Title                                                                                                                                                                    | Criteria |
| --- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------- |
| 1   | Enabler | The Cargo workspace at the root, its toolchain file, warnings denied, the `rust` lane in `check:all`; the F18 crates moved in                                            | 8        |
| 2   | Enabler | The Rust checks: a coverage floor with `cargo-llvm-cov`, `cargo deny check advisories`, their rows in `docs/tooling/dependencies.md`                                     | 8        |
| 3   | Enabler | Portable Rust tools: `heavy-lock` on `File::lock`, `mutate`'s `kill` and the Unix symlinks replaced where std has a portable call, `cargo check` for Windows             | 11       |
| 4   | Enabler | TypeScript 7 for `tsc`, TypeScript 6 aliased for TypeDoc, the exit code read as 1                                                                                        | 6        |
| 5   | Enabler | oxfmt instead of Prettier: configuration, scripts, lint-staged, the 3 union types                                                                                        | 5, 13    |
| 6   | Enabler | oxlint with its native and type-aware rules, the type check in it, the scopes as `overrides`, the 110 new findings decided                                               | 1, 2     |
| 7   | Enabler | The project rules, `no-return-void` and the plugins oxlint lacks as JS plugins (`createOnce`), `oxlint-plugin-eslint`, tests on `RuleTester`                             | 3        |
| 8   | Enabler | `naming-convention` and the five type-aware functional rules rewritten without types (or the refinement's choice)                                                        | 2        |
| 9   | Enabler | The 21 Markdown rules in a Rust tool                                                                                                                                     | 4        |
| 10  | Enabler | The docs build only when the docs change; ESLint and Prettier out of `check:all`; ADR on the Oxc toolchain superseding ADR-0009 and ADR-0013                             | 7        |
| 11  | Enabler | The hooks in Rust: `guard-bash`, `track-tool-call`, `log-tool-call`, `session-state`, the status line and its parts                                                      | 9        |
| 12  | Enabler | The metrics tools in Rust: `metrics-*`, `session-report`, `turns`, `activity`, `watch-task`                                                                              | 9        |
| 13  | Enabler | The backlog and repository tools in Rust: `backlog-status`, `story-finish`, `epic-check`, `replace-text`, `branches-clean`, `records-save`, `tool-versions`, `check-all` | 9        |
| 14  | Enabler | The library tools and MCP server in Rust on `rmcp`: `library-cli`, `library-mcp` and their store, pages, HTML, chunks and usage                                          | 10       |
| 15  | Enabler | Clean-up of E17: every script, hook, configuration, test and dev package left unused removed, checked by knip and a search; last before NAM-002                          | 12       |

### Story 15 — Clean-up of E17 (draft)

Last story of E17's features, before NAM-002: once the tools run on Oxc and Rust, remove what nothing uses any more.

- Given the end of E17, when the clean-up ends, then ESLint, Prettier, typescript-eslint and every ESLint plugin are out of `package.json`, with `eslint.config.ts`, `eslint/` (except the JS plugins oxlint loads), `.prettierrc.json` and `.prettierignore`.
- Given each `scripts/*.ts` and `.claude/hooks/*.ts` with a Rust twin, when the clean-up ends, then the TypeScript file and its test are gone, and `package.json`, `.mcp.json`, `.claude/settings.json` and the husky hooks call the Rust binary.
- Given `docs/tooling/dependencies.md`, when the clean-up ends, then each removed package's row is gone and each Rust crate added has one.
- Given knip and a search of the tracked files for each removed name, when they run, then neither finds an unused file, an unused dev package, nor a name of a removed file outside the history (journal, retrospectives, ADRs).
- Given `check:all`, when it runs after the clean-up, then it is green and its time is recorded for RET-004.

## Not moved, and why

- **Vitest**: VoidZero's runner on Vite; no faster sound runner found for this suite; the test lane's cost is its setup and import (64 %), a later option.
- **Vite and Rolldown**: nothing to move — Vite 8.3.4 already bundles with Rolldown 1.2; the playground builds in 0.6 s.
- **VitePress and TypeDoc**: the docs build (VitePress 27.4 s, TypeDoc 2.8 s) is skipped when the docs do not change (criterion 7).
- **`default-font-metrics.ts`**: it runs the library to write `inter-metrics.json`; a Rust copy would duplicate the font reader.
- **secretlint, knip, npm-check-updates, `npm audit`, commitlint, lint-staged, husky**: 0.7 to 2.6 s each beside the test lane, no measured need; the husky hooks stay shell scripts calling the binaries.

## Stories

In delivery order; the other stories are written after the spike.

| ID                                                      | Type       | Title                                | Status |
| ------------------------------------------------------- | ---------- | ------------------------------------ | ------ |
| [SP-021](../stories/E17-F19-SP-021-research-f19.md)     | Spike      | Research for F19, Oxc and Rust tools | ready  |
| [CHK-020](../stories/E17-F19-CHK-020-checkpoint-f19.md) | Checkpoint | Checkpoint in the middle of F19      | draft  |
| [AUD-021](../stories/E17-F19-AUD-021-audit-f19.md)      | Audit      | Audit F19                            | draft  |
| [VAL-020](../stories/E17-F19-VAL-020-validate-f19.md)   | Validation | Validate F19                         | draft  |
