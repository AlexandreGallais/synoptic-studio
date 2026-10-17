---
id: EN-069
epic: E17
feature: F19
title: A coverage floor and an advisory check for the Rust tools
status: ready
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
