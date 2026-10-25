---
id: EN-152
epic: E49
feature: F48
title: Add, search, show and list, by branch
status: ready
points: 3
---

# E49 · F48 · EN-152 — Add, search, show and list, by branch

The agent's one call to find a fact: the commands of [note 0035](../../research/notes/0035-e49-the-knowledge-database.md) (The tool; The rows of a branch) on EN-151's store.

## Acceptance criteria

- Given a document added by `knowledge add <kind> <file or ->`, when it is stored, then it is one row of `documents` with its id, kind, title and front-matter fields, its text cut into `sections` at its headings, its citations in `links`, its `branch` the current one and its state `draft`; its text read back by `show` is the text added.
- Given words, when `knowledge search <words>` runs, then it prints the best sections, BM25-ranked (`REF-SQLITE-FTS5`), each with its document's id, heading and a snippet, filtered by `--kind`, `--epic`, `--feature`, `--since` and `--branch`.
- Given rows of three branches, when a worktree reads, then it sees the `merged` rows and its own branch's `draft` rows; `--all` adds the `dropped` ones; `knowledge sync` marks `merged` the rows of every branch Git lists as merged into `develop` (`REF-GIT-BRANCH`).
- Given `knowledge list` with filters, when it runs, then it prints one line per document matching; given `show <id> [<heading>]`, a document or one section.
- Given an id no file holds any more, when `target/release/page <id>` runs, then it prints `knowledge show <id>`'s result.
- Given each command, when it runs, then its use is recorded as the other tools' are (EN-102).

## Tasks

- [ ] T1 — The tests of add, search, show, list, sync and the three states, failing (1 h)
- [ ] T2 — The commands, the tests of T1 passing (2 h)
- [ ] T3 — `page`'s fallback and `docs/tooling/knowledge.md`'s commands (45 min)

One task = one commit, referenced as `EN-152.T1` in the commit footer.

## For the main thread

### Journal

- `#run` EN-152: knowledge add, search, show, list and sync, and page's fallback on knowledge show; schema 2 keys a document by its id and its branch, so that a branch writes its draft of an id another holds, the four tables rebuilt by SQLite's procedure for a changed key (`REF-SQLITE-ALTER-TABLE`, added). Timings, a temporary records root of three documents: 27 to 39 ms a command, page's fallback 28 ms against 3 ms for a page found.
- `#run` EN-152: four connections opening a new file at once failed 1 run in 25 with `database is locked`: the switch to WAL is refused at once, without the busy timeout, while another connection writes; the opening now waits for that write by a `BEGIN IMMEDIATE` and switches again, a test reproducing it every time.

### Improvements

| 2026-10-25 | EN-152 | A story's rows stay drafts for good once `story:finish` deletes its branch: `git branch --merged develop` no longer lists it when its feature merges into `develop`. `story:finish` should carry the story's rows over to its feature branch, or mark them, as note 0035 plans (The rows of a branch) | tool |
| 2026-10-25 | EN-152 | `knowledge` is not in the list of the binaries the agent calls nor allowed by `.claude/settings.json`, which a worker may not edit: each call asks until the main thread adds it | tool |
