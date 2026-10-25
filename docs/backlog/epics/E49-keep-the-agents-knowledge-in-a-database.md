---
id: E49
title: Keep the agent's knowledge in a database, the docs light
status: draft
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

Cut by SP-034 (2026-10-25), in this order, each story of at most 3 tasks; their files are written once the Product Owner has answered the questions of [note 0035](../../research/notes/0035-e49-the-knowledge-database.md) (Questions for the Product Owner), which settle several stories' content.

| Feature | Expected stories | Status |
| --- | --- | --- |
| The knowledge database | the tables, opened by two worktrees; add, search, show and list, by branch; a clean snapshot into the synced folder (3) | to refine |
| The checks freed from the files | the `@see` to stories pointed at the rules; the tests and tools reading finished items; the log rows written into the database (3) | to refine |
| The move into the database | the finished stories, features and epics; the history, the logs, the retrospectives and the notes; the decisions and the memory (3) | to refine |
| The light docs | `shapes.md` and the domain's index as rule and example; `interaction.md` as rule and example; the pages kept, rewritten short; `AGENTS.md` and the agent's pages pointing to the tool (4) | to refine |

Size: 4 features, 13 stories, 20 + 25 × 13 + (45 to 75) × 4 = 525 to 645 active minutes, 8.75 to 10.75 hours (note 0034, §1); the domain's two long pages take a story each, and the checks are rewritten before the move so that it breaks nothing.

## Refinement

| Story | Status |
| --- | --- |
| [SP-034](../stories/E49-SP-034-refine-e49.md) — Refine E49, the knowledge database and the light docs | in-progress |

## Open points

The refinement's questions, asked at once at SP-034's end: Q1 to Q10 of note 0035 — when a finished item leaves the repository, what the public site keeps, the trace of a done feature's criteria, the ADRs, the bibliography's rows, the guide pages, the agent's memory, the snapshots kept, the domain's settled questions, and whether the records folder is synced; and the variable naming the synced folder, to set once on the machine.
