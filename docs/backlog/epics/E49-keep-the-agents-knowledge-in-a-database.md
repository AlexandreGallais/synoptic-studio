---
id: E49
title: Keep the agent's knowledge in a database, the docs light
status: in-progress
---

# E49 — Keep the agent's knowledge in a database, the docs light

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: where the agent finds what it needs, and what is left in the docs. Asked by the Product Owner on 2026-10-25, after REV-009, ordered before E21 since it speeds up every epic after it; its refinement is SP-034.

**The Product Owner's account** (2026-10-25, in neutral words): the docs hold about 800,000 words, most of them history the agent pays to read again; the reorganisations kept everything. The docs become a short, human-readable set for the Product Owner. Everything the agent uses — the history, the logs, the finished stories, the research, the decisions, its memory, private parts included, and metrics — goes into one SQLite database per project in the agent's records folder, outside the repository: no branch conflicts, since every branch and worktree reads the same database, and nothing private reaches the public repository. Its tables are the agent's to design, multi-branch work included. `AGENTS.md` stays and tells any agent to read its knowledge through the tool. The business rules stay readable in the docs, as short as can be: the rule and its example, without the history. The checks that read the docs are rewritten to read the database, or removed. The working database stays on the machine; the tool copies a clean snapshot into the Product Owner's synced cloud folder at each session's end and on request, since a sync service may corrupt a SQLite file copied while it is written. That folder's path names the Product Owner's account: it is given by a local setting, never written in the repository.

| Field | Content |
| --- | --- |
| For | the agent, and any strong coding agent after it |
| who | reads long pages of history to find one rule, and pays for every word read |
| the solution | a knowledge database with its tool, and light docs |
| is a | one SQLite database per project in the records folder, read and written by a Rust tool |
| that | answers a question with the few rows that hold it, sorted by epic, kind, state, date or branch |
| unlike | 800,000 words of Markdown, read page by page, and logs only the main thread may write |
| our solution | the docs keep what the Product Owner reads; the rest is rows the agent queries |
| Business outcomes | the docs shrink to what is true today; the agent finds a fact in one call |
| Leading indicators | words left in `docs/`; tokens read per story, before and after (`target/release/metrics`) |
| Non-functional requirements | a query under a second; one database shared by every branch and worktree; a snapshot never corrupted; nothing private in the repository; the checks of a pull request never need the database |
| In scope | the database and its tool; the move of the records, the logs, the decisions and the memory into it; the light docs; the business rules shortened; the checks that read the docs |
| Out of scope | the folder shared by every agent (E50); the library of sources, already a database (`tools/library`) |

## Features

Cut by SP-034 (2026-10-25), in this order, each story of at most 3 tasks; the Product Owner answered the questions of [note 0035](../../research/notes/0035-e49-the-knowledge-database.md) the same day ([Decisions](#decisions)); each feature's file is written when it starts.

| Feature | Expected stories | Status |
| --- | --- | --- |
| [F48](../features/E49-F48-the-knowledge-database.md) — The knowledge database | the tables, opened by two worktrees; add, search, show and list, by branch; a clean snapshot into the synced folder (3) | in-progress |
| The backlog in the database | the stories, features, statuses, test cards and log rows written in the database, never as files: no story file, index row nor `done` commit; `worktree-new`, `backlog-status`, `story-finish`, the commit hooks and the shared-files guard reading it; a feature's readable docs updated in one commit at its end (asked by the Product Owner on 2026-10-25) (3) | to refine |
| The checks freed from the files | the `@see` to stories pointed at the rules; the tests and tools reading finished items; the log rows written into the database (3) | to refine |
| The move into the database | the finished stories, features and epics; the history, the logs, the retrospectives and the notes; the decisions, the memory and the security rules explained (3) | to refine |
| The light docs | `shapes.md` and the domain's index as rule and example; `interaction.md` as rule and example; the pages kept, rewritten short; `AGENTS.md` and the agent's pages pointing to the tool, the configuration files the moved docs leave behind cleaned: rules, scripts, the site, tests (4) | to refine |

Size: 5 features, 16 stories, 20 + 25 × 16 + (45 to 75) × 5 = 645 to 795 active minutes, 10.75 to 13.25 hours, the backlog's feature added after F48 started (note 0034, §1); the domain's two long pages take a story each, and the checks are rewritten before the move so that it breaks nothing.

## Refinement

| Story | Status |
| --- | --- |
| [SP-034](../stories/E49-SP-034-refine-e49.md) — Refine E49, the knowledge database and the light docs | done |

## Decisions

The Product Owner's answers to the questions of [note 0035](../../research/notes/0035-e49-the-knowledge-database.md), on 2026-10-25, in neutral words:

| Question | Decision |
| --- | --- |
| Q1, when a finished item leaves the repository | in its own `done` commit, before its pull request: no commit is needed after a merge; a feature's file leaves with its audit's `done` commit (the agent's choice, which the Product Owner left to it, preferring this one) |
| Q2, what the public site keeps of finished work | one short page per epic done, generated at its end |
| Q3, the trace of a done feature's criteria | one data file of the criteria beside the tests, outside the docs, so that the CI still checks every one |
| Q4, the ADRs | moved into the database; each page that applies a decision states its principle in a sentence |
| Q5, the bibliography | each row cut to its source, address, use and status; the reading notes moved |
| Q6, the guide pages | moved; a try is guided by the playground's checklists and the stories' test cards |
| Q7, the agent's memory | moved; `MEMORY.md` cut to a few lines pointing to the tool, new memories imported at each session's end |
| Q8, the snapshots | one a day, the last 7 kept |
| Q9, the domain's settled questions | moved; each rule stays on its domain page |
| Q10, the records folder synced | no: checked on 2026-10-25, it is not synced; the working database lives in it |
| One file of records | the project's records are one SQLite file, `records.sqlite`: the knowledge, the library's store, the metrics, the saves and the archives; the raw transcripts, 2.3 GB, stay files it indexes, so that a daily snapshot stays near 100 MB rather than 2.4 GB (asked by the Product Owner the same day, the transcripts' place the agent's choice) |
| Security rules | every safety rule (no personal information in the repository nor in a request, no secret, the guards) is written in the database with its reason and an example, for agents less versed in security; `AGENTS.md` keeps the short list |

The variable naming the synced folder, `AGENT_KNOWLEDGE_SNAPSHOT`, is set once in the user's Claude Code settings beside `AGENT_RECORDS_ROOT`, never in the repository.
