# ADR-0063 — Runs with a limited number of agents, enforced by the tools

**Status**: Accepted (Product Owner, 2026-10-16, at RET-003). Supersedes ADR-0051, which it keeps and amends. Decision 1 is a trial, measured at E17's retrospective.

## Context

- **ADR-0051's trial** ran through E15: the main thread dispatched 138 subagents and stayed free. Main-thread turns fell from a median of 21.4 min (E14) to 12.7 min, the longest from 252 to 146 min.
- **The load was not limited by any tool.** On 2026-10-16 four subagents ran tests and `check:all` at once: the computer froze and the session was force-quit. One `check:all` alone brings the machine's load average to 93. ADR-0051's "one worker per feature branch" was a sentence, and it was bent.
- **The Product Owner could not see the work.** For about 22 minutes on 2026-10-15 the chat was silent and the status line said "waiting" while a worker and an audit ran.
- **Three reviewers per story** cost about a third more tokens per story (24 M against 18 M at E14); F17 tried one audit per feature instead and its audit still found every defect before the validation (ADR-0064).
- **The worker's brief lived in scratch files**, outside the repository, rewritten at each session.
- Claude Code can refuse a tool call from a `PreToolUse` hook, with a reason the agent reads; a `SubagentStart` hook cannot block (Claude Code docs, Hooks reference, read 2026-10-16).

## Decision

1. **At most three subagents at once**, any mix of heavy (code, tests, checks, push) and light (logs, research, reading). A fourth is **refused** by a `PreToolUse` hook on the `Agent` tool, with a message naming the subagents running; it is not queued. If the computer still lags during the trial, the limit becomes two heavy and one light.
2. **One heavy command at a time** across every worktree: `check:all`, a full Vitest run, `golden:check` and the docs build take a lock in the Git common directory (`.git/heavy.lock`), which holds its owner's process id. A second heavy command waits; a lock whose process no longer exists (after a force-quit) is taken over. A push inherits it through its pre-push hook.
3. **The audit per feature** replaces the reviews per story (ADR-0064).
4. **The worker's brief is versioned in the run skill** (`.claude/skills/run/`): its rules, the commands that fit a worktree, the heavy lock, the area pitfalls a worktree subagent does not load by itself.
5. **A heartbeat.** The main thread writes one chat line when a worker starts and when it ends, with an estimate of its duration; the status line shows each background subagent and how long it has run.
6. ADR-0051's decisions 1, 3 (except the reviews), 4 and 5, and its amendment, stand.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| The limit in the tools (chosen) | holds when the agent forgets; the refusal names what runs | a hook and a lock to write and test (EN-047) |
| The limit as a sentence (ADR-0051) | nothing to build | bent once, with a freeze |
| A queue for refused subagents | no work lost | a hidden wait; not asked for |
| Drop the trial | no parallel load | the chat blocked for long turns again |

## Consequences

- EN-047 builds the hook and the lock; until it is merged, `AGENTS.md` names the limit and the story.
- EN-053 finds why the status line missed background subagents and shows them.
- The run skill describes one audit per feature and holds the worker's brief.

## References

ADR-0050, ADR-0051, ADR-0064, ADR-0065, `docs/process/retrospectives/E15-retrospective.md`, Claude Code docs (subagents, hooks, worktrees, read 2026-10-16)
