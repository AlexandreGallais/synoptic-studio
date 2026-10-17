---
id: EN-108
epic: E17
feature: F21
title: The Git hooks in Rust, without husky, lint-staged, commitlint nor validate-branch-name
status: draft
points: 5
---

# E17 · F21 · EN-108 — The Git hooks in Rust, without husky, lint-staged, commitlint nor validate-branch-name

Four dev packages rewritten as one Rust crate (F21.AC4, the feature's table of dev packages): `husky`, which installs the hooks at `npm install`; `lint-staged`, which runs the formatter, the linters and the secret scan on the staged files; `@commitlint/cli` with `config-conventional` and `types`, which checks each commit message; `validate-branch-name`, which checks the branch name. Each took 0.7 to 2.6 s beside the test lane ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), Everything around the library); a commit runs two of them. Carried from EN-093's note (measured in `docs/conventions/rules.md`, Checks, hooks and conventions): the staged-file runner without ESLint's step; commitlint's rules as they are, with the 14 scopes (`eslint` dropped); the branch-name pattern as one regular expression; and a check added, that a commit on a story branch carries `Refs: <ID>.Tn` or `Closes: <ID>` with the branch's own id.

Friction of F20's run folded in (2026-10-17): the formatter, run at each commit on the staged files, prints "Tables without padding: 353 of 537 Markdown files changed" when it changed nothing.

## Acceptance criteria

- Given a fresh clone, when `npm install` runs, then a Rust command sets the hooks folder once (`core.hooksPath`), the hooks are short shell files calling the binaries, and `husky` is gone from `package.json`; a worktree made by `worktree:new` still gets its own hooks (EN-052), and `guard-bash`'s guard on `core.hooksPath` stays.
- Given staged files, when the pre-commit hook runs, then each is formatted, linted and scanned for secrets by the same commands, on the same patterns, as `.lintstagedrc.json` runs them today, the fixes staged again, and a partly staged file keeps its unstaged lines; `lint-staged` and `.lintstagedrc.json` are gone.
- Given a commit message, when the commit-msg hook checks it, then it gives the same verdict as commitlint with `config-conventional` and the project's rules on every commit of `develop`'s history and on the refused cases of commitlint's own tests, read at 21.2.3; the 14 scopes live in one place that the docs name, and commitlint's packages and `commitlint.config.ts` are gone.
- Given a commit on a story branch, when its message carries no `Refs: <ID>.Tn` nor `Closes: <ID>` with the branch's own id, then the hook refuses it, naming the footer expected; a feature, `develop` or `docs/` branch is not asked for one.
- Given a branch name, when the pre-push hook checks it, then one regular expression gives the same verdict as `.validate-branch-namerc.json` on every branch name of the repository's history, and `validate-branch-name` is gone.
- Given a pull request into `main`, when the CI's `history` job checks it, then it runs the same Rust checks on its branch name and on each of its commits, in place of `npx validate-branch-name` and `npx commitlint`.
- Given the formatter with nothing to change, when it runs, then it prints nothing; when it changes files, it names how many.
- Given a commit of one file and a push, when they are timed, then the pre-commit and commit-msg hooks' times and the branch check's are stated before and after.

## Files

`tools/git-hooks/` (new crate), `tools/format/src/main.rs`, `.husky/` (moved or renamed, decided in T1), `tools/worktree/`, `.github/workflows/ci.yml` (`history`), `package.json` (`prepare`, devDependencies), `package-lock.json`, `.lintstagedrc.json`, `commitlint.config.ts`, `.validate-branch-namerc.json` (removed), `tests/` cases that read them, `docs/tooling/git-workflow.md`, `docs/tooling/commands.md`, `docs/tooling/dependencies.md`, `docs/conventions/rules.md`, `AGENTS.md` and `.claude/skills/project-rules/SKILL.md` (where the scopes are named), `NOTICE` and `THIRD-PARTY-LICENSES.md` if a rule's code or cases are ported (ADR-0076).

## Measures (ADR-0069)

Before and after, 20 rounds each on this Mac: a commit of one Markdown file and of one TypeScript file, hooks included; the commit-msg hook alone; the branch check alone; `npm install`'s `prepare`.

## Tasks

One task = one commit, referenced as `EN-108.Tn`.

- [ ] T1 — The hooks installed by a Rust command, without husky; worktrees keep their own (3 h)
- [ ] T2 — The staged-file runner, a partly staged file kept; the formatter silent when nothing changed (4 h)
- [ ] T3 — The commit message: commitlint's rules and the 14 scopes, the story branch's footer; tested on the history and commitlint's cases (5 h)
- [ ] T4 — The branch name as one expression; the packages and configurations removed; docs; times measured (2 h)
