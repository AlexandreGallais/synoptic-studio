---
id: EN-065
epic: E17
feature: F20
title: One worker type per task kind, each started from what its task needs
status: done
points: 5
---

# E17 · F20 · EN-065 — One worker type per task kind, each started from what its task needs

Each agent receives what its task needs, smaller and better made, not everything (Product Owner, 2026-10-17). A general-purpose worker starts at 38,905 tokens, the project's typed agents at about 14,000 to 15,600; carried over a worker's 213 requests, the difference is about 5.1 million tokens read per worker ([research note 0024](../../research/notes/0024-f20-context-diet.md)). Decided at F20's second round: one worker type per task kind, each measured.

Sourced means (`REF-CC-SUBAGENTS`): an agent file in `.claude/agents/` with `omitClaudeMd: true` (no `CLAUDE.md` nor `AGENTS.md`), `skills` to preload the brief or its digest, `tools` for the tools the task uses, `mcpServers` for the `library` server only where research needs it.

## Acceptance criteria

- Given the task kinds of a run — story worker, research (spike), audit, log and backlog writer — when F20 ends, then each has its agent type in `.claude/agents/` with its tools, its preloaded brief and its reason, and the run skill names which type each story type uses.
- Given each type, when `context:size` measures its first request before and after, then both figures are written in this story, and a type that saves nothing keeps the general-purpose start.
- Given a worker of the new story type, when it works one real story, then it follows the same rules as with the full files: the checks, the commits, the brief's limits; any rule it missed is added to its brief, not to every agent's.
- Given the memory index, which this spike's worker received although the documentation says a subagent does not load it, when the types are measured, then whether it loads is stated from the measure.

## Tasks

One task = one commit, referenced as `EN-065.Tn`.

- [ ] T1 — Measure the first request of each task kind today with `context:size`, and list what each needs (2 h)
- [ ] T2 — The story worker's type and its brief as a preloaded skill, tried on one real story (3 h)
- [ ] T3 — The research, audit and log types, each measured (3 h)
- [ ] T4 — The run skill and the worker brief name the types; `docs/tooling/agent.md` updated (1 h)

## Measures

Measured with `npm run context:size` on the session of 2026-10-17 (77 subagents of F18, F19 and F20's runs), Claude Code 2.1.302. A first request's tokens are its input, cache writes and cache reads.

### Before (T1)

| Task kind | Seen in F18–F19 | Agent type today | First request, tokens | What the task needs |
| --- | --- | --- | --- | --- |
| story worker | EN-045 to EN-095, AUD fixes, CHK, VAL preparation | `general-purpose` | 38,900 (median of 71) | Bash, Read, Edit, Write, Grep, Glob; the `library` server for a `@see` source; the worker brief and the rules of `AGENTS.md` a story applies |
| research spike | SP-020, SP-021, SP-022 | `general-purpose` | 38,306 to 39,228 | the story worker's tools and `WebFetch`, `WebSearch`, the `library` server; the spike skill; the writing rules |
| audit review | AUD-020, AUD-021 | `auditor`, `verifier`, `doc-editor` | 15,477, 13,994, 13,865 | read-only tools and their own prompt, which names what to read |
| logs, backlog and merges | the run's set-up, journal entries, rebases | `general-purpose` | 38,839 to 39,419 | Bash, Read, Edit, Write, Grep, Glob; the writing rules, the shared files, one Git command per call |

What makes the difference: a `general-purpose` worker receives every tool's definition (`Agent`, `Artifact`, `Skill`, the MCP servers), the skill listing (26,805 characters) and the deferred tools' list; a typed agent with a `tools` list receives none of them. Every subagent, typed or not, received `CLAUDE.md` (5,155 characters), `AGENTS.md` (14,719) and the memory index `MEMORY.md` (5,454).

**The memory index loads** in every subagent measured, typed or not, as an instruction file of type `AutoMem`, although `REF-CC-SUBAGENTS` says "the main conversation's auto memory isn't loaded".

### After (T2, T3)

Each type started by a headless session (`claude -p`) in this story's worktree, on one real small task: list the tasks of EN-066's story file, read with `Read`. The same task given to a `general-purpose` agent in the same way is the comparison; a headless session connects fewer MCP servers than the Product Owner's, so its `general-purpose` start (27,843) is below the run's (38,900).

| Task kind | Agent type | Preloaded | First request, tokens | `general-purpose`, same task | Run's `general-purpose` |
| --- | --- | --- | --- | --- | --- |
| story worker | `story-worker` | `project-rules`, `worker-brief` | 9,627 | 27,843 | 38,900 |
| research spike | `researcher` | `project-rules`, `worker-brief`, `spike` | 11,201 | 27,843 | 38,306 to 39,228 |
| logs, backlog and merges | `clerk` | `project-rules` | 6,901 | 27,843 | 38,839 to 39,419 |

The reviewers already had their type; each now starts without `CLAUDE.md`, with `project-rules` preloaded. Before and after, on the same task:

| Agent type | Before, tokens | After, tokens | Saved |
| --- | --- | --- | --- |
| `auditor` | 15,103 | 7,608 | 7,495 |
| `verifier` | 13,753 | 6,258 | 7,495 |
| `doc-editor` | 13,641 | 6,146 | 7,495 |
| `math-expert` | 15,450 | 7,925 | 7,525 |
| `functional-analyst` | 15,439 | 7,916 | 7,523 |

Every type saves: none keeps the `general-purpose` start. A run's merges and story finishing stay with the main thread (`story:finish`), which already holds what they need; the `clerk` takes the records it hands off.

With `omitClaudeMd`, the memory index is not loaded either: the `story-worker`'s first request holds no instruction file. So the memory index travels with `CLAUDE.md`, not with the auto memory the page describes; the rules a worker needs from it (no personal information, the repository's tools rather than scripts) are in `project-rules`.

### Trial on a real task (T4)

A `story-worker`, started by a headless session with the run's prompt (story id, feature branch, worktree, the `cd` rule), worked T4 of this story: it read the story's measures, wrote the run's worker types into `docs/tooling/agent.md`, ran `npm run fix`, and committed with `Refs: EN-065.T4` and the attribution line, without pushing, as asked. Its edit of `.claude/skills/run/SKILL.md` and its writes to the scratch folder were refused by the headless session's permissions; it stopped there, tried no way around, and reported the edit to make, which the main thread applied in the same commit. No rule was missed: nothing added to its brief. Its first request: 9,627 tokens.

## For the main thread

- The `done` commit's footer: `Rule-docs: none affected, agent types and tooling docs only`.
- Workers of a headless session (`claude -p`) cannot edit `.claude/` nor write outside their worktree; a run's workers in the Product Owner's session can.
