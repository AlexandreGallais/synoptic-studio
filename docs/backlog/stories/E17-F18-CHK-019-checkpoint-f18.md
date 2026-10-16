---
id: CHK-019
epic: E17
feature: F18
title: Checkpoint in the middle of F18
status: done
points: 1
---

# E17 · F18 · CHK-019 — Checkpoint in the middle of F18

Bring the docs and methods up to date in the middle of F18, with a light evolvability check (ADR-0028).

## Acceptance criteria

- Given the stories merged so far, when the checkpoint ends, then `docs/tooling/`, the run skill and `AGENTS.md` describe the tools as built, and the improvement log holds what was found.

## Tasks

One task = one commit, referenced as `CHK-019.Tn`.

- [ ] T1 — Docs and rules in step; light evolvability check (1 h)

## Result

### Docs and rules brought in step

| Where                          | Change                                                                                                                                                                                                                                                                                                               |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| worker brief                   | the worktree made by `worktree:new` (branch, `node_modules`, its own hooks); every Bash call starts with `cd "<worktree>" \|\| exit 9`; the brief read inside the worktree, never the main checkout's; every new script in Rust; a `git push` never wrapped in a hand lock                                           |
| run skill                      | the worker's prompt points to the brief inside its worktree; the main checkout stays on `develop`, so a rule read there is `develop`'s; a story's worktree made by `worktree:new`; background Git commands of the main thread start with `cd` and check the branch; rebase conflicts resolved in the file and diffed |
| story skill                    | a new script, hook or agent tool in Rust                                                                                                                                                                                                                                                                             |
| `AGENTS.md`                    | a new script, hook or tool in TypeScript under **Never**; `worktree:new`, `mutate`, `val:prepare` among the agent tools                                                                                                                                                                                              |
| `.claude/rules/tooling.md`     | the Rust rule, with `check:rust`                                                                                                                                                                                                                                                                                     |
| `docs/tooling/agent.md`        | parallel sessions use `worktree:new`; new hooks in Rust                                                                                                                                                                                                                                                              |
| `docs/tooling/commands.md`     | `worktree:new`, `mutate`, `val:prepare` listed; the hand lock never taken                                                                                                                                                                                                                                            |
| `docs/tooling/agent-tools.md`  | the Rust rule; the crates listed in one place, `mutate` and `rule-docs` included                                                                                                                                                                                                                                     |
| `docs/process/improvements.md` | the rows `move-rows` had written as a second table and as bullets put back as rows of the one table; tonight's findings added                                                                                                                                                                                        |
| `docs/process/feedback.md`     | four slips of the main thread                                                                                                                                                                                                                                                                                        |

Already true, unchanged: `CLAUDE.md` (the heavy commands take the lock themselves, never by hand; worktrees named by `worktree:new`), `story:finish` from the story's worktree under the merge lock (agent-tools, commands, run skill), `ci.md`, `git-workflow.md` and `metrics.md`.

### Test cards written so far

| Story  | Card                                                                                                                        | State                     |
| ------ | --------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| EN-056 | `?checklist=E15`: the checklist is a column right of the drawing, never over it (section "What the Product Owner looks at") | merged                    |
| US-061 | the font list held greyed while the default font is read again, on a computer without Inter (section "Product Owner test")  | in progress, merged later |

### Light evolvability check

1. **Size**: `src/` holds 588 source files and 373 test files (model 252, geometry 118, math 111, io 100, render 7); the tooling counts 27 TypeScript scripts, 9 TypeScript hooks and 8 Rust crates (about 6 600 lines of Rust). No layer is a package yet: nothing yet.
2. **Tools**: Stryker's trigger (stryker-js issue 6210) has not happened, and `npm run mutate` (EN-058) now covers the need: the row of `docs/tooling/dependencies.md` to close at RET-004. `size-limit` and API Extractor: no npm publication yet, nothing yet.
3. **Fitness functions**: the modules per folder are counted (EN-061); the shared files are checked at push (EN-050). Still only written: "every new script in Rust" — a test could refuse a new `.ts` file under `scripts/` or `.claude/hooks/`: a story for F19.
4. **Patterns**: `agent-tools.md` described each crate in a numbered paragraph ("the first", "the second"…), out of order once two stories added one at once: now one list. `status-tasks.ts` and its Rust twin in `tools/hooks` stay until F19.
5. **Agent configuration**: `AGENTS.md` and `CLAUDE.md` together 153 lines, under 200. Recurring mistakes not yet covered by a check: the hand lock, a worker writing in the main checkout, `move-rows` taking any line: rows in the improvement log.
6. **Process**: the main checkout on `develop` during a run served stale rules to the workers; the brief is now read in the worktree. Cadences kept.
