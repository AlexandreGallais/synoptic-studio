---
id: SP-034
epic: E49
title: Refine E49, the knowledge database and the light docs
status: in-progress
points: 3
---

# E49 · SP-034 — Refine E49, the knowledge database and the light docs

The refinement of [E49](../epics/E49-keep-the-agents-knowledge-in-a-database.md), from `develop` (ADR-0100, decision 2): the Product Owner's account is in the epic's file; this spike reads what the docs hold and who reads it, sources the database's design, and cuts the features, every question for the Product Owner asked at once at its end.

## Acceptance criteria

- Given every file of `docs/`, `AGENTS.md`, `CLAUDE.md`, `.claude/` and the agent's memory, when the spike ends, then the note classes each kind: kept in the light docs, moved into the database, or dropped, with its words counted and who reads it (code, tests, tools, the agent, the Product Owner).
- Given the tests and tools that read the docs, when the spike ends, then the note lists each with what it reads and what it becomes: reading the database, kept on a page that stays, or removed.
- Given the database, when the spike ends, then the note proposes its tables, how rows written on a branch never merged are told apart, how two worktrees write at once, how a snapshot is copied safely while it is written, and the tool's commands, each choice with a read source.
- Given the features, when the spike ends, then each has 2 to 4 stories of at most 3 tasks, its size from the measured medians, and the questions for the Product Owner are listed with options and an example.

## Questions

- What each part of the docs is read for, and by whom: the links from `src/`, `playground/`, `tests/` and `tools/` (217 files name a story file), the checks, the agent's own reads (`target/release/metrics`).
- SQLite with several writers: the journal modes, WAL and a rollback journal, the busy timeout, files on a synced folder (`REF-*` to read: SQLite's own pages on WAL, locking and backup).
- A clean copy of a database in use: the backup API, `VACUUM INTO`.
- Full-text search: FTS5, its ranking and its tokenizers.
- How coding agents load their context: `AGENTS.md`, memory, retrieval by a tool, and what the vendors' docs say about it.
- The rows a branch writes: kept, marked, or removed when the branch is dropped.
- What stays readable for the Product Owner: the domain's rules shortened, the state of the project, how the agent works.
- The ADRs: kept as files, or rows of the database, their principle stated on the pages that apply them.

## Tasks

One task = one commit, referenced as `SP-034.Tn`.

- [ ] T1 — Inventory the docs, their readers and the checks that read them (1 h)
- [ ] T2 — Read and record the sources of the database and the agents' context (1 h)
- [ ] T3 — Write the research note, cut the features, list the questions (1 h)
