---
id: EN-113
epic: E17
feature: F21
title: Clean-up of E17, every tool left unused removed
status: draft
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

## Tasks

One task = one commit, referenced as `EN-113.Tn`.

- [ ] T1 — The search and the unused-file check; every file, test, rule, configuration and package left removed (4 h)
- [ ] T2 — `docs/tooling/dependencies.md`, the docs and the agent's rules in step; `check:all` timed for RET-004 (2 h)
