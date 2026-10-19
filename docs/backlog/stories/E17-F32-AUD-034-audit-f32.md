---
id: AUD-034
epic: E17
feature: F32
title: Audit F32
status: ready
points: 1
---

# E17 · F32 · AUD-034 — Audit F32

Audit F32 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F32's criteria, when the audit ends, then each is traced to a test titled `[F32.ACn] …` or `f32_acn_…`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — The findings and the tests of their fixes, seen failing; the mutations of the criteria's tests (1 h)
- [ ] T2 — The findings fixed: `metrics`' Sessions table, `val-prepare`'s struck criterion, the status line's ended calls and its seconds, `tokens_by_model`, two test comments, `docs/tooling/metrics.md` and `agent-tools.md` (2 h)

One task = one commit, referenced as `AUD-034.T1`.

## Findings

Reviewed by the `auditor` alone (ADR-0086), read-only; each finding reproduced before it is recorded, by a test of T1 seen failing on the code as F32 left it, or by reading where no test can show it. The mutations by hand, each restored from a copy of its file (a mutation leaving a variable or a function unused stops the build, a warning being denied, so each was written to compile).

| Id | Check | Result | Evidence | Action |
| --- | --- | --- | --- | --- |
| F1 | E1 — F32's criterion 1 says `metrics` reads a Sonnet subagent's tokens, but only `session-report --all` reads `modelUsage`: `metrics`' Sessions, Features and Stories tables sum the requests, which hold 657 of the 23,875 tokens a Sonnet subagent wrote (CHK-021) | reproduced: with Claude Code's count of the session after its requests, the Sessions row still shows the requests' 15 written and 1,015 in all | `tools/metrics/src/report.rs`, `work_sections`; `f32_ac1_counts_a_session_s_tokens_from_its_model_usage_in_the_sessions_table` fails (T1) | the Sessions table takes a session's tokens from its `modelUsage` when it has one and started in the period, the requests otherwise; the Features and Stories tables, cut by time, keep the requests, as `docs/tooling/metrics.md` says with the reason (T2) |
| F2 | E3 — `val-prepare` drops any criterion opening with `~~`, so F18's criterion 1, `~~Release pull request on develop~~, only the release on main.`, whose second half stands, gets no checklist line | reproduced: of four criteria opening with a strike, none kept, where criteria 1, 2 and 4 stand | `tools/val-prepare/src/lib.rs`, `criteria`; `f32_ac2_refuses_a_feature_whose_criteria_are_all_struck_and_counts_only_those_standing` fails (T1) | a criterion is struck only when its struck span is followed by nothing or by a new sentence (T2) |
| F3 | E3 — the status line's fallback removes a live subagent's call: a subagent resumed by `SendMessage` is a `Message` task until its result, left out of the earliest running start; a session started with `--agent` writes its type and no subagent id, so its calls are taken as a subagent's; a subagent over 3 h is dropped from the tasks (`LOST_TASK_MS`), which `metrics.md` does not say | reproduced: the call at 970 of a subagent resumed at 960 is removed, and so is a `--agent` session's call | `tools/hooks/src/status.rs`, `ended_calls`; `f32_ac1_keeps_the_calls_of_a_subagent_a_message_resumes_and_of_an_agent_session` fails (T1) | the `Message` tasks counted in the earliest running start; a call a subagent's only when its `agentId` is not empty, a file written before EN-120 left to `session-state` after a day; the 3-hour limit written in `metrics.md` (T2) |
| F4 | A4 — the `--since` Findings test was never seen failing | caught: the filter `finding.time >= inputs.since` removed, `f32_ac1_cuts_the_findings_to_the_period_as_it_cuts_the_totals` fails | `tools/metrics/src/report.rs`, `findings_section` | restored |
| F5 | B — two test comments misstate what they cite: #259's checks are said to be its own, CI failing three tests' job and CodeQL its analysis, where #259 failed one CI job and CodeQL passed; session `48a404b1` is dated 2026-10-15, its `startTime` 1792183009097 being 2026-10-16T20:36:49Z | reproduced: `gh pr view 259` lists one failure, `CI / Format, lint, secrets, types, tests, audit, versions, docs, build`, CodeQL a success | `tools/repo-tools/src/epic_check.rs`; `tools/metrics/tests/session.rs`, `transcript.rs` | the checks said made up for the case, the date 2026-10-16 (T2) |
| F6 | A3 — a `StatusContext` in state `EXPECTED`, a required status not reported yet, is untested | reproduced: no test names it | `tools/repo-tools/src/epic_check.rs`, `job` | a case in `f32_ac2_reports_a_ci_passed_running_or_absent`, running (T1); caught once `EXPECTED` is removed from the match, restored |
| F7 | D1 — `tokens_by_model` sums each model's tokens again, as `summed_usage` does | reproduced by reading | `tools/metrics/src/session.rs` | built from `summed_usage(&counted_usage(events))` (T2) |
| F8 | E3 — the status line's `short_duration` rounds the seconds after choosing its unit from the milliseconds: 59.5 to 59.999 s show `60 s`, the case I111 fixed in `format_duration` | reproduced: `short_duration(59_500)` is `60 s` | `tools/hooks/src/time.rs`; `chooses_the_unit_from_the_seconds_rounded_as_shown` fails (T1) | the unit chosen from the seconds rounded as shown, the minutes rounded down and at least 1 (T2) |
| C1 | A1, A2 — F32 holds no formula but `page_tokens`' ratio, measured by EN-097 | no finding | `tools/hooks/src/context.rs` | — |
| C2 | A4 — `tables.rs`, `command_key`: an `exit` after `\|\|` named | caught | `f32_ac1_names_the_command_after_the_brief_s_cd_or_exit`, `is_empty_when_nothing_runs` | restored |
| C3 | A4 — `transcript.rs`: the session's count kept after a request | caught | `f32_ac1_keeps_the_last_count_of_a_session_unless_a_request_came_after_it` | restored |
| C4 | A4 — `record.rs`, `is_call_of`: an empty id matching; `log-tool-call` on `SubagentStop` given no id | caught, each | `f32_ac1_tells_the_calls_of_an_ended_subagent_by_its_id`, `f32_ac1_removes_the_running_files_of_a_subagent_when_it_stops` | restored |
| C5 | A4 — `format.rs`, `format_duration`: the unit chosen from the seconds unrounded | caught | `f32_ac2_carries_the_minute_a_rounded_second_reaches` | restored |
| C6 | A4 — `epic_check.rs`: a `dropped` story left open | caught | `f32_ac2_counts_a_dropped_story_as_closed` | restored |
| C7 | A4 — `context.rs`, `page_tokens`: rounded down | caught | `f32_ac2_counts_a_page_tokens_at_the_ratio_measured` | restored |
| C8 | A5 — properties: F32 adds no `math` nor `geometry` function | no finding | `src/` unchanged | — |
