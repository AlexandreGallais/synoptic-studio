# F24 — The CI in parallel jobs, its tests shown as they run

The CI of a pull request into `main` no longer runs `check:all` as one job, silent until its end: each lane is a job and a check of its own, the TypeScript tests show file by file with their failures annotated on the pull request, and the Rust lane restores a cache saved by each push to `develop` ([ADR-0087](../adr/0087-the-ci-in-parallel-jobs-its-rust-cache-saved-from-develop.md)). A tooling feature, validated by the agent (VAL-025).

## Why

On GitHub's runner the single job of run 39131147001 took 139 s for `lint`, 265 s for `docs:build`, 326 s for the tests, and its Rust lane was still compiling every crate from nothing after 26 min 07 s, when it was cancelled; none of it showed before the end.

## What changed

| Before | After |
| --- | --- |
| one job, one check | five jobs in parallel: `Lint and types`, `TypeScript tests`, `Docs, playground and build`, `Format, secrets and dependencies`, `Rust`; a failing one leaves the others' results |
| the tests' output at the end only | each test file as it ends, each failure annotated at its line on the pull request |
| the Rust cache saved by a push to `main` only | saved by a push to `develop` too, in the background: nothing waits for it, nothing is deployed or released from `develop` |

## How to try it

Open the checks of the pull request `develop` → `main`: five lane checks appear, each with its own time and log. No playground line: nothing is drawn.

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. one job per lane, each a check | `f24_ac1_…` (3 Rust tests): every lane in exactly one job, `fail-fast: false`, `main`'s required checks equal the jobs' names | passed |
| 2. tests file by file, failures annotated | `[F24.AC2]`: the reporters `default`, `github-actions` and `junit` in the CI | passed |
| 3. the Rust cache from `develop`, its warm time | `[F24.AC3]`: saved on a push to `develop` and `main`, restored by key then prefix, the same in every job (`REF-GH-CACHE`); cold 26 min 07 s | the warm time pending: measured at #370's run on `develop`'s cache, written at the next review |

## Fixed by F25

The first run of #370 with these jobs failed its `TypeScript tests` check: `[F19.AC9]` runs `cargo deny` from the tests, and only the `Rust` job installed it and fetched the RustSec database. Since F25 (EN-126) the test job does both too.

## Fixed by F26

The same run's `Rust` check never ended: `tools/mutate`'s test `stops_a_mutation_that_never_ends` waited 46 min, Linux's `kill` reading the process group's `-<pid>` as a signal. Since F26 (EN-127) the group comes after `--`, and the command itself is killed after it.

## Fixed by F27

With every lane green, the history check refused six F19 commits written with the scope `eslint`, valid then and removed by EN-085. Since F27 (EN-128) the history check accepts a retired scope (`RETIRED_SCOPES`), and the `commit-msg` hook still refuses it in a new commit.
