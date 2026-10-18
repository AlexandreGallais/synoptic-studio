---
id: EN-136
epic: E17
feature: F30
title: Each CI job finds the tools its scripts call
status: ready
points: 2
---

# E17 · F30 · EN-136 — Each CI job finds the tools its scripts call

F24 split `check:all`'s lanes into jobs and checked that each lane runs once, not what a lane's tests need from another lane's setup: `[F19.AC9]`, in the test lane, called `cargo deny` and its advisory database, which only the Rust lane installed, and #370's CI failed with exit 101 (AUD-027, improvement row of 2026-10-18). The tools a job may lack are the ones its steps install: `cargo-llvm-cov`, `cargo-deny` and the RustSec database, the `git-hooks` binary passed as an artifact, Node.js and its packages. F28's `[F28.AC3]` (`tests/workflows.test.ts`) already checks the lanes of `ci.yml`: each cargo tool, the RustSec database, every target's crates and `npm ci` only in the lanes that call them, #370's case included; this story covers what it leaves out (AUD-030).

## Acceptance criteria

- Given each job of each workflow but `ci.yml`'s lanes (`rust-caches.yml`, `site.yml`, `release.yml`, `ci.yml`'s `history`), when `[F30.AC2]` reads it, then every tool called by its steps and the npm scripts they run is installed by a step of that job, or received as an artifact; a job lacking one fails naming the job and the tool.
- Given the `history` job of `ci.yml`, when `[F30.AC2]` reads it, then the `git-hooks` binary it runs is the artifact the lint lane uploads, under the same name and path, made executable; a job running it without the download fails naming the job.
- Given a job that runs a program npm installs or Node.js itself, when none of its steps sets Node.js up and installs the packages, then the test fails naming the job and Node.js.
- Given `[F28.AC3]`'s reading of the jobs, their steps and the cargo calls of the tests, when `[F30.AC2]` is written, then it extends that reading, never a copy of it.

## Tasks

- [ ] T1 — `[F30.AC2]` in `tests/workflows.test.ts`, the tools of the jobs `[F28.AC3]` does not read, the `git-hooks` artifact and Node.js, failing on a job lacking one (1 h)
- [ ] T2 — The tools' table and the steps it asks for, `docs/tooling/ci.md` (1 h)

One task = one commit, referenced as `EN-136.T1` in the commit footer.
