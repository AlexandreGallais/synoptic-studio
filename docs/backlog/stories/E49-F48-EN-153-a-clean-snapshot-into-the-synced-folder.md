---
id: EN-153
epic: E49
feature: F48
title: A clean snapshot into the synced folder
status: done
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
