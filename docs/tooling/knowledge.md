# The project's records database

The project's records in one SQLite file, shared by every branch and worktree and kept outside Git (E49, [research note 0035](../research/notes/0035-e49-the-knowledge-database.md)): the knowledge tables first (EN-151); the library's store, the metrics, the saves and the archives join it with the move into the database, the raw transcripts staying files it indexes.

## Where it lives

- **The file**: `$AGENT_RECORDS_ROOT/<repo>/records.sqlite`, in the project's records folder (ADR-0081), `<repo>` the last part of `origin`'s address without `.git`, else the main checkout's folder, as `records-save` names it: both call `knowledge::records` (`tools/knowledge/src/records.rs`). Every worktree finds the same file, its main checkout read from `git rev-parse --git-common-dir`.
- **Without the variable** the tool fails and says what to set; a root that does not exist fails too, so that a mistyped path never makes a new folder. The project's folder and its file are made when missing.
- **On the machine only**: SQLite works where the data and the program share a machine (`REF-SQLITE-OVER-NETWORK`), and WAL does not work across machines (`REF-SQLITE-WAL`, §1).

## Its tables

Schema 2, made in one `BEGIN IMMEDIATE` (`tools/knowledge/src/store.rs`):

| Table | One row per | Columns |
| --- | --- | --- |
| `documents` | a former file or record, as one branch wrote it: a story, an ADR, a note, a log row, a memory | `id`, `kind`, `title`, `epic`, `feature`, `status`, `date`, `branch`, the pair `id` and `branch` the key, `state` (`draft`, `merged` or `dropped`, any other refused), `private` (0 or 1), `path` |
| `fields` | a front-matter field or a table cell worth filtering | `document`, `branch`, `key`, `value`, indexed by key and value, and by document |
| `sections` | a part of a document between two headings | `id`, `document`, `branch`, `position` (one per document), `heading`, `text` |
| `sections_index` | FTS5 over `sections`, external content, `porter unicode61` as the library (`REF-SQLITE-FTS5`) | `heading`, `text`, kept by the triggers `sections_insert`, `sections_delete` and `sections_update` (§4.4.3) |
| `links` | a citation from one document to another | `"from"` and `branch` (a document), `"to"` (an id, held by a file or a document), the three once |
| `calls` | a tool call of `tool-calls.jsonl` | its fields under their names ([metrics](./metrics.md#the-record-of-each-call)), `toolUseId` once, a field an older line lacks left null |
| `turns` | a turn of `turns.jsonl` | `session`, `start` (the pair once), `wallMs`, `activeMs`, `tokens`, `outputTokens`, `calls`, `failures`, `commits` |

A document deleted takes its fields, sections and links with it, never another branch's row of its id (`ON DELETE CASCADE` on the pair, `PRAGMA foreign_keys` set on each connection), its sections leaving the index by the trigger.

## Two worktrees at once

- **WAL** (`REF-SQLITE-WAL`): set on the file at its first opening, persistent (§3.3), so that a read never waits for another worktree's write. The switch needs the file alone, and SQLite refuses it at once, without the busy timeout, while another connection writes (§9): the opening then waits for that write by a `BEGIN IMMEDIATE`, reads the mode again and switches once more (EN-152, met by four connections opening a new file at once, 1 run in 25).
- **Each write in `BEGIN IMMEDIATE`** (`REF-SQLITE-TRANSACTION`, §2.2): `knowledge::store::write` takes the write lock before its first statement, commits when the work succeeds and rolls it back whole otherwise.
- **The busy timeout** rusqlite sets on each connection, 5,000 ms (`REF-RUSQLITE-CONNECTION`, `REF-SQLITE-BUSY-TIMEOUT`): a second writer waits for the first; several connections opening a new file at once make one schema, the first in its transaction while the others wait.

## The schema's number

`PRAGMA user_version` holds the schema's number (`REF-SQLITE-PRAGMA`), `SCHEMA_VERSION` in the code. A file of an older number is brought up to it at opening, each migration of `MIGRATIONS` from its number on, a new file running them all; changes add a table, a column or a wider key, never rename nor drop a column, so that a worktree whose binary is older still reads a newer file. Such a binary refuses to write to it — `the file's schema is 3, newer than this binary's 2: rebuild the tools from your base, target/release/git-sequences sync` — the number read once the lock is held.

| Schema | Story | Change |
| --- | --- | --- |
| 1 | EN-151 | the tables of note 0035, a document keyed by its id |
| 2 | EN-152 | a document keyed by its id and its branch, so that a branch writes its draft of an id another branch holds; its fields, sections and links take its branch. The four tables rebuilt by SQLite's procedure for a changed key, with the foreign keys off: new tables made and filled, the old dropped, the new renamed (`REF-SQLITE-ALTER-TABLE`, §8), each row and each section's rowid kept, so that the index still points at its sections |

## The commands

Called by the binary from the checkout's root, `target/release/knowledge`, each use recorded as the other tools' (`tool_use::run`, EN-102, [metrics](./metrics.md#the-uses-of-each-tool)); an error goes to the error output, exit 1, a wrong call printing the usage.

| Command | Does | Example |
| --- | --- | --- |
| `init` | opens the file, made or brought up to the schema, and prints its path and schema, `, newer than this binary's 2: read only` after a newer one | `knowledge init` |
| `path` | prints the file, opening nothing | `knowledge path` |
| `add <kind> <file or ->` | writes a document or a log row, read from the file or the standard input (`-`), as a `draft` of the branch checked out, and prints `<id> added on <branch>, a draft`; a row of the same id on that branch is replaced. Refused on a detached head, for a kind other than one word of lower-case letters (`story`, `log-row`) and for an empty text | `knowledge add journal -` |
| `search <words> [<filters>]` | the 8 best sections, ranked by `bm25()` (`REF-SQLITE-FTS5`): those holding every word, else those holding any, each word quoted as the library's search does; each printed `<id> › <heading>`, then its snippet on one indented line, the words found between `[` and `]` | `knowledge search busy timeout --kind note` |
| `show <id> [<heading>] [<filters>]` | the document's text as it was added; with a heading, its sections under it, each after `<id>:<line>`, as `page` prints a section; exit 1 when the branch reads no document of the id, or no heading of it | `knowledge show EN-152 Tasks` |
| `list [<filters>]` | one line per document, by id: `<id> [<kind>, <status>, <date>] <title> — <state> on <branch>`, an empty status left out | `knowledge list --kind story --epic E49` |
| `sync` | marks `merged` the drafts of every branch `git branch --merged develop` lists (`REF-GIT-BRANCH`), and prints `<n> rows marked merged, of the <m> branches merged into develop` | `knowledge sync` |

The filters, the same for `search`, `show` and `list`, each followed by its value but `--all`:

| Filter | Keeps |
| --- | --- |
| `--kind`, `--epic`, `--feature`, `--status` | the documents of that kind, epic, feature or status |
| `--since <date>` | the documents dated that day or later |
| `--branch <branch>` | the rows written on that branch, read as that branch reads them, its drafts included |
| `--state <state>` | the rows of that state, `draft`, `merged` or `dropped`; `dropped` reads them as `--all` does |
| `--all` | the `dropped` rows too |

### A document added

- **Its id**: its front matter's `id`; without one, `<KIND>-<date>-<n>`, `n` one more than the highest of that kind and day among every row of the file, whatever its branch or state, so that two branches never make the same id (`JOURNAL-2026-10-25-3`).
- **Its columns**: its title the front matter's `title`, else its first heading's text, else its first line; its `epic`, `feature` and `status` the front matter's, empty without them; its date the front matter's `date`, else the day it is added; `path` the file given, empty for the standard input; `private` 0.
- **Its fields**: every `key: value` line of its front matter, read by `backlog-index`'s function.
- **Its sections**: cut before each heading outside a fenced block, by `page`'s functions, the text above the first heading a section of an empty heading; joined, they give the text back, so that `show` prints what was added.
- **Its citations**, in `links`: each id its text cites once, its own left out — `REF-` and capitals (`REF-GIT-BRANCH`), `DERIV-` and lower case (`DERIV-bit`), an epic or a feature (`E49`, `F48`), or two capitals or more, a dash, three digits or more, then dashed numbers (`EN-152`, `ADR-0081`, `JOURNAL-2026-10-25-3`).

### What a branch reads

- **Its own drafts and the merged rows** (research note 0035, The rows of a branch): another branch's drafts are never read but by `--branch`; its draft of an id is read over the merged row of it, so that a branch sees what it changed.
- **The dropped rows** with `--all` alone; `show` of an id then reads its draft, else its merged row, else the dropped row written last.
- **`sync`** marks merged the drafts of the branches merged into `develop`: each replaces the merged row of its id, and of two drafts of one id merged in one `sync`, the one written last stays. A row written on `develop` is merged by the next `sync`.

Measured on 2026-10-25 with `target/release/bench --runs 20`, a records root of the temporary folder holding three documents, a story, research note 0035 and a log row (the file 140 KB), load average 9 to 17: `init` 27 ms on a file already made (28 ms on a new one, then 68 KB), `list` 27 ms, `search busy timeout` 33 ms, `show EN-152` and `show EN-152 Tasks` 34 ms, `add` of the story 34 ms, `sync` 39 ms, its Git call included; most of each is Git's call for the main checkout.

### The fallback of `page`

`target/release/page <id>` falls back on `knowledge show <id>` for an id no page holds — its kind unknown to `page` (`NOTE-`, `JOURNAL-`) or its file gone — so that an id read in a commit or a test's title still opens once its file has moved into the database (EN-152). It runs the `knowledge` binary beside its own, its error output left out: when the records hold no document of it either, `page` prints its own message, exit 1. A page found is printed without asking `knowledge`, 3 ms; a fallback costs the `knowledge` call, 28 ms (`tools/page/src/fallback.rs`).

## Its tests

The crate's tests (`tools/knowledge/tests/`) make every file in a folder of the system's temporary folder, the binary's run with `AGENT_RECORDS_ROOT` set to such a folder or removed and `HOME` pointed there: no test reads the real records folder, and no check of a pull request needs the database. `page`'s tests run it without `AGENT_RECORDS_ROOT` nor a `GIT_*` variable, its fallback's against a script standing for `knowledge` (`tools/page/tests/binary.rs`).
