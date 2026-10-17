---
id: EN-102
epic: E17
feature: F21
title: Each tool of our own records its use
status: draft
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

## Tasks

One task = one commit, referenced as `EN-102.Tn`.

- [ ] T1 — The shared crate: one function writing one line, its tests (a record written, a folder missing, a concurrent append from two processes) (2 h)
- [ ] T2 — Every Rust tool of the workspace records its use (2 h)
- [ ] T3 — The Tools section of `npm run metrics`, the docs and the retrospective's list; times measured (2 h)
