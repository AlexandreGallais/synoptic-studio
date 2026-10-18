# ADR-0082 — The npm scripts kept for a person or a caller, the agent's tools by their binary only

**Status**: Accepted (the agent, 2026-10-18, under ADR-0074, on the Product Owner's request of the same day, to be told to the Product Owner; written by EN-115). Supersedes ADR-0078, decision 1, where it keeps an npm script for each of the agent's tools: "`npm run <script>` stays the form for people", and its consequence that `docs/tooling/agent-tools.md` maps each script to its binary.

## Context

- `package.json` held 52 scripts on 2026-10-18. The Product Owner runs a few of them and found the rest clutter.
- Since ADR-0078 the agent calls each Rust tool by its binary, `target/release/<binary>`; the scripts of its tools were kept beside the binaries, for people, and `agent-tools.md` mapped one to the other.
- Something calls some scripts by their name: npm's lifecycle (`prepare`), the Git hooks (`check:all`, `golden:check`), the CI (`rust:build`, `check:all`, `build`, `docs:build`, `playground:build`), `check-all`'s lanes, which read their commands from `package.json`, the tools `backlog-status` and `story-finish` (`backlog:index`), and other scripts (EN-115, Inventory).

## Decision

1. **`package.json` keeps the scripts a person runs** — `dev`, `build`, `preview`, `docs:dev`, `fix`, `test`, `check:all` — **and those something calls by their name**, each caller found by a search: npm's lifecycle, a Git hook, a workflow of `.github/`, a lane of `check:all`, a tool, another script kept. No other script is added or kept.
2. **The agent's tools have no script**: the agent and the pages call their binary, and `agent-tools.md` lists the binaries. A command for a person at the screen that no one calls (`npx vitest`, `npx tsc --noEmit`, `npx vitepress preview docs`) is written in the pages, not in `package.json`.
3. **`story:finish` and `mutate` stay** although only the agent runs them: each runs its tool under a lock (`heavy-lock --lock merge`, `heavy-lock`), and `.claude/settings.json` allows neither their binary nor `heavy-lock`, which asks at each call (AUD-023). Calling the binary alone would take no lock; the permissions are the Product Owner's.
4. `guard-bash`'s refusal of a missing script says that a tool is called by its binary; it refuses the golden file's writing in a worktree by its Vitest mode, `golden-write`, since `golden:write` is gone.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| A — the scripts of a person and of a caller (chosen) | 26 scripts instead of 52; each one left has a reason that a search shows | the agent's tools are learnt by their binary only |
| B — A, and the lanes' commands moved into `check-all` | 17 scripts | the formatter's and the linters' targets, which `fix` shares with the lanes, written twice, and the criterion tests of F18 and F19 that read them in `package.json` rewritten |
| C — every script kept | nothing to change | the clutter the request names; two names for each tool |

## Consequences

- A script added later needs its caller; `tools/repo-tools/tests/check_all.rs` fails on a lane naming a script that `package.json` lacks.
- `metrics` still counts the removed scripts in the hook's log, since the logs of the runs before EN-115 hold them; the binaries' uses are counted from `tool-uses.jsonl` (EN-102).
- The journal, the retrospectives, the done stories, the ADRs and the validated checklists keep the scripts' names as they were written.

## References

ADR-0073, ADR-0074, ADR-0078, [EN-115](../backlog/stories/E17-F21-EN-115-fewer-npm-scripts.md), `docs/tooling/agent-tools.md`, `docs/tooling/commands.md`
