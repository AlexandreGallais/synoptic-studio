---
id: AUD-028
epic: E17
feature: F26
title: Audit F26
status: done
points: 1
---

# E17 · F26 · AUD-028 — Audit F26

A two-line change of `tools/mutate`, audited by the main thread against `kill`'s documentation and #370's next run (ADR-0086).

## Acceptance criteria

- Given F26's criterion, when the audit ends, then it is traced to a test titled `f26_ac1_…`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criterion traced, `kill`'s argument rule read (main thread)

One task = one commit, referenced as `AUD-028.T1`.

## Findings

Audited by the main thread (ADR-0086).

| Id | Check | Result |
| --- | --- | --- |
| 1 | `f26_ac1_names_the_group_after_the_end_of_kills_options` traced, seen failing before the fix (EN-127.T1) | passed after T2 |
| 2 | `kill`'s arguments: POSIX `kill` ends its options at `--`, a negative pid then naming a process group; procps reads an argument starting with `-` before `--` as a signal, which the hang of run 39138216016 shows | no finding; the proof on Linux is #370's `Rust` check |
| 3 | The Unix branch and the other systems' twin paired (`[F19.AC10]`), Clippy clean for the Windows target | no finding |
| 4 | A mutation's stop was tested only on macOS: no check ran the Rust tests on Linux before the CI of a pull request into `main` | a row for the next review |
