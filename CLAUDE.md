@AGENTS.md

# Claude Code

What only Claude Code uses; the rules above hold for every agent. Configuration: `docs/tooling/agent.md`.

## Skills, agents, tools

| Need | Use |
| --- | --- |
| a story, a spike, an audit, a derivation | `/story`, `/spike`, `/audit`, `/derivation` |
| a run of stories, only when the Product Owner starts one | `/run`, whose rules then hold for the main thread |
| a demo, a review, a retrospective; GitHub issues | `/review` (VAL and REV), `/retro`; `/triage` |
| a second opinion, in the background | the read-only agents `auditor`, `math-expert`, `verifier`, `functional-analyst`, `doc-editor` |
| a source | the `library` server: `library_search`, then `library_get` and `library_read`; `WebFetch` only for a page it cannot read |
| a choice for the Product Owner | the questionnaire (AskUserQuestion): options with numeric examples, no drawing in characters, a last question left open for remarks |

## While working

- **Show where the work is**: the current step in one short French line in `.git/metrics/step`, shown by the status line under the epic, feature and story; `target/release/activity` for what a subagent or a background command does (`docs/tooling/metrics.md`).
- **Never wait in the foreground**: a command expected to last over a minute runs with `run_in_background`, watched by Monitor with `watch-task` (`tools/metrics`: stall, end, time limit). A subagent's end arrives as a notification: never poll it with a loop, whose string may never appear. A call shown red on the status line has run over 5 minutes: look at it. Before a stop message, stop every background task still running.
- **One agent at a time** (AGENTS.md, Work, ADR-0086): one subagent runs at once, in the background, so that the chat stays free; the heavy commands take the lock `tools/heavy-lock` themselves, never by hand; the hook `limit-agents`, which refuses a fourth subagent started by `Agent` or resumed by `SendMessage`, and `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` at 3 stay as a backstop (EN-047, EN-063).
- A worktree is made by `target/release/worktree-new <ID>` and named after its story, never left anonymous (EN-052).
- **Chat messages** start with a marker, never in repository files: ✅ done, ⏳ in progress or waiting, ⚠️ attention, ❌ error, ❓ question, 🟢 understood or agreed, 🟠 my mistake or a correction, ▶️ next step. Explain a behaviour by the situation it covers, in names or numbers, never in the words of the code.
- At a session's end only, the session report: `target/release/session-report --all --markdown` (without `--all` it counts the current turn only). Before a pause, log the session in `docs/process/journal.md`, then `target/release/records-save` (ADR-0081).

## Guards

The `guard-bash` hook denies `gh pr merge`, `--no-verify` (and `commit -n`, a change of `core.hooksPath`) and force pushes without lease (`+branch` included), and pushes to `main` or `develop`: never look for a way around it. The area rules load with their files: `.claude/rules/library.md` (`src/`, `playground/`), `docs.md` (Markdown), `tooling.md` (configuration, tests, CI, `.claude/`).
