---
id: EN-127
epic: E17
feature: F26
title: The mutation runner stops a mutation on Linux
status: ready
points: 1
---

# E17 · F26 · EN-127 — The mutation runner stops a mutation on Linux

`tools/mutate/src/runner.rs`'s `stop_with_children` runs `kill -KILL -<pid>`: Linux's `kill` (procps) reads `-<pid>` as a signal, so the group is never signalled and `stops_a_mutation_that_never_ends` waited 46 min on GitHub's runner (run 39138216016).

## Acceptance criteria

- Given a test command over its limit, when it is stopped, then `kill` gets `-KILL`, `--`, then `-<pid>`, and the command is killed too, so that the wait ends on Linux as on macOS.

## Tasks

- [ ] T1 — The arguments' test, failing (main thread)
- [ ] T2 — `stop_with_children` with `--` and `Child::kill` (main thread)

One task = one commit, referenced as `EN-127.T1` in the commit footer.
