---
id: EN-088
epic: E17
feature: F20
title: A SubagentStop hook in Rust that checks a worker's branch
status: done
points: 2
---

# E17 · F20 · EN-088 — A SubagentStop hook in Rust that checks a worker's branch

Check, when a worker stops, that its branch is pushed, and prepare `story:finish` for the main thread, with a `SubagentStop` hook written in Rust in the workspace of F19 (EN-068). Asked by the Product Owner on 2026-10-17.

Sourced by SP-022 ([research note 0024](../../research/notes/0024-f20-context-diet.md), `REF-CC-HOOKS`): the hook gets `agent_id`, `agent_type`, `agent_transcript_path`, `stop_hook_active` and `background_tasks`; the report sent by `SubagentHandback` reaches a `PostToolUse` hook on that tool as `tool_input.message`; `decision: "block"` with a `reason` keeps the worker running with the reason as its next instruction, at most 8 times in a row. Measured: a worker's transcript names the main checkout and its branch, never the worktree, so the hook finds the worktree from the worker's first `cd` in the hook log, tied to its `agentId`.

## Acceptance criteria

- Given a story worker that stops with its branch not pushed or behind `origin`, when the hook runs, then it blocks the stop once, saying what is missing; with `stop_hook_active` set, it lets the worker stop and tells the main thread.
- Given a worker whose branch is pushed, when the hook runs, then the main thread gets one line naming the story, its branch and `story:finish`.
- Given an internal agent (empty `agent_type`) or a type other than a story worker, when it stops, then the hook does nothing.
- Given the hook, when it is built, then it is a crate of the workspace with its tests, called by `.claude/settings.json`.

## Tasks

One task = one commit, referenced as `EN-088.Tn`.

- [ ] T1 — The hook's crate: the input read, the worktree found from the log, the push checked, its tests (3 h)
- [ ] T2 — The line for the main thread, the hook in `.claude/settings.json`, the worker brief updated (2 h)

## Decisions

- **`PreToolUse` on `SubagentHandback`, not `SubagentStop`** (2026-10-17): a worker that reports through `SubagentHandback` has delivered its report before `SubagentStop` fires (`REF-CC-HOOKS`), so a block there would come after the main thread got it. Refusing the hand-back itself keeps the report back and gives the worker the reason as its next instruction. `stop_hook_active` does not reach this event: the hook counts the worker's earlier `SubagentHandback` calls in its transcript instead, refuses only the first, and lets the next one through with what is missing written above the report.
- **The worktree from the worker's own transcript**: the first `.claude/worktrees/<id>` it names, its prompt's, rather than the hook log (`tool-calls.jsonl`, 22 MB on 2026-10-17); the agent's transcript is beside the session's, under `subagents/agent-<id>.jsonl`.
- **The pre-push is not recorded**: the branch on `origin` at the worktree's head means its pre-push `check:all` passed, since `guard-bash` refuses a push without it.
- **Who is checked**: every subagent with a worktree, except the main thread, an internal agent (empty type) and the read-only types (`Explore`, `Plan`, `auditor`, `verifier`, `doc-editor`, `math-expert`, `functional-analyst`, `claude-code-guide`, `statusline-setup`).
- **Measured** on 2026-10-17, a 1.2 MB worker transcript: 54 ms median for the binary (two Git calls, one `for-each-ref` reading the branch, its copy on `origin` and the feature branch), 5 ms for a read-only agent, 180 ms through `cargo run --release`, which the settings use like `limit-agents`; once per worker.
- **Settings**: only the hook's line is added to `.claude/settings.json`, under the change of settings the Product Owner allowed on 2026-10-17.
