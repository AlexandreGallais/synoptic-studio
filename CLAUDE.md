@AGENTS.md

# Claude Code

What only Claude Code uses; the rules above hold for every agent. Configuration: `.claude/` (settings, skills, agents, rules).

## Skills, agents, tools

| Need | Use |
| --- | --- |
| a story, a spike, an audit, a derivation | `/story`, `/spike`, `/audit`, `/derivation` |
| a run of stories, only when the Product Owner starts one | `/run`, whose rules then hold for the main thread |
| a review, only when the Product Owner asks; GitHub issues | `/review`, `/retro` for its method part; `/triage` |
| a story, in the background | a fresh `story-worker` per story, never one agent chaining stories: its cost is its context re-read at each call |
| a research spike | a `researcher` |
| a second opinion, in the background, only where its independence pays | the read-only `auditor` at an audit; `math-expert` and `functional-analyst` only for mathematics or business rules to check; each alone, none for small tasks |
| a source | the `library` server: `library_search`, then `library_get` and `library_read`; `WebFetch` only for a page it cannot read |
| a choice for the Product Owner | the questionnaire (AskUserQuestion): options with numeric examples, no drawing in characters, a last question left open for remarks |

## While working

- **Show where the work is**: the current step in one short French line in `.git/metrics/step`, shown by the status line; `target/release/activity` for what a subagent or a background command does.
- **Never wait in the foreground**: a command expected to last over a minute runs with `run_in_background`, watched by Monitor with `watch-task`. A subagent's end arrives as a notification: never poll it with a loop. A call shown red on the status line has run over 5 minutes: look at it. Before a stop message, stop every background task still running.
- **One agent at a time**: one subagent runs at once, in the background, so that the chat stays free; the heavy commands take the lock themselves, never by hand; the hook `limit-agents` and `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` at 3 stay as a backstop.
- A worktree is made by `target/release/worktree-new <ID>` and named after its story.
- **Chat messages** start with a marker, never in repository files: ✅ done, ⏳ in progress or waiting, ⚠️ attention, ❌ error, ❓ question, 🟢 understood or agreed, 🟠 my mistake or a correction, ▶️ next step. Explain a behaviour by the situation it covers, in names or numbers, never in the words of the code.
- **A session's end**: the `SessionEnd` hook snapshots the records, saves them and writes the session report; a journal row goes into the records before a pause.

## Guards

The `guard-bash` hook denies `gh pr merge`, `--no-verify` (and `commit -n`, a change of `core.hooksPath`), force pushes without lease (`+branch` included), pushes to `main` or `develop`, and the heavy lock taken by hand: never look for a way around it. The area rules load with their files: `.claude/rules/library.md` (`src/`, `playground/`), `docs.md` (Markdown), `tooling.md` (configuration, tests, CI, `.claude/`).
