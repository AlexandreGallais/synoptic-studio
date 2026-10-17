---
id: F20
epic: E17
title: A context diet for every agent
status: in-progress
---

# E17 · F20 — A context diet for every agent

**Benefit hypothesis**: each agent starts with only what its task needs, reads rules stated once as principles, makes fewer calls through tools of its own, and runs on the model its task needs, so that a run spends fewer tokens and less time; and the Product Owner sets a run's scope, follows it on one page, and is called only when a question or a blocker needs them.

Asked by the Product Owner on 2026-10-16 (RET-003), building on the lighter context planned for E02 at RET-002; widened on 2026-10-17 during SP-021 and SP-022. Sources and measures: [research note 0024](../../research/notes/0024-f20-context-diet.md). The tool `context:size` (`docs/tooling/agent-tools.md`, Context) is its measure.

## Decisions of the refinement (2026-10-17)

- **Measure first**: every change is measured in tokens before and after; a cut that saves nothing is not made. Cache reads are 78 % of a subagent's cost, so what an agent carries, times its requests, is the lever.
- **Runs**: the Product Owner sets a run's scope at its start, as a time or an end; the agent sets no bound of its own. A question parks its story and the run goes on with independent work; a run stops only on a problem it cannot get around, at its scope's end, or when the Product Owner says so. Pushes go to the desktop and the phone only for a question or a blocker that prevents finishing the scope.
- **The run's report** holds only its duration and tokens; gains and incidents go to the agent's logs and the retrospective.
- **The one-hour cache** is already on the main thread and would cost more than it saves on subagents: EN-064 is dropped.
- **Agents**: one worker type per task kind, each measured; Sonnet for logs, backlog, merges and step texts, and one measured trial on a story worker.
- **Tools of our own**: the Git sequences and the library route for every page fetch in F20; the log of each tool's use and the reader of one row or section in F21. No tool for reading files or memory: the measure shows no gain.
- **ADRs stay as written**; the pages that apply them state the principle.

## Acceptance criteria

1. Each task kind of a run has its agent type, started from what its task needs, its first request measured before and after (EN-065).
2. Each agent type names its model and reason; one story worked on Sonnet is compared with Opus (EN-066).
3. A worker that stops with its branch unpushed is told so, and the main thread gets a line to finish its story (EN-088).
4. The status line and the session report give a run's duration and tokens since its start, across compactions and sessions (EN-092).
5. A claude.ai page shows a run's tree, statuses, durations, agents, tokens and commits, updated after each story at the same address (EN-087).
6. A run works to the scope the Product Owner set, parks a question, and pushes only a question or a blocker (EN-086).
7. The frequent Git sequences take one call each, and every page fetched goes through the library with its project (EN-100, EN-101).
8. The agent-facing pages read as principles, measured in tokens before and after, no rule lost (EN-096 to EN-099).

## Stories

In delivery order. Stories stay `draft` until F20's `/run` sets them `ready`.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-022](../stories/E17-F20-SP-022-research-f20.md) | Spike | Research for F20, the context diet | done |
| [EN-064](../stories/E17-F20-EN-064-one-hour-prompt-cache.md) | Enabler | A one-hour prompt cache on long agents | dropped |
| [EN-065](../stories/E17-F20-EN-065-workers-started-from-a-digest.md) | Enabler | One worker type per task kind, each started from what its task needs | ready |
| [EN-066](../stories/E17-F20-EN-066-model-per-agent-type.md) | Enabler | A model chosen per agent type | ready |
| [EN-088](../stories/E17-F20-EN-088-subagent-stop-hook-in-rust.md) | Enabler | A SubagentStop hook in Rust that checks a worker's branch | ready |
| [EN-092](../stories/E17-F20-EN-092-run-totals-and-stop-report.md) | Enabler | The duration and tokens of a whole run | ready |
| [EN-087](../stories/E17-F20-EN-087-progress-page-as-an-artifact.md) | Enabler | A progress page as a claude.ai Artifact, updated during a run | ready |
| [EN-086](../stories/E17-F20-EN-086-push-notification-at-each-stop.md) | Enabler | A run that parks a question and pushes it, and stops only when it must | ready |
| [CHK-021](../stories/E17-F20-CHK-021-checkpoint-f20.md) | Checkpoint | Checkpoint in the middle of F20 | ready |
| [EN-100](../stories/E17-F20-EN-100-git-sequences-as-rust-commands.md) | Enabler | The agent's frequent Git sequences as Rust commands | done |
| [EN-101](../stories/E17-F20-EN-101-every-page-fetch-through-the-library.md) | Enabler | Every page fetched through the library, with the project recorded | done |
| [EN-096](../stories/E17-F20-EN-096-docs-written-as-principles.md) | Enabler | The agent's rules and skills written as principles | ready |
| [EN-097](../stories/E17-F20-EN-097-conventions-written-as-principles.md) | Enabler | The conventions written as principles | ready |
| [EN-098](../stories/E17-F20-EN-098-process-written-as-principles.md) | Enabler | The process pages written as principles | ready |
| [EN-099](../stories/E17-F20-EN-099-tooling-docs-written-as-principles.md) | Enabler | The tooling docs written as principles | ready |
| [AUD-022](../stories/E17-F20-AUD-022-audit-f20.md) | Audit | Audit F20 | ready |
| [VAL-021](../stories/E17-F20-VAL-021-validate-f20.md) | Validation | Validate F20 | ready |
