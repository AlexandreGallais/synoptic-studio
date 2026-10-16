---
id: SP-020
epic: E17
feature: F18
title: Research for F18, the sources of RET-003 recorded
status: ready
points: 1
---

# E17 · F18 · SP-020 — Research for F18, the sources of RET-003 recorded

Record as a research note the sources read on 2026-10-16 for RET-003 (Claude Code's subagents, hooks and worktrees pages, Git's gitattributes and rerere, the fragment pattern, flaky tests, `lockf`), and read what each F18 story still needs (ADR-0020).

## Acceptance criteria

- Given the sources of the retrospective record, when the spike ends, then each one is a `REF-*` row with `[verified]` and its date, or a research request.
- Given each story of F18, when the spike ends, then its open technical questions are answered with a source (how a `PreToolUse` hook counts running subagents, where `.git`'s common directory is from a worktree, Prettier's `proseWrap`).

## Tasks

One task = one commit, referenced as `SP-020.Tn`.

- [ ] T1 — Read and record the sources in `docs/references.md` and a research note (2 h)
