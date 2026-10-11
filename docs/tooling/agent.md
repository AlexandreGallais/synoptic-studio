# Agent configuration

The code is written by Claude Code under the Product Owner's review. Its configuration is committed in `.claude/` (ADR-0021); `docs/` stays the source of truth, `.claude/` only adds procedures, pitfalls and guards.

| Path                             | Role                                                                                     | Loaded                                   |
| -------------------------------- | ---------------------------------------------------------------------------------------- | ---------------------------------------- |
| `CLAUDE.md`                      | rules true in every session, where to read, commands (under 200 lines)                   | every session                            |
| `.claude/rules/library.md`       | pitfalls of `src/` and `playground/`                                                     | when those files are touched             |
| `.claude/rules/docs.md`          | pitfalls of Markdown, backlog and bibliography                                           | when Markdown is touched                 |
| `.claude/rules/tooling.md`       | pitfalls of lint, tests, dependencies, CI, hooks                                         | when tooling is touched                  |
| `.claude/skills/story/`          | `/story` — a story from branch to pull request                                           | on demand                                |
| `.claude/skills/spike/`          | `/spike` — research spike of a feature (ADR-0020)                                        | on demand                                |
| `.claude/skills/derivation/`     | `/derivation` — writing a `DERIV-*`                                                      | on demand                                |
| `.claude/skills/audit/`          | `/audit` — feature audit with the auditor subagent                                       | on demand                                |
| `.claude/skills/review/`         | `/review` — feature demo page, epic review (ADR-0023)                                    | on demand                                |
| `.claude/skills/retro/`          | `/retro` — epic retrospective, evolvability, playbook (ADR-0023)                         | on demand                                |
| `.claude/skills/triage/`         | `/triage` — open issues presented with a recommendation (ADR-0026)                       | on demand                                |
| `.claude/skills/run/`            | `/run` — autonomous run on the feature branch, validation with local previews (ADR-0028) | on demand                                |
| `.claude/agents/auditor.md`      | independent read-only auditor, fresh context, tries to refute                            | from `/audit`                            |
| `.claude/settings.json`          | permissions for routine commands; hooks below                                            | every session                            |
| `.claude/hooks/guard-bash.ts`    | `PreToolUse`: denies `gh pr merge`, `--no-verify`, force push without lease              | every shell command                      |
| `.claude/hooks/session-state.ts` | `SessionStart`: prints branch, uncommitted files, progress of started features           | session start, resume, clear, compaction |

## Parallel sessions

The Product Owner may talk to a second session while a run works in the background. Both share the same folder, and the run owns it (its branch, its uncommitted files). The second session:

1. **Never touches the run's folder**: no `git switch`, `stash` nor commit there. It works in its own worktree from `main`, in its scratchpad: `git worktree add -b <type>/<topic> <scratchpad>/<topic> origin/main`, then `npm ci` in it (hooks and `check:all` need `node_modules`).
2. **Puts each decision in the repository**, the only state both sessions read: domain, ADR, backlog, `CLAUDE.md`, skills. One branch, one pull request into `main` (or straight to `main` when the Product Owner allows it for tooling and backlog). Durable facts also go to the agent's memory, read by every new session.
3. **Tells the run at once**: `ListAgents`, then `SendMessage` to the busy session with what changed, where it is written and what it changes for the run. A run never acts on a message alone: it waits for the change to reach `main`, as with any decision.
4. **Removes its worktree** once the pull request is merged (`git worktree remove <path>`, then delete the branch).

The run picks up `main` before every story (run skill, step 1): when `main` has new commits in `CLAUDE.md`, `.claude/`, `docs/domain/`, `docs/adr/`, `docs/backlog/` or `docs/conventions/`, it rebases the feature branch on `main`, reads them and follows them from that story on. A skill or rule loaded at the start of the run is replaced by the new version; a change to the run's scope or stories is a stop. So the Product Owner never has to repeat in one chat what was decided in the other.

## Searching past conversations

Claude Code keeps each conversation as a JSONL file in `~/.claude/projects/<project-path>/` (local, never committed). The session journal ([journal](../process/journal.md), ADR-0024) is read first; the transcripts are the full archive:

```sh
cd ~/.claude/projects/<project-path>
grep -l "radius" *.jsonl                                     # which conversation mentions a word
jq -r 'select(.type == "user") | .message.content | strings' <id>.jsonl | grep -i "radius"
```

## Rules for this folder

- Hooks are TypeScript run by `node` (type stripping), linted as tooling, tested (`.claude/hooks/*.test.ts`): pure functions exported, effects under `if (import.meta.main)`.
- A rule enforced by a hook is not repeated as an instruction; a guard that blocks legitimate work is changed by a new ADR, never bypassed.
- `.claude/settings.local.json` (personal, not committed) may add permissions; it never removes a guard.
- Changes go to `main` like CI changes (tooling), with `chore(tooling): …` commits.
