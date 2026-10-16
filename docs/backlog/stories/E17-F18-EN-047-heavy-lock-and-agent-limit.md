---
id: EN-047
epic: E17
feature: F18
title: A lock for heavy commands and a limit on subagents
status: ready
points: 3
---

# E17 · F18 · EN-047 — A lock for heavy commands and a limit on subagents

Make the tools enforce ADR-0063: one heavy command at a time across every worktree, and at most three subagents at once (F18.AC2, F18.AC3).

## Acceptance criteria

- Given `check:all`, a full Vitest run, `golden:check` or the docs build running in one worktree, when another one starts in any worktree, then it waits for the lock in `.git/heavy.lock` and says which process holds it.
- Given a lock whose process no longer exists (after a force-quit), when a heavy command starts, then it takes the lock over at once.
- Given three subagents running, when the agent starts a fourth with the `Agent` tool, then a `PreToolUse` hook refuses it with a message naming the three running.
- Given the limit, when `AGENTS.md` is read, then it states the limit in one line and points to the hook and its test.
- Given several worktrees testing at once, when Vitest runs, then it uses half the machine's cores.

## Tasks

One task = one commit, referenced as `EN-047.Tn`.

- [ ] T1 — The lock in the heavy scripts, with its owner's process id and its take-over; tests (2 h)
- [ ] T2 — The `PreToolUse` hook on `Agent`, counting running subagents from `.git/metrics/`; tests (2 h)
- [ ] T3 — Vitest's workers capped at half the cores; docs and the rule pointing to the check (1 h)
