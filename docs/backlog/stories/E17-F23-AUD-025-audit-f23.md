---
id: AUD-025
epic: E17
feature: F23
title: Audit F23
status: ready
points: 1
---

# E17 · F23 · AUD-025 — Audit F23

Audit F23 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F23's criteria, when the audit ends, then each is traced to a test titled `[F23.ACn] …` or `f23_acn_…`, broken once on purpose and seen failing.
- Given the cards C1, C3, C4 and C6 and those of `worktree-new` from the main checkout and of an epic's `SP`, when audited, then each is replayed and the tool now does what was needed, or the card says why not.

## Tasks

One task = one commit, referenced as `AUD-025.Tn`.

- [ ] T1 — The auditor's findings verified, the findings table; the tests of N1 to N4, N6, N9, N11 and N12, failing (1 h)
- [ ] T2 — The fixes in `tools/shared-files`, `tools/repo-tools`, `tools/worktree` and `tests/logs.test.ts`, the docs they change (1 h)
- [ ] T3 — The run skill and the docs of N5 to N8, the criteria broken once, the cards replayed, the mutations, G and H (1 h)

## Findings

The `auditor` reviewed sections A to E alone, read-only, in the background (ADR-0086, decision 1); the main thread decided each finding's action. Each finding below was reproduced before it was recorded.

| Id | Finding | Evidence | Result |
| --- | --- | --- | --- |
| N1 | a `done` commit may make any change to any log: another story's rows, a deletion, a rewrite | `tools/shared-files/src/shared.rs`, `refused_files`: `Kind::Log => !done`, the log's content never read | reproduced by `f23_ac1_refuses_a_log_in_a_task_commit_and_lets_through_the_done_commit_holding_rows` (another row, a line deleted, a line rewritten, each passing) and `f23_ac1_lets_the_done_commit_write_the_logs_but_not_a_checklist`, failing (T1) |
| N2 | a card written `###` or `##`, or a `#####` heading inside a card, is neither moved nor reported: `rows_left` counts 0 and `story-finish` passes | `take_rows`: a heading not `#### ` closes the log's block, its lines kept unreported; a `## ` ends the section | reproduced by `f23_ac1_reports_a_card_heading_of_another_level_under_incidents` and `f23_ac1_reports_a_deeper_heading_inside_a_card_and_keeps_the_incidents_heading`, failing (T1); `rows_left` counting the lines left, `f23_ac1_counts_the_rows_and_card_lines_a_story_file_still_holds`, failing |
| N3 | an indented continuation or a nested bullet of a card is split from it, left in the story file | `shape`: `in_card && line.starts_with("- ")` only | reproduced by `f23_ac1_takes_the_indented_lines_of_an_open_card`, failing (T1) |
| N4 | `rows_refusal` reads the story file in the working tree, and a read error gives `None`, a pass | `tools/repo-tools/src/story_finish.rs`, `fs::read_to_string(…).ok()?` | reproduced: `f23_ac1_opens_no_pull_request_while_the_story_file_holds_rows` with the rows removed from the working tree only passes the check, failing (T1); `f23_ac1_refuses_a_story_whose_file_holds_lines_or_cannot_be_read` for the read error |
| N5 | the run skill's step 8 says no `autonomous` label for a `VAL` nor a `REV` before the Product Owner validated them, while its review step and `merger` label them | `.claude/skills/run/SKILL.md` steps 8 and Review 2; `merger` in `story_finish.rs` | reproduced; the main thread's decision (ADR-0072, ADR-0086), written in T3, no code |
| N6 | `worktree`'s branch rule is a third copy of the branch form, still accepting any kebab-case topic, and `branch_type` maps `NAM` and `RET`, which the pre-push refuses | `tools/worktree/src/lib.rs`, `branch_refusal`, `branch_type`; `tools/git-hooks/src/branch.rs` | reproduced by `f23_ac2_refuses_the_branches_the_pre_push_refuses` and `types_a_branch_by_the_story_kind`, failing (T1); the retrospective's parked row names RET-005, removed by EN-123 |
| N7 | `docs/tooling/agent-tools.md`'s `story:finish` row says it labels every pull request `autonomous` and waits for it, without the epic's `SP` | its row 15; `merger` | reproduced; fixed in T3 |
| N8 | `story-finish` tells to `git commit --amend --no-edit` the rows left, against ADR-0091's options and `git-sequences` | `rows_refusal`'s message | reproduced by `f23_ac1_refuses_a_story_whose_file_holds_lines_or_cannot_be_read`, failing (T1) |
| N9 | the logs' uniqueness test compares single bullets in `tool-incidents.md`, where two cards may share one; the comment "the incidents log whole cards" names no test of it | `tests/logs.test.ts`, `repeatedRows`, `STRAY` | reproduced; `[F23.AC1] names a card written twice, not a bullet two cards share`, failing (T1) |
| N10 | no permission rule for `target/release/move-rows`: each `done` commit asks | `.claude/settings.json` | reproduced; no settings change now: a row for the next review, below |
| N11 | two mutations likely surviving: the `in_card = false` of a new heading dropped, `if !text.is_empty()` of `append_cards` made always true | `take_rows`, `append_cards` | both run by hand: both survived `cargo test -p shared-files` (17 and 13 tests passing), restored by `git checkout`; `f23_ac1_closes_the_card_at_a_log_heading` and `f23_ac1_writes_the_first_card_of_an_empty_log_without_a_blank_line_before_it` added (T1) |
| N12 | three finders of a story file by its id, `STORIES` written twice | `move-rows.rs` `story_file`, `story_finish.rs` `STORIES` with `item_file`, `backlog_status.rs` `file_of`, `worktree` `STORIES` and `story_of_file` | reproduced; one finder, `shared_files::backlog`, its tests failing (T1) |

## For the main thread

### Improvements

| 2026-10-19 | AUD-025 | Allow `target/release/move-rows` in `.claude/settings.json` beside `backlog-status`, and list it in `docs/tooling/agent-tools.md`'s binaries the agent calls: each `done` commit asks for it now (N10) | tool |
