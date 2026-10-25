---
id: SP-034
epic: E49
title: Refine E49, the knowledge database and the light docs
status: done
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

## Questions for the Product Owner

Found by the spike, written whole in [note 0035](../../research/notes/0035-e49-the-knowledge-database.md) (Questions for the Product Owner); nothing chosen.

- **Q1 — When a finished item leaves the repository**: (a) at its own `done` commit; (b) at its feature's merge into `develop`; (c) at its epic's end. Example: F42's 7 files leave one by one with (a), together at its merge with (b), at F47's end with (c).
- **Q2 — What the public site keeps of the finished work**: (a) nothing; (b) a page per epic done, generated at its end; (c) the release notes alone. Example: E02's 80,526 words of stories and features would be one page of about 600 with (b).
- **Q3 — The trace of a done feature's criteria (ADR-0027)**, which the CI cannot read from the database: (a) run once at the feature's end, the CI checking open features only; (b) a file of the 212 criteria kept, 7,457 words, the CI checking all; (c) no trace once done. Example: a test `[F42.AC4]` renamed by mistake fails the CI with (b) only.
- **Q4 — The ADRs**, 86 files, 51,159 words: (a) kept, each cut to its decision and reason; (b) moved, one index row each left; (c) moved, their principle on the pages that apply them. Example: ADR-0081, 413 words, about 150 with (a), a row of 30 with (b).
- **Q5 — The bibliography's rows**, 591 rows, 40,665 words, which the lint reads in the CI: (a) kept whole; (b) cut to source, URL, use and status, the reading notes moved. Example: `REF-CC-HOOKS`, about 430 words, about 40 with (b).
- **Q6 — The guide pages**, 45 pages, 51,596 words: (a) kept; (b) moved, one short page per area left; (c) moved, the checklists alone guiding a try. Example: `f42-zoom-and-pan.md`, 1,712 words, a section of about 200 with (b).
- **Q7 — The agent's memory**, `MEMORY.md` about 4,000 tokens at each session: (a) the memories moved, `MEMORY.md` a few lines, new memory files imported at each session's end; (b) Claude Code's auto memory turned off, every memory written through the tool; (c) the files kept, only indexed. Example: about 200 tokens at a session's start with (a) or (b).
- **Q8 — The snapshots kept in the synced folder**: (a) one; (b) one a day for 7 days; (c) one a day for 30 days. Example: a database of about 40 MB uses 40 MB, 280 MB or 1.2 GB.
- **Q9 — The domain's settled questions**, 7,106 words of `docs/domain/README.md`: (a) moved, the rules staying in their pages; (b) one line each kept. Example: Q21 no line with (a), about 15 words with (b).
- **Q10 — To confirm: is the records folder synced to the cloud?** If so, the working database lives elsewhere on the machine. Example: the main thread and a worker writing at once could leave a torn cloud copy.
- **To set once on the machine**: the variable naming the synced folder, in the user's Claude Code settings beside `AGENT_RECORDS_ROOT`, never in the repository.

## Tasks

One task = one commit, referenced as `SP-034.Tn`.

- [x] T1 — Inventory the docs, their readers and the checks that read them (1 h)
- [x] T2 — Read and record the sources of the database and the agents' context (1 h)
- [x] T3 — Write the research note, cut the features, list the questions (1 h)
