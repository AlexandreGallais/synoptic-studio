---
id: EN-047
epic: E17
feature: F18
title: A lock for heavy commands and a limit on subagents
status: done
points: 3
---

# E17 · F18 · EN-047 — A lock for heavy commands and a limit on subagents

Make the tools enforce ADR-0063: one heavy command at a time across every worktree, and at most three subagents at once (F18.AC2, F18.AC3).

SP-020 ([research note 0022](../../research/notes/0022-f18-parallel-run.md)): a `PreToolUse` hook on `Agent` gets no count of running subagents nor the tasks in flight (only `Stop` and `SubagentStop` get `background_tasks`), so the hook rebuilds the count, from the transcript as `status-tasks.ts` does or from a state file kept by `PreToolUse`, `PostToolUse` and `SubagentStop` (`REF-CC-HOOKS`); `git rev-parse --path-format=absolute --git-common-dir` gives the same `.git` from every worktree (`REF-GIT-REV-PARSE`, `REF-GIT-WORKTREE`); `process.kill(pid, 0)` tells whether the lock's owner still runs (`REF-NODE-PROCESS-KILL`); `maxWorkers: '50%'` (`REF-VITEST-MAXWORKERS`).

## Acceptance criteria

- Given `check:all`, a full Vitest run, `golden:check` or the docs build running in one worktree, when another one starts in any worktree, then it waits for the lock in `.git/heavy.lock` and says which process holds it.
- Given a lock whose process no longer exists (after a force-quit), when a heavy command starts, then it takes the lock over at once.
- Given three subagents running, when the agent starts a fourth with the `Agent` tool, then a `PreToolUse` hook refuses it with a message naming the three running.
- Given the limit, when `AGENTS.md` is read, then it states the limit in one line and points to the hook and its test.
- Given several worktrees testing at once, when Vitest runs, then it uses half the machine's cores.

## Tasks

One task = one commit, referenced as `EN-047.Tn`.

- [x] T1 — The lock in the heavy scripts, with its owner's process id and its take-over; tests (2 h)
- [x] T2 — The `PreToolUse` hook on `Agent`, counting running subagents from `.git/metrics/`; tests (2 h)
- [x] T3 — Vitest's workers capped at half the cores; docs and the rule pointing to the check (1 h)

## Follow-up

A waiter read the lock twice per look: a lock released between the two readings was named "taken by hand", a false line seen once under load (2026-10-16). Each look now reads the lock once and a lock gone since is taken at the next look; the lock is written to a draft then linked, so it is never read half written; its age is left out when its start is missing, in the future or over a day ago, and written in hours past an hour (`fix/en-047-lock-released-while-read`, `Refs: EN-047`).

## For the main thread

### The hook's line in `.claude/settings.json`, to show the Product Owner

Not committed: Claude Code reads the hook only once the Product Owner accepts this entry, added to `hooks.PreToolUse` after the `Bash` entry. The first call builds the tool (about 10 s); a failure of the hook lets the call through.

```json
{
  "matcher": "Agent",
  "hooks": [
    {
      "type": "command",
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "-q",
        "--manifest-path",
        "${CLAUDE_PROJECT_DIR}/tools/hooks/Cargo.toml",
        "--bin",
        "limit-agents"
      ]
    }
  ]
}
```
