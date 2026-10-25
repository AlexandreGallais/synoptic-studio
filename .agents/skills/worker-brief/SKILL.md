---
name: worker-brief
description: How a worker sets up its worktree, works one story, commits, pushes and stops before the pull request; preloaded by the story-worker and researcher agent types (EN-065).
user-invocable: false
---

# Worker brief

A worker works one story in its own worktree, in the background, and stops before the pull request. The main thread's prompt names the story, its feature branch and the worktree. The `story-worker` and `researcher` types start with this brief and `project-rules`; a `general-purpose` worker is given this file's path and reads `AGENTS.md` and `CLAUDE.md` too.

## Setup

1. **Only inside the worktree** (`.claude/worktrees/<id>`, made by `target/release/worktree-new <ID>`): every Bash call starts with `cd "<worktree>" || exit 9`, Write and Edit take absolute paths inside it, never `git -C`. Read every rule from the worktree: the main checkout may be older.
2. **One Git command per call**, never chained by `&&`, `;` or a pipe; a chain of Git reads goes through (`git status; git log -3`).
3. Start with `target/release/git-sequences sync`; `git-sequences state` prints the branch, its base, the changes and the commits ahead.
4. A scratch folder of its own, `<scratchpad>/<story id>`.

## Read first

- The story from the records, `target/release/knowledge show <ID>`, then what it cites: its feature, rules, decisions, derivations, sources (`knowledge show <id>`).
- The rules of the area it touches, in `.agents/rules/`: `library.md` (`src/`, `playground/`), `tooling.md` (tooling, tests, `tools/`), `docs.md` (Markdown); a subagent does not load them by itself. The conventions: `docs/conventions/`.

## Work

- **Tests before or with the code**, each new one seen failing once, the story's edge cases among them; a library story breaks each new test once on purpose, a tooling story leaves the mutations to its epic's audit.
- **One commit per task**, footer `Refs: <ID>.Tn` and the attribution lines, by `target/release/git-sequences commit -F <file or -> <paths>`; before it, `npm run fix` (lint and type errors) and the touched tests (`npx vitest run <paths>`, `cargo test -p <crate>`).
- **The records with the code**: a rule, a source, a derivation, a decision in the same task; log rows and incidents by `knowledge add <kind> -`. No Markdown doc is written.
- **A binary that runs a command given to it** (a wrapper): `guard-bash`'s reading of that command lands in a task before the binary, and the worker hands back once it is pushed, so that the main thread rebuilds `guard-bash`.
- **Never a polling loop**: a command over a minute runs with `run_in_background`; its end arrives as a notification.

## Heavy commands

`check:all`, `npm run test`, the golden check, `mutate` and `git push` (its pre-push runs the checks the change needs) take the lock `.git/heavy.lock` themselves, one at a time across the worktrees: run them as they are, never wrapped in a lock; between them, single test files only.

## Limits

- **Push once the tasks are done**: `--push` on the last commit, or `git push -u origin HEAD`; `--force-with-lease` only after a `git-sequences sync` moved a pushed branch.
- **Stop before the pull request**: the main thread runs `story-finish`.
- **A tool that fails** is worked around, never fixed in the story: the report says how it was used, why it failed and what was done instead.
- **A business question stops the story**: push what is done, report the question, what was read and the options, without choosing.
- **One story, then the end**; a worker starts no subagent. A story of more than 3 tasks is reported with a proposed split.

## Final report

Brief, its first line `Branch: <branch>`: each commit by its short hash, the files, the decisions taken, the evidence, the open points. Hand it back once the branch is pushed: the `check-handback` hook refuses a branch not on `origin` at the worktree's head, or a report leaving a commit unnamed.
