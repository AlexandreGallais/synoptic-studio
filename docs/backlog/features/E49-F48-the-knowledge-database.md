---
id: F48
epic: E49
title: The knowledge database
status: in-progress
---

# E49 · F48 — The knowledge database

**Benefit hypothesis**: the agent finds a fact in one call that returns the few sections holding it, from any branch or worktree, and the Product Owner's synced folder keeps a clean copy of the last seven days.

The first feature of E49, cut by SP-034 ([note 0035](../../research/notes/0035-e49-the-knowledge-database.md), The database; The features), the Product Owner's answers in the epic's Decisions. A tooling feature: nothing moves into the database yet, the tool and its file come first. The file is the one file of the project's records (Product Owner, 2026-10-25): the library's store, the metrics, the saves and the archives join it with the move into the database; the raw transcripts, 2.3 GB, stay files indexed by it, so that a daily snapshot stays near 100 MB. Started by the Product Owner's run on 2026-10-25.

## Acceptance criteria

1. One SQLite file per project, the one file of its records, `$AGENT_RECORDS_ROOT/<repo>/records.sqlite`, made by `target/release/knowledge` with the tables of note 0035 (`documents`, `fields`, `sections`, `sections_index` in FTS5, `links`, `calls`, `turns`), in WAL, its schema's number in `PRAGMA user_version`; two connections writing at once both succeed, each write in `BEGIN IMMEDIATE`; a binary refuses to write to a file whose schema is newer than its own (EN-151).
2. `knowledge add`, `search`, `show`, `list` and `sync` work as note 0035's table says; every row carries its branch and a state, `draft`, `merged` or `dropped`; a worktree reads the merged rows and its own branch's drafts, `--all` the dropped ones too; `page <id>` falls back on `knowledge show <id>` for an id no file holds (EN-152).
3. `knowledge snapshot` copies the database by `VACUUM INTO` a temporary file of the synced folder named by `AGENT_KNOWLEDGE_SNAPSHOT`, renamed to the day's snapshot, the last seven days kept; without the setting it copies nothing and says what to set; it refuses a working database inside the synced folder; a `SessionEnd` hook runs it, its time measured (EN-153).
4. No check of a pull request needs the database: the tool's tests make theirs in a temporary folder; the synced folder's path is never written in the repository.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-151](../stories/E49-F48-EN-151-the-tables-opened-by-two-worktrees.md) | EN | The tables, opened by two worktrees | done |
| [EN-152](../stories/E49-F48-EN-152-add-search-show-and-list-by-branch.md) | EN | Add, search, show and list, by branch | done |
| [EN-153](../stories/E49-F48-EN-153-a-clean-snapshot-into-the-synced-folder.md) | EN | A clean snapshot into the synced folder | done |
| [AUD-050](../stories/E49-F48-AUD-050-audit-f48.md) | AUD | Audit F48 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-151 | AUD-050 |
| 2 | EN-152 | AUD-050 |
| 3 | EN-153 | AUD-050 |
| 4 | EN-151, EN-152, EN-153 | AUD-050 |
