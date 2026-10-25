---
name: worker-brief
description: How a run's worker sets up its worktree, works a story, commits, takes the heavy lock and stops before the pull request; preloaded by the story-worker and researcher agent types (EN-065).
user-invocable: false
---

# Worker brief

How a run's worker works one story in its own worktree. The `story-worker` and `researcher` types start with this brief and `project-rules` preloaded, without `CLAUDE.md` nor `AGENTS.md`; a `general-purpose` worker is given this file's path inside its worktree and reads `AGENTS.md` and `CLAUDE.md` too. The main thread's prompt names the story, its feature branch and its worktree. A run works one story at a time, one worker for it, in the background.

## Setup

1. **Work only inside the worktree.** The main thread made it with `target/release/worktree-new <ID>`: `.claude/worktrees/<id>`, on the story's branch from the feature branch, with its own hooks. The main checkout stays on `develop` during a run, so a page read there may be older: read every rule from the worktree.
2. **Every Bash call starts with `cd "<worktree>" || exit 9`**: a subagent's calls start in the main checkout whatever its prompt says. Write and Edit take absolute paths inside the worktree; never `git -C`; never a file outside the worktree.
3. **One Git command per call**, never chained to another by `&&`, `;` or a pipe; after the brief's `cd`, a chain of Git commands that only read goes through (`git status; git log -3`). `target/release/git-sequences state` prints the branch, its base, `status --short`, `diff --stat HEAD` and the commits ahead; `git-sequences sync` fetches, rebases the story on its feature branch and rebuilds the release binaries; `git-sequences commit -F <message file|-> [--push] <paths>` stages the files named, commits and pushes. Start with `target/release/git-sequences sync`. Every agent tool is called by its binary from the worktree's root; `npm run` stays for the scripts `package.json` keeps.
4. A scratch folder of its own: `<scratchpad>/<story id>-<role>`.

## Read first

- The story and its feature from the records: `target/release/knowledge sql "SELECT text, criteria, tasks FROM stories WHERE id = '<ID>'"`, then what they cite — ADRs, derivations, rules, `REF-*` — by `knowledge show <id>`.
- The rules of the area the story touches: `.claude/rules/library.md` (`src/`, `playground/`), `.claude/rules/docs.md` (Markdown), `.claude/rules/tooling.md` (tooling, tests, `tools/`). A worktree subagent does not load them by itself.
- `docs/conventions/README.md` and the pages it lists for what the story touches.

## Work

- **Tests first**: the tests from the criteria written before or with the code, each seen failing once, in one commit with the code or apart as is faster; the pre-push `check:all` runs only the checks the change needs.
- **One commit per task**, footer `Refs: <ID>.Tn` and the attribution lines, the message in a file of the scratch folder or a here-document (`-F -`), made by `target/release/git-sequences commit`.
- **Before each commit**: `npm run fix`, which reports the type errors with the lint, and the touched tests (`npx vitest run <paths>`, `cargo test -p <crate>`).
- **Edge cases and mutations**: no review checks a story, so its worker writes the story's edge cases into its tests; a library story breaks each new test once on purpose to see it fail, a tooling story leaves the mutations to its epic's audit.
- **The records in the same task as the code** they describe: a rule changed or added (`rules`), a source read, a derivation; log rows (journal, improvement, feedback) written with `knowledge add <kind> -`.
- **A binary that runs a command given to it** (`heavy-lock`, `bench`, a wrapper): `guard-bash`'s reading of that command lands in a task before the one that builds the binary, and the worker hands back once that task is pushed, so that the main thread rebuilds the main checkout's `guard-bash` from the branch.
- **The current step**, one short line in French, goes to `.git/metrics/step` of the main checkout (`git rev-parse --git-common-dir` gives its folder).
- **Never a polling loop**: a command expected to last over a minute runs with `run_in_background`, and its end arrives as a notification.

## Heavy commands

One heavy command at a time across every worktree: `check:all`, `npm run test`, `golden:check`, `docs:build`, and `git push`, whose pre-push runs `check:all`. They take the lock `.git/heavy.lock` themselves and a waiting one names the process holding it: run them as they are, in the background when they may last over a minute. Never take the lock by hand nor wrap a command in a lock. Between heavy commands, run single test files only (`npx vitest run <paths>`).

## Limits

- **Push once the tasks are done**: `--push` on the last task's `git-sequences commit`, or `git push -u origin HEAD`. Never a force push; after a `git-sequences sync` that moved a pushed story branch, `--force-with-lease`.
- **Stop before the pull request**: the main thread runs `story:finish`, which opens it, awaits its merge and marks the story done in the records.
- **A tool that fails or does not do what was needed** is worked around, never fixed in the story: the report names it, how it was used, why it failed and what was done instead, for the incident card, fixed at the next review.
- **A business question stops the story, not the run**: push what is done, then report the question, what was read and the options, without choosing; the main thread parks the story and goes on with independent work.
- **One story, then the end**: a worker is started fresh for one story and never takes a second; it starts no subagent.
- **A story of more than 3 tasks** is not worked as written: report it with a proposed split and an example, for the Product Owner.

## Final report

Brief, its first line `Branch: <branch>`: the commits, each by its short hash, the files, the decisions taken, the evidence, the open points. Hand it back once the branch is pushed: the hook `check-handback` refuses a first hand-back whose branch is not on `origin` at the worktree's head, or whose report leaves a commit unnamed.
