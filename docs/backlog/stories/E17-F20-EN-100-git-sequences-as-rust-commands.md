---
id: EN-100
epic: E17
feature: F20
title: The agent's frequent Git sequences as Rust commands
status: ready
points: 2
---

# E17 · F20 · EN-100 — The agent's frequent Git sequences as Rust commands

A worktree refuses chained Git commands, so a worker sends each in its own call. From 2026-10-14 to 2026-10-17, 793 of 2,297 Git calls came right after another Git call of the same agent — `add` then `commit` 59 times, `commit` then `push` 46, `fetch` then `rebase` 39, `fetch` then `diff` 18 — and each call saved is about 2.6 s and 166,000 tokens read from the cache in a subagent: about 34 minutes and 132 million tokens over the three days ([research note 0024](../../research/notes/0024-f20-context-diet.md)). Chosen at F20's refinement (2026-10-17).

## Acceptance criteria

- Given a Git sequence seen more than 30 times in the hook log, when the agent needs it, then one Rust command of the workspace does it in one call — the feature branch fetched and the story rebased on it; the files named staged and committed from a message file — and stops at the first failing step with Git's own message.
- Given the worker brief and the run skill, when they are read, then they name the commands in place of the sequences.
- Given a run after the change, when the hook log is read, then the Git calls per story are counted and compared with F19's.
- Given each command, when it runs, then it records its use as every tool of our own does (F21), or in the hook log until then.

## Tasks

One task = one commit, referenced as `EN-100.Tn`.

- [ ] T1 — The commands, their crate and tests, stopping at the first failure (3 h)
- [ ] T2 — The brief and the run skill updated; the calls per story compared after one run (1 h)
