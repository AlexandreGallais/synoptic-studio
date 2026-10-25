# The project's records database

The project's records in one SQLite file, shared by every branch and worktree and kept outside Git (E49, `NOTE-0035`): the knowledge tables first (EN-151), then the backlog — its epics, features, stories, tasks and criteria — in tables of its own, read and written by free SQL (EN-154), then everything else the docs and the records folder held (EN-155): the decisions, the research, the derivations, the bibliography, the business rules, the logs, the retrospectives, the guide, the agent's memories, the metrics, the library's store, the archives, the saves, and what the transcripts hold that serves, the transcripts themselves no longer copied. The database is the agent's working store, shaped for how it searches rather than as human documents: a table or an index is added when a question needs one, registered with its purpose.

## Where it lives

- **The file**: `$AGENT_RECORDS_ROOT/<repo>.sqlite`, directly in the records root (ADR-0081), `<repo>` the last part of `origin`'s address without `.git`, else the main checkout's folder: `knowledge`, `records-save`, `epic-check`, the hooks and the library find it by `git_repository::records_place` (`tools/git-repository/src/records_place.rs`), re-exported by `knowledge::records`. Every worktree finds the same file, its main checkout read from `git rev-parse --git-common-dir`. The former file, `$AGENT_RECORDS_ROOT/<repo>/records.sqlite`, is read while it alone exists, until it is moved.
- **Without the variable** the tool fails and says what to set; a root that does not exist fails too, so that a mistyped path never makes a new folder. The file is made when missing.
- **On the machine only**: SQLite works where the data and the program share a machine (`REF-SQLITE-OVER-NETWORK`), and WAL does not work across machines (`REF-SQLITE-WAL`, §1).

## Its tables

Schema 5, made in one `BEGIN IMMEDIATE` (`tools/knowledge/src/store.rs`); `tables_registry` lists every table and what one row holds (`knowledge sql "SELECT * FROM tables_registry"`):

| Table | One row per | Columns |
| --- | --- | --- |
| `documents` | a former file or record, as one branch wrote it: a story, an ADR, a note, a log row, a memory | `id`, `kind`, `title`, `epic`, `feature`, `status`, `date`, `branch`, `state` (`draft`, `merged` or `dropped`, any other refused), the three `id`, `branch` and `state` the key, `private` (0 or 1), `path` |
| `fields` | a front-matter field or a table cell worth filtering | `document`, `branch`, `state`, `key`, `value`, indexed by key and value, and by document |
| `sections` | a part of a document between two headings | `id`, `document`, `branch`, `state`, `position` (one per document), `heading`, `text` |
| `sections_index` | FTS5 over `sections`, external content, `porter unicode61` as the library (`REF-SQLITE-FTS5`) | `heading`, `text`, kept by the triggers `sections_insert`, `sections_delete` and `sections_update` (§4.4.3) |
| `links` | a citation from one document to another | `"from"`, `branch` and `state` (a document), `"to"` (an id, held by a file or a document), the four once |
| `calls` | a tool call of `tool-calls.jsonl` | its fields under their names ([metrics](./metrics.md#the-record-of-each-call)), `toolUseId` once, a field an older line lacks left null |
| `turns` | a turn of `turns.jsonl` | `session`, `start` (the pair once), `wallMs`, `activeMs`, `tokens`, `outputTokens`, `calls`, `failures`, `commits` |
| `tables_registry` | a table of the file | `name` (once), `purpose`, what one row holds |
| `epics` | an epic | `id` (once), `title`, `status`, `position` (the Product Owner's order, 0 for a dropped one), `topic`, `text` (its former file), `branch`, `state` |
| `features` | a feature | `id` (once), `epic`, `title`, `status`, `position` (its order in its epic), `topic`, `criteria` (the section), `text`, `branch`, `state`, indexed by epic and position |
| `stories` | a story | `id` (once), `epic`, `feature` (empty for an epic's own), `kind` (`EN`, `US`, `AUD`…), `title`, `status`, `points`, `position` (its order in its feature), `topic` (its branch's), `criteria`, `tasks`, `test_card`, `question`, `text`, `branch`, `state`, indexed by feature, epic, status and branch |
| `tasks` | a story's task | `story`, `number` (the pair once), `title`, `done` (0 or 1) |
| `criteria` | a feature's numbered criterion | `id` (`F48.AC1`, once), `feature`, `number`, `text`, `struck` (struck through, 0 or 1), `retired` (why no test proves it any more, empty otherwise) |
| `decisions` | an ADR, `ADR-0081`, edited in place from now on | `id` (once), `title`, `status` (`accepted`, `superseded`…), `date`, `principle` (its decision's first sentence), `text` |
| `notes` | a research note `NOTE-0035`, a request `REQUEST-0004`, or a page of the research `PAGE-research` | `id` (once), `kind` (`note`, `request`, `page`), `title`, `status`, `date`, `text` |
| `derivations` | a derivation, `DERIV-text-box` | `id` (once), `title`, `used_by`, `text` |
| `sources` | a source of the bibliography, `REF-SQLITE-WAL` | `id` (once), `title`, `url`, `used_for`, `status` (`verified` or `unverified`), `verified` (the day read), `topic` (its former part), `notes` (what was read in it), indexed by topic |
| `rules` | a business rule, a bullet of a domain page | `id` (`RULE-<page>-<anchor>-<name>`, once), `page`, `section` (its part's id, `RULE-shapes-2-corner-radius`), `name`, `rule`, `example` (its last sentence), `position`, indexed by section |
| `journal` | an entry of the journal | `id`, `date`, `session` (its heading), `tag` (`run`, `decision`…), `text`, the date, session and text once |
| `improvements` | an idea, an irritant or a lesson | `id`, `date`, `origin`, `item`, `kind`, the date and item once |
| `feedback` | a critique of the agent's work | `id`, `date`, `happened`, `should_have`, `prevented_by`, the date and what happened once |
| `incidents` | a tool incident (ADR-0085) | `id`, `date`, `tool` (the tool and what for), `used`, `failed`, `instead`, the date and tool once |
| `retrospectives` | an epic's retrospective, `RETRO-E17` | `id` (once), `epic`, `title`, `text` |
| `memories` | a memory of Claude Code's memory folder for the project, private | `name` (once), `description`, `type`, `private` (1), `modified`, `text` |
| `tool_uses` | a use of an agent tool, of `tool-uses.jsonl` | `tool`, `subcommand`, `worktree`, `start`, `durationMs`, `exitCode`, the four first once |
| `archives` | a Git bundle of a branch archived | `name` (once), `size`, `modified` (seconds), `bytes` |
| `saves` | a save of the records | `at`, the instant, once |
| `sessions` | a session's transcript | `id` (once), `start`, `end`, `wallMs`, `inputTokens` (cached ones included), `outputTokens`, `messages`, `commands` |
| `messages` | a message the Product Owner typed, verbatim, private | `id` (the line's `uuid`), `session`, `time`, `text`, indexed by session and time |
| `commands` | a Bash command of a transcript, the main thread's or an agent's | `id` (the tool use's), `session`, `agent` (`main` or the agent's file), `time`, `command`, `description` |
| `transcript_files` | a transcript read | `path` (once), `size`, `modified`: one unchanged is not read again |
| `pages`, `chunks`, `project_uses` | the library's store (`tools/library`), made by `library::store::make_library_tables` | as the library writes them; the library opens the records by default |

A row of the backlog's tables, or of a table the agent adds with a `branch` and a `state` column, belongs to the branch whose work it is: written without a branch, `knowledge sql` gives it the branch checked out, and its state is `draft` until that branch is merged (`story-finish`, `branches-clean`, `branch-state`), `dropped` when the branch is deleted without its merge. Unlike a document, the row is one per id, read by every branch at once: a status set on a story branch is the status everywhere.

A document deleted takes its fields, sections and links with it, never another branch's row of its id nor its branch's row of another state (`ON DELETE CASCADE` on the three, `PRAGMA foreign_keys` set on each connection), its sections leaving the index by the trigger; its state changed, they follow it (`ON UPDATE CASCADE`, `REF-SQLITE-FOREIGN-KEYS`, §4.3).

## Two worktrees at once

- **WAL** (`REF-SQLITE-WAL`): set on the file at its first opening, persistent (§3.3), so that a read never waits for another worktree's write. The switch needs the file alone: while another connection wrote, SQLite refused it at once, without the busy timeout, as EN-152 measured (met by four connections opening a new file at once, 1 run in 25), no source read saying so; the opening then waits for that write by a `BEGIN IMMEDIATE`, reads the mode again and switches once more. That switch failing, or returning another mode, as it does when it cannot change it (`REF-SQLITE-PRAGMA`), fails the opening: `<file>: the file stays in the delete journal mode, WAL refused`.
- **Each write in `BEGIN IMMEDIATE`** (`REF-SQLITE-TRANSACTION`, §2.2): `knowledge::store::write` takes the write lock before its first statement, commits when the work succeeds and rolls it back whole otherwise.
- **The busy timeout** rusqlite sets on each connection, 5,000 ms (`REF-RUSQLITE-CONNECTION`, `REF-SQLITE-BUSY-TIMEOUT`): a second writer waits for the first; several connections opening a new file at once make one schema, the first in its transaction while the others wait.

## The schema's number

`PRAGMA user_version` holds the schema's number (`REF-SQLITE-PRAGMA`), `SCHEMA_VERSION` in the code. A file of an older number is brought up to it at opening, each migration of `MIGRATIONS` from its number on, a new file running them all; changes add a table, a column or a wider key, never rename nor drop a column, so that a worktree whose binary is older still reads a newer file. Such a binary refuses to write to it — `the file's schema is 6, newer than this binary's 5: rebuild the tools from your base, target/release/git-sequences sync` — the number read once the lock is held.

| Schema | Story | Change |
| --- | --- | --- |
| 1 | EN-151 | the tables of note 0035, a document keyed by its id |
| 2 | EN-152 | a document keyed by its id and its branch, so that a branch writes its draft of an id another branch holds; its fields, sections and links take its branch. The four tables rebuilt by SQLite's procedure for a changed key, with the foreign keys off: new tables made and filled, the old dropped, the new renamed (`REF-SQLITE-ALTER-TABLE`, §8), each row and each section's rowid kept, so that the index still points at its sections |
| 3 | AUD-050 | a document keyed by its id, its branch and its state, so that a branch writes a new draft of an id while its merged row of it stays read by every other branch; its fields, sections and links take its state and follow it (`ON UPDATE CASCADE`, `REF-SQLITE-FOREIGN-KEYS`), the four tables rebuilt as for schema 2. A binary of schema 2 still reads the file, but for an id a branch holds both merged and as a draft, whose sections it reads mixed |
| 4 | EN-154 | the backlog's tables, `epics`, `features`, `stories`, `tasks` and `criteria`, with their indexes, and `tables_registry`, every table listed: tables and indexes added, nothing rebuilt |
| 5 | EN-155 | the tables of what moved out of the Markdown and the records folder — `decisions`, `notes`, `derivations`, `sources`, `rules`, `journal`, `improvements`, `feedback`, `incidents`, `retrospectives`, `memories`, `tool_uses`, `archives`, `saves`, `sessions`, `messages`, `commands`, `transcript_files` — and the library's `pages`, `chunks` and `project_uses` with its view `uses`, made by the library's own function after the batch: tables and indexes added, nothing rebuilt |

## The commands

Called by the binary from the checkout's root, `target/release/knowledge`, each use recorded as the other tools' (`tool_use::run`, EN-102, [metrics](./metrics.md#the-uses-of-each-tool)); an error goes to the error output, exit 1, a wrong call printing the usage.

| Command | Does | Example |
| --- | --- | --- |
| `init` | opens the file, made or brought up to the schema, and prints its path and schema, `, newer than this binary's 5: read only` after a newer one | `knowledge init` |
| `path` | prints the file, opening nothing | `knowledge path` |
| `add <kind> <file or ->` | writes a document or a log row, read from the file or the standard input (`-`), as a `draft` of the branch checked out, and prints `<id> added on <branch>, a draft`; a draft of the same id on that branch is replaced, its merged row kept, read by the other branches until this draft is merged. Refused on a detached head, for a kind other than one word of lower-case letters (`story`, `log-row`) and for an empty text | `knowledge add journal -` |
| `search <words> [<filters>]` | the 8 best sections, ranked by `bm25()` (`REF-SQLITE-FTS5`): those holding every word, else those holding any, each word quoted, by the library's own search (`library::store::every_word_else_any`); each printed `<id> › <heading>`, then its snippet on one indented line, the words found between `[` and `]` | `knowledge search busy timeout --kind note` |
| `show <id> [<heading>] [<filters>]` | the document's text as it was added; with a heading, its sections under it, each after `<id>:<line>`, as `page` prints a section; exit 1 when the branch reads no document of the id, or no heading of it | `knowledge show EN-152 Tasks` |
| `list [<filters>]` | one line per document, by id: `<id> [<kind>, <status>, <date>] <title> — <state> on <branch>`, an empty status left out | `knowledge list --kind story --epic E49` |
| `sync` | marks `merged` the drafts of `develop` and of every branch a merge commit brought into it ([what a branch reads](#what-a-branch-reads)), and prints `<n> rows marked merged, of the <m> branches merged into develop` | `knowledge sync` |
| `snapshot` | writes the day's copy of the file into the synced folder, the last seven days kept, and prints `<repo>-<day>.sqlite written into the folder AGENT_KNOWLEDGE_SNAPSHOT names: <n> kept, <m> removed` ([the snapshot](#the-snapshot)) | `knowledge snapshot` |
| `sql "<statement>" [--purpose "<text>"]` | one statement ([free SQL](#free-sql)): a reading prints its column names, then a row a line, its cells joined by tabs, a tab, a line end and a backslash escaped; a write runs in `BEGIN IMMEDIATE` and prints `<n> rows changed`, a table it creates registered with its purpose | `knowledge sql "SELECT id, status FROM stories WHERE feature = 'F48' ORDER BY position"` |
| `import-backlog <folder>` | reads the backlog's files — `epics/`, `features/`, `stories/`, the epics' index for their order — into the backlog's tables and each item's text into a merged document, and prints the count of each; run again, each item's rows are replaced, a criterion's `retired` kept | `knowledge import-backlog docs/backlog` |
| `export-criteria <file or ->` | writes the criteria of the done features but those struck through, one JSON object a line, `{ "id": "F48.AC1" }`, `"retired"` and its reason for one no test needs to prove: the data file `tests/backlog.test.ts` traces to the tests, which the CI reads without the records (ADR-0027; E49, Decisions, Q3) | `knowledge export-criteria tests/data/criteria.json` |
| `check-ids <paths>` | every id the text files under the paths cite that the records should hold and lack, `<path>:<line>: <id> is not in the records`, exit 1: a backlog id missing from its table, or an id of a kind the records hold documents of; a business rule `RULE-…` read in `rules` too; an id of a kind they hold nothing of, a placeholder of zeros (`US-000`, `F00`) and the tests and fixtures are not read. `story-finish` runs it on a change before its pull request into `develop` | `knowledge check-ids docs playground src` |
| `link-ids <paths>` | writes as its id each link to a backlog file, `EN-151` becoming `EN-151` (its text kept when it says more: `the audit (AUD-050)`), and each bare path to one; a link to an index or a folder of the backlog keeps its text; reads the id from the file's name, no records needed | `knowledge link-ids docs playground src` |
| `branch-state <branch> merged\|dropped` | marks the branch's drafts, of the documents and of every table with a `branch` and a `state`, merged or dropped, as `story-finish` and `branches-clean` do | `knowledge branch-state chore/en-160-x dropped` |
| `import-decisions <folder>` | reads the ADRs, `NNNN-*.md`, into `decisions` and each into a merged document `ADR-NNNN`, the folder's index the page `PAGE-adr` ([the move](#the-move-into-the-records)); every import prints `<n> rows and <m> documents imported from <source>`, and may run again, a row replaced or kept, never doubled | `knowledge import-decisions docs/adr` |
| `import-research <folder>` | its notes `notes/NNNN-*.md` as `NOTE-NNNN`, its requests `requests/NNNN-*.md` as `REQUEST-NNNN`, its protocol and the request's template as pages, into `notes` and documents | `knowledge import-research docs/research` |
| `import-derivations <folder>` | each derivation `<name>.md` as `DERIV-<name>` into `derivations` and a document, the index `PAGE-derivations` | `knowledge import-derivations docs/derivations` |
| `import-sources <file>` | each row of the bibliography into `sources` and a document `REF-…`, which `page REF-…` falls back on; the page's opening `PAGE-references` | `knowledge import-sources docs/references.md` |
| `import-rules <folder>` | each bullet of the domain's pages into `rules`, each part of a page a document `RULE-<page>-<anchor>`, the page's opening `RULE-<page>`, the path recorded `docs/domain/<file>` whatever folder they are read from (a version of Git's, `git archive`) | `knowledge import-rules docs/domain` |
| `import-logs <folder>` | the journal, the improvements, the feedback and the tool incidents into their tables, each log a document `LOG-<name>`, the history's too; the retrospectives `RETRO-<epic>` into `retrospectives`; the process's pages that stay left alone | `knowledge import-logs docs/process` |
| `import-guide <folder>` | each page of the guide a document `GUIDE-<name>`, its index and template pages | `knowledge import-guide docs/guide` |
| `import-memories <folder>` | the memories of Claude Code's memory folder of the project into `memories` and private documents `MEMORY-<name>`, the files read only | `knowledge import-memories <the memory folder>` |
| `import-metrics <folder>` | `tool-calls.jsonl`, `turns.jsonl` and `tool-uses.jsonl` of a folder into `calls`, `turns` and `tool_uses`, a call without its `toolUseId` keyed `<session>-<time>` | `knowledge import-metrics .git/metrics` |
| `import-library <file>` | another library's file, opened read-only, into the library's tables of the records, a page the records read later kept | `knowledge import-library <the former library file>` |
| `import-archives <folder>` | each file of a folder of bundles into `archives`, its bytes a blob | `knowledge import-archives <the former archives folder>` |
| `import-saves <file>` | each instant of a file, one a line as `last-save.txt` held it, into `saves` | `knowledge import-saves <the former last-save.txt>` |
| `import-transcripts <folder>` | Claude Code's transcripts of the project, `<session>.jsonl` and `<session>/subagents/*.jsonl`, each changed since read: a session's instants and tokens into `sessions`, its Bash commands into `commands`, the messages the Product Owner typed into `messages` | `knowledge import-transcripts <Claude Code's folder of the project>` |

The filters, the same for `search`, `show` and `list`, each followed by its value but `--all`:

| Filter | Keeps |
| --- | --- |
| `--kind`, `--epic`, `--feature`, `--status` | the documents of that kind, epic, feature or status |
| `--since <date>` | the documents dated that day or later |
| `--branch <branch>` | the rows written on that branch, read as that branch reads them, its drafts included |
| `--state <state>` | the rows of that state, `draft`, `merged` or `dropped`; `dropped` reads them as `--all` does |
| `--all` | the `dropped` rows too |

### Free SQL

`knowledge sql` runs one statement, for the agent's own questions and its own tables (EN-154). What the statement does is read from SQLite, by the authorizer it calls for each action while it compiles the statement and while it runs, a change of the schema compiling it again (`REF-SQLITE-AUTHORIZER`); never from its text, but for `ALTER TABLE`, whose kind of change the authorizer does not name:

- **Allowed**: any reading; `CREATE TABLE`, `CREATE INDEX`, `CREATE VIEW`, a trigger on the agent's tables; `INSERT`, `UPDATE`, `DELETE` on any table but the documents'; `ALTER TABLE … ADD`, on a core table too; `DROP` of a table, index, view or trigger of the agent's own; a `PRAGMA` that reads.
- **Refused**, naming why: `DROP` of a core table — `documents`, `fields`, `sections`, `sections_index` and its own tables, `links`, `calls`, `turns`, `tables_registry`, `epics`, `features`, `stories`, `tasks`, `criteria`, the tables of schema 5 and the library's with its index's own (`chunks_…`) — or of an index or trigger on one; any other `ALTER TABLE` of a core table; a write to `documents`, `fields`, `sections`, `sections_index`, `links`, or the library's `pages` and `chunks`, which `add`, `import-backlog` and `sync` keep with the full-text index; a trigger on those; a `PRAGMA` that sets; `ATTACH`, `DETACH`, a transaction or a savepoint, each write being its own `BEGIN IMMEDIATE`; more than one statement.
- **A new table** needs `--purpose`, its row of `tables_registry` written with it, struck when the table is dropped; a row written without a branch takes the branch checked out, in every table with a `branch` and a `state` column.

### The backlog

The backlog's files were imported once by `import-backlog` on 2026-10-25 (EN-154), then removed but `docs/backlog/README.md`, the order of the epics and how work is planned: 50 epics, 48 features, 403 stories, 220 criteria and 989 tasks, in 0.55 s. Since then the records alone hold it:

- **A status**: `target/release/backlog-status <ID> <status>`, one row.
- **A story's branch**: `worktree-new <ID>` reads its kind, feature and topic; `git-sequences sync` and `state`, and the `check-handback` hook, its feature branch.
- **A story merged**: `story-finish` marks its branch's rows merged and the story `done`; `branches-clean` marks a deleted branch's rows merged when a branch of `origin` holds its tip, else dropped (AUD-050).
- **What reads it besides**: `epic-check` (an epic's items neither done nor dropped), `run-totals should-stop` (the parked stories), the `session-state` hook (each feature started), `metrics` (a story's feature and epic), `release notes` (a story's title), `val-prepare` (a feature's criteria).
- **The CI** reads none of it: the criteria of the done features are exported to `tests/data/criteria.json`; the `@see` of the code cite a backlog, `REF-`, `DERIV-`, `RULE-` or `ADR-` id by its form alone (`local/see-references`), and `check-ids` checks the ids before a pull request into `develop`.

### The move into the records

On 2026-10-25 (EN-155) the rest of what the agent reads moved in by the import commands, then its files left the repository; measured on that day, load average 9 to 17:

| Source | Rows | Documents | Time |
| --- | --- | --- | --- |
| `docs/adr/`, then the 15 ADRs removed into ADR-0101's record, read from Git's history | 101 decisions | 102 | 0.11 s |
| `docs/research/` | 40 notes, requests and pages | 40 | 0.13 s |
| `docs/derivations/` | 25 derivations | 26 | 0.09 s |
| `docs/references.md` | 595 sources | 596 | 0.13 s |
| `docs/domain/`, as EN-156 rewrote it (`git archive` of its commit), the pages removed by EN-157 | 537 rules | 85 | 0.08 s |
| `docs/process/` and its retrospectives | 187 journal entries, 73 improvements, 10 critiques, 4 incidents, 5 retrospectives | 12 | 0.08 s |
| `docs/guide/` | — | 45 | 0.10 s |
| the memory folder of Claude Code for the project | 71 memories, private | 71 | 0.10 s |
| the records folder's `metrics/`, then `.git/metrics/` | 52,021 calls, 30 turns, 150,244 tool uses | — | 0.62 s, 1.01 s |
| the library's file, then the records folder's copy | 653 pages, 16,083 sections, 2,857 uses | — | 28 s |
| the records folder's `archives/` and `last-save.txt` | 1 bundle, 1 save | — | 0.08 s, 0.05 s |
| the records folder's copy of the transcripts, 2.5 GB, then Claude Code's folder | 18 sessions, 46,360 commands, 314 messages | 1,040 files read | 13 s, 9 s |

The file then held 1,481 documents, 202 MB; `search corner radius clamp` answered in 42 ms. Three stories removed before the backlog moved were read from Git's history by `import-backlog`, as the 15 ADRs were by `import-decisions`; the ids the moved text cites that no file ever held stay as written in it.

- **`records-save`** no longer copies: it imports the main checkout's `.git/metrics/`, the transcripts changed since their last reading, the memories, and the library of another file while one is named, then writes the save's instant into `saves` and takes the snapshot. Its lines: `metrics: <n> rows`, `transcripts: …`, `memories: …`, `library: …`, each `missing` without its source.
- **The library** opens the records file by default (`hooks::pages::library_file`): `AGENT_LIBRARY_FILE`, else `$AGENT_RECORDS_ROOT/<repo>.sqlite`, else the home's data folder.
- **`epic-check`** reads the last save in `saves`.
- **The citations**: the code cites a rule by its part's id, `@see RULE-shapes-2-corner-radius`, as `link-ids` writes a `@see` of `docs/domain/<page>.md#<anchor>`; the pages were then removed (EN-157), the Product Owner's readable page being `docs/the-editor.md`.

### A document added

- **Its id**: its front matter's `id`; without one, `<KIND>-<date>-<n>`, `n` one more than the highest of that kind and day among every row of the file, whatever its branch or state, so that two branches never make the same id (`JOURNAL-2026-10-25-3`).
- **Its columns**: its title the front matter's `title`, else its first heading's text, else its first line; its `epic`, `feature` and `status` the front matter's, empty without them; its date the front matter's `date`, else the day it is added; `path` the file given, empty for the standard input; `private` 0.
- **Its fields**: every `key: value` line of its front matter (`tools/knowledge/src/front_matter.rs`, `backlog-index`'s function before EN-154).
- **Its sections**: cut before each heading outside a fenced block, by `page`'s functions, the text above the first heading a section of an empty heading; joined, they give the text back, so that `show` prints what was added.
- **Its citations**, in `links`: each id its text cites once, its own left out — `REF-` and capitals (`REF-GIT-BRANCH`), `DERIV-` and lower case (`DERIV-bit`), an epic or a feature (`E49`, `F48`), or two capitals or more, a dash, three digits or more, then dashed numbers (`EN-152`, `ADR-0081`, `JOURNAL-2026-10-25-3`).

### What a branch reads

- **Its own drafts and the merged rows** (research note 0035, The rows of a branch): another branch's drafts are never read but by `--branch`; its draft of an id is read over the merged row of it, so that a branch sees what it changed.
- **The dropped rows** with `--all` alone; `show` of an id then reads its draft, else its merged row, else the dropped row written last.
- **`sync`** marks merged the drafts of the branches merged into `develop`: each replaces the merged row of its id, and of two drafts of one id merged in one `sync`, the one written last stays. A row written on `develop` is merged by the next `sync`.
- **A branch merged** is one whose tip a merge commit of `develop` brought in, as every merge of the project is one ([Git workflow](./git-workflow.md)): its tip, among the branches `git branch --merged develop --format=%(refname:short) %(objectname)` lists (`REF-GIT-BRANCH`, `REF-GIT-FOR-EACH-REF`), is a parent other than the first of a commit `git log --merges --format=%P develop` prints (`REF-GIT-LOG`), the branch that merge brought in (`REF-GIT-REV-PARSE`, `<commit>^-`). A tip only reachable from `develop` is not enough: a branch made from `develop` without a commit of its own is listed by `--merged` too, and its drafts are not merged (AUD-050). A branch made at the tip of a branch already merged, before a commit of its own, still reads as merged.

Measured on 2026-10-25 with `target/release/bench --runs 20`, a records root of the temporary folder holding three documents, a story, research note 0035 and a log row (the file 140 KB), load average 9 to 17: `init` 27 ms on a file already made (28 ms on a new one, then 68 KB), `list` 27 ms, `search busy timeout` 33 ms, `show EN-152` and `show EN-152 Tasks` 34 ms, `add` of the story 34 ms, `sync` 39 ms, its Git call included (59 ms since AUD-050, with its second Git call, `git log --merges`, measured the same way on a file of one document, load average 15); most of each is Git's call for the main checkout.

### The fallback of `page`

`target/release/page <id>` falls back on `knowledge show <id>` for an id no page holds — its kind unknown to `page` (`NOTE-`, `JOURNAL-`) or its file gone — so that an id read in a commit or a test's title still opens once its file has moved into the database (EN-152). It runs the `knowledge` binary beside its own, its error output left out: when the records hold no document of it either, `page` prints its own message, exit 1. A page found is printed without asking `knowledge`, 3 ms; a fallback costs the `knowledge` call, 28 ms (`tools/page/src/fallback.rs`).

## The snapshot

The copy kept in a synced folder (research note 0035, A clean snapshot; E49, Decisions, Q8), by `knowledge snapshot` (`tools/knowledge/src/snapshot.rs`, EN-153):

- **The folder**: the one `AGENT_KNOWLEDGE_SNAPSHOT` names, set once in the `env` of the user's `~/.claude/settings.json` beside `AGENT_RECORDS_ROOT`. Its path names the user's account, so it is never written in the repository, a test, a commit or a log, and no message of the tool prints it: each names the variable instead, a path inside an error replaced by `$AGENT_KNOWLEDGE_SNAPSHOT`.
- **The copy**: `VACUUM INTO` a temporary file of that folder, `.<repo>-<pid>.sqlite.tmp` (`REF-SQLITE-VACUUM`: a consistent snapshot of a live database, into a file that does not exist), renamed to the day's name, `<repo>-YYYY-MM-DD.sqlite` (`synoptic-studio-2026-10-25.sqlite`; the former file's `records-YYYY-MM-DD.sqlite`), the day in UTC (`REF-POSIX-RENAME`: the name is the old file or the new, never half of one). Another connection writing meanwhile is not waited for: the copy holds every committed row, never an uncommitted one. The working file and its `-wal` are never copied, a sync service catching them mid-transaction (`REF-SQLITE-CORRUPT`, §1.2); the copy is in the rollback journal, so that reading it makes no `-wal` in the synced folder.
- **Seven days kept**: a second snapshot of the day replaces the first; then the snapshots beyond the seven newest, by the date of their name, are removed, and so is a temporary file over an hour old, the leftover of a copy killed by the hook's timeout. A file another snapshot removed first, two running at once, is no error. Any other file of the folder is left alone.
- **Without the variable**, set or empty, it copies nothing, prints `AGENT_KNOWLEDGE_SNAPSHOT is not set: name the synced folder in the env of ~/.claude/settings.json, beside AGENT_RECORDS_ROOT; nothing copied.` and exits 0, whatever `AGENT_RECORDS_ROOT`, so that a session's end never fails for it. A folder named that does not exist fails, exit 1, and is never made; without a working file yet, it says so and copies nothing.
- **A working file in the synced folder** is refused by every command, `path` included, before any message that names a path, a missing records root's too: in the folder itself or in a folder below it, links resolved and a folder not made yet read as written, the message naming that relation by the two variables, `the records database lies in a folder inside the one AGENT_KNOWLEDGE_SNAPSHOT names, which a sync service copies mid-write: move AGENT_RECORDS_ROOT out of it, or name another synced folder.`
- **When**: on request, at each session's end by the `SessionEnd` hook ([hooks](./agent.md#hooks)), and by `records-save` after its copies, which prints the same line.

**Its time**, measured on 2026-10-25 with `target/release/bench --runs 10`, `Git`'s call for the main checkout included: `AGENT_KNOWLEDGE_SNAPSHOT` is not set on this machine yet and no real `records.sqlite` exists, so the synced folder was one of the temporary folder and the working file a database made for the measure, the repository's 767 Markdown pages added by `knowledge add` (12 MB): 52 ms, load average 12 to 19; the same with the library's tables copied in, as the library will join the file (82 MB): 201 ms, load average 11 to 18. The `SessionEnd` hook has 1.5 seconds by default, raised by its handler's `timeout` up to 60 (`REF-CC-HOOKS`): its `timeout` is 30 seconds, over the 1.8 s of the 201 ms nine times slower, as a loaded machine ran tests (EN-129), so that a synced folder slower to write than the temporary one, or a file grown to the 100 MB the feature expects, still ends within it.

## Its tests

The crate's tests (`tools/knowledge/tests/`) make every file in a folder of the system's temporary folder, the binary's run with `AGENT_RECORDS_ROOT` and `AGENT_KNOWLEDGE_SNAPSHOT` set to such a folder or removed and `HOME` pointed there: no test reads the real records folder nor writes to the real synced folder, and no check of a pull request needs the database. `page`'s tests run it without `AGENT_RECORDS_ROOT` nor a `GIT_*` variable, its fallback's against a script standing for `knowledge` (`tools/page/tests/binary.rs`). The tools that read the records through `knowledge sql` (`knowledge-call`, EN-154) are tested the same way: a copy of their binary beside a script standing for `knowledge`, which prints the rows a test needs or fails as records without `AGENT_RECORDS_ROOT` do; `story-finish`, `branches-clean`, `backlog-status` and `epic-check`, which link the crate, against records of the test's own folder.
