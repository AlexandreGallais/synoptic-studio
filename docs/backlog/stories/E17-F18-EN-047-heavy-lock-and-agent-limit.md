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

SP-020 ([research note 0022](../../research/notes/0022-f18-parallel-run.md)): a `PreToolUse` hook on `Agent` gets no count of running subagents nor the tasks in flight (only `Stop` and `SubagentStop` get `background_tasks`), so the hook rebuilds the count, from the transcript as `status-tasks.ts` does or from a state file kept by `PreToolUse`, `PostToolUse` and `SubagentStop` (`REF-CC-HOOKS`); `git rev-parse --path-format=absolute --git-common-dir` gives the same `.git` from every worktree (`REF-GIT-REV-PARSE`, `REF-GIT-WORKTREE`); `process.kill(pid, 0)` tells whether the lock's owner still runs (`REF-NODE-PROCESS-KILL`); `maxWorkers: '50%'` (`REF-VITEST-MAXWORKERS`).

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

### Rows for the logs

- Improvement log: the Rust hook `tools/hooks` reads the transcript's background tasks with its own code, a temporary twin of `.claude/hooks/status-tasks.ts` and of `withTasks` in `status-tokens.ts`; F19 ports the status line onto `tools/hooks` and removes the TypeScript copy (decided 2026-10-16, Rust-only new tools, no crates.io dependency before SP-021).
- Improvement log: once EN-047 is merged, workers no longer take the heavy lock by hand with `mkdir`: a heavy command waiting for a hand-taken folder lock cannot see its owner, and a worker holding one while running `npm run check:all` waits for itself.
- Improvement log: the CI needs `cargo` for `check:all` (`check:rust` and the heavy lock); GitHub's Ubuntu runners have it, to be confirmed at the first pull request into `main`.
