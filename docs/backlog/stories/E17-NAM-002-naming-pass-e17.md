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

- [ ] T1 — The inventory: the calculations to name, the audit's findings, the times before (1 h)
- [ ] T2 — First pass: dates and times, the cooldown, the hash functions, named once each (3 h)
- [ ] T3 — Second pass and the epic's full audit: what the first missed, the Git helpers, the merge of the parallel branches (2 h)
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

## The epic's full audit

AUD-023's G left three findings to this story; the others come from reading E17's crates side by side.

| Finding | Where |
| --- | --- |
| 25 `Command::new("git")` in 19 files, a dozen of them the same helper: its output, or its error output as the reason | `deps`, `format`, `git-hooks`, `heavy-lock`, `hooks`, `metrics`, `repo-tools`, `rule-docs`, `run-totals`, `secret-scan`, `shared-files`, `worktree` |
| the checkout's Git folders found from the files Git writes, twice | `tool_use::checkout`, `hooks::git::checkout_of` |
| `hooks::json` beside `serde_json` | every hook, `metrics`, `run-totals` (improvement log, AUD-023) |
| the worktree's text matchers of `guard.rs` beside the word reader of `shell.rs` | `tools/hooks` |
| two versions of Semantic Versioning: the dependency check's, with pre-releases, and the release tool's, without | `deps::version`, `release::version` |
| two medians | `metrics::tables` (the mean of the middle two), `hooks::context` (the lower middle) |
| the merge of E17's parallel branches | F18 to F21 |

## Times before

On this Mac, 2026-10-18, the worktree on `develop` at `695f3b62`, load average 3.5 to 5:

| Command | Before |
| --- | --- |
| `npm run test` | 30.0 s (Vitest 29.6 s, 460 files, 3,315 tests) |
| `npm run check:all` | 106.0 s (its report 103.3 s: `check:rust` 103.1, `test` 58.8, `lint` 23.7, the others in parallel) |
