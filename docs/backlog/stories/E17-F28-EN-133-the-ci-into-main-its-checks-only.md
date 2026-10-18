---
id: EN-133
epic: E17
feature: F28
title: The CI into main, its checks only
status: done
points: 2
---

# E17 · F28 · EN-133 — The CI into main, its checks only

The CI of `develop` → `main` is the one blocking check (ADR-0088, decision 3). At run 39145798246 (7 min 59 s) each of its five lanes ran `npm ci` (1 min 51 s to 2 min 13 s), whose `prepare` builds the release binaries, so the step `npm run rust:build`, kept to name the crates it compiles, took 0 s; the cargo tools were installed again in two lanes (2 min 47 s, 2 min 57 s), the `develop` cache missing, which F30 fixes. Its `workflow_dispatch`, for timing the lanes on a branch (EN-111), makes two jobs skip themselves by `if: github.event_name == 'pull_request'` (research note 0026, §2 and §3).

## Acceptance criteria

- Given `ci.yml`, when `[F28.AC1]` reads it, then `pull_request` into `main` is its only trigger and no job has an `if:`, `ci.yml` leaving the test's exceptions (EN-132); `docs/tooling/ci.md` no longer offers it on demand.
- Given each job's steps, when `[F28.AC3]` reads them, then each step is needed by a script of the job's lane: the release binaries built once per job, in the step that names its crates or in `npm ci`'s `prepare`, not both; a lane that runs no Node.js tool installs no Node.js packages; the cargo tools and the RustSec database only in the lanes whose scripts or tests call them, chosen by a field of the lane's matrix entry rather than a comparison of its scripts' names.
- Given the steps removed or moved, when the story ends, then `docs/tooling/ci.md` lists each with its time at run 39145798246, and the time of each job is measured again at the next review's CI and written there by that review.

## Tasks

- [ ] T1 — `[F28.AC3]` in `tests/workflows.test.ts`, `[F28.AC1]` for `ci.yml`, failing (1 h)
- [ ] T2 — `ci.yml` without its dispatch nor its conditions, its steps cut to what its lanes call, `docs/tooling/ci.md` (2 h)

One task = one commit, referenced as `EN-133.T1` in the commit footer.

## For the main thread

- No job's name changed: the checks `main`'s ruleset requires stay `Lint and types`, `TypeScript tests`, `Docs, playground and build`, `Format, secrets and dependencies`, `Rust`, `Commit messages and branch name`, `Dependency review`.
- Rule-docs: none affected, a tooling story that changes no business rule of `docs/domain/`.
- The after times of `docs/tooling/ci.md` (The steps of the lanes) are left to the next review's CI, as the story's third criterion says.

### Journal

- `#run` EN-133: the Rust lane keeps `npm ci`, since the formatter's tests, run by `cargo llvm-cov`, run the project's oxfmt; the release binaries' build inside `npm ci` took 49 s to 2 min 12 s per lane at run 39145798246, and a build passed as an artifact would add its time before every lane rather than save it.

### Improvements

| 2026-10-18 | agent | `[F28.AC3]`'s `NODE_READERS` in `tests/workflows.test.ts` is kept by hand: a crate that starts reading `node_modules` is not found by itself; a list derived from the crates' sources would find it (EN-133) | tool |
