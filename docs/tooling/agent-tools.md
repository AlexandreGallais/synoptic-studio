# The agent's tools

Small repository scripts that do in one command what the agent used to write by hand at each story. Decided at RET-002 (ADR-0050).

## Why

At the end of E14 the metrics showed where the time went:

- **Writing the command costs more than running it.** 1 147 Python edit scripts took 389 minutes, and most of that was the model writing them.
- **The checks run too often, one after the other.** `check:all` ran at each of 49 pushes (about 44 minutes), and `npm run fix` took 28 s each time.

A tool written once removes the writing. It is tested, linted and named after what it does.

## Tools

| Command                                         | Does                                                                                                                                                                          | Replaces                                         |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| `npm run replace`                               | reads blocks `@@ file`, `<<<`, old text, `===`, new text, `>>>` on standard input and writes nothing unless each old text is found exactly once                               | a Python script of `replace` calls per edit      |
| `npm run backlog:status -- <ID> <status>`       | changes an item's status in its file, its folder's index and its parent's table — the three places `backlog.test.ts` checks                                                   | three hand edits per status change               |
| `npm run story:finish -- <base> <title> <body>` | opens the story's pull request with the `autonomous` label, waits at most three minutes for GitHub's merge, brings the base up to date, deletes the branch here and on GitHub | a waiting loop and four cleaning commands        |
| `npm run golden:write` / `golden:check`         | about 400 results of the library from shapes drawn with a fixed seed, written before a refactor and compared bit for bit after each change (`NAM`, ADR-0049)                  | a probe written for F13                          |
| `npm run activity`                              | the calls running, the latest calls of the main agent and of each subagent, the end of the background commands' output                                                        | reading transcripts to know what a subagent does |
| `npm run epic:check -- <epic> <first day>`      | the full check of what an epic produced, as a table for the retrospective: checks, branches, pull requests, issues, failed calls, calls left running, backlog, records        | a dozen commands at each retrospective           |
| `npm run check:all`                             | the checks in four parallel lanes: 54 s instead of 90 s                                                                                                                       | the checks one after the other                   |

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

| Tool           | Does                                                                                                                                                       | Signal                                                                        | Rust |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | ---- |
| `context:size` | what a session loads before its first word — `AGENTS.md`, `CLAUDE.md`, rules, memory index, skills' descriptions — in characters and an estimate of tokens | the conversation memory filling up fast (planned for E02, the epic after E15) |      |

## Rust

Rust is installed on the machine. The tools above that run once in a while stay in TypeScript: the time goes to writing commands and to the checks, not to the tools. The tools marked **Rust** are the ones that run thousands of times or read large files. They will be written as one separate Rust project of agent tools, with an ADR, once their measured cost justifies a second language: the hooks and the status line first.
