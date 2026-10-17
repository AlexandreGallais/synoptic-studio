---
id: EN-102
epic: E17
feature: F21
title: Each tool of our own records its use
status: ready
points: 3
---

# E17 · F21 · EN-102 — Each tool of our own records its use

Moved from F20 at its refinement (2026-10-17, F20's research note 0024): each tool of our own records its own use — the call, its time, its duration, its outcome — through one function shared by the Rust crates, so that `npm run metrics` lists it and the retrospective reviews the list as a tools team would. First of F21's enablers, so that every tool ported after it records its use from its first commit, and EN-100's `git:sync` and `git:commit` stop relying on the hook log.

## Acceptance criteria

- Given a tool of the workspace, when it runs, then one line is appended to `.git/metrics/tool-uses.jsonl` of the main checkout (a worktree's uses included) with the tool, its subcommand, the worktree, its start, its duration in milliseconds and its exit code, through one function of a shared crate.
- Given the record cannot be written (no `.git`, a read-only folder), when the tool runs, then the tool's own output and exit code are unchanged.
- Given the tools already in Rust — `git-sequences`, `worktree`, `run-totals`, `backlog-index`, `val-prepare`, `progress-page`, `heavy-lock`, `format`, `lint`, `mutate` and the hooks of `tools/hooks` — when the story ends, then each records its use.
- Given `npm run metrics`, when it runs, then a **Tools** section lists each tool's calls, failures and median and slowest durations over the period asked, and `docs/process/retrospectives/README.md` names the section among what the retrospective reads.
- Given one recorded use, when it is timed, then writing it adds under 1 ms to the tool's call.

## Files

`tools/tool-use/` (new crate), the `main.rs` of each crate named above, `scripts/metrics-report.ts` and `scripts/metrics-tools.ts` (still TypeScript, ported by EN-107), `docs/tooling/agent-tools.md`, `docs/tooling/metrics.md`, `docs/process/retrospectives/README.md`.

## Measures (ADR-0069)

The time one record adds to a call, before and after, on `git:sync` and `format --check` (20 calls each, release build); the size of the file after one run. Nothing to time before: no use is recorded today ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md)). No crate: the standard library appends the line (0025, The crates).

Measured on 2026-10-18 between 01:00 and 01:02 local time, load average 2.1 to 4.5: each binary called by its path in the worktree, the release build of T1 (no record) against that of T2 (the record), calls interleaved, each a process started and waited for, its wall time taken by a timer script of the scratch folder (Python's `perf_counter`), as note 0025 timed its tools.

| Call | Calls | Before, median | After, median | Before, mean | After, mean |
| --- | --- | --- | --- | --- | --- |
| `git-sequences sync` (a fetch from GitHub, then the rebase) | 20 each | 636.07 ms | 641.81 ms | 659.79 ms | 642.24 ms |
| `format --check` on the targets of `format:check` | 20 each | 471.44 ms | 469.71 ms | 470.17 ms | 469.91 ms |
| `git-sequences` without arguments: an error at once, the record alone around it | 200 each | 2.91 ms | 3.08 ms | 2.98 ms | 3.14 ms |
| `format` without arguments, the same | 200 each | 2.75 ms | 2.88 ms | 2.87 ms | 3.02 ms |

- **The record adds 0.13 to 0.17 ms to a call** (medians of the calls doing no work, 200 each), under the criterion's 1 ms; on `git:sync` and `format --check` it is lost in their own spread (±20 ms between calls).
- **The file**: 464 lines, 56,305 bytes after these measures, 121 bytes a line. A run writes one line per call of each tool; the most frequent is `run-totals status`, which the status line calls once a second while a run is open: a run of 5 hours adds about 18,000 lines, 2.2 MB.

## Tasks

One task = one commit, referenced as `EN-102.Tn`.

- [ ] T1 — The shared crate: one function writing one line, its tests (a record written, a folder missing, a concurrent append from two processes) (2 h)
- [ ] T2 — Every Rust tool of the workspace records its use (2 h)
- [ ] T3 — The Tools section of `npm run metrics`, the docs and the retrospective's list; times measured (2 h)
