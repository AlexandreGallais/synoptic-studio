---
id: EN-141
epic: E17
feature: F33
title: A bench binary for the stories' measures
status: ready
points: 2
---

# E17 · F33 · EN-141 — A bench binary for the stories' measures

From RET-004's "Builds and lanes" (P8, story 4), the rows of that theme F29 does not take (I90, I100, I107): a `bench` binary for the stories' measures. A story that sets a limit from a measure, as EN-129 does, times a command by hand today, its runs and its median written each time anew.

## Acceptance criteria

- Given `target/release/bench --runs 5 -- <command>`, when it runs, then it runs the command 5 times, prints each duration, the median and the load average before and after, and exits with the command's first failing status, if any.
- Given a command that fails on its first run, when `bench` runs it, then it stops there and says so, measuring nothing.
- Given `docs/tooling/agent-tools.md` and `.claude/settings.json`, when read, then `bench` is listed and allowed by its name, as the other binaries (`tools/hooks/tests/settings.rs`).

## Tasks

- [ ] T1 — The crate's interface and its tests, failing (1 h)
- [ ] T2 — `bench`, its row in the agent's tools and its permission (2 h)

One task = one commit, referenced as `EN-141.T1` in the commit footer.
