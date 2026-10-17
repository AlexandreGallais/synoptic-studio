---
id: EN-065
epic: E17
feature: F20
title: One worker type per task kind, each started from what its task needs
status: draft
points: 5
---

# E17 · F20 · EN-065 — One worker type per task kind, each started from what its task needs

Each agent receives what its task needs, smaller and better made, not everything (Product Owner, 2026-10-17). A general-purpose worker starts at 38,905 tokens, the project's typed agents at about 14,000 to 15,600; carried over a worker's 213 requests, the difference is about 5.1 million tokens read per worker ([research note 0024](../../research/notes/0024-f20-context-diet.md)). Decided at F20's second round: one worker type per task kind, each measured.

Sourced means (`REF-CC-SUBAGENTS`): an agent file in `.claude/agents/` with `omitClaudeMd: true` (no `CLAUDE.md` nor `AGENTS.md`), `skills` to preload the brief or its digest, `tools` for the tools the task uses, `mcpServers` for the `library` server only where research needs it.

## Acceptance criteria

- Given the task kinds of a run — story worker, research (spike), audit, log and backlog writer — when F20 ends, then each has its agent type in `.claude/agents/` with its tools, its preloaded brief and its reason, and the run skill names which type each story type uses.
- Given each type, when `context:size` measures its first request before and after, then both figures are written in this story, and a type that saves nothing keeps the general-purpose start.
- Given a worker of the new story type, when it works one real story, then it follows the same rules as with the full files: the checks, the commits, the brief's limits; any rule it missed is added to its brief, not to every agent's.
- Given the memory index, which this spike's worker received although the documentation says a subagent does not load it, when the types are measured, then whether it loads is stated from the measure.

## Tasks

One task = one commit, referenced as `EN-065.Tn`.

- [ ] T1 — Measure the first request of each task kind today with `context:size`, and list what each needs (2 h)
- [ ] T2 — The story worker's type and its brief as a preloaded skill, tried on one real story (3 h)
- [ ] T3 — The research, audit and log types, each measured (3 h)
- [ ] T4 — The run skill and the worker brief name the types; `docs/tooling/agent.md` updated (1 h)
