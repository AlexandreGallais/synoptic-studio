---
id: EN-107
epic: E17
feature: F21
title: The metrics tools in Rust
status: draft
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

## Tasks

One task = one commit, referenced as `EN-107.Tn`.

- [ ] T1 — The transcript reader and the session report in Rust, model tokens included, read incrementally (5 h)
- [ ] T2 — The metrics report in Rust, the Tools section kept, the Findings section added (5 h)
- [ ] T3 — `turns`, `activity` and `watch-task` in Rust; `package.json` and the docs; times measured (4 h)
