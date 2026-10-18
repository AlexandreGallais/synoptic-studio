---
name: worker-brief
description: How a run's worker sets up its worktree, works a story, commits, takes the heavy lock and stops before the done commit (ADR-0072); preloaded by the story-worker and researcher agent types (EN-065).
user-invocable: false
---

# Worker brief (ADR-0072)

How a run's worker works one story in its own worktree. The `story-worker` and `researcher` types start with this brief and `project-rules` preloaded, without `CLAUDE.md` nor `AGENTS.md` (EN-065); a `general-purpose` worker is given this file's path inside its worktree and reads `AGENTS.md` and `CLAUDE.md` too. The main thread's prompt names the story, its feature branch and its worktree. A run works one story at a time, one worker for it, in the background (ADR-0086).

## Setup

1. **Work only inside the worktree.** The main thread made it with `target/release/worktree-new <ID>`: `.claude/worktrees/<id>`, on the story's branch from the feature branch, with its own hooks. The main checkout stays on `develop` during a run, so a page read there may be older: read every rule from the worktree.
2. **Every Bash call starts with `cd "<worktree>" || exit 9`**: a subagent's calls start in the main checkout whatever its prompt says, and files once landed there (EN-049). Write and Edit take absolute paths inside the worktree; never `git -C`; never a file outside the worktree.
3. **One Git command per call**, never chained to another by `&&`, `;` or a pipe: Claude Code refuses compound commands in a worktree. The frequent sequences are one command each (EN-100), called by their binary from the worktree's root (ADR-0078): `target/release/git-sequences sync` fetches, rebases the story on its feature branch and rebuilds the release binaries; `target/release/git-sequences commit -F <message file|-> [--push] <paths>` stages the files named, commits and pushes, and stops at the first failing step with Git's message. Start with `target/release/git-sequences sync`. Every agent tool is called so, as `docs/tooling/agent-tools.md` lists them (The binaries the agent calls); `npm run` stays for the scripts `package.json` keeps (ADR-0082).
4. A scratch folder of its own: `<scratchpad>/<story id>-<role>`.

## Read first

- The story, its feature, and what they cite: ADRs, derivations, domain sections, `REF-*` rows. A row or a section is read with `target/release/page [<file>] <id or heading>` rather than `Read` of its whole page: `REF-CC-HOOKS` prints 2,335 characters of the 198,307 of `docs/references.md` (EN-105); `Read` stays for a page read whole or to edit.
- The rules of the area the story touches: `.claude/rules/library.md` (`src/`, `playground/`), `.claude/rules/docs.md` (Markdown), `.claude/rules/tooling.md` (tooling, tests, `tools/`). A worktree subagent does not load them by itself.
- `docs/conventions/README.md` and the pages it lists for what the story touches.

## Work

- **Tests first** (ADR-0084): `T1` the interface and its tests from the criteria, seen failing, which stands for breaking each new test once; `T2` the code; the pre-push `check:all` runs on the head.
- **One commit per task**, footer `Refs: <ID>.Tn` and the attribution lines, the message in a file of the scratch folder or a here-document (`-F -`), made by `target/release/git-sequences commit`.
- **Before each commit**: `npm run fix`, which reports the type errors with the lint (EN-091), and the touched tests (`npx vitest run <paths>`, `cargo test -p <crate>`).
- **Edge cases and mutations** (ADR-0064): no review checks a story, so its worker writes the story's edge cases into its tests and breaks each new test once on purpose to see it fail.
- **Docs in the same task as the code** they describe: domain, derivations, references, playground.
- **The current step**, one short line in French, goes to `.git/metrics/step` of the main checkout (`git rev-parse --git-common-dir` gives its folder): the status line shows it.

## Shared files (ADR-0065, ADR-0091)

The files every story touches are written only by the main thread, so that a story's pull request never conflicts with the rows the main thread wrote meanwhile: the indexes of `docs/backlog/`, the logs (`journal.md`, `improvements.md`, `feedback.md`, `tool-incidents.md`), the feature file and the checklists (`playground/checklists/epic-*`). The pre-push hook refuses a branch that changes one and names it (EN-050), except in the story's `done` commit, the main thread's.

A row meant for a log goes in the story file, under `## For the main thread`, beneath `### Journal`, `### Improvements`, `### Feedback` or `### Incidents`, written as the log writes it; `move-rows` moves it in the story's `done` commit:

- under `### Journal`, a bullet opening with its tag: ``- `#run` EN-050: …``;
- under `### Improvements` and `### Feedback`, a table row of the log's columns, its first cell the date (`| 2026-10-17 | EN-050 | … | tool |`), with no header nor separator;
- under `### Incidents`, a card for a tool that failed (ADR-0085): its heading `#### 2026-10-19 — <tool, and what for>`, then its bullets `- Used: …` (how it was used), `- Failed: …` (why it failed) and `- Instead: …` (what was done instead).

Rows only, one per line, and no heading without rows (never `- None.`): any other line stays in the story file and is reported by `move-rows`. Anything else for the main thread goes in that section, outside those headings.

## Heavy commands (ADR-0072)

One heavy command at a time across every worktree: `check:all`, `npm run test`, `golden:check`, `docs:build`, and `git push`, whose pre-push runs `check:all`. They take the lock `.git/heavy.lock` themselves (EN-047) and a waiting one names the process holding it: run them as they are, in the background when they may last over a minute. Never take the lock by hand nor wrap a command in a lock: a `git push` inside it would wait forever for its own pre-push `check:all`, and every other worktree with it.

Between heavy commands, run single test files only (`npx vitest run <paths>`), never `npx vitest run` without a path.

## Limits

- **Push once the tasks are done**: `--push` on the last task's `target/release/git-sequences commit`, or `git push -u origin HEAD`. Never a force push; after a `git-sequences sync` that moved a pushed story branch, `--force-with-lease`.
- **Stop before the pull request and the `done` commit**: the main thread makes that commit in the worktree, since it writes the shared files, then runs `story:finish`.
- **A rule changed in `docs/domain/`** brings, in the story's tasks, the Q row of `docs/domain/README.md`, the decisions to confirm of its feature's `VAL` and its guide page; the `done` commit's hook refuses the story otherwise (`tools/rule-docs`, EN-059). A doc the change does not affect: say so in the report, with its reason, for the footer `Rule-docs: none affected, <why>`.
- **A tool that fails or does not do what was needed** is worked around, never fixed in the story: the report names it, how it was used, why it failed and what was done instead, for the main thread's incident card, fixed at the next review (ADR-0085, ADR-0086).
- **A business question stops the story, not the run**: a rule missing, or code, domain and ADRs that contradict each other. Push what is done, then report the question, what was read and the options, without choosing; the main thread parks the story and goes on with independent work.
- **One story, then the end**: a worker is started fresh for one story and never takes a second, since its cost is its context re-read at each call (ADR-0086); it starts no subagent, a specialist being the main thread's call where its independence pays.
- **A story of more than 3 tasks, or a feature of more than 4 stories**, is not worked as written: report it with a proposed split and an example, for the Product Owner (ADR-0084, EN-123).

## Final report

Brief, its first line `Branch: <branch>`: the commits, each by its short hash, the files, the decisions taken, the evidence, the open points. Hand it back once the branch is pushed: the hook `check-handback` (EN-088) refuses a first hand-back whose branch is not on `origin` at the worktree's head, or whose report leaves a commit unnamed, and says what is missing.
