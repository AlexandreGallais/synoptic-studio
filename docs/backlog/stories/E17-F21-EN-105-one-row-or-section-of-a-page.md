---
id: EN-105
epic: E17
feature: F21
title: One row or one section of a repository page
status: done
points: 2
---

# E17 · F21 · EN-105 — One row or one section of a repository page

Moved from F20 at its refinement (2026-10-17, F20's research note 0024, The agent's most frequent calls): `docs/references.md` was read 121 times in three days, about 6,000 characters a read, and `docs/process/improvements.md` 98 times, when the agent needed one row or one section. A command prints that row or that section only, as `library_read` does for a web page (F21.AC8).

## Acceptance criteria

- Given an id that starts a table row, such as `REF-CC-HOOKS`, `ADR-0069` or `EN-092`, when `npm run -s page -- <file> <id>` runs, then it prints that row only, preceded by its table's header line; an id found in no row exits 1 naming the file.
- Given a heading's text, such as `Hooks` in `docs/tooling/agent.md`, when the command runs, then it prints that section down to the next heading of the same or a higher level; a heading found twice prints both, each with its line number.
- Given no file named, when the command runs with an id of `REF-`, `ADR-`, `DERIV-` or a story, then it looks in the file that holds those ids (`docs/references.md`, `docs/adr/`, `docs/derivations/`, `docs/backlog/`) and prints the row or the file's first section.
- Given the docs for the agent, when they are read, then `docs/tooling/agent-tools.md` and the worker brief name the command beside `Read`, for a row or a section.
- Given the command, when it is timed, then it answers in under 10 ms, and the characters it prints are stated against the whole file's for `REF-CC-HOOKS` and for the section `Hooks` of `docs/tooling/agent.md` (`docs/process/improvements.md` is one table without sections, its rows starting with a date: SP-023).

## Files

`tools/page/` (new crate, or a binary of the crate EN-103 makes), `package.json`, `docs/tooling/agent-tools.md`, `.claude/skills/worker-brief/SKILL.md`, `.claude/skills/project-rules/SKILL.md`.

## Measures (ADR-0069)

Time per call (20 calls, release build) and characters printed, against `cat` of the whole file; after one run, the reads of `docs/references.md` counted in the hook log against the 121 of the three days measured by SP-022.

Before, measured by SP-023 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md)): `cat docs/references.md` 3.0 ms for 189,199 characters; the row `REF-CC-HOOKS` alone is 1,856 characters; `grep` of it 4.7 ms. No crate: the standard library (0025, The crates). An open point for the story: `docs/process/improvements.md` and the other logs hold rows that start with a date, not an id; printing the rows that name an id in any cell would serve them, if the story finds it worth a criterion.

## Tasks

One task = one commit, referenced as `EN-105.Tn`.

- [x] T1 — The command: a row by its id, a section by its heading, the file found from the id; its tests on fixture pages (3 h)
- [x] T2 — The brief and the agent-tools page name it; characters and time measured (1 h)

## For the main thread

After, measured on 2026-10-18 (`docs/tooling/agent-tools.md`, Reading one row or one section): `page REF-CC-HOOKS` prints 2,335 characters, the row and its header, of the 198,307 of `docs/references.md`, in 3.2 ms against `cat`'s 2.9 ms (medians of 20 calls, the binary by its path); `npm run -s page` adds about 160 ms. The count of reads of `docs/references.md` in the hook log against SP-022's 121 is left to the run after this one. The open point on rows starting with a date (`docs/process/improvements.md`) is not a criterion: those rows are found by `grep` of the id, and the command keeps one meaning for an id, the first cell.
