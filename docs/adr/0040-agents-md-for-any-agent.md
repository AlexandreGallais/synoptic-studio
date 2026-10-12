# ADR-0040 — `AGENTS.md` holds the rules for any coding agent

**Status**: Accepted (Product Owner, RET-001, 2026-10-13). Supersedes the "not adopted: `AGENTS.md`" point of ADR-0021.

## Context

ADR-0021 kept every rule in `CLAUDE.md` and left `AGENTS.md` out: one agent was used. The Product Owner wants the setup reusable by other coding tools, offline, for this project and the next ones. `AGENTS.md` is an open format read by many agents (`REF-AGENTS-MD`); Claude Code reads it through an `@AGENTS.md` import at the top of `CLAUDE.md`, the Claude-specific instructions below it (`REF-CC-MEMORY`).

## Decision

1. `AGENTS.md` holds what any agent needs: the project, where to read, the golden rules, key decisions, what is never done, how work flows, the definition of done, when to stop, commands and pitfalls.
2. `CLAUDE.md` starts with `@AGENTS.md`, then adds only what Claude Code uses: skills, expert agents, the source library's MCP tools, the step file of the status line, the session report, the guards.
3. Together they stay short (under 200 lines); area pitfalls stay in `.claude/rules/` (ADR-0021).

## Consequences

- Another tool reads the same rules without a copy; Claude Code reads them first, then its own.
- A rule is written once, in `AGENTS.md`; a Claude-only habit in `CLAUDE.md`.

## References

`REF-AGENTS-MD`, `REF-CC-MEMORY`, ADR-0021
