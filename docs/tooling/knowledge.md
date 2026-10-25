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

## The command

| Command | Does |
| --- | --- |
| `target/release/knowledge init` | opens the file, made or brought up to the schema, and prints its path and schema, `, newer than this binary's 1: read only` after a newer one |
| `target/release/knowledge path` | prints the file, opening nothing |

Measured on 2026-10-25 with `target/release/bench --runs 20`, a records root of the temporary folder: `init` on a file already made, median 27 ms, load average 8 to 15, most of it the two Git calls; on a new file, 28 ms, the file 68 KB.

## Its tests

The crate's tests (`tools/knowledge/tests/`) make every file in a folder of the system's temporary folder, the binary's run with `AGENT_RECORDS_ROOT` set to such a folder or removed and `HOME` pointed there: no test reads the real records folder, and no check of a pull request needs the database.
