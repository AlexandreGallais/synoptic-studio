---
id: AUD-050
epic: E49
feature: F48
title: Audit F48
status: ready
points: 3
---

# E49 · F48 · AUD-050 — Audit F48

Full check of F48 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill. A tooling feature: no mathematics; the depth is in the concurrency, the snapshot's safety and the privacy of the synced folder's path.

## Acceptance criteria

- Given the crate `tools/knowledge`, when audited, then each function survives a mutation spot-check, and two writers, a snapshot under a writer and a newer schema are each tried for real, not only in the tests.
- Given the sources, when audited, then every `REF-*` F48 cites is re-verified and dated.
- Given the repository, when audited, then no file, test, commit or log holds the synced folder's path or any personal information, and no duplicate with `tools/library`'s store is left unrecorded.

## Tasks

One task = one commit, referenced as `AUD-050.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; mutation spot-checks; the real tries (1 h 30)
- [ ] T2 — B to E. Sources, provenance, duplicates, consistency; the fixes (1 h)
- [ ] T3 — F to H. Quality gates, what the next feature needs, the findings table (30 min)

## Findings

Mutations (A4), the fixes of T1 in place, against `cargo test -p knowledge`: `npm run mutate` on the crate's eight source files, its first mutation of each kind, 13 caught and 1 survived, `SEARCH_LIMIT` 8 → 9, no test holding more than eight hits; then 55 by hand, one at least in each of the crate's 55 functions, `next_id`'s and `init`'s left to the tool's: 42 caught, 7 survived, 6 failed to compile. The survivors:

- `SEARCH_LIMIT` (`read.rs`): caught since by `f48_ac2_searches_at_most_eight_sections`, nine stories found, eight printed.
- `main_and_origin` (`records.rs`), the missing remote read as `x`: no test ran the binary without `origin`; caught since by `f48_ac1_names_the_file_after_the_main_checkouts_folder_without_a_remote`.
- `is_called_right` (`knowledge.rs`), `show` taking three words: caught since by `show EN-900 Tasks more` among the wrong calls.
- `unless_gone` (`snapshot.rs`), a file gone read as a failure again: caught one run in three by six snapshots at once, the race being the processes'; caught always since by `f48_ac3_reads_a_file_already_gone_as_nothing_and_any_other_failure_as_one`, the function made public for it.
- `show` (`read.rs`), the draft and the merged row swapped in its order: equivalent, no action — a branch reads no merged row of an id it holds a draft of, so the two never meet in that query.
- `synced` (`knowledge.rs`), `git log` without `--merges`: equivalent, no action — a commit of one parent has no parent past the first; `--merges` only shortens the output.
- `hidden` (`snapshot.rs`), the resolved path no longer replaced: no action — no failure met in the synced folder prints its resolved path, `std::io::Error` naming none and SQLite's `VACUUM INTO` neither; the replacement stays as a guard.
- `take_snapshot` (`snapshot.rs`), `is_file` read as `exists`: no action — the two differ only for a folder named `records.sqlite`, whose opening then fails with its own error.

The six that failed to compile, the lints denying an unused binding or an unreachable arm, were made again another way, `init`'s apart, and caught; so were the four survivors given a test. The new tests were seen failing each by its mutation, or before its fix.

The real tries, with the crate's binary in a repository of the scratch folder, `AGENT_RECORDS_ROOT` set there:

- **Two writers**: two loops of 15 `knowledge add journal -` at once, 30 rows listed, no failure, the file in WAL at schema 3.
- **A snapshot under a writer**: `sqlite3` holding `BEGIN IMMEDIATE` and an uncommitted row of `turns` for 3 seconds while `knowledge snapshot` ran: written at once, its 30 documents in it and the uncommitted row not, in the `delete` journal mode.
- **A newer schema**: `user_version` set to 4: `init` prints `schema 4, newer than this binary's 3: read only`, `add` fails with the rebuild message, `list` still reads the 30 rows.
- **Finding 1, from a file of schema 2**: the file made by the binary of schema 2, ADR-0900 added on `develop` and synced, then opened by this binary, brought up to 3; ADR-0900 added again on `develop`: `story/b` still reads the first text until the next `sync`, the second after. The binary of schema 2 reads that id's two rows' sections mixed, as `knowledge.md` says of schema 3.
