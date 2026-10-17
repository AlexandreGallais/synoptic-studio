---
id: EN-096
epic: E17
feature: F20
title: The agent-facing docs written as principles, not as records of a conversation
status: draft
points: 5
---

# E17 · F20 · EN-096 — The agent-facing docs written as principles, not as records of a conversation

Many rules in `AGENTS.md`, `CLAUDE.md`, `.claude/`, `docs/conventions/`, `docs/process/` and the ADRs were written close to how a decision was first explained in conversation: its examples, its order, its wording. An agent reading them later has to infer the principle behind the example, and may doubt where it applies. Asked by the Product Owner on 2026-10-17: rewrite them so that each states the idea in general terms — the principle, its reason, how to tell a case, what to do on conflict — explained better than the conversation did, with an example only where it removes a doubt.

This belongs to F20 (the context an agent reads): a rule stated once, generally, is shorter to read and leaves fewer doubts.

## Acceptance criteria

- Given each page an agent reads to work (`AGENTS.md`, `CLAUDE.md`, `.claude/rules`, `.claude/skills`, the worker brief, `docs/conventions/`, `docs/process/`), when the pass ends, then every rule reads as a principle with its reason and its limits; dated decision notes stay as the record of when and by whom, never as the rule's body.
- Given an accepted ADR, when its wording follows a conversation too closely, then it is left as written (an accepted ADR is not rewritten) and the page that applies it states the principle.
- Given the pages before and after, when they are compared, then their length in tokens is written in the story, and no rule was lost (each old rule is traced to its new sentence).

## Tasks

To write at F20's refinement.
