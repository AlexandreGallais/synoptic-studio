---
id: EN-050
epic: E17
feature: F18
title: Shared files written only at merge
status: done
points: 2
---

# E17 · F18 · EN-050 — Shared files written only at merge

Keep the shared files — the indexes, the logs, the feature file, the checklist — to the main thread at merge: a worker writes its rows in its story file (ADR-0065, decision 4; F18.AC6).

## Acceptance criteria

- Given a story branch that edits a shared file, when it is pushed, then the pre-push check refuses it and names the file.
- Given a story file holding rows for a log under a heading the tool knows, when the story is merged, then `story:finish` moves them into the log.

## Tasks

One task = one commit, referenced as `EN-050.Tn`.

- [ ] T1 — The list of shared files and the pre-push check; tests (1 h)
- [ ] T2 — The rows moved at merge by `story:finish`; the worker brief updated (1 h)

## For the main thread

- Technical choices made by the worker: the check covers `US` and `EN` branches only, the stories a worker implements; the others (`SP`, `CHK`, `AUD`, `VAL`, `NAM`, `REV`, `RET`) are the main thread's and may write shared files. The story's `done` commit (`Closes: <ID>`) may still set its status in the indexes and the feature's table, since `backlog.test.ts` wants the three to agree on the story branch; it may not write a log nor a checklist.
- Once rows were moved, `story:finish` pushes the feature branch: its pre-push runs `check:all` under the heavy lock, one more full check per story that had rows. EN-051 reworks `story:finish` for worktrees and keeps this step.
- The known headings are `### Journal`, `### Improvements` and `### Feedback`; another heading under `## For the main thread`, such as EN-047's `### Rows for the logs`, stays in the story file.

## Follow-up

2026-10-17, during the run of F18: `move-rows` copied every non-blank line under a log's heading, so EN-058's table header, EN-055's and EN-061's bullets and US-061's `- None.` reached the logs, and the last broke the format of `feedback.md` and a pre-push `check:all`. It now moves a log's own rows only — a dated table row for `improvements.md` and `feedback.md`, a tagged bullet for the journal — drops placeholders, headers and separators, and leaves any other line in the story file, named on standard error. The three logs were repaired, and `tests/logs.test.ts` now fails on a stray line in a table log, an untagged journal bullet, or a row repeated with other padding.
