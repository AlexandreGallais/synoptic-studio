---
id: EN-069
epic: E17
feature: F19
title: A coverage floor and an advisory check for the Rust tools
status: done
points: 2
---

# E17 · F19 · EN-069 — A coverage floor and an advisory check for the Rust tools

Give the Rust tools a coverage floor and check their crates against the RustSec advisories, as the Product Owner decided on 2026-10-17. Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Rust coverage and advisories"; `REF-CARGO-LLVM-COV`, `REF-CARGO-DENY-ADVISORIES`, `REF-CARGO-AUDIT` (cargo-deny chosen over it).

## Acceptance criteria

- Given the `rust` lane, when it runs, then `cargo llvm-cov` measures the workspace's tests and fails under the floor of lines and of functions set in this story from the measure, the floor written in `docs/tooling/` (F19.AC9).
- Given the `rust` lane in the pre-push, when it runs, then `cargo deny check advisories --offline` checks every crate of the root `Cargo.lock`; the CI on the pull request into `main` runs it with the database fetched (F19.AC9).
- Given `cargo-llvm-cov` (0.9.1 or later), `cargo-deny` (0.20.2 or later) and rustup's `llvm-tools-preview` component, when the story ends, then each has its row in `docs/tooling/dependencies.md`, its release notes read, a version at least 7 days old, and the lane's added time is measured and recorded.
- Given a crate with a known advisory planted in a copy, when the check runs, then it fails naming the advisory.

## Tasks

One task = one commit, referenced as `EN-069.Tn`.

- [ ] T1 — `cargo-llvm-cov` and its component installed, the workspace measured, the floor set and the lane failing under it (2 h)
- [ ] T2 — `cargo deny check advisories`, offline in the pre-push and online in the CI, a planted advisory caught (2 h)
- [ ] T3 — The rows in `docs/tooling/dependencies.md`, the lane's time measured and recorded (1 h)

## For the main thread

- **Floor**: 93 % of lines and 93 % of functions of the whole workspace, the measure rounded down. Measured first at 93.40 % of 2,773 lines and 93.28 % of 461 functions; after the rebase on EN-070 and EN-073, 93.88 % of 3,299 lines and 93.45 % of 550 functions, so the floor holds unchanged. Per crate after the rebase, lines then functions: `backlog-index` 98.5 / 98.6, `format` 96.6 / 94.3, `heavy-lock` 92.6 / 100, `hooks` 97.9 / 96.6, `mutate` 82.6 / 74.0 (its `main.rs` untested), `rule-docs` 94.0 / 100, `shared-files` 96.3 / 98.9, `val-prepare` 97.8 / 95.1, `worktree` 87.6 / 83.8. cargo-llvm-cov's floor is the workspace's total only; a per-crate floor would need a run per crate.
- **cargo-deny 0.20.2** takes `--offline` before `check` (`cargo deny --offline check advisories`): the form `cargo deny check advisories --offline` of the story is refused since its command-line refactor. `deny.toml` at the root keeps the default database and makes unmaintained and yanked crates fail.
- **The planted advisory** is a test, `tests/rust-advisories.test.ts`: smallvec 1.6.0 is added to a copy of `cargo metadata`'s output, passed with `--metadata-path`, so nothing is downloaded; the check fails naming RUSTSEC-2021-0003. It needs cargo-deny and its database on the machine running `npm run test`, as the CI now installs.
- **Time**: `check:rust` about 17 to 20 s on a built workspace against 11 to 14 s with `cargo test` and EN-070's Windows check (four runs each, the machine loaded by other worktrees); the advisories take 0.3 s, the instrumented build and run of the tests the rest. check:all's total is unchanged: the test lane stays the slowest.
- **Installed on this machine** (for the next worktrees): rustup's `llvm-tools`, `cargo install --locked cargo-llvm-cov@0.9.1 cargo-deny@0.20.2` (about 3 minutes), the RustSec database already under `~/.cargo/advisory-dbs/`. The CI job `quality` adds the same `cargo install` and `cargo deny fetch db` before `check:all`, not yet seen running.
- `deps:outdated` and `deps:tools` do not see the two cargo tools: their versions are checked by hand (`docs/tooling/dependencies.md`, Rust tools).
