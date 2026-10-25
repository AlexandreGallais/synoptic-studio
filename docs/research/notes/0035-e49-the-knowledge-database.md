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

## The sources

Read through the library on 2026-10-25, recorded in `docs/references.md` (A knowledge database for the agent), each claim of a behavior from two independent sources or one specification:

| Question | Sources | What they settle |
| --- | --- | --- |
| several writers, the journal modes | `REF-SQLITE-WAL`, `REF-SQLITE-LOCKING`, `REF-SQLITE-TRANSACTION` | one writer at a time in every mode; WAL lets readers run beside the writer, on one machine only, with two files beside the database |
| waiting for the other writer | `REF-SQLITE-BUSY-TIMEOUT`, `REF-RUSQLITE-CONNECTION` | a busy timeout waits, then `SQLITE_BUSY`; rusqlite sets 5,000 ms on each new connection |
| a file on a synced or network folder | `REF-SQLITE-OVER-NETWORK`, `REF-SQLITE-CORRUPT` (§2.1), `REF-SQLITE-WAL` (§1) | never the working database there: locks may fail, WAL does not work across machines |
| a clean copy of a database in use | `REF-SQLITE-BACKUP`, `REF-SQLITE-VACUUM`, `REF-SQLITE-CORRUPT` (§1.2), `REF-POSIX-RENAME` | `VACUUM INTO` or the backup API give a consistent snapshot; a plain copy during a transaction may be corrupt; a rename replaces the last snapshot in one step |
| the schema across tool versions | `REF-SQLITE-PRAGMA` | `user_version` holds the schema's number |
| full-text search | `REF-SQLITE-FTS5` | FTS5 over an external content table kept by triggers, ranked by `bm25()`, the `porter unicode61` tokenizer the library already uses |
| rows of a branch never merged | `REF-GIT-BRANCH`, `REF-GIT-WORKTREE` | a branch merged or not is a fact Git gives; every worktree shares one common folder |
| how coding agents load their context | `REF-CC-MEMORY`, `REF-AGENTS-MD`, `REF-CODEX-AGENTS-MD`, `REF-GH-COPILOT-INSTRUCTIONS`, `REF-ANTHROPIC-CONTEXT` | every agent loads its instruction file whole at start (Codex caps it at 32 KiB, Claude Code loads 200 lines or 25 KB of `MEMORY.md`); the rest is read on demand by a tool, "just in time" |

## The database

Technical choices, each from the sources above; what the Product Owner reads or decides is left to the questions.

### Where it lives

- **The file**: `$AGENT_RECORDS_ROOT/<repo>/knowledge/knowledge.sqlite`, beside `library/` and `metrics/` in the records folder (ADR-0081), never in a checkout: every worktree opens the same file by the same variable, whichever branch it is on (`REF-GIT-WORKTREE`), and nothing of it reaches Git.
- **On the machine only**: SQLite is "designed for situations where the data and application coexist on the same machine" (`REF-SQLITE-OVER-NETWORK`); a synced or network folder may break its locks (`REF-SQLITE-CORRUPT`, §2.1) and WAL does not work across machines (`REF-SQLITE-WAL`, §1). The tool refuses to open a working database that sits inside the synced folder, by comparing the two paths.
- **The synced folder** is given by a local setting, a variable in the `env` of the user's Claude Code settings as `AGENT_RECORDS_ROOT` is (the records folder's README, step 1), never in the repository; without it, `snapshot` says what to set and copies nothing.

### Its tables

One generic shape for every kind of knowledge, as the library's store is one shape for every page (`tools/library/src/store.rs`): a document, cut into sections at its headings, searched by FTS5.

| Table | One row per | Columns |
| --- | --- | --- |
| `documents` | a former file or record: a story, a feature, an epic, an ADR, a note, a retrospective, a log row, a memory, a guide page | `id` (`SP-034`, `ADR-0081`, `NOTE-0035`, `JOURNAL-2026-10-25-3`, `MEMORY-feedback-say-the-branch`), `kind`, `title`, `epic`, `feature`, `status`, `date`, `branch`, `state`, `private`, `path` (its file before the move, so that an old link still finds it) |
| `fields` | a front-matter field or a table cell worth filtering | `document`, `key`, `value` |
| `sections` | a part of a document between two headings | `document`, `position`, `heading`, `text` |
| `sections_index` | FTS5, external content on `sections` (`REF-SQLITE-FTS5`, §4.4.3), kept by insert, delete and update triggers, `porter unicode61` as the library | `heading`, `text` |
| `links` | a citation from one document to another (`@see`, an id in the text) | `from`, `to` |
| `calls`, `turns` | a tool call and a turn of the metrics, imported from the hooks' JSONL | the JSONL's fields |

The text is stored whole: a document read again is its sections joined, so the move loses nothing. The schema's number is `PRAGMA user_version` (`REF-SQLITE-PRAGMA`); changes are additive (a table or a column added, never one renamed), so that a worktree whose binary is older still reads the file, and a binary refuses to write to a file whose version is newer than its own, telling the agent to rebuild from its base.

### Two worktrees writing at once

- **WAL** (`REF-SQLITE-WAL`): the reads of one worktree never wait for the other's write, and WAL is set once on the file, persistent (§3.3). The library keeps the rollback journal, being read far more than written; the knowledge database is written at every story's end and by the metrics' import, while other sessions read it.
- **Each write in `BEGIN IMMEDIATE`** (`REF-SQLITE-TRANSACTION`, §2.2): the write lock taken at the start, so a transaction never fails half-way when it upgrades from a read; the library's migration does the same.
- **The busy timeout** rusqlite sets, 5,000 ms (`REF-RUSQLITE-CONNECTION`, `REF-SQLITE-BUSY-TIMEOUT`): a second writer waits for the first; one transaction per command keeps each write in milliseconds, far below it.
- **The hooks keep their JSONL**: a hook records each call in a few milliseconds without opening the database (the hooks' log, EN-104); the metrics are imported into `calls` and `turns` at the session's end and by `metrics`, in one transaction.

### The rows of a branch

Every row carries the `branch` it was written on and a `state`:

| State | When | Read by default |
| --- | --- | --- |
| `draft` | written on a story's or a feature's branch | by that branch only |
| `merged` | its branch merged into `develop`, set by `story:finish` once GitHub has merged, or by `knowledge sync`, which asks Git which branches are merged (`git branch --merged`, `REF-GIT-BRANCH`) | by every branch |
| `dropped` | its branch deleted unmerged by `branches-clean`, which already archives such a branch as a bundle (ADR-0081) | never; `--all` shows them |

Rows of an abandoned branch are kept, marked, as the bundle keeps its commits: what was tried and dropped stays findable, and a query never mixes it with what holds. A worktree reads the `merged` rows and its own branch's drafts, so two stories in flight never see each other's half-written rows.

### A clean snapshot

`knowledge snapshot` writes `VACUUM INTO` a new file of the synced folder (`REF-SQLITE-VACUUM`: a consistent snapshot of a live database, into a file that "must not previously exist"), under a temporary name, then renames it over the last snapshot (`REF-POSIX-RENAME`: the name always refers to the old or the new file, never to half of one). Neither the working file nor its `-wal` is ever copied by the sync service, which may catch it mid-transaction (`REF-SQLITE-CORRUPT`, §1.2; `REF-SQLITE-BACKUP`, §1). The backup API would serve too, but restarts on every write of another connection and may never end (`REF-SQLITE-BACKUP`, §3.1); `VACUUM INTO` is the copy `records-save` already makes of the library.

It runs on request and from a `SessionEnd` hook, whose default budget is 1.5 seconds, raised by the hook's own `timeout` up to 60 (`REF-CC-HOOKS`, read again 2026-10-25); the snapshot's time is measured in its story, as `records-save` was (`docs/tooling/agent-tools.md`, 16 to 41 seconds with the library). How many snapshots are kept is question Q8.

### The tool

A crate `tools/knowledge` on the workspace's `rusqlite` 0.40.2 (`bundled`, reviewed for the library: `docs/tooling/dependencies.md`), binary `target/release/knowledge`, a command line like `page` (ADR-0078), each use recorded as the other tools' (EN-102):

| Command | Does | Example |
| --- | --- | --- |
| `search <words> [--kind --epic --feature --since --branch --all]` | the best sections, BM25-ranked, with their document's id and a snippet | `knowledge search busy timeout --kind note` |
| `show <id> [<heading>]` | a document, or one of its sections, as `page` prints a row | `knowledge show ADR-0081 Decision` |
| `list [--kind --epic --status --since --state]` | the documents matching, one line each | `knowledge list --kind story --epic E02 --status parked` |
| `add <kind> <file or ->` | a document or a log row, written on the current branch as a draft | `knowledge add journal -` |
| `import <paths>` | the files given, moved in as documents, their links kept in `links` | `knowledge import docs/process/journal.md` |
| `sync` | the states of the branches' rows from Git | at `story:finish` and on request |
| `snapshot` | the clean copy into the synced folder | at a session's end |

`page <id>` falls back on `knowledge show <id>` for an id no file holds any more, so that an id read in a commit or a test's title still opens. No check of a pull request reads the database: the tool's tests make their own in a temporary folder, and the CI never has one.

## The features

Cut by what each leaves working: the database first, then the checks freed from the files, then the move, then the pages rewritten; each feature of 2 to 4 stories of at most 3 tasks (ADR-0084), sized as note 0034 §1 (20 + 25 × stories + 45 to 75 per feature's audit, in active minutes).

### The knowledge database

The tool and its file, before anything moves.

| Story | Tasks | It holds |
| --- | --- | --- |
| EN — The tables, opened by two worktrees | T1 tests: the schema made, WAL, `user_version`, two connections writing at once, a newer file refused for writing; T2 the crate; T3 `docs/tooling/knowledge.md` and the records folder's README row | the location, the tables, the concurrency |
| EN — Add, search, show and list, by branch | T1 tests from the table above, the three states; T2 the commands and `sync`; T3 `page`'s fallback | the agent's one call to find a fact |
| EN — A clean snapshot into the synced folder | T1 tests: a copy while another connection writes, the rename over the last one, the setting missing, the working file refused inside the synced folder; T2 `snapshot` and the `SessionEnd` hook; T3 its time measured, `records-save` taking it | the copy the Product Owner keeps |

3 stories: 25 × 3 + 45 to 75 = 120 to 150 minutes (the epic's spike, 20, counted once in the epic's total).

### The checks freed from the files

What reads a finished item or a log is rewritten while the files are still there, so that the move breaks nothing.

| Story | Tasks | It holds |
| --- | --- | --- |
| EN — The `@see` to stories pointed at the rules | T1 a lint test refusing a `@see` to a backlog file; T2 the 288 `@see` of 190 playground files, to 50 stories, pointed at the domain sections those stories built (none in `src/`); a rule found in a story and not in `docs/domain/` stops the story, a question for the Product Owner | the code's links that would die |
| EN — The tests and tools reading finished items | T1 the trace of a done feature's criteria as Q3 decides; T2 `rule-docs` without its branch up to F47, `metrics`, `release` draft and `epic-check` reading the database; T3 their pages | `backlog.test.ts`, `tools/rule-docs`, `tools/metrics`, `tools/release`, `tools/repo-tools` |
| EN — The log rows written into the database | T1 tests: a story's `For the main thread` rows land as drafts, merged with it; T2 `story-finish` and `move-rows` writing them, `shared-files` no longer guarding the logs; T3 `logs.test.ts` removed, its rules as the tables' constraints | the logs only the main thread could write |

3 stories: 75 + 45 to 75 = 120 to 150 minutes.

### The move into the database

| Story | Tasks | It holds |
| --- | --- | --- |
| EN — The finished stories, features and epics moved | T1 the import's tests on copies; T2 the import, the files removed as Q1 decides; T3 the 789 links of `docs/` to them rewritten as ids in code font | 384 stories, 45 features, 5 epics, 333,000 words |
| EN — The history, the logs, the retrospectives and the notes moved | T1 tests; T2 the import and the files removed; T3 the indexes and pages naming them | 120,000 words |
| EN — The decisions and the memory moved | T1 tests; T2 the ADRs as Q4 decides, the memory as Q7 decides, the metrics imported; T3 `MEMORY.md` and the records folder's README | 51,159 and 11,370 words, 32 MB of metrics |

3 stories: 75 + 45 to 75 = 120 to 150 minutes.

### The light docs

The pages left rewritten short, the rules unchanged; the business rules are the Product Owner's, so their rewrite is checked rule by rule by the feature's audit (ADR-0100), each rule's example kept.

| Story | Tasks | It holds |
| --- | --- | --- |
| EN — `shapes.md` and the domain's index as rule and example | T1 the list of rules before, to compare; T2 the page and the index rewritten, settled questions as Q9 decides; T3 `long-page-bullets.test.ts`'s parts and the `@see` anchors | 23,295 words |
| EN — `interaction.md` as rule and example | T1 the list of rules before; T2 the page rewritten; T3 the anchors its 350 `@see` name | 21,064 words |
| EN — The pages kept, rewritten short | T1 conventions, tooling, process and research pages; T2 `references.md` as Q5 decides, the guide as Q6 decides; T3 the site's sidebar | about 150,000 words |
| EN — `AGENTS.md` and the agent's pages pointing to the tool | T1 `agent-pages.test.ts` naming the tool; T2 `AGENTS.md`, `CLAUDE.md`, the brief, the skills' "read first" lists through `knowledge`; T3 the words and tokens before and after (`target/release/metrics`) | the leading indicators of the epic |

4 stories: 100 + 45 to 75 = 145 to 175 minutes.

**The epic**: 4 features, 13 stories, 20 + 25 × 13 + (45 to 75) × 4 = 525 to 645 active minutes, 8.75 to 10.75 hours, against 8 to 10 when it was written (11 stories): the domain's two long pages take a story each, and the checks are a feature of their own so that the move breaks nothing.

## Questions for the Product Owner

Asked at once (ADR-0100, decision 2). Each gives its options without choosing one.

**Q1 — When a finished item leaves the repository.** (a) at its own `done` commit: the feature's table then names ids the repository no longer holds while the feature runs; (b) at its feature's merge into `develop`, in one commit: the feature's files stay readable until it ends; (c) at its epic's end. Example: F42 has 7 backlog files (its spike, 4 stories, its audit, its validation); with (a) its table names ids read through the tool from its first `done` on, with (b) the 7 files leave together at its merge, with (c) they stay until E02's last feature, F47, five features later.

**Q2 — What the public site keeps.** The site (`site.yml`) publishes all of `docs/` today, the 384 finished stories included; once moved they are in Git's history only. (a) nothing of them: the site shows what holds today; (b) a page per epic done, generated from the database at its end: what it delivered, in a few lines; (c) the release notes alone. Example: E02's 13 features, 80,526 words of stories and features today, would be one page of about 600 words with (b).

**Q3 — The trace of a done feature's criteria (ADR-0027).** `backlog.test.ts` checks in the CI that each of the 212 criteria of the 45 done features has a test titled with its id; the CI never has the database. (a) the trace runs once, at the feature's end, from the database, and the CI checks only open features; (b) a file of the criteria alone stays (`docs/backlog/criteria.md`, 212 rows, 7,457 words), the CI keeps the check; (c) no trace for a done feature. Example: a test titled `[F42.AC4]` renamed by mistake fails the CI with (b), is found at no check with (a) or (c).

**Q4 — The ADRs.** 86 files, 51,159 words, a median of 467; 17 `@see ADR-` in the code and the lint checks that the file exists. (a) kept as files, each cut to its decision and its reason, history out; (b) moved, one index row per ADR left (number, title, status, the decision in one line), the lint reading the index; (c) moved, their principle stated on the pages that apply them, the 17 `@see` pointed at those pages. Example: ADR-0081, 413 words, would be about 150 with (a), one row of about 30 with (b), a paragraph of `docs/tooling/agent.md` with (c).

**Q5 — The bibliography's rows.** 591 rows, 40,665 words, 69 a row; 342 `@see REF-` in the code and the lint reads every row's status in the CI, so the rows stay. (a) kept whole; (b) each row cut to its source, its URL, a line of what is used and its status, the reading notes ("Read again …") moved as one document per row. Example: `REF-CC-HOOKS`, about 430 words today, would be about 40 with (b), the rest found by `knowledge show REF-CC-HOOKS`.

**Q6 — The guide pages.** 45 pages, 51,596 words, one per feature or epic, each how to try what it delivered. (a) kept; (b) moved, one short page per area left (drawing, editing, text, the tooling), what the editor does today; (c) moved, the playground's checklists alone guiding a try. Example: `f42-zoom-and-pan.md`, 1,712 words, would be a section of about 200 in an "Editing" page with (b).

**Q7 — The agent's memory.** 63 files, 11,370 words; Claude Code loads the first 200 lines or 25 KB of `MEMORY.md` at every session (1,172 words, 9,248 characters, about 4,000 tokens at the 2.30 characters per token measured in F20) and writes new memories as files of its own folder (`REF-CC-MEMORY`). (a) the 62 memories moved, `MEMORY.md` cut to a few lines telling the agent to search the tool, Claude Code's auto memory left on and its new files imported at each session's end; (b) the auto memory turned off, every memory written through the tool; (c) the files kept where they are, only indexed in the database. Example: with (a) or (b) a session starts with about 200 tokens of memory instead of 4,000, and "say the branch" is found by `knowledge search branch --kind memory`.

**Q8 — The snapshots kept in the synced folder.** (a) one, replaced at each copy; (b) one per day for the last 7 days; (c) one per day for 30 days. Example: for a database of about 40 MB (to measure), (a) uses 40 MB of the synced folder, (b) 280 MB, (c) 1.2 GB.

**Q9 — The settled questions of the domain.** `docs/domain/README.md` holds 7,106 words of settled questions, each a decision with its date, beside the rules of the domain's pages. (a) moved: each rule's text stays in its page, the question and its date found by the tool; (b) one line per question kept (its number, the rule's section, the date), the rest moved. Example: Q21 (the layout box) would be no line with (a), one of about 15 words with (b).

**Q10 — A fact to confirm: is the records folder synced?** It sits under the user's documents folder, which some systems sync to the cloud as a whole. If it is synced, the working database must live elsewhere on the machine (the library's default folder, `~/.local/share/`, is one), the snapshot alone in the synced folder. Example: with the documents folder synced, a write of the main thread and one of a worker could leave the cloud copy torn, which the snapshot is there to prevent.

**To set by the Product Owner, once**: the variable naming the synced folder, in the `env` of the user's Claude Code settings, as `AGENT_RECORDS_ROOT` is; its name is chosen by the first story (`AGENT_KNOWLEDGE_SNAPSHOT` proposed), its value never written in the repository.
