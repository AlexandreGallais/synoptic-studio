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

Each one is written when a retrospective's metrics show the need.

| Tool           | Does                                                                                                                          | Signal that calls for it                        |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `story:start`  | branch from the up-to-date feature, step file written, status `in-progress`                                                   | the same five commands at each story's start    |
| `commit:task`  | a task's commit from its id and a subject, the `Refs` and attribution lines written                                           | refused commits for a missing footer or scope   |
| `backlog:next` | the next story of the feature in progress, its criteria and its files, as one short text (an MCP server if it is asked often) | index files read again and again in the metrics |
| `refs:add`     | a `REF-*` row added in its order, with its verified date, from a library page                                                 | bibliography rows corrected after review        |

## Rust

Rust is installed on the machine, but the scripts run in less than a second: their own speed does not matter. The time goes to the model writing commands and to the checks, and the checks are other programs. Rust would add a second language, a compiler to the setup and a review of its packages. A separate project of tools written in Rust is kept for when a measured tool's execution, not its writing, takes the time.
