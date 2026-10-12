# ADR-0021 — Agent working environment: skills, rules, hooks and an independent auditor

**Status**: Accepted, partly superseded by ADR-0038 (configuration changes go to `develop`) and ADR-0040 (`AGENTS.md`)

## Context

The code is written by a coding agent (Claude Code). Its instruction file, `CLAUDE.md`, is loaded in every session and grows with each lesson; past ~200 lines adherence drops, and procedures used once per story cost context in every session (`REF-CC-MEMORY`, `REF-CC-BEST-PRACTICES`). Some rules must hold every time — never merge a story pull request, never bypass the Git hooks — and instructions are only advisory (`REF-CC-HOOKS`). After a context compaction the agent must reload the project state cheaply (`REF-ANTHROPIC-CONTEXT`). The Product Owner wants the audit not to be graded by the agent that did the work.

## Decision

Configuration in `.claude/`, committed:

| Mechanism                          | Content                                                                                                                                                             | Loaded                          |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- |
| `CLAUDE.md`                        | rules true in every session, where to read, commands — under 200 lines                                                                                              | every session                   |
| `.claude/rules/*.md` with `paths:` | pitfalls of one area: `src/`, docs Markdown, root tests and tooling                                                                                                 | when matching files are touched |
| `.claude/skills/`                  | procedures: `story`, `spike`, `audit`, `derivation`                                                                                                                 | on demand, `/name`              |
| `.claude/agents/auditor.md`        | read-only reviewer in a fresh context: re-derives, recomputes, tries to refute; reports only gaps affecting correctness or requirements                             | by the `audit` skill            |
| `.claude/settings.json` hooks      | `PreToolUse` guard (deny `gh pr merge`, `--no-verify`, force push without lease); `SessionStart` state (branch, uncommitted files, `ready` / `in-progress` stories) | every matching event            |

The source of truth stays in `docs/`: skills and rules point to it and add only the procedure or the pitfall. A rule that a hook enforces is not repeated as an instruction.

Not adopted: `AGENTS.md` (one agent, `REF-AGENTS-MD`); a `Stop` hook running the checks (`check:all` takes about a minute per turn); a formatting hook after each edit (`npm run fix` and lint-staged already format, and a rewrite behind the agent's back invalidates its view of the file).

## Consequences

- Shorter `CLAUDE.md`; pitfalls appear where they matter.
- Guards hold even when an instruction is forgotten; a guard that blocks a legitimate action is changed in this ADR's successor, not bypassed.
- The audit gets a second opinion; its findings are still checked by the agent before being recorded (a reviewer asked for gaps tends to report some).
- `.claude/` is part of the tooling: changed on `main` like CI, listed in `docs/tooling/`.

## References

`REF-CC-BEST-PRACTICES`, `REF-CC-MEMORY`, `REF-CC-SKILLS`, `REF-CC-HOOKS`, `REF-CC-SUBAGENTS`, `REF-ANTHROPIC-CONTEXT`, ADR-0020, [research note 0002](../research/notes/0002-agent-and-verification-practices.md)
