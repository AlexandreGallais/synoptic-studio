---
id: EN-111
epic: E17
feature: F21
title: The CI on the Rust tools, built and cached once
status: draft
points: 2
---

# E17 · F21 · EN-111 — The CI on the Rust tools, built and cached once

The pull request into `main` is the only CI run (ADR-0038, `docs/tooling/ci.md`). Once the checks, the hooks and the dependency tools are Rust binaries (EN-103 to EN-110), each job that calls them builds the workspace from nothing, and `cargo-llvm-cov` and `cargo-deny` are compiled at each run (`cargo install`). The CI builds the workspace once per run, caches the build and the installed cargo tools between runs, and every job and the release workflow use them (F21.AC4). Before the release story (EN-112), whose publishing job runs a Rust binary on `main`.

## Acceptance criteria

- Given a pull request into `main`, when the CI runs, then the workspace is built once in release mode, its `target/` and Cargo's registry cached by the official `actions/cache` keyed on `Cargo.lock` and `rust-toolchain.toml`, and `check:all`, `history` and `build` use that build.
- Given `cargo-llvm-cov` and `cargo-deny`, when the CI needs them, then they come from a cache keyed on their versions, installed only when the key changes.
- Given a second run on an unchanged `Cargo.lock`, when it starts, then no crate of the workspace's dependencies is compiled again, shown by the job's log.
- Given the workflows, when the story ends, then no step calls a dev package the earlier stories removed, and `docs/tooling/ci.md` describes the build and its caches.
- Given the CI, when it is timed, then a run's duration, cold and with its caches, is stated before and after.

## Files

`.github/workflows/ci.yml`, `.github/workflows/release.yml` (`docs` job), `docs/tooling/ci.md`, `docs/references.md` (the caching page of GitHub's documentation, read through the library).

## Measures (ADR-0069)

Before and after: the CI's duration on a pull request into `main`, each job's, cold and with its caches (the run's own timings on GitHub); the cache's size.

## Tasks

One task = one commit, referenced as `EN-111.Tn`.

- [ ] T1 — The workspace built once and cached, the cargo tools cached by version; GitHub's cache documentation read and cited (3 h)
- [ ] T2 — Each job on the cached build; the docs; a cold and a warm run timed by `workflow_dispatch` on the story's branch, the CI given that trigger (2 h)
