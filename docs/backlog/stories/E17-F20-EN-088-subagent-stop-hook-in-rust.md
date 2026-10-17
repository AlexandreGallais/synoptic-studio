---
id: EN-088
epic: E17
feature: F20
title: A SubagentStop hook in Rust that checks a worker's branch
status: draft
points: 2
---

# E17 · F20 · EN-088 — A SubagentStop hook in Rust that checks a worker's branch

Check, when a worker stops, that its branch is pushed and its checks green, and prepare `story:finish` for the main thread, with a `SubagentStop` hook written in Rust in the workspace of F19 (EN-068). Asked by the Product Owner on 2026-10-17, during SP-021 ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), "The Product Owner's second round"); the hook's input is sourced by SP-022 before the story is set `ready`.

## Acceptance criteria

- Given a worker that stops, when the hook runs, then it checks that the worker's branch is pushed and its checks green, and says what is missing otherwise.
- Given a worker whose branch is pushed and green, when the hook runs, then it prepares `story:finish` for the main thread, naming the story and its branch.
- Given the hook, when it is built, then it is a crate of the workspace with its tests, called by `.claude/settings.json`.

## Tasks

One task = one commit, referenced as `EN-088.Tn`.

- [ ] T1 — The hook's crate: the stop's input read, the branch's push and checks found, its tests (3 h)
- [ ] T2 — `story:finish` prepared, the hook in `.claude/settings.json`, the worker brief updated (2 h)
