---
id: F20
epic: E17
title: A context diet for every agent
status: draft
---

# E17 · F20 — A context diet for every agent

**Benefit hypothesis**: each agent starts with only what it needs — a rule in one line rather than a paragraph, clear wording, an ADR read through a one-line digest — so that a session and each subagent spend fewer tokens and the conversation memory fills up more slowly.

Asked by the Product Owner on 2026-10-16 (RET-003), building on the lighter context planned for E02 at RET-002. A Rust context tool of F19 may serve it.

## Goal

- Go over everything an agent receives as input — `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, the skills, the agent definitions, the worker brief, the docs and ADRs they read — and keep only what is needed.
- **Measure first**: the tokens of each input, per agent type, before and after. A cut that saves no tokens is not worth making.
- **ADRs stay as they are**: an accepted ADR is never rewritten. Agents read a one-line digest per ADR (the index) instead of the full files, and read a full ADR only when they need it.

## Open questions

- How to measure the tokens each agent type loads (session start, worker, auditor, light agents), and where to keep the figures (the tool `context:size` of `docs/tooling/agent-tools.md`).
- Which inputs cost the most, and which can be cut without losing a rule.
- What the one-line digest of an ADR holds, and how it stays in step with the ADR (a check).

## Stories

None yet: written at the refinement, after the measure.
