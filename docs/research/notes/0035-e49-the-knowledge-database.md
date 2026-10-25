# 0035 — E49: the knowledge database and the light docs

Spike SP-034, 2026-10-25, the refinement of E49 (ADR-0100, decision 2). The Product Owner's account is in the epic's file and is not asked again: one SQLite database per project in the records folder (`$AGENT_RECORDS_ROOT/<repo>/`, ADR-0081), outside Git, shared by every branch and worktree; the history, the logs, the finished stories, the research, the decisions, the agent's memory and the metrics in it; the docs short and readable, the business rules as rule and example; the checks that read the docs rewritten or removed; a clean snapshot copied into the synced folder given by a local setting, at each session's end and on request; no check of a pull request needs the database. Nothing below is decided where it touches what the Product Owner reads: those points are the questions at the end.

## The inventory

Measured at `develop`'s head of 2026-10-25 with `wc -w` (words split at white space, tables and code included), the generated API (`docs/api/`) left out. "Read by" names who opens the file: **code** (a `@see` of `src/` or `playground/`), **tests** (`tests/*.test.ts`), **tools** (a crate of `tools/`, a hook), **agent** (the agent's own reads), **PO** (the Product Owner, on the site or in the repository). The agent's reads come from the hook's log of tool calls (`tool-calls.jsonl` of the records folder, 39,445 calls since 2026-10-12): 811 `Read` calls, of which about 200 open a page of `docs/`, and the `Bash` calls whose first 80 characters name a folder, a lower bound since the log cuts each command there.

### What the docs hold

| Kind | Files | Words | Read by | Proposed |
| --- | --- | --- | --- | --- |
| finished stories (`backlog/stories`, `status: done`) | 384 | 297,807 | code (288 `@see`: 240 to a `US`, 27 to an `EN`, 19 to a `VAL`, 2 to a `RET`); tools (`page`, `metrics`, `release` draft, `rule-docs`); tests (`backlog.test.ts`); agent (49 `Read`, 169 `Bash`) | moved |
| open stories | 15 | 3,426 | tools (`worktree-new`, `git-sequences`, `run-totals`, `backlog-status`, `backlog-index`, `shared-files`); tests; agent | kept while open, moved when done |
| finished features | 45 | 29,584 | tests (`backlog.test.ts` traces each criterion of a done feature to a test, ADR-0027); tools (`release` draft, `metrics`); agent (14 `Read`, 112 `Bash`) | moved, the criteria's trace a question (Q3) |
| open features | 2 | 772 | tools (`val-prepare`); tests; agent | kept while open |
| epics done | 5 | 5,382 | agent; PO | moved |
| epics not done | 45 | 20,160 | PO (the order of the list); agent | kept |
| backlog indexes (four `README.md`) and templates | 11 | 10,777 | tools; tests; agent | kept |
| history (`process/history.md`) | 1 | 18,748 | agent | moved |
| logs (`journal`, `improvements`, `feedback`, `tool-incidents`) | 4 | 15,355 | tests (`logs.test.ts`); tools (`shared-files`, `story-finish`, `epic-check`); agent (41 `Bash`) | moved |
| retrospectives, their index and template | 7 | 13,854 | agent; PO | moved |
| process pages (`agile.md`, `README.md`) | 2 | 2,046 | agent; PO | kept, short |
| ADRs | 86 | 51,159 | code (17 `@see ADR-`); tools (`local/see-references` checks the file exists, `page`); agent (13 `Read`, 49 `Bash`); PO | a question (Q4) |
| research notes and requests, their indexes | 38 | 72,646 | tests (`long-page-bullets.test.ts` names the folder); agent (10 `Read`, 15 `Bash`) | moved |
| research protocol (`research/README.md`) | 1 | 1,268 | agent | kept, short |
| bibliography (`references.md`) | 1 | 40,665 | code (342 `@see REF-`); tools (`local/see-references` reads every row's status, in the CI); tests (`references.test.ts`); agent (22 `Read`, 42 `Bash`) | kept, its rows' reading notes a question (Q5) |
| derivations | 26 | 39,988 | code (225 `@see DERIV-`); tools (`local/see-references`, `page`); tests (`long-page-bullets.test.ts`); agent (43 `Bash`) | kept |
| domain | 6 | 46,600 | code (350 `@see` to `interaction.md`, 80 to `shapes.md`); tools (`rule-docs`); tests (`long-page-bullets.test.ts`); agent (11 `Read`, 64 `Bash`); PO | kept, rewritten as rule and example |
| guide pages, one per feature or epic | 45 | 51,596 | tools (`rule-docs`, for a feature up to F47); agent (41 `Bash`); PO (each feature's try) | a question (Q6) |
| conventions | 16 | 21,004 | tools (the lint rules name their page; `local/folder-layout`); tests (`exceptions.test.ts`, `folders.test.ts`); agent (10 `Read`, 30 `Bash`) | kept, short |
| tooling | 12 | 28,952 | tests (`agent-pages`, `check-targets`, `package`); tools (`deps`, hook messages); agent (39 `Read`, 22 `Bash`) | kept, short |
| playbook | 7 | 4,968 | agent; PO | kept |
| `AGENTS.md`, `CLAUDE.md` | 2 | 3,156 | the agent, loaded at every session; tests (`agent-pages.test.ts`) | kept, pointing to the tool |
| `.claude/` (agents 2,269, skills 8,846, rules 2,272) | 20 | 13,387 | Claude Code, each loaded by its type or path; tests (`agent-pages`, `agent-models`) | kept (E50 for the shared part) |
| the agent's memory (outside the repository) | 63 | 11,370 | the agent: `MEMORY.md`, 1,172 words, loaded at every session; the 62 others on demand | moved, a question (Q7) |
| metrics (`metrics/*.jsonl` of the records folder) | 2 | 32 MB | tools (`metrics`, `epic-check`, `run-totals`) | moved into tables |

Totals: 776,993 words of Markdown in `docs/` besides the API; the rows marked moved hold 453,376 of them (58 %), the finished stories alone 297,807 (38 %). The rest, 323,617 words, stays before it is shortened; the domain, the bibliography, the derivations and the ADRs are 178,412 of those, the guide pages 51,596.

What it means for the cost: the agent reads the finished stories, the features and the logs most (the `Bash` counts above), mostly by `grep` and `sed` over many files, because a fact may sit in any of 384 stories. That is the read the database replaces by one query.

### The checks that read the docs

| Check | Runs | Reads | Becomes |
| --- | --- | --- | --- |
| `tests/backlog.test.ts` | CI and `check:all` | every backlog file: front matter, indexes, parents' tables, tasks, criteria of done features traced to tests | kept for the open items; the trace of a done feature's criteria a question (Q3) |
| `tests/logs.test.ts` | CI | the four logs: tables, dated cards, no repeated card | removed, its rules made constraints of the database's tables, tested in the tool's crate |
| `tests/long-page-bullets.test.ts` | CI | parts of `shapes.md`, `interaction.md`, two derivations | kept on the pages that stay, its parts renamed when the domain is rewritten |
| `tests/references.test.ts` | CI | `references.md`: one id once, a status per row | kept |
| `tests/exceptions.test.ts`, `tests/folders.test.ts`, `tests/package.test.ts` | CI | `exceptions.md`, `files.md`, `dependencies.md` | kept, pages that stay |
| `tests/agent-pages.test.ts`, `tests/check-targets.test.ts` | CI | `AGENTS.md`, `CLAUDE.md`, `.claude/`, `agent-tools.md`, `commands.md` | kept |
| `local/see-references` (`tools/lint`) | CI and pre-commit | that each `@see` target exists: `REF-*` row, `DERIV-*` file, `ADR-NNNN` file, `docs/…` page | kept; its 288 targets in finished stories repointed before the stories leave (the light docs) |
| `rule-docs` (`check-rule-docs`, commit-msg of a `done` commit) | local | `domain/README.md` Q rows, the feature's `VAL` story, its guide page, up to F47 | its branch up to F47 removed (every feature up to F47 is done); from F48 the `AUD` line kept while the feature is open |
| `shared-files`, `move-rows`, `story-finish` | local, pre-push and `done` | the logs, the indexes, the story's `For the main thread` rows | rewritten: the rows go into the database at the `done` commit, the logs no longer files to guard |
| `epic-check` | local | the logs, the metrics, the CI | reads the database |
| `metrics`, `run-totals` | local | the backlog's file names, the metrics files, the parked stories | `metrics` reads the database; `run-totals` keeps reading the open stories |
| `release` (draft) | local | the front matter of stories and features named by commits' `Refs:` | reads the database for the finished ones; `release publish` in the CI reads pull request bodies only, unchanged |
| `page` | local | an id's page: `REF-` row, ADR, derivation, backlog item | kept for the pages that stay; a finished item read through the new tool |
| `backlog-index`, `backlog-status`, `val-prepare`, `worktree-new`, `git-sequences` | local | the open items | kept |
| `guard-bash`, `library-pages` and other hooks | local | none: their messages name a page | the names updated when a page moves |
