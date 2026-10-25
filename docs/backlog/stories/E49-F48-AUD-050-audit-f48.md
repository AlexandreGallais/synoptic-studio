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
- [x] T2 — B to E. Sources, provenance, duplicates, consistency; the fixes (1 h)
- [x] T3 — F to H. Quality gates, what the next feature needs, the findings table (30 min)

## Findings

The `auditor` reviewed F48 read-only (ADR-0021) and found 12 problems, each reproduced by the main thread before it was recorded; the fixes are T1's and T2's, each test seen failing first, by its fix or by a mutation.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | `add_document` deleted its branch's row of the id whatever its state: an id added on `develop`, synced, then added again on `develop` lost its merged row, and `story/b` read no document of it | fixed (T1): schema 3 keys a document by its id, its branch and its state, its fields, sections and links following its state by `ON UPDATE CASCADE` (`REF-SQLITE-FOREIGN-KEYS`); an add replaces its branch's draft alone, the merged row read by every other branch until the new draft is merged. Tests first: `f48_ac2_keeps_a_merged_row_read_by_every_branch_while_its_branch_writes_a_new_draft_of_it` (`story/b` printed nothing before), `f48_ac2_searches_a_branchs_new_draft_and_the_others_its_merged_row_never_both`, `f48_ac1_keys_a_document_by_its_id_its_branch_and_its_state_from_schema_3`; `knowledge.md` (Its tables, The schema's number, `add`). A binary of schema 2 reads the file still, but for such an id the two rows' sections mixed, as `knowledge.md` says |
| 2 | `sync` marked merged the drafts of a branch without a commit of its own, `git branch --merged develop` listing a branch at `develop`'s tip | fixed (T1): a branch is merged when a merge commit of `develop` brought its tip in, its tip a merge's parent other than the first (`git log --merges --format=%P develop`, `REF-GIT-LOG`, `REF-GIT-REV-PARSE`, `REF-GIT-FOR-EACH-REF`), every merge of the project being one (`git-workflow.md`); `develop`'s own drafts merged by each `sync`. Tests first: `f48_ac2_marks_merged_the_rows_of_the_branches_merged_into_develop_and_no_other`, `feature/a` at `develop`'s tip now a draft, `chore/m` merged by a merge commit merged; `f48_ac2_reads_as_merged_develop_and_the_branches_a_merge_commit_brought_into_it`. A branch made at the tip of a branch already merged, before a commit of its own, still reads as merged: `knowledge.md` says so |
| 3 | `dropped` and `merged` are never set by `branches-clean` nor `story:finish`: the drafts of a branch deleted unmerged, or merged without a `sync`, stay drafts | no action in F48, the main thread's decision: the next feature's scope, the backlog in the database |
| 4 | `in_wal` ignored the result of its second switch to WAL: a switch leaving the file in another mode opened it anyway, against its doc comment | fixed (T1): the second switch failing, or returning another mode (`REF-SQLITE-PRAGMA`, `journal_mode`), fails the opening, its doc comment and `open_store`'s saying so. Test first: `f48_ac1_fails_the_opening_when_the_file_cannot_be_put_in_wal`, an in-memory database opened `Ok` before |
| 5 | `store.rs` and `knowledge.md` cited `REF-SQLITE-WAL` §9 for the switch refused at once under a writer, which §9 does not say | fixed (T2): both cite EN-152's measurement, no source read saying it; the mode a refused switch returns cites `REF-SQLITE-PRAGMA`; the store test's comment too |
| 6 | Two snapshots at once failed on pruning, `NotFound` on a file the other had removed | fixed (T1): `unless_gone` reads a file already gone as nothing, for the leftover's age, its removal and an old snapshot's. Tests first: `f48_ac3_takes_several_snapshots_at_once_each_pruning_without_failing`, six processes at once, failing 4 runs in 6 before; `f48_ac3_reads_a_file_already_gone_as_nothing_and_any_other_failure_as_one` |
| 7 | `tests/binary.rs` ran the binary with the developer's `AGENT_KNOWLEDGE_SNAPSHOT` | fixed (T1): its `knowledge` helper removes the variable, as `tests/snapshot.rs`'s does. Test: `f48_ac4_runs_the_binary_without_the_real_records_root_nor_the_real_synced_folder`, failing with the removal taken out |
| 8 | `project_records`, called before `refuse_synced`, printed a missing root's path, maybe inside the synced folder; `path` printed the file inside it | fixed (T1): every command, `path` included, refuses the synced folder before any message naming a path. Test first: `f48_ac3_refuses_to_open_a_working_file_inside_the_synced_folder`, `path` and a root mistyped inside the synced folder refused, no path printed; `knowledge.md` (The snapshot) |
| 9 | `REF-SQLITE-VACUUM` and `REF-RUSQLITE` dated 2026-10-13 and 2026-10-18, their "Used for" not naming F48 | fixed (T2): both read again through the library on 2026-10-25, their "Used for" naming F48; the rusqlite README now names SQLite 3.54.0 as of 0.41.0, the workspace pinned to 0.40.2 |
| 10 | `epic-check` repeated `main_and_origin`; `knowledge search`'s fallback from every word to any repeated `library::store`'s; `records-save` copies by `sqlite3`'s `VACUUM INTO` and `knowledge snapshot` by rusqlite's | fixed (T2): `epic-check` calls `main_and_origin`; the fallback is `library::store::every_word_else_any`, called by both searches, its test seen failing on a placeholder first. The two `VACUUM INTO` stay, one Improvements row below: sharing them would move `records-save` to rusqlite, a dependency and another tool's change |
| 11 | A row of `improvements.md` is out of date | the main thread's, in the done commit, the log being a shared file |
| 12 | `knowledge snapshot` deletes files in a folder under an allow rule | no action, the main thread's decision: it deletes only its own `records-*.sqlite` beyond the seven kept and its own temporary files, in the folder it is given, never another file |

The quality gates (F): the pre-push `check:all` passed on T1's and T2's heads. What the next feature needs (G): finding 3, the states `merged` and `dropped` set by `story:finish` and `branches-clean`, for the backlog in the database. The breadth (H): no tracked file nor commit message of the feature branch holds a synced folder's path (`git grep` and `git log --grep` for `/Users/`, `CloudStorage`, `Mobile Documents`, `iCloud`, `Dropbox`, `OneDrive`, `Google Drive`: two test paths, `/Users/a/…`, made up), and no duplicate of `tools/library`'s store is left unrecorded (finding 10).

The sources (B): the 18 `REF-*` rows `tools/knowledge` and `knowledge.md` cite, the four of T1 included (`REF-GIT-FOR-EACH-REF`, `REF-SQLITE-FOREIGN-KEYS` new; `REF-GIT-LOG`, `REF-GIT-REV-PARSE` read again), are all `[verified] 2026-10-25`.

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

## For the main thread

### Improvements

| 2026-10-25 | AUD-050 | Two copies by `VACUUM INTO`: `records-save` copies the library through the `sqlite3` command, read-only, and `knowledge snapshot` through rusqlite; one function of `knowledge::snapshot`, opening its source read-only, would serve both and spare `records-save` the `sqlite3` command, at the cost of rusqlite in `repo-tools` | tool |
| 2026-10-25 | AUD-050 | `records-save` prints a missing records root's path and `Saved in <records folder>` before `take_snapshot` refuses a records folder inside the synced folder, the path finding 8 kept `knowledge` from printing: refuse first, as `knowledge` does since AUD-050 | tool |

### Incidents

#### 2026-10-25 — mutate, the mutation spot-checks of the knowledge crate

- Used: `npm run mutate -- <the crate's eight source files> -- env CARGO_TARGET_DIR=<scratch>/target cargo test -q -p knowledge`, the target folder given so that the copy builds once.
- Failed: it counts as caught a mutation that does not compile: `+` swapped to `-` in `text + "\n"` (`tools/knowledge/src/bin/knowledge.rs:61`), a `String`, failed the build, not a test, and was reported `caught`.
- Instead: one mutation by hand in each function, the build checked apart by `cargo test --no-run` before the tests, each one that failed to compile made again another way; `lines` was caught so.
