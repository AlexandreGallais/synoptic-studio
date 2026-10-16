# The agent's tools

Small repository scripts that do in one command what the agent used to write by hand at each story. Decided at RET-002 (ADR-0050).

## Why

At the end of E14 the metrics showed where the time went:

- **Writing the command costs more than running it.** 1 147 Python edit scripts took 389 minutes, and most of that was the model writing them.
- **The checks run too often, one after the other.** `check:all` ran at each of 49 pushes (about 44 minutes), and `npm run fix` took 28 s each time.

A tool written once removes the writing. It is tested, linted and named after what it does.

## Tools

| Command                                         | Does                                                                                                                                                                                                                                                                                                                                                                          | Replaces                                                  |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `npm run replace`                               | reads blocks `@@ file`, `<<<`, old text, `===`, new text, `>>>` on standard input and writes nothing unless each old text is found exactly once                                                                                                                                                                                                                               | a Python script of `replace` calls per edit               |
| `npm run backlog:status -- <ID> <status>`       | changes an item's status in its file, its folder's index and its parent's table — the three places `backlog.test.ts` checks; a story's index is generated again rather than edited                                                                                                                                                                                            | three hand edits per status change                        |
| `npm run backlog:index`                         | writes `docs/backlog/stories/README.md` from each story's front matter, epic by epic in the order of the epic's and the features' tables, those they do not list by file name; a Rust tool (`tools/backlog-index/`, EN-049, ADR-0065)                                                                                                                                         | the index edited by hand on every branch                  |
| `npm run story:finish -- <base> <title> <body>` | opens the story's pull request with the `autonomous` label, waits at most three minutes for GitHub's merge, brings the base up to date, deletes the branch here and on GitHub, then moves the rows the story file holds for the logs into them, commits and pushes (`move-rows`, EN-050)                                                                                      | a waiting loop and four cleaning commands                 |
| `npm run golden:write` / `golden:check`         | about 400 results of the library from shapes drawn with a fixed seed, written before a refactor and compared bit for bit after each change (`NAM`, ADR-0049); the pre-push runs `golden:check` whenever the file is written (EN-055)                                                                                                                                          | a probe written for F13                                   |
| `npm run activity`                              | the calls running, the latest calls of the main agent and of each subagent, the end of the background commands' output                                                                                                                                                                                                                                                        | reading transcripts to know what a subagent does          |
| `npm run epic:check -- <epic> <first day>`      | the full check of what an epic produced, as a table for the retrospective: checks — a failed `check:all` named by its failing checks, their lanes and failing tests —, branches, pull requests, issues, failed calls with the worktree isolation's refusals counted apart, calls lost at a force-quit (started over an hour ago, never ended), backlog, records (EN-055)      | a dozen commands at each retrospective                    |
| `npm run check:all`                             | the checks in four parallel lanes: 54 s instead of 90 s                                                                                                                                                                                                                                                                                                                       | the checks one after the other                            |
| heavy lock (`tools/heavy-lock/`, Rust)          | `check:all`, `npm run test`, `golden:check` and `docs:build` take `.git/heavy.lock`, holding their process id: one at a time across every worktree, the others wait and say which process holds it; a lock whose process is gone is taken over; a check started by `check:all` runs inside its lock (ADR-0063, EN-047, [commands](./commands.md#one-heavy-command-at-a-time)) | the lock taken by hand with `mkdir` in the worker's brief |

## Specifications for later

Each tool replaces something the agent writes by hand today. A tool is written when a retrospective's metrics show its signal. The **Rust** column marks the tools that run so often, or read so much, that a compiled program would save time on its own (decided at RET-002).

### Stories and backlog

| Tool                         | Does                                                                                                   | Signal                                                  | Rust |
| ---------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------- | ---- |
| `story:start <ID>`           | branch from the up-to-date feature branch, step file written, status `in-progress` in the three places | the same five commands at each story's start            |      |
| `story:new <type> <feature>` | the next free id, the story file from its template, its rows in the index and the feature's table      | ids and rows corrected by `backlog.test.ts`             |      |
| `story:show <ID>`            | the story's criteria, tasks, feature plan row and linked files, in one short text                      | story, feature and index files read again and again     | yes  |
| `backlog:next`               | the next story of the feature in progress, or the next feature in the Product Owner's order            | the session-state hook not enough at the start of a run | yes  |
| `adr:new <title>`            | the next number, the file from its template, its row in the index                                      | ADR numbers or index rows fixed after a review          |      |
| `journal:add <tags> <text>`  | an entry at the top of the journal, dated, with its tags                                               | journal entries written by hand at each pause           |      |
| `log:feedback`, `log:idea`   | a row in the feedback log or the improvement log, dated, its columns in order                          | rows with a missing column                              |      |

### Commits and pull requests

| Tool                            | Does                                                                                                                       | Signal                                                 | Rust |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ | ---- |
| `commit:task <ID.Tn> <subject>` | the commit with its scope checked against the list, its `Refs` and attribution lines; on a refused commit, the errors only | refused commits (scope, sentence case, missing footer) |      |
| `pr:body`                       | the pull request's body from the branch's commits, stories and evidence                                                    | pull request bodies written by hand                    |      |
| `val:prepare <feature>`         | the guided steps file, its test and the demo page from their templates, filled with the feature's criteria                 | each validation prepared from scratch                  |      |

### Reading code without reading whole files

| Tool                  | Does                                                                                        | Signal                                                         | Rust |
| --------------------- | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------- | ---- |
| `code:outline <path>` | a file's or a folder's exports, their kind, `@see` and first JSDoc line, without the bodies | `cat` and `sed -n` on whole files (1 224 and 907 calls in E14) | yes  |
| `code:find <name>`    | where a function or type is defined, its tests, and every file that uses it                 | chains of `grep`, `ls` and `cat` to follow one name            | yes  |
| `refs:show <REF>`     | a `REF-*` or `DERIV-*` row and whether it is verified, and the functions that cite it       | the bibliography read whole to check one reference             | yes  |

### Checks and tests

| Tool                                    | Does                                                                                                            | Signal                                                | Rust |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ---- |
| `check:changed`                         | fix, lint, types and the tests related to the files changed since the base, errors only: the loop inside a task | `npx vitest` and `npx eslint` on chosen files by hand |      |
| `test:mutate <file> <old> <new> <test>` | applies one change on purpose, runs the test, expects it to fail, restores the file                             | mutation checks done by hand with copies              |      |
| `api:diff`                              | the public exports before and after a change                                                                    | a renamed export noticed late                         |      |

### Pictures and the playground

| Tool                       | Does                                                                                      | Signal                                          | Rust |
| -------------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------- | ---- |
| `svg:draw <shape.json>`    | the SVG file of a shape, with its dashed box, for the guide's pictures                    | pictures written by hand from a probe (REV-002) |      |
| `playground:probe <steps>` | runs guided steps without a browser and prints what each shows: path data, rows, messages | probes written in the scratchpad (REV-002)      |      |

### Hooks, status line and metrics

These run at every tool call or every second, each one starting Node.js. Measured on 2026-10-14: about 55 ms for Node.js to start, 65 ms for `guard-bash` before each shell command, 149 ms for the status line at each refresh. In Rust each would take a few milliseconds.

| Tool                                             | Does                                                                                    | Signal                                                                | Rust |
| ------------------------------------------------ | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ---- |
| status line                                      | the same lines, the transcript read from where it stopped                               | 149 ms each second                                                    | yes  |
| `guard-bash`, `track-tool-call`, `log-tool-call` | the same checks and records                                                             | 65 ms before every shell command; 3 551 commands in E14 (about 4 min) | yes  |
| `transcript:find <text>`                         | the lines of the session's transcript holding an id or a text, with their time and type | the stuck task found by `grep` in a 30 MB transcript                  | yes  |
| `metrics` report                                 | the same report, faster on large transcripts                                            | the report taking more than a few seconds                             | yes  |

### Context

| Tool           | Does                                                                                                                                                       | Signal                                                              | Rust |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | ---- |
| `context:size` | what a session loads before its first word — `AGENTS.md`, `CLAUDE.md`, rules, memory index, skills' descriptions — in characters and an estimate of tokens | the conversation memory filling up fast (E17, F20 the context diet) | yes  |

### Asked by E15 (RET-003)

Each is a story of F18 (E17), with the signal measured in E15.

| Tool                                | Does                                                                            | Signal                                                               | Rust |
| ----------------------------------- | ------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ---- |
| `worktree:new <ID>`                 | a worktree named after its story, on its branch (EN-052)                        | worktrees named `agent-a13cbef…`; 549 commands refused in worktrees  |      |
| `story:finish` from a worktree      | one merge at a time, clean-up only once merged (EN-051)                         | 4 clean-ups by hand, 2 branches deleted with their pull request open |      |
| stories index generated             | the index written from each story's front matter (EN-049)                       | 18 commits rewrote more than 100 of its 181 lines                    | yes  |
| `epic:check` failures               | the failing tests, worktree refusals apart, calls lost at a force-quit (EN-055) | "failed: test" with no name; 549 refusals counted as failures        |      |
| `mutate <files>`                    | mutations in a copy of the repository, caught and surviving (EN-058)            | the runner written again at each audit (F05 to F17)                  |      |
| `val:prepare`                       | the feature's checklist lines, not guided steps (EN-056)                        | guided-steps files written and unused since US-051                   |      |
| a summary of OpenType and CSS Fonts | the fields the project uses, read once                                          | OS/2 read 38 times, CSS Fonts 4 27 times                             |      |

## Rust

Rust is installed on the machine. Decided at RET-003 (Product Owner, 2026-10-16): the agent's tooling moves to Oxc and Rust now, in E17, before E02 — not once each tool's cost justifies it, as RET-002 had planned. The measures of E15 say where Rust pays:

| Where                             | E15's measure                                                      | Rust worth it                                              |
| --------------------------------- | ------------------------------------------------------------------ | ---------------------------------------------------------- |
| lint (ESLint and its plugins)     | 170 s in a loaded `check:all`, about 25 s alone at E14             | yes: oxlint, and the rules it lacks rewritten on Oxc (F19) |
| format (Prettier)                 | 33 s in `check:all`; `npm run fix` 195 × 25 s                      | yes if oxfmt formats as Prettier does (F19's spike)        |
| hooks and status line             | 65 ms per shell command, 149 ms per refresh; 13 127 calls ≈ 14 min | yes                                                        |
| a context tool for each new agent | 24 M tokens per story; inputs read whole                           | yes, measured first (F20)                                  |
| tools run once in a while         | seconds, against minutes of the model writing commands             | no: the model is 93–100 % of each turn                     |

The Rust tools live in this repository, beside the TypeScript, with their own checks (rustfmt, clippy); the library stays dependency-free.

The first is `tools/backlog-index/` (EN-049): a Cargo crate without a crates.io dependency, its tests beside its code, checked by `npm run check:rust` (`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`), part of `check` and `check:all`; its build output `target/` is ignored by Git and by every check tool. SP-021 proposes the lasting layout of the crates.

The second is `tools/heavy-lock/` (EN-047), the heavy lock, run by `check:all`, `test`, `golden:check` and `docs:build` (`cargo run --release`) and checked by `check:rust` too.

The third is `tools/hooks/` (EN-047): the Claude Code hooks written in Rust and what they share — a small JSON reader, the transcript's times, the background tasks of a session, a temporary twin of `status-tasks.ts` until F19 ports the status line onto it. Its binary `limit-agents` refuses a fourth subagent ([agent](./agent.md#hooks)).

The fourth is `tools/shared-files/` (EN-050, ADR-0065 decision 4): the shared files — the backlog's three indexes, the three logs of `docs/process/`, the feature files, the checklists `playground/checklists/epic-*` — are written by the main thread only, at merge. Its binary `refuse-shared-files`, first in the `pre-push` hook, reads the refs pushed and refuses a worker's story branch (`US`, `EN`) whose commits not yet on the remote change one, naming each file and its commit; the story's `done` commit (`Closes: <ID>`) may still set its status in the indexes and the feature's table. The other branches — `develop`, the feature branches, the stories the main thread writes (`SP`, `CHK`, `AUD`, `VAL`, `NAM`, `REV`, `RET`) — pass. Its binary `move-rows <branch>`, run by `story:finish` once the story is merged and its base pulled, moves the rows the story file holds under `## For the main thread`, beneath `### Journal`, `### Improvements` or `### Feedback`, into that log — the journal's at the end of its newest section, the others' after their table's last row — and drops those headings from the story file; `story:finish` then formats, commits and pushes them on the base.
