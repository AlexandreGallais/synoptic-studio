@AGENTS.md

# Claude Code

What only Claude Code uses; the rules above hold for every agent. Configuration: `docs/tooling/agent.md`.

## Skills, agents, tools

| Need                                | Use                                                                                                                                                                                                                                                                                                         |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| a story, a run, a spike, an audit   | `/story`, `/run` (only when the Product Owner starts one), `/spike`, `/audit`, `/derivation`                                                                                                                                                                                                                |
| a demo, a review, a retrospective   | `/review` (VAL and REV), `/retro`                                                                                                                                                                                                                                                                           |
| GitHub issues                       | `/triage`                                                                                                                                                                                                                                                                                                   |
| a second opinion, in the background | agents `auditor`, `math-expert`, `verifier`, `functional-analyst`, `doc-editor` (read-only)                                                                                                                                                                                                                 |
| a source                            | MCP `library_search`, then `library_get` and `library_read`; WebFetch only for a page they cannot read                                                                                                                                                                                                      |
| a choice for the Product Owner      | the questionnaire (AskUserQuestion), options with numeric examples, no drawings in characters; its last question always leaves room for free remarks; a refinement in two rounds — first questions with a long free comment, then every remaining question at once, after which the Product Owner may leave |

## While working

- What a subagent or a background command is doing: `npm run activity`; the status line says when the chat is free (`docs/tooling/metrics.md`).
- Write the current step to `.git/metrics/step` (one short line, French): the status line shows it under the epic, feature and story.
- A command expected to last more than a minute: `run_in_background`, then Monitor with `scripts/watch-task.ts` (stall, end, time limit — `docs/tooling/metrics.md`); never a foreground waiting loop. A call in red on the status line has run over 5 minutes: look at it.
- During a run the main thread only dispatches (ADR-0063): every piece of work goes to a subagent, and each Product Owner comment is acknowledged at once, saying when it is handled; one chat line when a worker starts and ends, with an estimate.
- At most three subagents at once and one heavy command at a time (ADR-0063): EN-047's hook and lock enforce it; until they are merged, count the running subagents before each `Agent` call and take the lock of the run skill's [worker brief](.claude/skills/run/worker-brief.md).
- A worktree is named after its story (`.claude/worktrees/us-061`), never left anonymous (`worktree:new`, EN-052).
- During `/run`, say nothing between tool calls: the commits and the status line show the progress; one summary at the stop.
- At a session's end only, the session report: `npm run metrics:session -- --all --markdown` (without `--all` it counts the current turn only).
- Chat messages to the Product Owner start with a marker, never in repository files: ✅ done, ⏳ in progress or waiting, ⚠️ attention, ❌ error, ❓ question, 🟢 understood or agreed, 🟠 my mistake or a correction, ▶️ next step.
- Explain a behaviour by the situation it covers, in names or numbers, never in the words of the code.
- Before a pause, log the session in `docs/process/journal.md`, then `npm run records:save` (ADR-0041, ADR-0044).

## Guards

The `guard-bash` hook denies `gh pr merge`, `--no-verify` (and `commit -n`, `HUSKY=0`, `core.hooksPath`) and force pushes without lease (`+branch` included): never look for a workaround. Area pitfalls load with their files: `.claude/rules/library.md` (`src/`, `playground/`), `docs.md` (Markdown), `tooling.md` (lint, tests, CI, hooks).
