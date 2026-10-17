---
id: F21
epic: E17
title: Agent tools and the library server in Rust
status: draft
---

# E17 · F21 — Agent tools and the library server in Rust

**Benefit hypothesis**: with every hook, script and agent tool of the project, and the `library` MCP server, rewritten as Rust binaries, each call answers in milliseconds instead of Node.js's 60 to 280 ms, and E17 ends with no TypeScript or JavaScript tool of the project's own left.

Split from [F19](./E17-F19-oxc-and-rust-tools.md) by the Product Owner on 2026-10-17; its sources come from F19's spike ([SP-021](../stories/E17-F19-SP-021-research-f19.md), [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md)). To refine with the Product Owner ([E17](../epics/E17-fast-tooling-with-oxc-and-rust.md)); it comes after F19, which builds the Cargo workspace it uses.

## Decisions of the Product Owner (2026-10-17)

- Every tool around the library is written in Rust or Go; the project's own in Rust. Vite, Vitest and VitePress stay.
- The `library` MCP server moves to Rust too.
- Every dev package is rewritten in Rust when that is sensible (fewer dependencies, less supply-chain risk), kept otherwise with its reason (table below).
- At the end of E17, one story removes every TypeScript or JavaScript script, hook, configuration and dev package left unused, last before NAM-002.
- The tools stay on the Product Owner's Mac: Rust's portable standard calls where one exists, a goal and not a blocker.

## Acceptance criteria

1. Every hook of `.claude/hooks/` is a Rust binary of the workspace, its output and its exit code unchanged, called by `.claude/settings.json`.
2. Every script of `scripts/` is a Rust binary of the workspace, its output unchanged, called by `package.json`, except `default-font-metrics.ts`, which runs the library itself.
3. The `library` MCP server is a Rust binary on `rmcp` (3.5.1 or later, at least 7 days old), with the same tools, the same answers and the same store, called by `.mcp.json`.
4. Every dev package of `package.json` is kept with its reason or rewritten as a Rust binary of the workspace: the Git hooks and their checks, the dependency checks and the secret scan; the rewritten packages are removed.
5. At the end of E17, no ESLint, Prettier or plugin package, configuration, TypeScript script or hook with a Rust twin, nor their tests and `docs/tooling/dependencies.md` rows, is left: the unused-file check and a search for their names find nothing; `dependencies` is still empty.

## The dev packages, one by one

Asked by the Product Owner on 2026-10-17: each dev package of `package.json` is rewritten in Rust by the project when that is sensible — fewer dependencies, less supply-chain risk — and kept only when a rewrite is not sensible, with its reason.

| Package | Does here | Verdict |
| --- | --- | --- |
| `vite`, `vitest`, `@vitest/coverage-v8`, `vitepress` | builds, tests and coverage, the documentation site | kept: the Product Owner's exception; VoidZero's tools on Oxc and Rolldown |
| `typescript` (7) | the type check and the declarations | kept: a compiler, already native (Go) |
| oxlint, oxfmt, `oxlint-tsgolint` (added by F19) | lint, format, type-aware rules | kept: native (Rust, Go), maintained by Oxc |
| `happy-dom` | the DOM of the playground's tests | kept: Vitest's test environment, part of the test runner |
| `fast-check`, `@fast-check/vitest` | property tests of the library | kept: the library's tests are TypeScript and run in Vitest |
| `@types/node` | Node.js types for tests and configuration files | kept: type declarations, no code run |
| `mermaid` | diagrams of the documentation site, drawn in the browser | kept: part of the VitePress site |
| `husky` | installs the Git hooks at `npm install` | rewritten: a Rust `hooks install` sets the hooks folder once; the guard on `core.hooksPath` stays for the agent's commands, the install named in the ADR |
| `lint-staged` | runs the formatter, the linters and the secret scan on the staged files | rewritten: the staged files from Git, then the same commands |
| `@commitlint/cli`, `@commitlint/config-conventional`, `@commitlint/types` | checks each commit message (type, scope, subject, footer) | rewritten: a pattern and a few rules on one message |
| `validate-branch-name` | checks the branch name at commit | rewritten: one pattern |
| `secretlint`, `@secretlint/secretlint-rule-preset-recommend` | scans every staged and tracked file for secrets | rewritten: each pattern of the preset carried with its source and a test; the preset's patterns are read first, so that none is lost |
| `npm-check-updates` | every package on its latest version at least 7 days old | rewritten: one registry request per package, the release dates compared; `deps:tools` (`tool-versions.ts`) joins it |
| `knip` | unused files, exports and dev packages | rewritten on Oxc's parser and `oxc_resolver`, for this repository's layout (one export per file, barrels) |
| `eslint`, `@eslint/js`, `@eslint/markdown`, `typescript-eslint`, `@typescript-eslint/*`, `eslint-config-prettier`, the ESLint plugins, `@stylistic/eslint-plugin`, `prettier`, `jiti`, `typedoc` and its plugins, TypeScript 6 | lint, format, API pages | removed by F19 and this feature's clean-up |

`npm audit` is part of npm, not a package: kept.

## Stories to write

In delivery order; numbers are given at the refinement.

| # | Type | Title | Criteria |
| --- | --- | --- | --- |
| 1 | Enabler | The hooks in Rust: `guard-bash`, `track-tool-call`, `log-tool-call`, `session-state`, the status line and its parts | 1 |
| 2 | Enabler | The metrics tools in Rust: `metrics-*`, `session-report`, `turns`, `activity`, `watch-task` | 2 |
| 3 | Enabler | The backlog and repository tools in Rust: `backlog-status`, `story-finish`, `epic-check`, `replace-text`, `branches-clean`, `records-save`, `tool-versions`, `check-all` | 2 |
| 4 | Enabler | The library tools and MCP server in Rust on `rmcp`: `library-cli`, `library-mcp` and their store, pages, HTML, chunks and usage; a SQLite crate with full-text search chosen | 3 |
| 5 | Enabler | The Git hooks in Rust: hooks installed without husky, the staged-file runner (lint-staged), the commit message (commitlint) and the branch name (validate-branch-name) | 4 |
| 6 | Enabler | The dependency checks in Rust: latest versions at least 7 days old (npm-check-updates, `tool-versions.ts`), unused files, exports and packages (knip) on Oxc | 4 |
| 7 | Enabler | The secret scan in Rust: secretlint's recommended patterns carried, each with its source and a test | 4 |
| 8 | Enabler | Clean-up of E17: every script, hook, configuration, test and dev package left unused removed, checked by the unused-file check and a search; last before NAM-002 | 5 |

### Story 8 — Clean-up of E17 (draft)

Last story of E17's features, before NAM-002: once the tools run on Oxc and Rust, remove what nothing uses any more.

- Given the end of E17, when the clean-up ends, then ESLint, Prettier, typescript-eslint, every ESLint plugin, TypeScript 6 and TypeDoc are out of `package.json`, with `eslint.config.ts`, `eslint/`, `.prettierrc.json`, `.prettierignore` and `typedoc.json`.
- Given each `scripts/*.ts` and `.claude/hooks/*.ts` with a Rust twin, when the clean-up ends, then the TypeScript file and its test are gone, and `package.json`, `.mcp.json`, `.claude/settings.json` and the husky hooks call the Rust binary.
- Given `docs/tooling/dependencies.md`, when the clean-up ends, then each removed package's row is gone and each Rust crate added has one.
- Given the unused-file check of story 6 and a search of the tracked files for each removed name, when they run, then neither finds an unused file, an unused dev package, nor a name of a removed file outside the history (journal, retrospectives, ADRs).
- Given `check:all`, when it runs after the clean-up, then it is green and its time is recorded for RET-004.

## Stories

Written at the refinement, with the checkpoint, audit and validation stories of every feature.
