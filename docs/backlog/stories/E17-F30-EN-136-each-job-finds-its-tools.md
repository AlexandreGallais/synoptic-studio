---
id: EN-136
epic: E17
feature: F30
title: Each CI job finds the tools its scripts call
status: ready
points: 2
---

# E17 · F30 · EN-136 — Each CI job finds the tools its scripts call

F24 split `check:all`'s lanes into jobs and checked that each lane runs once, not what a lane's tests need from another lane's setup: `[F19.AC9]`, in the test lane, called `cargo deny` and its advisory database, which only the Rust lane installed, and #370's CI failed with exit 101 (AUD-027, improvement row of 2026-10-18). The tools a job may lack are the ones its steps install: `cargo-llvm-cov`, `cargo-deny` and the RustSec database, the `git-hooks` binary passed as an artifact, Node.js and its packages.

## Acceptance criteria

- Given each job of each workflow, when `[F30.AC2]` reads it, then every tool called by its scripts, by the npm scripts they run, and by the tests of its lane (a test file that starts `cargo deny` or `cargo llvm-cov`) is installed by a step of that job, or received as an artifact; a job lacking one fails naming the job and the tool.
- Given the test lane's job without its `cargo deny fetch db` step, when the test runs, then it fails naming `TypeScript tests` and the RustSec database (the case of #370, broken once on purpose).
- Given a tool added to a script, when no job of its lane installs it, then the test fails before any push reaches GitHub.

## Tasks

- [ ] T1 — `[F30.AC2]` in `tests/workflows.test.ts`, the tools each script and test calls, failing on #370's case (1 h)
- [ ] T2 — The tools' table and the steps it asks for, `docs/tooling/ci.md` (1 h)

One task = one commit, referenced as `EN-136.T1` in the commit footer.
