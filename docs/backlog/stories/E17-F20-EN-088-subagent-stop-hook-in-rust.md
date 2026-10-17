---
id: EN-088
epic: E17
feature: F20
title: A SubagentStop hook in Rust that checks a worker's branch
status: ready
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
