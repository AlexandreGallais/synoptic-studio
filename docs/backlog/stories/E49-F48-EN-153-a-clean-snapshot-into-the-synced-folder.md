---
id: EN-153
epic: E49
feature: F48
title: A clean snapshot into the synced folder
status: ready
points: 3
---

# E49 · F48 · EN-153 — A clean snapshot into the synced folder

The copy the Product Owner keeps ([note 0035](../../research/notes/0035-e49-the-knowledge-database.md), A clean snapshot), one a day, the last seven kept (E49, Decisions, Q8). The synced folder is named by `AGENT_KNOWLEDGE_SNAPSHOT`, set once in the user's Claude Code settings beside `AGENT_RECORDS_ROOT`: its path names the Product Owner's account, so it is never written in the repository, a test, a commit or a log.

## Acceptance criteria

- Given the setting, when `knowledge snapshot` runs while another connection writes, then it writes `VACUUM INTO` a temporary file of the synced folder (`REF-SQLITE-VACUUM`), renamed to the day's name, `records-YYYY-MM-DD.sqlite` (`REF-POSIX-RENAME`), which opens and holds every committed row; the working file and its `-wal` are never copied.
- Given a snapshot of the same day, when it runs again, then the day's file is replaced; given eight days of snapshots, then the oldest is removed and seven are left.
- Given the setting missing, when it runs, then it copies nothing, says which variable to set and where, and exits without failing the session.
- Given a working database inside the synced folder, when the tool opens it, then it refuses, naming the two paths' relation, never the path itself in a file of the repository.
- Given a session's end, when Claude Code fires `SessionEnd`, then a hook runs the snapshot within its `timeout` (`REF-CC-HOOKS`); its time is measured on the real database and written in `docs/tooling/knowledge.md`, and `records-save` takes the snapshot too.

## Tasks

- [ ] T1 — The tests of the snapshot under a writer, the day's name, the seven kept, the setting missing, the folder refused, failing (1 h)
- [ ] T2 — `snapshot`, the `SessionEnd` hook, `records-save`, the tests of T1 passing (1 h 30)
- [ ] T3 — Its time measured; `docs/tooling/knowledge.md` and `docs/tooling/agent.md`'s hooks (30 min)

One task = one commit, referenced as `EN-153.T1` in the commit footer.

## For the main thread

### Journal

- `#decision` EN-153: the snapshot's choices left to the agent — the `SessionEnd` hook calls `knowledge snapshot` itself, the one hook outside `tools/hooks` (`hooks` cannot depend on `knowledge`, which depends on it through `library`), its `timeout` 30 seconds; the day is UTC's, as `calendar::today`; a temporary file over an hour old is a killed copy's leftover and removed; without a working file it copies nothing, exit 0; a synced folder named but missing fails, exit 1, never made; every command but `path` refuses a working file in the synced folder or below it.
- `#run` EN-153: the snapshot measured on a working file made for the measure, `AGENT_KNOWLEDGE_SNAPSHOT` not being set on this machine and no real `records.sqlite` existing yet: 52 ms on the repository's 767 pages (12 MB), 201 ms with the library's tables copied in (82 MB).
