---
id: EN-119
epic: E02
feature: F23
title: The guard in a worktree, and the feature rebased with lease
status: draft
points: 3
---

# E02 · F23 · EN-119 — The guard in a worktree, and the feature rebased with lease

From the card C2 and the rows I12, I22, I40, I87, I93 and I103 (RET-004), and ADR-0084's decision 3.

## Acceptance criteria

- Given a worktree, when a worker calls `target/release/git-sequences state`, then it prints the branch, its base, `status --short`, `diff --stat` and the commits ahead in one call (I93).
- Given the guard, when it reads a command, then it splits it by its words (a `|` inside quotes is not a chain), lets a multi-line commit message and a URL in it through, and refuses `mkdir …heavy.lock` and a `python3` writing a file in a worktree (C2, I22, I40, I87, I103).
- Given a feature branch behind `develop`, when the main thread runs `git-sequences sync` on it, then it is rebased and pushed with lease, the only force push the guard lets through on a feature branch (ADR-0084).
- Given a worker's prompt naming its worktree, when it writes into the main checkout, then the write is refused (I12).

## Tasks

- [ ] T1 — Each criterion's tests first, failing, with AUD-023's table of cases; then the guard and `git-sequences` (4 h)

One task = one commit, referenced as `EN-119.T1` in the commit footer.
