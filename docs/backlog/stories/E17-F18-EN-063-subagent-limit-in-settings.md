---
id: EN-063
epic: E17
feature: F18
title: The subagent limit set in Claude Code's settings
status: done
points: 1
---

# E17 · F18 · EN-063 — The subagent limit set in Claude Code's settings

Set the limit of simultaneous subagents in Claude Code's settings, beside the hook of EN-047 (ADR-0063). Recommended by the research of 2026-10-16; the change is shown to the Product Owner before it is applied.

SP-020 ([research note 0022](../../research/notes/0022-f18-parallel-run.md)): `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` under `env` in `.claude/settings.json`, a positive whole number that lowers the cap of 20; its refusal names no subagent and a resume escapes it, so it backs the hook of EN-047 up rather than replacing it (`REF-CC-ENV-VARS`, `REF-CC-SUBAGENTS`).

## Acceptance criteria

- Given the recommended setting shown to the Product Owner and accepted, when it is applied, then the settings file holds the limit and `docs/tooling/agent.md` names it beside the hook of EN-047.
- Given the Product Owner declines or changes the value, when the story ends, then the decision is written in the story and in the docs.
- Given three subagents running, when a `SendMessage` resumes a fourth, then it is refused like an `Agent` call, naming the three running (the hook's matcher covers `Agent` and `SendMessage`).

## Tasks

One task = one commit, referenced as `EN-063.Tn`.

- [x] T1 — Show the setting to the Product Owner, apply it, document it (1 h)

## Decision

On 2026-10-17 the Product Owner accepted the setting as recommended: in `.claude/settings.json`, the hook `limit-agents` of EN-047 runs before every `Agent` and every `SendMessage` call, so a fourth subagent is refused whether it is started or resumed, and `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` is set to 3 under `env` as a backup cap. The value stays 3, as ADR-0063 sets.

The hook read a `SendMessage` like an `Agent` call, so it also refused a message to one of the three subagents running, which only queues and starts none: the main thread could no longer pass a remark of the Product Owner to a worker. It now lets a message through when it goes to the main conversation or to a running subagent, found by the agent id its launch or resume result names, or by the `name` its `Agent` call gave; a message to any other agent is refused when three run.
