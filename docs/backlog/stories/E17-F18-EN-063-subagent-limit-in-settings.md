---
id: EN-063
epic: E17
feature: F18
title: The subagent limit set in Claude Code's settings
status: ready
points: 1
---

# E17 · F18 · EN-063 — The subagent limit set in Claude Code's settings

Set the limit of simultaneous subagents in Claude Code's settings, beside the hook of EN-047 (ADR-0063). Recommended by the research of 2026-10-16; the change is shown to the Product Owner before it is applied.

SP-020 ([research note 0022](../../research/notes/0022-f18-parallel-run.md)): `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` under `env` in `.claude/settings.json`, a positive whole number that lowers the cap of 20; its refusal names no subagent and a resume escapes it, so it backs the hook of EN-047 up rather than replacing it (`REF-CC-ENV-VARS`, `REF-CC-SUBAGENTS`).

## Acceptance criteria

- Given the recommended setting shown to the Product Owner and accepted, when it is applied, then the settings file holds the limit and `docs/tooling/agent.md` names it beside the hook of EN-047.
- Given the Product Owner declines or changes the value, when the story ends, then the decision is written in the story and in the docs.

## Tasks

One task = one commit, referenced as `EN-063.Tn`.

- [ ] T1 — Show the setting to the Product Owner, apply it, document it (1 h)
