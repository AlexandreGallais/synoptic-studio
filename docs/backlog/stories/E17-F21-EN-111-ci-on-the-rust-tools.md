---
id: EN-111
epic: E17
feature: F21
title: The CI on the Rust tools, built and cached once
status: done
points: 2
---

# E17 · F21 · EN-111 — The CI on the Rust tools, built and cached once

The pull request into `main` is the only CI run (ADR-0038, `docs/tooling/ci.md`). Once the checks, the hooks and the dependency tools are Rust binaries (EN-103 to EN-110), each job that calls them builds the workspace from nothing, and `cargo-llvm-cov` and `cargo-deny` are compiled at each run (`cargo install`). The CI builds the workspace once per run, caches the build and the installed cargo tools between runs, and every job and the release workflow use them (F21.AC4). Before the release story (EN-112), whose publishing job runs a Rust binary on `main`.

## Acceptance criteria

- Given a pull request into `main`, when the CI runs, then the workspace is built once in release mode, its `target/` and Cargo's registry cached by the official `actions/cache` keyed on `Cargo.lock` and `rust-toolchain.toml`, and `check:all`, `history` and `build` use that build.
- Given `cargo-llvm-cov` and `cargo-deny`, when the CI needs them, then they come from a cache keyed on their versions, installed only when the key changes.
- Given a second run on an unchanged `Cargo.lock`, when it starts, then no crate of the workspace's dependencies is compiled again, shown by the job's log.
- Given that a cache saved by a pull request's run is scoped to its merge ref and restored only by re-runs of that pull request, while a run restores the caches of its base and of the default branch, `main` (`REF-GH-CACHE`, SP-023), when the next pull request into `main` starts, then it restores a cache saved by a run on `main` (the release workflow's run after the merge, or one of its own), never one saved by the previous pull request.
- Given the workflows, when the story ends, then no step calls a dev package the earlier stories removed, and `docs/tooling/ci.md` describes the build and its caches.
- Given the CI, when it is timed, then a run's duration, cold and with its caches, is stated before and after.

## Files

`.github/workflows/ci.yml`, `.github/workflows/release.yml` (`docs` job), `docs/tooling/ci.md`, `docs/references.md` (the caching page of GitHub's documentation, read through the library).

## Measures (ADR-0069)

Before and after: the CI's duration on a pull request into `main`, each job's, cold and with its caches (the run's own timings on GitHub); the cache's size.

Before, read by SP-023 in GitHub's timings of three runs on 2026-10-16 and 2026-10-17 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md)): the `quality` job 5 min 9 s to 8 min 29 s, `history` 16 to 19 s, the dependency review 5 to 8 s. GitHub removes a cache unused for 7 days and keeps 10 GB per repository (`REF-GH-CACHE`): with a release every week or two, a cache saved on `main` may be gone by the next pull request; the story states how often its run found one.

## Tasks

One task = one commit, referenced as `EN-111.Tn`.

- [ ] T1 — The workspace built once and cached, the cargo tools cached by version; GitHub's cache documentation read and cited (3 h)
- [ ] T2 — Each job on the cached build; the docs; a cold and a warm run timed by `workflow_dispatch` on the story's branch, the CI given that trigger (2 h)

## For the main thread

The workflows were not run: no GitHub Actions run is made from a worktree, and actionlint is not installed here. They were read against the pages of the library (`REF-GH-CACHE`, `REF-GH-CACHE-RESTORE`, `REF-GH-ARTIFACT-MODES`, `REF-GH-WORKFLOW-DISPATCH`), parsed as strict YAML, and their caches' keys, paths and commands are held by `tests/ci-caches.test.ts`. The timed runs of T2 could not be made on the story's branch: `workflow_dispatch` runs only once `ci.yml` with that trigger is on `main` (`REF-GH-WORKFLOW-DISPATCH`), and a branch restores only its own caches and `main`'s, which hold none until `rust-cache` has run there. The "after" measures come from the first runs on GitHub:

1. **The pull request `develop` → `main` at E17's end runs cold**: `main` holds no cache yet. Its quality job's log shows `Cache not found` for `rust-tools-Linux-…` and `rust-build-Linux-…`, the `cargo install` step run, `npm run rust:build` compiling every crate; the history job starts after the quality job, downloads `git-hooks` and runs it, compiling nothing; the required checks keep their names. Its durations are the cold "after".
2. **After its merge, `release.yml` on `main`**: the `rust-cache` job passes, `-- --list` printing the tests' names without running them, and its post steps say `Cache saved with key: rust-tools-Linux-…` and `rust-build-Linux-…`; the caches' sizes are on _Actions → Caches_. The `docs` job, beside it, may still miss them on that first run.
3. **A warm run**: `gh workflow run ci.yml --ref develop` once the merge is on `main`, or the next pull request into `main` on an unchanged `Cargo.lock`: `Cache restored from key: …` for both, the `cargo install` step skipped, and the `npm run rust:build` step's log listing `Compiling` for the workspace's own crates only, no crate of crates.io (this story's third criterion). Its durations are the warm "after"; whether a pull request found `main`'s caches, unused for 7 days or less, is counted at each release.

If the toolchain restored from `~/.rustup/toolchains` is not taken by rustup (a `info: syncing channel updates` line before the first build despite a cache hit), the tools cache still saves the cargo tools' compilation; the toolchain path would then be dropped from it.
