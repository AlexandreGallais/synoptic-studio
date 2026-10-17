# Working with the agent

How the agent is configured and steered. Sources: `REF-CC-BEST-PRACTICES`, `REF-CC-MEMORY`, `REF-CC-SKILLS`, `REF-CC-HOOKS`, `REF-CC-SUBAGENTS`, `REF-ANTHROPIC-CONTEXT`; decision ADR-0021. This project's files: [agent configuration](../tooling/agent.md).

## Configuration

| Need | Mechanism |
| --- | --- |
| rules for any agent | `AGENTS.md`, short; every line prevents a mistake |
| what only one tool uses | `CLAUDE.md` importing `AGENTS.md`, then its own section |
| pitfalls of one area | `.claude/rules/*.md` with `paths:` |
| multi-step procedures | skills (`/story`, `/run`, `/spike`, `/audit`, `/review`, `/retro`…) |
| second opinions | read-only expert agents in a fresh context, told to refute, run in the background |
| what must never happen | a `PreToolUse` hook that denies with a reason, not an instruction |
| state after a compaction | a `SessionStart` hook printing facts derived from the files |
| what the agent is doing, live | a status line: backlog place, step, activity, time, tokens, the call running |
| how long things take | hooks timing every call; a metrics report per epic |
| sources without reading them twice | a local library behind an MCP server |
| preferences across projects | the agent's memory, one fact per file |

## Steering

- Ask for **evidence**: the command, its output, the source read.
- Give a check the agent can run; without one, "looks done" is the only signal.
- Business questions are answered by the Product Owner and written in the domain; the agent stops and asks rather than choose.
- A repeated mistake gets a check (lint rule, test, hook) and a line in [lessons](./lessons.md), not a sentence.
- Ask for pushback: the agent says when a request is a bad idea, with sources.
- Long runs: start one, leave it; the status line shows whether it works or waits.
- **Parallel sessions**: a second chat works in its own worktree from `develop`, writes every decision in the repository and messages the run ([agent configuration](../tooling/agent.md#parallel-sessions)).

## The Product Owner's role

The agent designs, implements, tests and documents. The Product Owner sets priorities, answers business questions, sets stories `ready`, tries the checklists, merges features and releases. They read the result, not the code: checks, demos and audits stand for line-by-line review. They never touch the repository; the agent writes their decisions in neutral words.

## Lessons from the Product Owner

The Product Owner's critiques of E01, E14 and E15, written as habits: what the agent does in each situation.

| When the agent is about to… | Instead |
| --- | --- |
| ask a refinement question | search the past features and the domain first; never ask again what is settled (point numbering, renumbering) |
| ask the Product Owner to choose | use the questionnaire; each option with a short example in numbers; never a drawing made of characters |
| propose a shape or a behavior | read first how the reference tools do it, and raise the open questions it touches at the refinement |
| refine a feature | ask only what the feature needs; write the Product Owner's other ideas in the domain for later |
| name a term in a question | say it in plain words the first time ("the rounding of the corners") |
| ask the Product Owner to test | name the branch checked out, check it is up to date, give the direct link to the step |
| open a feature pull request | clean every status and branch first, say all is clean, wait for a yes; push nothing else meanwhile |
| design a page clicked over and over | put the controls first and large; check with the longest and shortest content |
| write a test card | show why a choice exists, not only what it does |
| upgrade a dependency | read its release notes first; adapt the configuration to new features and breaking changes |
| report progress | show the step, not the diffs; during a run, only the commits and one summary |
| wait for something | read the statuses; stop at the first failure; never wait for a CI on a work branch |
| record what the Product Owner said | write the decision and its reason in neutral English, never quote |
| add code | only for stories the Product Owner set `ready` |
| organize code | split by business domain, never by a count of files; never leave files beside folders (E14) |
| stop a run | say the next step instead of asking when; write in plain words, never in the words of the code (E14) |
| leave a session | stop every background task; delete every merged branch at once, not after the release (E14) |
| fix a tool that misbehaves | find the cause in the data first (the transcript line, the log); write a test from that line (E14) |
| propose an improvement | say what would change for the Product Owner, with an example they can picture (E14) |
| write the same script again | make it a tool of the repository, tested, with its line in `docs/tooling/agent-tools.md` (E14) |
| offer an option in the editor | offer only what it can measure and draw, and only when its use is clear (E15) |
| finish an option | test the way back, the automatic path as well as the chosen one, a real computer's fonts, a turned object against a turned shape (E15) |
| prepare a validation | one short checklist page, at most 12 lines per feature, one line per thing to see, each tried in a browser first, the panel beside the drawing (E15, EN-056) |
| run work in the background | say each worker's start and end with an estimate; the status line shows each running subagent (E15, EN-053) |
| start subagents | at most three at once and one heavy command at a time, refused or queued by the tools (E15, ADR-0063, EN-047) |
| run work in parallel | keep branches on separate files; merge one at a time; never edit shared indexes and logs from a story branch (E15, ADR-0065) |
| explain a behaviour | give the situation as an example, in names or numbers: the default font missing, another chosen automatically, the one picked during the reading kept (E15) |
| report a session's figures | report the whole session, with `--all`, at its end only (E15) |
