---
id: EN-125
epic: E17
feature: F24
title: The Rust cache saved from develop
status: ready
points: 2
---

# E17 · F24 · EN-125 — The Rust cache saved from develop

The Rust lane compiled every crate from nothing on GitHub's runner (over 21 min with coverage): the cache is saved only by a push to `main`, which has not happened since 0.2.0. Which branches' caches a pull request may restore is read from GitHub's documentation first.

## Acceptance criteria

- Given a push to `develop` or `main`, when its workflow runs, then the Rust build and its tests' artifacts are saved in a cache that a pull request into `main` may restore, as GitHub's documentation on cache scopes states (a `REF-` row read through the library).
- Given a pull request into `main` with a warm cache, when the Rust lane runs, then it compiles only what changed; its time warm and cold is written in `docs/tooling/ci.md`.

## Tasks

- [x] T1 — The cache's scope read and written; the job saving it on a push to `develop` and `main`, a test of the workflow's triggers failing first (1 h)
- [ ] T2 — The Rust lane restoring it; the times measured (1 h)

One task = one commit, referenced as `EN-125.T1` in the commit footer.
