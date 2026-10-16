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

## Questions

Answered in [research note 0022](../../research/notes/0022-f18-parallel-run.md):

- How a `PreToolUse` hook counts running subagents: it gets no count, it rebuilds one from the transcript or from a state file kept by the hooks (`REF-CC-HOOKS`).
- Where `.git`'s common directory is from a worktree: `git rev-parse --path-format=absolute --git-common-dir` (`REF-GIT-REV-PARSE`, `REF-GIT-WORKTREE`).
- What Prettier's `proseWrap: "never"` does to tables: it removes the padding only of a table wider than `printWidth` (`REF-PRETTIER-TABLE-CODE`), which ADR-0065 decision 1 did not expect.
- Whether GitHub's merge applies `merge=union`: undocumented, to be tried by EN-048.
- How release-please splits the release pull request from the release: `target-branch` and the two `skip-` inputs (`REF-RELEASE-PLEASE-ACTION`, `REF-RELEASE-PLEASE-DESIGN`); to be checked on the next release by EN-046.

## Tasks

One task = one commit, referenced as `SP-020.Tn`.

- [x] T1 — Read and record the sources in `docs/references.md` and a research note (2 h)

## For the main thread

- Product Owner question, before EN-048.T1: ADR-0065 decision 1 expects `proseWrap: "never"` to remove the padding of tables; Prettier's code removes it only from tables wider than 80 characters (about 546 of 607 here), keeps it on the others, and joins every paragraph written on several lines (281 of 453 files under `docs/` change). Options, none chosen: keep decision 1 as is; add a Markdown override (`printWidth: 1`, `embeddedLanguageFormatting: "off"`) so that every table loses its padding; leave Markdown to another formatter (F19).
