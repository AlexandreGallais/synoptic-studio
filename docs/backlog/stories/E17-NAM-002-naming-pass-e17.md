---
id: NAM-002
epic: E17
title: Naming pass over the code of E17
status: ready
points: 2
---

# E17 · NAM-002 — Naming pass over the code of E17

Before the review, go over the code E17 changed twice so that every calculation a field names is a named function with its source, with the full audit of the epic beside it (ADR-0049, ADR-0064).

## Acceptance criteria

- Given the code E17 changed, when the two passes end, then every calculation a field names is a function of that name with its `@see`, and the full audit's findings (duplicates, shareable code, the merge of parallel branches) are fixed or logged.
- Given every test and every tool of the repository, when the tooling and tests pass ends, then each claim proven twice is merged or removed, each slow preparation is done once, each wait or slow step found is fixed or logged, and `npm run test` and `check:all` are timed before and after (ADR-0069, ADR-0070).

## Tasks

One task = one commit, referenced as `NAM-002.Tn`.

- [x] T1 — The inventory: the calculations to name, the audit's findings, the times before (1 h)
- [x] T2 — First pass: dates and times, the cooldown, the hash functions, named once each (3 h)
- [x] T3 — Second pass and the epic's full audit: what the first missed, the Git helpers, the merge of the parallel branches (2 h)
- [ ] T4 — Tooling and tests pass: claims proven twice, slow preparations, waits; the times after (2 h)

## No golden file

E17 changed tooling, not the library: no file of `src/` and, of `playground/`, the checklists only. `npm run golden:check` draws shapes with the library, which no task touches; the behaviour of each tool is proven by its own tests, none of whose expected values is edited: a test moves with the function it proves, its cases unchanged.

## First pass: the calculations to name

Each row is a calculation written bare, or more than once, in the code E17 wrote (`git log --since=2026-10-16`, `tools/`), with the name and the source it gets. Every one is on integers or bits, its operations kept, so that no result changes.

| Where | Written bare | Named | Source |
| --- | --- | --- | --- |
| `tool-use`, `hooks`, `library`, `metrics`, `shared-files`, `deps`, `release`, `repo-tools` | the milliseconds of a second, a minute, an hour and a day declared in nine places, as `86_400_000`, `3_600_000`, `60_000`, `86_400` | `SECOND_MS`, `MINUTE_MS`, `HOUR_MS`, `DAY_MS`, declared once | `REF-HINNANT-DATES` (a day of 86,400 s, a duration in hours, minutes, seconds) |
| `hooks::time`, `tool-use` | `days_from_civil` and `civil_from_days` in two crates, a copy of the first in a test as its oracle | one crate of dates, `calendar` | `REF-HINNANT-DATES` |
| `library::time`, `metrics::time` | the days of a month counted two ways: the leap year written out, and the difference of two `days_from_civil` | `is_leap`, `days_in_month` (`last_day_of_month`) | `REF-HINNANT-DATES` |
| `hooks::time`, `library::time`, `metrics::time` | a time of day or an offset as `h · 3 600 000 + m · 60 000 + s · 1 000 + ms`, or `((h · 60 + m) · 60 + s) · 1 000 + ms` | `duration_ms` | `REF-HINNANT-DATES` (`days(…) + hours(20) + minutes(59) − utc_offset`) |
| `tool-use`, `library`, `metrics`, `shared-files`, `deps`, `release`, `repo-tools` | the date or the minute of an instant written six ways: `date_of_days(ms / DAY)`, `iso_instant(ms)[..10]`, `civil_from_days` and a `format!`, `minute_of`, `minute_text` | `iso_date`, `iso_minute`, prefixes of `iso_instant` | `REF-HINNANT-DATES` |
| `hooks::time`, `tool-use` | the clock read as epoch milliseconds twice | `now_ms`, once | — |
| `hooks::time`, `hooks::status` | `short_duration` written twice | once, in `hooks::time` | — |
| `deps` | the cooldown written six times: `now − published ≥ COOLDOWN_MS`, or `<` | `is_past_cooldown` | `REF-NCU-COOLDOWN` |
| `secret-scan::sha` | SHA-1's choice and majority, SHA-256's and SHA-512's `CH`, `MAJ`, `BSIG0`, `BSIG1`, `SSIG0`, `SSIG1` inline, the first two in three functions | `ch`, `maj`, `bsig0_256`… `ssig1_512` | `REF-RFC-6234` §5, `REF-RFC-3174` §5 |

### What the first pass did

- **`tools/calendar`**, a crate of its own, the standard library only, that `tool-use` and every crate with a date depend on: the four durations, `days_from_civil` and `civil_from_days` moved from `hooks` and `tool-use`, `is_leap`, `days_in_month`, `duration_ms`, `iso_date`, `iso_minute`, `iso_instant`, `temporal_instant` (`REF-TEMPORAL-INSTANT`, new, read through the library), `today` and `now_ms`. Removed: `shared_files::date`, `library::time::iso_date`, `metrics::time::{day_text, minute_text, temporal_text}`, `hooks::record::instant_text`, `repo_tools::epic_check::minute_of`, `repo_tools::records_save::instant`, `release`'s `today`, `deps::outdated::date`'s own formula, and the second `short_duration` of `hooks::status`.
- **Their tests moved with them**, their cases unchanged, into `tools/calendar/tests/calendar.rs`: the dates of `shared-files`, the instants and the 1900–2200 sweep of `tool-use`, the days of `hooks`, the minute of `epic-check`, the Temporal instants of `hooks::record` and `records-save`. The sweep's oracle, a copy of `days_from_civil` in `tool-use`'s tests (AUD-023), is gone: both functions now live in one crate, and the sweep proves `civil_from_days` on its own (each day the next date) before it checks `days_from_civil` against it. New: a leap year, the days of each month, against the difference of two `days_from_civil` from 1600 to 2400, `duration_ms`, and `REF-TEMPORAL-INSTANT`'s own example.
- **`deps::is_past_cooldown`**, its own test at exactly 7 days, broken once (`>=` → `>`) and seen failing; the six comparisons call it.
- **`secret-scan::sha`**: `ch`, `maj` (shared by SHA-1, SHA-256 and SHA-512), `bsig0_256` to `ssig1_512`, each tested on a single bit and on RFC 6234's vectors; `ch` and a shift broken once each and seen failing.
- `npm run mutate -- --all tools/calendar/src/lib.rs -- cargo test -q -p calendar`: 117 caught, 3 survived, all three a fallback no input reaches (a nanosecond count under 10⁹ that `u32::try_from` always takes, a clock before 1970).
- One behaviour moved, below zero only: `run-totals` and the agent limit used the copy of `short_duration` that truncated, the status line the one that rounds as `Math.round` does; they now share the second. A duration they show is never negative but for a clock set back, where `−44.6 s` now reads `-45 s`, as the status line already wrote it.

### Left inline, with their reason

| Where | Calculation | Reason |
| --- | --- | --- |
| `heavy-lock` | `MINUTE`, `HOUR`, `DAY` in seconds | the lock's file writes its start in seconds, read back by older binaries in other worktrees: a change of unit is a change of format |
| `hooks::session::STALE_MS`, `metrics::activity::RECENT_MS` | a day and an hour as `u128` | compared with `Duration::as_millis`; Clippy's pedantic casts refuse `DAY_MS as u128`, and `u128::from` is not `const` |
| `metrics::format`, `metrics::watch`, `metrics::activity` | `SECOND_MS = 1000.0`, `MINUTE_SECONDS = 60.0` | floats of `Math.round` ports, whose `as f64` of the integer constants Clippy refuses |
| `secret-scan::sha` | SHA-1's `f(t)` for t from 20 to 39 and 60 to 79, `b ^ c ^ d` | RFC 3174 names it only as `f(t)` of those steps, inline in its `match` |
| `repo_tools::check_all::seconds` | `(ms + 50) / 100`, tenths rounded half up | named by its function, the one place it is written |
| `hooks::context::median`, `metrics::tables::median` | two medians | not one calculation: the lower middle of whole token counts, as the TypeScript tool printed it, and the mean of the two middles of durations |
| `hooks::time::epoch_ms`, `library::time::instant_ms`, `metrics::time::offset_instant_ms` | three readers of ISO 8601 | each accepts what its input writes (a transcript's `Z`, every form Temporal reads, Git's `%aI`); one reader would refuse or accept other texts. Their arithmetic is `calendar`'s |

## The epic's full audit

AUD-023's G left three findings to this story; the others come from reading E17's crates side by side.

| Finding | Where | Done |
| --- | --- | --- |
| 25 `Command::new("git")` in 19 files, a dozen of them the same helper: its output, or its error output as the reason | `deps`, `format`, `git-hooks`, `heavy-lock`, `hooks`, `metrics`, `mutate`, `repo-tools`, `rule-docs`, `run-totals`, `secret-scan`, `shared-files`, `worktree` | merged (T3): `git_repository::stdout_of`, a crate of its own, takes the caller's command and gives its output or a `Failure` (not started, or exited with its words and error output); 17 helpers in 15 files call it, each keeping its own message and trim. Left with their own handling: `secret-scan`'s two (paths split from raw bytes at NUL; a child fed on its standard input), `git-sequences`' (its steps streamed, a message on standard input), `pre_commit`'s (a repository's own environment), `story_finish`'s step and status checks, `branches-clean`'s count and worktree removal (a status, or the error's first line as the reason kept) |
| the checkout's Git folders found from the files Git writes, twice | `tool_use::checkout`, `hooks::git::checkout_of` | merged (T3): `git_repository::checkout_of`, with `branch_of`, `common_folder` and `lexical_join`, their tests moved from `hooks`; `tool_use::checkout` names the worktree from it. Its two tests' expected paths are now written joined, `main/.git` for `main/.git/worktrees/en-102/../..`: the same file, the line written there as before |
| the Git common folder asked of Git by a hook and by a tool polled once a second | `limit-agents`, `run-totals` | read from Git's files, as the other hooks do (T3): `run-totals status` 15.1 → 4.9 ms, `limit-agents` 13.1 → 3.4 ms with its input piped (mean of 100 calls, twice, load average 2.8). `heavy-lock` and `metrics` keep `git rev-parse`: the lock's path is the one `GIT_DIR`, which a hook sets, and Git agree on |
| `hooks::json` beside `serde_json` | every hook, `metrics`, `run-totals` | logged (AUD-023's row of the improvement log): every hook's input and output goes through it, and its `quote` and `number_text` write as `JSON.stringify` does, which `serde_json` does not for −0 and large numbers |
| the worktree's text matchers of `guard.rs` beside the word reader of `shell.rs` | `tools/hooks` | logged (improvement row below): the guard decides what the agent may run, and AUD-023 tested it on 77 cases; moving its worktree forms onto the word reader changes what it refuses, a story of its own with that table |
| two versions of Semantic Versioning: the dependency check's, with pre-releases, and the release tool's, without | `deps::version`, `release::version` | logged (improvement row below): one crate would make `release` depend on the dependency check's crate, or a third crate; `git_hooks::message::is_version` is a third reading, node-semver's `valid` as commitlint calls it, kept |
| the story id of a branch read five ways | `shared_files::shared::story_of`, `worktree::story_of_branch`, `git_hooks::footer::story_of_branch`, `hooks::handback::story_of`, `hooks::status`'s prefixes | logged (improvement row below): they accept different branches (three digits or any, the types listed or any lowercase word), so one reader changes what each tool takes |
| `SECOND_MS = 1000.0` declared in three modules of `metrics` | `format`, `watch`, `activity` | merged (T3): declared once in `metrics::format` |
| two medians | `metrics::tables` (the mean of the middle two), `hooks::context` (the lower middle) | not a duplicate (Left inline) |
| the merge of E17's parallel branches | F18 to F21 | checked (T3): no id written twice (`REF-*`, ADR numbers, story ids), no row twice in the journal, the improvement log or `docs/tooling/dependencies.md`, no heading twice in `AGENTS.md`; the test `f18_ac9_counts_a_subagent_resumed_by_a_message_until_its_notification` is in `hooks/tests/tasks.rs` and `hooks/tests/status.rs` (T4) |

## Times before

On this Mac, 2026-10-18, the worktree on `develop` at `695f3b62`, load average 3.5 to 5:

| Command | Before |
| --- | --- |
| `npm run test` | 30.0 s (Vitest 29.6 s, 460 files, 3,315 tests) |
| `npm run check:all` | 106.0 s (its report 103.3 s: `check:rust` 103.1, `test` 58.8, `lint` 23.7, the others in parallel) |
