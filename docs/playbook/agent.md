# Working with the agent

Sources: `REF-CC-BEST-PRACTICES`, `REF-CC-MEMORY`, `REF-CC-SKILLS`, `REF-CC-HOOKS`, `REF-CC-SUBAGENTS`, `REF-ANTHROPIC-CONTEXT`; decision ADR-0021.

## Configuration

| Need                                 | Mechanism                                                                          |
| ------------------------------------ | ---------------------------------------------------------------------------------- |
| rules true in every session          | `CLAUDE.md`, under 200 lines; every line must prevent a mistake                    |
| pitfalls of one area                 | `.claude/rules/*.md` with `paths:`                                                 |
| multi-step procedures                | skills (`/story`, `/spike`, `/derivation`, `/audit`, `/review`, `/retro`)          |
| a second opinion                     | read-only subagent in a fresh context, told to refute and to report only real gaps |
| what must never happen               | `PreToolUse` hook (deny with a reason), not an instruction                         |
| state after a compaction             | `SessionStart` hook printing facts derived from the files                          |
| personal preferences across projects | the agent's memory, one fact per file                                              |

## Steering

- Ask for **evidence**: the command, its output, the source read.
- Give a check the agent can run; without one, "looks done" is the only signal.
- Business questions are answered by the Product Owner and written in the domain docs; the agent must stop and ask rather than choose.
- Prefer one task per conversation turn of work; let the agent compact, the hooks and the repository restore the state.
- When the agent repeats a mistake, do not add a sentence: add a check (lint rule, test, hook) and a line in [lessons](./lessons.md).
- Ask the agent for pushback: it should say when a request is a bad idea, with sources.
- **Parallel sessions**: a second chat during a run works in its own worktree from `main`, writes every decision in the repository, messages the run; the run reads `main` before each story ([agent configuration](../tooling/agent.md#parallel-sessions)).

## The Product Owner's role

The agent designs, implements, tests and documents; the Product Owner sets priorities, answers business questions, sets stories `ready`, tests the demos, merges pull requests, and writes in the improvement log. The Product Owner calls this mode "vibe coding"; in its original sense the term means accepting code without reading it, whereas here every change is reviewed, tested and sourced — through checks rather than line-by-line reading. The Product Owner may never touch the repository: the agent reads their messages for intent, asks when a decision is ambiguous, and writes their feedback into the repository, quoting them.

## Lessons from the Product Owner

"When I do this, do that instead": the [feedback log](../process/feedback.md) grouped at each retrospective, one line per habit. Empty until the first retrospective (E01).
