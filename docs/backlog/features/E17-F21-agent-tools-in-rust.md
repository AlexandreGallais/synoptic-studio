---
id: F21
epic: E17
title: Agent tools and the library server in Rust
status: in-progress
---

# E17 · F21 — Agent tools and the library server in Rust

**Benefit hypothesis**: with every hook, script and agent tool of the project, and the `library` MCP server, rewritten as Rust binaries, each call answers in milliseconds instead of Node.js's 60 to 280 ms, and E17 ends with no TypeScript or JavaScript tool of the project's own left.

Split from [F19](./E17-F19-oxc-and-rust-tools.md) by the Product Owner on 2026-10-17; its sources come from F19's spike ([SP-021](../stories/E17-F19-SP-021-research-f19.md), [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md)). Refined by the agent on 2026-10-17 from the decisions already taken (below), as authorised that day ([E17](../epics/E17-fast-tooling-with-oxc-and-rust.md)); it comes after F19, which builds the Cargo workspace it uses.

## Decisions of the Product Owner (2026-10-17)

- Every tool around the library is written in Rust or Go; the project's own in Rust. Vite, Vitest and VitePress stay.
- The `library` MCP server moves to Rust too.
- Every dev package is rewritten in Rust when that is sensible (fewer dependencies, less supply-chain risk), kept otherwise with its reason (table below).
- At the end of E17, one story removes every TypeScript or JavaScript script, hook, configuration and dev package left unused, last before NAM-002.
- The tools stay on the Product Owner's Mac: Rust's portable standard calls where one exists, a goal and not a blocker.

## Acceptance criteria

1. Every hook of `.claude/hooks/` is a Rust binary of the workspace, its output and its exit code unchanged, called by `.claude/settings.json` (EN-104).
2. Every script of `scripts/` is a Rust binary of the workspace, its output unchanged, called by `package.json`, except `default-font-metrics.ts`, which runs the library itself (EN-103, EN-107).
3. The `library` MCP server is a Rust binary on `rmcp` (3.5.1 or later, at least 7 days old), with the same tools, the same answers and the same store, called by `.mcp.json` (EN-106).
4. Every dev package of `package.json` is kept with its reason or rewritten as a Rust binary of the workspace: the Git hooks and their checks, the dependency checks and the secret scan; the rewritten packages are removed (EN-108 to EN-111).
5. At the end of E17, no ESLint, Prettier or plugin package, configuration, TypeScript script or hook with a Rust twin, nor their tests and `docs/tooling/dependencies.md` rows, is left: the unused-file check and a search for their names find nothing; `dependencies` is still empty (EN-113).
6. A release is prepared and published by a Rust tool of the workspace in place of release-please: the version and the changelog are committed in the pull request `develop` → `main` before it merges, the merge into `main` is tagged and its GitHub release published with the same notes, and no commit follows the merge; the notes follow the rules of `docs/tooling/releases.md`, drawn from `REF-KEEP-A-CHANGELOG` and `REF-COMMON-CHANGELOG` (EN-112).
7. Each tool of the workspace records each use — the tool, its start, its duration, its exit code — through one shared function into `.git/metrics/tool-uses.jsonl`, and `npm run metrics` lists the uses per tool (EN-102).
8. One command prints one row of a repository table or one section of a page, found by its id or its heading, and nothing else (EN-105).
9. `check:all` fails, naming the lines, when a run changed `.git/config`, and every Rust test that runs `git` starts it without the inherited `GIT_*` variables (EN-103).

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
| `eslint`, `@eslint/js`, `typescript-eslint`, `@typescript-eslint/*`, `eslint-config-prettier`, the ESLint plugins, `@stylistic/eslint-plugin`, `prettier`, `jiti`, `typedoc` and its plugins, TypeScript 6 | lint, format, API pages | removed by F19 (EN-072, EN-073, EN-085) |

`npm audit` is part of npm, not a package: kept.

## The refinement (2026-10-17)

Each row of the draft table of stories, now in this file's history, became one story; stories 11 and 12 were moved from F20 at its refinement (F20's research note 0024). The order below differs from the draft's for these reasons:

- **The record of each tool's use first** (draft 11, EN-102): every tool ported after it records its use from its first commit, instead of being changed again.
- **The backlog and repository tools early** (draft 3, EN-103): most frictions of F20's run sit in them, and every merge of F21's own run meets them. `tool-versions` moves to the dependency checks (draft 6, EN-109), where the feature's table of dev packages already put it.
- **The library server before the metrics** (drafts 4 and 2, EN-106 then EN-107): the metrics' Sources section reads the library's store, so the SQLite crate is chosen once.
- **The CI before the release** (drafts 9 and 8, EN-111 then EN-112): the release's publishing job runs a Rust binary on `main`, on the cached build.
- **A spike** (SP-023): the feature frame asks for one (ADR-0020); it records the crates, the rewritten packages' behaviour and licences, and each tool's time before its port, so that each story states its time after against the same table (ADR-0069).
- **Each port removes its own TypeScript twin** once its output is proven the same; the clean-up (draft 10, EN-113) finds what is left.

Frictions logged during F20's run, folded into the stories: `story:finish`'s push of a rebased branch and the done step's conflicts, `git:sync`'s silent stop, the `.git/config` guard and the tests' `GIT_*` variables, `replace`'s unread block (EN-103); `guard-bash`'s working folder, its refusal of the brief's `cd` form, and a script missing from an older branch (EN-104); `model-tokens.ts` with the session report (EN-107); the formatter's message when nothing changed (EN-108).

## Decisions of 2026-10-17 (Product Owner)

- The release tool replaces release-please entirely, the GitHub release included. The version bump and the changelog are written inside the pull request `develop` → `main`, before it merges, so that no commit is made by the CI after the merge. The release notes are to be better than release-please's, from what the documentation on good release notes says. The tool may improve anything else it finds.
- The library server moves to Rust on a SQLite crate with full-text search, checked as every crate is (version at least 7 days old, licence, advisories).
- F21 is refined by the agent from the decisions already taken (EN-093's verdicts, ADR-0073, ADR-0076); a question that only the Product Owner can answer parks its story and is pushed to them, and the run goes on.

## Note from EN-093 (2026-10-17)

- **For F21's feature file** (its draft table of stories, as a note): carry only what EN-093 kept, measured in `docs/conventions/rules.md` (Checks, hooks and conventions). Story 5 (Git hooks): the staged-file runner without ESLint's step; commitlint's rules as they are, with the 14 scopes (`eslint` dropped); **add** a check that a commit on a story branch carries `Refs: <ID>.Tn` or `Closes: <ID>` with the branch's own id; the branch-name pattern as one regular expression. Story 3 (repository tools): `check-all` without a `check` twin to keep in step (the story skill no longer runs `npm run check` per task); `replace-text` kept, it is the agent's most used edit tool. Story 2 (metrics): kept whole; the friction count by rule that EN-093 made by hand from the transcripts would be a cheap addition to `metrics-report` (findings by rule, per epic), so that the next review of the rules reads it instead of grepping.

## Feature plan

| Criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-104 | AUD-023, VAL-022 |
| 2 | EN-103, EN-107 | AUD-023, VAL-022 |
| 3 | EN-106 | AUD-023, VAL-022 |
| 4 | EN-108, EN-109, EN-110, EN-111 | AUD-023, VAL-022 |
| 5 | EN-113 | AUD-023, VAL-022 |
| 6 | EN-112 | AUD-023, VAL-022 |
| 7 | EN-102 | AUD-023, VAL-022 |
| 8 | EN-105 | AUD-023, VAL-022 |
| 9 | EN-103 | AUD-023, VAL-022 |

## Stories

In delivery order. Stories stay `draft` until F21's `/run` sets them `ready`.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-023](../stories/E17-F21-SP-023-research-f21.md) | Spike | Research for F21 | done |
| [EN-102](../stories/E17-F21-EN-102-each-tool-records-its-use.md) | Enabler | Each tool of our own records its use | done |
| [EN-103](../stories/E17-F21-EN-103-backlog-and-repository-tools-in-rust.md) | Enabler | The backlog and repository tools in Rust | done |
| [EN-104](../stories/E17-F21-EN-104-claude-code-hooks-in-rust.md) | Enabler | The Claude Code hooks and the status line in Rust | done |
| [EN-105](../stories/E17-F21-EN-105-one-row-or-section-of-a-page.md) | Enabler | One row or one section of a repository page | done |
| [EN-106](../stories/E17-F21-EN-106-library-server-in-rust.md) | Enabler | The library server and its command line in Rust on rmcp | done |
| [EN-107](../stories/E17-F21-EN-107-metrics-tools-in-rust.md) | Enabler | The metrics tools in Rust | done |
| [CHK-022](../stories/E17-F21-CHK-022-checkpoint-f21.md) | Checkpoint | Checkpoint in the middle of F21 | ready |
| [EN-108](../stories/E17-F21-EN-108-git-hooks-in-rust.md) | Enabler | The Git hooks in Rust, without husky, lint-staged, commitlint nor validate-branch-name | done |
| [EN-109](../stories/E17-F21-EN-109-dependency-checks-in-rust.md) | Enabler | The dependency checks in Rust, without npm-check-updates nor knip | ready |
| [EN-110](../stories/E17-F21-EN-110-secret-scan-in-rust.md) | Enabler | The secret scan in Rust, without secretlint | ready |
| [EN-111](../stories/E17-F21-EN-111-ci-on-the-rust-tools.md) | Enabler | The CI on the Rust tools, built and cached once | ready |
| [EN-112](../stories/E17-F21-EN-112-release-tool-in-rust.md) | Enabler | The release made by our own tool, in place of release-please | ready |
| [EN-113](../stories/E17-F21-EN-113-clean-up-of-e17.md) | Enabler | Clean-up of E17, every tool left unused removed | ready |
| [AUD-023](../stories/E17-F21-AUD-023-audit-f21.md) | Audit | Audit F21 | ready |
| [VAL-022](../stories/E17-F21-VAL-022-validate-f21.md) | Validation | Validate F21 | ready |
