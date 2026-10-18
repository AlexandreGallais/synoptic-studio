---
id: EN-114
epic: E17
feature: F21
title: The private records kept in a local folder shared by every project
status: done
points: 2
---

# E17 · F21 · EN-114 — The private records kept in a local folder shared by every project

Decided on 2026-10-18, after GitHub refused the records' push (a 238 MB transcript, over its 100 MB limit): the records stay on the computer only, without Git nor backup, in a folder the Product Owner chose outside the project, which every future project with the same method uses too, each in a folder of its own (ADR-0081). It joins F21, done, since it changes one of F21's tools after its validation.

## Acceptance criteria

- Given `AGENT_RECORDS_ROOT`, when `records-save` runs, then it copies the metrics, the transcripts and the library into `<AGENT_RECORDS_ROOT>/<project>`, the project named after its repository, writes the instant into `last-save.txt`, and runs no Git command in that folder: nothing is committed nor sent.
- Given `AGENT_RECORDS_ROOT` unset, or naming a folder that does not exist, when `records-save` runs, then it fails, says why and writes nothing.
- Given the tools of the repository, when the story ends, then none skips a `records/` folder any more, and `epic-check` reports the last save instead of a Git status.
- Given the records of `records/`, when the story ends, then they are moved into the chosen folder, and the variable is set in the user's `~/.claude/settings.json`, never in a tracked file: its path names the user's account.

## Tasks

One task = one commit, referenced as `EN-114.Tn`.

- [x] T1 — `records-save` and `epic-check` on the shared folder, the exclusions of `records/` removed, ADR-0081 and the pages (1 h)

## Measures (ADR-0069)

On 2026-10-18, this repository's real records (1.9 GB of transcripts, a 53 MB library), the release binary, one run each:

| Save | Before (copy, commit, push) | After (copy only) |
| --- | --- | --- |
| first, after the move | — | 647 ms |
| nothing changed but the library | push refused by GitHub, a 238 MB file | 175 ms |

The time left is the library's `VACUUM INTO`, run at each save; the transcripts already copied are skipped by their size and time.

## For the main thread

Journal, under 2026-10-18:

- `#tooling` EN-114: GitHub refused the records' push, a 238 MB transcript being over its 100 MB limit. The Product Owner chose to keep the records on the computer only, without Git nor backup, in a folder they chose and that every future project uses, one folder per repository (ADR-0081). `records-save` copies there and sends nothing; the 2.3 GB of `records/` were moved, its Git history deleted; the private GitHub repository is left for the Product Owner to delete.
