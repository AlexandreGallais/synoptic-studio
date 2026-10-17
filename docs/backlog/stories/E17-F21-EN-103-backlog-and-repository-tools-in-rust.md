---
id: EN-103
epic: E17
feature: F21
title: The backlog and repository tools in Rust
status: ready
points: 8
---

# E17 · F21 · EN-103 — The backlog and repository tools in Rust

The scripts the agent runs at each story and each push — `check-all`, `backlog-status`, `story-finish`, `epic-check`, `replace-text`, `branches-clean`, `records-save` — rewritten as Rust binaries of the workspace, their output unchanged (F21.AC2). Each saves Node.js's start, 30 to 130 ms per call ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), Hooks and agent tools); `backlog-status` took 0.11 s. Early in F21 because the frictions logged during F20's run sit in these tools, and every merge of the run meets them. Carried from EN-093's note: `check-all` has no `check` twin to keep in step; `replace-text` is kept, the agent's most used edit tool. `tool-versions` goes to EN-109 with the dependency checks.

Frictions of F20's run folded in (2026-10-17):

- `story:finish` does not push a done commit rebased by hand, and `gh pr create` fails with "you must first push the current branch" (`docs/process/improvements.md`, EN-063, EN-067).
- The status rows of the feature's Stories table conflict at every merge, each story's done commit editing the same table.
- `git:sync` on a tree with uncommitted changes exits 1 with no message.
- A test inherited the pre-push hook's `GIT_DIR` and wrote `core.bare=true` and a test user into the shared `.git/config` (2026-10-17); TypeScript tests already remove every `GIT_*` variable (`.claude/rules/tooling.md`), Rust test helpers each do it their own way or not at all.
- `npm run replace` ignores a second `<<<` block written without its own `@@ file` line and still prints "1 edits".

## Acceptance criteria

- Given each of the seven scripts, when its Rust binary runs on the cases of its TypeScript tests and on this repository, then its output, its files written and its exit code are the same as the script's, and `package.json` calls the binary; the script and its test are removed in the same task.
- Given `npm run check:all`, when it runs, then it reads `.git/config` of the main checkout byte for byte before and after its lanes and, if a lane changed it, fails naming the lines added or removed and restores nothing by itself.
- Given a Rust test of any crate that runs `git`, when it runs under the pre-push hook, then it starts `git` through one helper of a shared test-support crate that removes every inherited `GIT_*` variable, and a test of that helper proves a `GIT_DIR` set by the caller does not reach the command.
- Given a story whose branch was rebased after its push, when `npm run story:finish` runs, then it pushes the branch with `--force-with-lease` before opening the pull request.
- Given two stories of one feature finishing one after the other, when the second is rebased on the feature branch that holds the first's done commit, then its done step applies without a conflict: the status rows are written by `story:finish` after the rebase (`backlog:status`, the stories index regenerated), never carried through it.
- Given `npm run git:sync` on a tree with uncommitted changes, when it stops, then it prints why and the files that block it, and exits 1.
- Given `npm run replace` with a block it cannot read, when it runs, then it refuses the whole input, naming the block, and edits nothing.
- Given each binary, when it is timed, then its time per call before and after is stated.

## Files

New crates of `tools/` (one per tool or one `repo-tools` crate with a binary each, decided in T2), `tools/test-support/` (new), `tools/git-sequences/`, the Rust code and tests that run `git` (`format`, `git-sequences`, `heavy-lock`, `hooks`, `progress-page`, `rule-docs`, `run-totals`, `shared-files`, `worktree`), `package.json`, `scripts/check-all.ts`, `scripts/backlog-status.ts`, `scripts/story-finish*.ts`, `scripts/epic-check*.ts`, `scripts/replace-text.ts`, `scripts/branches-clean.ts`, `scripts/records-save.ts` and their tests (removed), `docs/tooling/agent-tools.md`, `docs/tooling/commands.md`, `.claude/rules/tooling.md`, `.claude/skills/run/SKILL.md`.

## Measures (ADR-0069)

Before (SP-023's table) and after, 20 calls each on this Mac: `backlog:status`, `epic:check`, `replace` on a two-file edit, `branches:clean` and `records:save` with nothing to do, `story:finish` up to its pull request on a test repository; `check:all`'s own overhead outside its lanes.

Before, measured by SP-023 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), Each tool's time before its port), median of 20 calls: `backlog-status` 355 ms (its formatter included), `replace` on a two-file edit 61 ms, `branches-clean` with nothing to clean 286 ms, `records-save` without the transcripts' copy 706 ms; `check-all`, `epic-check` and `story-finish` not timed as a whole. Each "after" states the binary alone and the call as the agent makes it: `npm run -s` adds about 107 ms and `cargo run` about 47 ms to any binary (0025), so a port behind both cannot go under about 150 ms; how the binaries are called is EN-104's question to the Product Owner. No crate: the standard library and the `git` command (0025, The crates).

## Tasks

One task = one commit, referenced as `EN-103.Tn`.

- [ ] T1 — The test-support crate: `git` without inherited `GIT_*`, used by every Rust test that runs it; a `GIT_DIR` planted to prove it (2 h)
- [ ] T2 — `check:all` in Rust: its lanes, lock and report as today, `.git/config` compared before and after; times measured (5 h)
- [ ] T3 — `backlog:status` and `epic:check` in Rust, on their TypeScript tests' cases (4 h)
- [ ] T4 — `story:finish` in Rust: push with lease when the heads differ, the status rows written after the rebase, tested on two stories of one feature (6 h)
- [ ] T5 — `git:sync` names the uncommitted files that stop it (1 h)
- [ ] T6 — `replace` in Rust, refusing a block it cannot read (3 h)
- [ ] T7 — `branches:clean` and `records:save` in Rust; the docs and the run skill name the binaries (3 h)
