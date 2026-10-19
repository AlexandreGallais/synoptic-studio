---
id: EN-141
epic: E17
feature: F33
title: A bench binary for the stories' measures
status: done
points: 2
---

# E17 · F33 · EN-141 — A bench binary for the stories' measures

From RET-004's "Builds and lanes" (P8, story 4), the rows of that theme F29 does not take (I90, I100, I107): a `bench` binary for the stories' measures. A story that sets a limit from a measure, as EN-129 does, times a command by hand today, its runs and its median written each time anew.

## Acceptance criteria

- Given `target/release/bench --runs 5 -- <command>`, when it runs, then it runs the command 5 times, prints each duration, the median and the load average before and after, and exits with the command's first failing status, if any.
- Given a command that fails on its first run, when `bench` runs it, then it stops there and says so, measuring nothing.
- Given `docs/tooling/agent-tools.md` and `.claude/settings.json`, when read, then `bench` is listed and allowed by its name, as the other binaries (`tools/hooks/tests/settings.rs`).

## Tasks

- [x] T1 — The crate's interface and its tests, failing (1 h)
- [x] T2 — `bench`, its row in the agent's tools and its permission (2 h)
- [x] T3 — `guard-bash` reads the command `bench` runs, in every spelling of its path (1 h)

One task = one commit, referenced as `EN-141.T1` in the commit footer.

## For the main thread

- A question for the Product Owner: does `bench` run without asking? The third criterion allows it by name in `.claude/settings.json`, as the other binaries; but `bench` runs any command given after its `--runs <count> --`, as `heavy-lock` does, which is left to ask (`tools/hooks/tests/settings.rs`, AUD-023). Since T3, `guard-bash` judges the command `bench` runs as it judges `heavy-lock`'s, so the listed refusals hold under it; a command the guard does not refuse would then run unasked. Options: (a) allow it, `"Bash(target/release/bench *)"` in `permissions.allow` after `"Bash(target/release/backlog-status *)"` and `"bench"` in `ALLOWED_BINARIES` (21 becoming 22), both in one commit since the test fails on either alone; (b) leave it asking, as `heavy-lock`, and strike the criterion's "allowed by its name" with the reason. This branch makes neither change.
- `.claude/rules/tooling.md`, the timing bullet, could name the tool: "The measure is written beside the limit as `alone: 210 ms`, the median of 5 runs of its file alone (`target/release/bench --runs 5 -- npx vitest run <file>` times the whole command, Vitest's start included, so read Vitest's own duration for a test under a second)".
- Rule-docs: none affected, no rule of `docs/domain/` changed (a tool of `tools/` only).
