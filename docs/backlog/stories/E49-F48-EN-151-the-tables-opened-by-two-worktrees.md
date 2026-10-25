---
id: EN-151
epic: E49
feature: F48
title: The tables, opened by two worktrees
status: ready
points: 3
---

# E49 · F48 · EN-151 — The tables, opened by two worktrees

The crate `tools/knowledge` and its file, as [note 0035](../../research/notes/0035-e49-the-knowledge-database.md) designs them (Where it lives; Its tables; Two worktrees writing at once), on the workspace's `rusqlite` 0.40.2 already reviewed for `tools/library` (`docs/tooling/dependencies.md`): its store's patterns reused, nothing copied across crates that a shared module would serve better.

## Acceptance criteria

- Given `AGENT_RECORDS_ROOT` set, when the tool opens its database, then the file is `$AGENT_RECORDS_ROOT/<repo>/records.sqlite`, `<repo>` the last part of `origin`'s address as `records-save` names it, made with every table of note 0035 and its FTS5 triggers, in WAL (`REF-SQLITE-WAL`), `PRAGMA user_version` set to the schema's number (`REF-SQLITE-PRAGMA`); without the variable it fails and says what to set.
- Given two connections, when both write at once, then both writes succeed, each in `BEGIN IMMEDIATE` (`REF-SQLITE-TRANSACTION`), the second waiting within rusqlite's busy timeout (`REF-RUSQLITE-CONNECTION`).
- Given a file whose `user_version` is newer than the binary's, when the binary is asked to write, then it refuses and says to rebuild from its base; it still reads.
- Given the tests, when they run, then each makes its database in a temporary folder and none reads the real records folder.

## Tasks

- [ ] T1 — The tests of the schema, WAL, `user_version`, two writers and a newer file refused, failing (1 h)
- [ ] T2 — The crate `tools/knowledge`, its store opening and migrating the file, the tests of T1 passing (2 h)
- [ ] T3 — `docs/tooling/knowledge.md` and the records folder's layout row (`records.sqlite`) (30 min)

One task = one commit, referenced as `EN-151.T1` in the commit footer.

## For the main thread

The records folder's README (outside the repository) needs its layout row, which a worker cannot write: `| <project>/records.sqlite | the project's records, its knowledge tables first (target/release/knowledge, EN-151) |`. The binary `knowledge` is not in `.claude/settings.json`'s allow list nor in agent-tools.md's list of the binaries the agent calls: a permission change, left to the main thread for EN-152, when the agent starts calling it.

### Journal

- `#run` EN-151: `tools/knowledge` makes `$AGENT_RECORDS_ROOT/<repo>/records.sqlite` with note 0035's tables in WAL, each write in `BEGIN IMMEDIATE`, a newer schema refused for writing; `records-save` finds its folder through the same functions.

### Improvements

| 2026-10-25 | EN-151 | `f21_ac7_…` of `tools/tool-use/tests/binaries.rs` counts the workspace's binaries by hand, so a new binary fails the pre-push `rust:coverage` lane three minutes in rather than at `cargo test -p` of its crate: a check of the count by the binaries' names, or a note in agent-tools.md's Rust section | tool |
