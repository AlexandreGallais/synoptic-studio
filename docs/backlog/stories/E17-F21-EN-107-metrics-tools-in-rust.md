---
id: EN-107
epic: E17
feature: F21
title: The metrics tools in Rust
status: ready
points: 5
---

# E17 · F21 · EN-107 — The metrics tools in Rust

The metrics scripts — `metrics-report` with `metrics-events`, `-tables`, `-time`, `-tools`, `-work`; `session-report` with `model-tokens` (EN-066's split of it); `turns`, `activity`, `watch-task` — rewritten in Rust, their output unchanged (F21.AC2). `session-report` took 0.15 s and `activity` 0.28 s per call ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md)); each reads the session's transcripts, which grow to hundreds of megabytes over a run, so the port reads them incrementally, as `run-totals` does (EN-092). Kept whole at EN-093, with one addition from its note: the friction count by rule that EN-093 made by hand from the transcripts, so that the next review of the rules reads it instead of searching.

## Acceptance criteria

- Given each script, when its Rust binary runs on the cases of its TypeScript tests and on the transcripts of one recorded session, then its output is the same, and `package.json` (`metrics`, `metrics:session`, `activity`) and the Monitor's command in `docs/tooling/metrics.md` call the binary; the script and its test are removed in the same task.
- Given `npm run metrics:session`, when it runs, then the tokens per model of `model-tokens.ts` come out of the same binary, with the same figures.
- Given `npm run metrics`, when it runs, then a **Findings** section counts, per epic, each rule's findings met in the transcripts — the hooks' refusals by hook and reason, the lint and check findings by rule — as EN-093 counted them for `docs/conventions/rules.md`, and the **Tools** section of EN-102 is kept.
- Given `watch-task` under Claude Code's Monitor, when the output stalls, comes back, matches its end pattern or runs out of time, then it prints the same lines as today.
- Given each binary, when it is timed, then its time per call before and after is stated, on the same session.

## Files

`tools/metrics/` (new crate, one binary per command), `package.json`, `scripts/metrics-*.ts`, `scripts/session-report.ts`, `scripts/model-tokens.ts`, `scripts/turns.ts`, `scripts/activity.ts`, `scripts/watch-task.ts` and their tests (removed), `docs/tooling/metrics.md`, `docs/tooling/agent-tools.md`, `docs/conventions/rules.md` (where the counts are read), `.claude/skills/run/SKILL.md` if it names a command.

## Measures (ADR-0069)

Before (0023's table and SP-023's) and after, 20 calls each on this Mac, on one recorded session: `metrics`, `metrics:session`, `metrics:session -- --run`, `activity`; `watch-task`'s CPU time over one minute.

Before, measured by SP-023 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), Each tool's time before its port) on the project's 1.5 GB of transcripts, where 0023 measured 0.15 s for `session-report`: `metrics` 8.2 s (median of 5), `metrics:session -- --all` 5.1 s (5), `metrics:session` 5.9 s and `-- --run` 18.7 s (one call each), `activity` 248 ms, `turns` 75 ms (20 each). The transcripts are read as JSON lines with `serde_json`, already pulled by EN-106's `rmcp` (0025, The crates).

After, measured on 2026-10-18 between 02:19 and 02:24 on one frozen copy of the 1.6 GB of transcripts, the logs and the library (a scratch clone, `HOME` pointing to the copy), before and after in the same series; SP-023's method (a Python timer around each process, outputs through pipes), the TypeScript scripts as `node --harmony-temporal`, the binaries by their path, the transcripts' cache warm; the load average 13 to 20 from the other worktrees, so the TypeScript times are above SP-023's:

| Tool | Before, median (calls) | After, median of 20 | After, first call without the cache |
| --- | --- | --- | --- |
| `metrics` | 9,402 ms (5) | 463 ms | 6.4 s |
| `metrics:session -- <session> --all` | 5,577 ms (5) | 108 ms | 3.8 s for that session's 1.1 GB |
| `metrics:session -- <session>`, the current turn | 10,069 ms (5) | 100 ms | — |
| `metrics:session -- --run` | 107 ms (5) | 15 ms | — |
| `activity` | 211 ms (20) | 125 ms | — |
| `watch-task`, CPU over one minute, a look every 5 s | 0.08 s | under 0.01 s | — |

`activity` through `cargo run`, as `npm run activity` calls it, on the live logs: 251 ms median against 138 ms by its path (load average 32). `turns.ts` has no command of its own: its record is written by `session-report` and its table by `metrics`, measured with them.

## Tasks

One task = one commit, referenced as `EN-107.Tn`.

- [ ] T1 — The transcript reader and the session report in Rust, model tokens included, read incrementally (5 h)
- [ ] T2 — The metrics report in Rust, the Tools section kept, the Findings section added (5 h)
- [ ] T3 — `turns`, `activity` and `watch-task` in Rust; `package.json` and the docs; times measured (4 h)

## For the main thread

The evidence, on 2026-10-18, on a frozen copy of this session's transcripts (1.6 GB), logs and library, in a scratch clone:

- **The same output**: `metrics:session` for three sessions, each with `--all`, `--all --markdown` and the current turn, and the lines saved in `turns.jsonl`: byte for byte the same. `metrics` and `metrics -- --since 2026-10-16`: the same, the Findings section added, but for one row of the Tools section, `library stats`: the script ran it as a command, which recorded one use more before the report read the uses; the binary reads the library in its own process. `activity` and `activity 3` on the copy, and `activity` on the live logs (16 background outputs, the running calls): the same. `watch-task` beside `watch-task.ts` on one scripted output (a stall, its end, a second stall, the task's end) and on a static one (`--max 7`): the same lines.
- **Fixed on the way**: `metrics:session` reads the main checkout's transcripts from a worktree (`tools/metrics/tests/session_binary.rs` runs it in one). A request counts once with its last line, as the script's map did and `run-totals` does.
- **Decisions taken**: the transcripts and logs parsed by `serde_json` rather than `hooks::json`, a fifth faster on 1.1 GB (4.5 s of CPU against 5.6 s), `serde_json` being already a workspace dependency; a cache of each transcript's events, `.git/metrics/transcripts-cache.jsonl` (31 MB for 1.6 GB), so that the reports read only the new lines; `turns` folded into `session-report` (the record) and `metrics` (the table), as it has no command; the library's report read through `tools/library`'s functions instead of a `cargo run`; the Findings section's forms and naming documented in `docs/tooling/metrics.md` (Findings); a hook's refusal named by Claude Code's `<event>:<tool>` and its reason, since the message does not name the hook's file.
- **Not changed, to decide**: the command key keeps the script's rule, so `cd … || exit 9` counts as `exit` (8,270 calls, the first row of Commands by count): a row of the improvements below.
- **Rule docs**: none affected, the story changes tools only.

### Improvements

| 2026-10-18 | EN-107 | `npm run metrics`' command key cuts at `\|\|`, so the brief's `cd "<worktree>" \|\| exit 9; …` counts as `exit`, 8,270 of the calls, the first row of Commands by count: skip an `exit` after `\|\|` to name the command that ran | tool |
| 2026-10-18 | EN-107 | The Findings section of `npm run metrics` lists every rule met since E01, about 400 lines: a `--since` reading per epic, or a cut at a few findings, would shorten what the retrospective reads | tool |
