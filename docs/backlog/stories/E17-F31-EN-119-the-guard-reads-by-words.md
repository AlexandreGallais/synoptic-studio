---
id: EN-119
epic: E17
feature: F31
title: The guard reads a command by its words
status: ready
points: 3
---

# E17 · F31 · EN-119 — The guard reads a command by its words

From the card C2 and the rows I12, I22, I40, I87, I93 and I103 (RET-004), with AUD-023's table of the guard's cases. Moved from F23 on 2026-10-18 (SP-024). Its former criterion on a feature branch pushed with lease after its rebase onto `develop` (ADR-0084, decision 3) is dropped: ADR-0086 merges a feature into `develop` by a merge commit and supersedes that rebase.

## Acceptance criteria

- Given a worktree, when a worker calls `target/release/git-sequences state`, then it prints the branch, its base, `status --short`, `diff --stat` and the commits ahead, in one call (I93).
- Given the guard, when it reads a command, then it splits it by its words as the shell does: a `|` or a `;` inside quotes is not a chain, a chain of read-only Git commands after `cd <worktree> || exit 9` goes through, a multi-line commit message and a URL in it go through; `mkdir …heavy.lock` and a `python3` writing a file in a worktree are refused, each with the command to use instead (C2, I22, I40, I87, I103).
- Given a worker whose prompt names its worktree, when it writes a file into the main checkout, then the write is refused, naming the worktree's path for the same file (I12).

## Tasks

- [ ] T1 — Each criterion's tests in `tools/hooks` and `tools/git-sequences`, AUD-023's table of cases among them, failing (1 h)
- [ ] T2 — The guard on its word reader, the writes outside the worktree refused (2 h)
- [ ] T3 — `git-sequences state`, the worker brief and `docs/tooling/agent-tools.md` (1 h)

One task = one commit, referenced as `EN-119.T1` in the commit footer.
