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

Drafts from the refinement of 2026-10-16 (Claude Code settings recommended by research, each shown to the Product Owner before it is applied) and of 2026-10-17 (EN-086 to EN-088, asked during SP-021, each tool they rely on sourced by SP-022 before the story is set `ready`); the context diet's own stories are written after the measure.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-022](../stories/E17-F20-SP-022-research-f20.md) | Spike | Research for F20, the context diet | draft |
| [EN-064](../stories/E17-F20-EN-064-one-hour-prompt-cache.md) | Enabler | A one-hour prompt cache on long agents | draft |
| [EN-065](../stories/E17-F20-EN-065-workers-started-from-a-digest.md) | Enabler | Workers started from a digest instead of all of CLAUDE.md | draft |
| [EN-066](../stories/E17-F20-EN-066-model-per-agent-type.md) | Enabler | A model chosen per agent type | draft |
| [EN-086](../stories/E17-F20-EN-086-push-notification-at-each-stop.md) | Enabler | A push notification at each stop, question or validation ready | draft |
| [EN-087](../stories/E17-F20-EN-087-progress-page-as-an-artifact.md) | Enabler | A progress page as a claude.ai Artifact, updated during a run | draft |
| [EN-088](../stories/E17-F20-EN-088-subagent-stop-hook-in-rust.md) | Enabler | A SubagentStop hook in Rust that checks a worker's branch | draft |
| [EN-092](../stories/E17-F20-EN-092-run-totals-and-stop-report.md) | Enabler | The totals of a whole run, and a report when it stops | draft |
| [CHK-021](../stories/E17-F20-CHK-021-checkpoint-f20.md) | Checkpoint | Checkpoint in the middle of F20 | draft |
| [AUD-022](../stories/E17-F20-AUD-022-audit-f20.md) | Audit | Audit F20 | draft |
| [VAL-021](../stories/E17-F20-VAL-021-validate-f20.md) | Validation | Validate F20 | draft |
