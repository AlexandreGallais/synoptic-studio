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

Proposed when the epic was written (2026-10-25) as drafts: SP-034 cuts them again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The knowledge database | its tables, multi-branch included; the tool's add, search and query; the snapshot into the synced folder (3) | to refine |
| The move into the database | the logs and the history; the finished stories, the research notes, the guide pages, the retrospectives; the decisions and the memory (3) | to refine |
| The light docs | the pages kept, rewritten short; the business rules as rule and example; `AGENTS.md` pointing to the tool (3) | to refine |
| The checks | the tests and tools that read the docs, rewritten for the database or removed (2) | to refine |

Size: 4 features, 11 stories, 20 + 25 × 11 + (45 to 75) × 4 = 475 to 595 active minutes, 8 to 10 hours (note 0034, §1).

## Refinement

| Story | Status |
| --- | --- |
| [SP-034](../stories/E49-SP-034-refine-e49.md) — Refine E49, the knowledge database and the light docs | in-progress |

## Open points

The refinement's questions, asked at once at SP-034's end.
