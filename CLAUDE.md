@AGENTS.md

# Claude Code

Only what Claude Code needs beyond `AGENTS.md`. The skills, agent briefs and area rules live in `.agents/`, reached from `.claude/` by links; `.claude/settings.json` holds the permissions and hooks.

- **The records through their tool**: `target/release/knowledge` (`show`, `search`, `list`, `sql`, `add`); the sources through the `library` server (`library_search`, `library_get`, `library_read`), `WebFetch` only for a page it cannot read.
- **Skills**: `/story`, `/spike`, `/audit`, `/run` (only when the Product Owner starts one), `/review` (only when they ask), `/triage`.
- **Subagents**, in the background, one at a time: a fresh `story-worker` per story, a `researcher` per spike, the `auditor` at an audit, `math-expert` or `functional-analyst` only for mathematics or business rules to check.
- **Never wait in the foreground**: a command over a minute runs in the background; a subagent's end arrives as a notification, never polled.
- **Chat**: each message starts with the time it was sent (`HH:MM`) and one marker — ✅ done, ⏳ in progress, ⚠️ attention, ❌ error, ❓ question, 🟢 agreed, 🟠 my mistake, ▶️ next — short, the detail on request; a choice for the Product Owner goes through the questionnaire, options with numeric examples.
- **Guards**: the `guard-bash` hook denies `gh pr merge`, `--no-verify`, force pushes without lease, pushes to `main` or `develop`: never look for a way around it.
- **A session's end**: the `SessionEnd` hook snapshots the records, saves them and writes the session's report.
