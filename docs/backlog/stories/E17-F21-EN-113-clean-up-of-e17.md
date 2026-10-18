---
id: EN-113
epic: E17
feature: F21
title: Clean-up of E17, every tool left unused removed
status: ready
points: 3
---

# E17 · F21 · EN-113 — Clean-up of E17, every tool left unused removed

Decided on 2026-10-17 (F21's decisions): at the end of E17, one story removes every TypeScript or JavaScript script, hook, configuration and dev package left unused, last among the features' stories, before NAM-002 (F21.AC5). The ports of F21 remove their own TypeScript twins as they go; this story finds what is left, with the unused-file check of EN-109 and a search of the tracked files for each removed name. `default-font-metrics.ts` stays: it runs the library itself (F21.AC2).

## Acceptance criteria

- Given F19's removals of ESLint, Prettier, typescript-eslint, the ESLint plugins, TypeScript 6 and TypeDoc (EN-072, EN-073, EN-085), when the clean-up ends, then a search finds none of their names outside the history.
- Given each `scripts/*.ts` and `.claude/hooks/*.ts` with a Rust twin, when the clean-up ends, then the TypeScript file and its test are gone, and `package.json`, `.mcp.json`, `.claude/settings.json` and the Git hooks call the Rust binary.
- Given `docs/tooling/dependencies.md`, when the clean-up ends, then each removed package's row is gone and each Rust crate added has one.
- Given the unused-file check of EN-109 and a search of the tracked files for each removed name, when they run, then neither finds an unused file, an unused dev package, nor a name of a removed file outside the history (journal, retrospectives, ADRs, research notes).
- Given the rules and tests written for the TypeScript tools only — oxlint's and the project linter's settings for `scripts/` and `.claude/hooks/`, `tests/node-run-imports.test.ts`, the `knip.json` entries — when nothing they cover is left, then they are removed too.
- Given `package.json`, when the clean-up ends, then `dependencies` is still empty, and each dev package left is one the feature's table keeps, with its reason.
- Given `check:all`, when it runs after the clean-up, then it is green, and its time and each lane's are recorded for RET-004 beside E17's start (170 to 217 s).

## Files

What the search finds: `scripts/`, `.claude/hooks/`, `tests/`, `package.json`, `package-lock.json`, `.oxlintrc.json`, `tools/lint/src/settings.rs`, `tsconfig.json`, `docs/tooling/dependencies.md`, `docs/tooling/*.md`, `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/skills/`, `NOTICE`, `THIRD-PARTY-LICENSES.md`.

## Measures (ADR-0069)

`check:all` and each of its lanes, three runs on this Mac, against the figures of E17's start; the count of files, lines and dev packages before and after.

The tracked files, their lines, the TypeScript files of the tooling outside `tools/` (`scripts/`, `.claude/`, `tests/`, the root configurations) and the dev packages, counted by `git ls-tree` at each commit:

| At | Files | Lines | TypeScript tooling files | Dev packages |
| --- | --- | --- | --- | --- |
| E17's start (`6e15c0c6`, RET-003 merged) | 2,096 | 115,765 | 87 | 38 |
| this story's start (`6fb8346a`, EN-112 done) | 2,732 | 277,803 | 48 | 13 |

## The search (T1)

On 2026-10-18, from this story's start: `target/release/dependencies unused` finds no unused file, export nor dev package; `scripts/` holds `default-font-metrics.ts` and its test only, `.claude/` no TypeScript; `package.json`, `.mcp.json`, `.claude/settings.json` and `.githooks/` call only Rust binaries, `node scripts/default-font-metrics.ts` and the kept packages. A search of the tracked files for each removed package (`eslint`, `prettier`, `typedoc`, `husky`, `lint-staged`, `commitlint`, `secretlint`, `knip`, `npm-check-updates`, `validate-branch-name`, `jiti`, `release-please`) and each removed folder (`.claude/hooks`) found, outside the history:

| Found | Done |
| --- | --- |
| `.claude/hooks/**/*.ts` in the tooling globs of `.oxlintrc.json`, `tools/lint/src/scopes.rs` (and its `node-run` scope), a test path of `tools/lint/src/rules/mod.rs`, `docs/conventions/lint.md` | removed |
| `release-please--` in four `if:` of `ci.yml` and `codeql.yml`, two assertions of `tests/ci-caches.test.ts`, `docs/tooling/ci.md` | removed; a test proves neither workflow names release-please |
| Temporal: `--harmony-temporal` in `vite.config.ts`, `esnext.temporal` in `tsconfig.json`, its line in `docs/tooling/metrics.md`: only the metrics scripts used it | removed |
| the "hooks and scripts" Node.js runs, in `tsconfig.json` and `.claude/rules/tooling.md` | now the one script left, `scripts/default-font-metrics.ts` |
| F18's criterion 1, release-please's pull request on `develop` | its first half to strike through, superseded by ADR-0079: the line is under `## For the main thread`, a feature file being shared (the pre-push hook refuses it on a story branch) |
| `HUSKY=0` in `CLAUDE.md`, `AGENTS.md`, `project-rules` and `guard-bash` | T4 |
| `.husky/` in ADR-0078 | T5, a new ADR |

Kept, with their reason:

- `tests/node-run-imports.test.ts` and the lint scope `node-run`: `scripts/default-font-metrics.ts` still runs under Node.js (F21.AC2).
- The names of the ported packages in `NOTICE`, `THIRD-PARTY-LICENSES.md`, `tests/third-party-licences.test.ts` and the module comments of `tools/` ("ported from", "rewritten from"): ADR-0076 asks a ported tool to name its source and its licence.
- The rule names of `tools/lint` (`@typescript-eslint/naming-convention`, `eslint-plugin-unicorn`'s), of `.oxlintrc.json` and of the disable directives: oxlint and the project's linter keep ESLint's names for the rules they run (ADR-0075).
- `tests/eslint-rules-accounted.test.ts` and `tests/eslint-rules.txt`, `tests/oxlint-config.test.ts`'s "ESLint out", `tests/check-targets.test.ts`'s "never Prettier", `tests/release-flow.test.ts`'s "no release-please": each proves a removal stays done, as F19's and F21's criteria ask.
- `prettier` in `.editorconfig`, `docs/conventions/formatting.md`, `docs/references.md` and the guides, `typedoc` in `docs/tooling/documentation-site.md` and `tools/api-pages`: the source of a rule kept (the width, the API pages' layout), a reference row, or a page of a validated feature.
- `jiti` in `package-lock.json`: an optional peer of Vite, never installed.

## Tasks

One task = one commit, referenced as `EN-113.Tn`.

- [ ] T1 — The search and the unused-file check; every file, test, rule, configuration and package left removed (4 h)
- [ ] T2 — `docs/tooling/dependencies.md`, the docs and the agent's rules in step (2 h)

Added by the main thread on 2026-10-18, from the frictions of F21's run:

- [ ] T3 — The `.git/config` guard of `check-all` compares only what a test could break: `[core]`, `[user]`, `[commit]` and the remotes' URLs, not the branch tracking lines a parallel worktree's `push -u` writes (a false positive at CHK-022) (1 h)
- [ ] T4 — `guard-bash`: a `|` inside quotes splits nothing; `HUSKY=0` no longer refused, husky being gone, while `--no-verify`, `commit -n` and any change of `core.hooksPath` still are; reading the hooks path is let through; its messages name the Rust Git hooks (2 h)
- [ ] T5 — A new ADR for the Git hooks installed in `.githooks/` by `git-hooks install`, superseding ADR-0078's `.husky/` (1 h)
- [ ] T6 — `npm run fix` runs `cargo fmt` and `cargo clippy --fix` too (1 h)
- [ ] T7 — `worktree:new` copies the main checkout's `target/release` before building (1 h)
- [ ] T8 — A rule: no personal data of the Product Owner in a request an agent sends or a tool codes (30 min)
- [ ] T9 — The dependency bump due on 2026-10-18 (`serde_json` 1.0.152 → 1.0.154) (30 min)
- [ ] T10 — `check:all` timed for RET-004, three runs (1 h)

## For the main thread

F18's criterion 1, for the `done` commit (the pre-push hook refuses a feature file on a story branch, ADR-0065), in `docs/backlog/features/E17-F18-a-safe-and-cheap-parallel-run.md`:

```markdown
1. ~~Release pull request on `develop`~~, only the release on `main`. The first half superseded on 2026-10-18 by [ADR-0079](../../adr/0079-the-release-on-a-release-branch-by-our-own-tool.md): the project's tool prepares a release on a release branch from `develop`, without release-please's pull request (EN-112).
```
