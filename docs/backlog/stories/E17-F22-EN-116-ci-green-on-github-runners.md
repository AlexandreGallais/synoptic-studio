---
id: EN-116
epic: E17
feature: F22
title: The CI's tests green on GitHub's runner
status: draft
points: 2
---

# E17 · F22 · EN-116 — The CI's tests green on GitHub's runner

The CI of `develop` → `main` (#259) failed 6 of 6 runs on three tests that pass on the Mac (RET-004, §1): `[F19.AC6]` expects oxlint's plain output, which prints GitHub annotations when `GITHUB_ACTIONS` is set; `[F19.AC9]` parses `cargo metadata`'s output, empty after a crate download failed; `[F02.AC1]`, the guided test's walk, goes over Vitest's 5 s on the runner.

## Acceptance criteria

- Given the CI of a pull request into `main`, when it runs, then `[F19.AC6]`, `[F19.AC9]` and `[F02.AC1]` pass on GitHub's runner and on the Mac, each still failing when the fault it guards is planted.
- Given `cargo metadata` failing, when `[F19.AC9]` runs, then it fails with Cargo's message, not a JSON parse error.

## Tasks

- [ ] T1 — The three tests: the cause of each read from the CI's log, the test made to hold on both machines, each broken once on purpose (2 h)

One task = one commit, referenced as `EN-116.T1` in the commit footer.
