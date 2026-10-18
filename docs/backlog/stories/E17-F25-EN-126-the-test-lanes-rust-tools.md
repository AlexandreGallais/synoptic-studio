---
id: EN-126
epic: E17
feature: F25
title: The test lane's Rust tools
status: ready
points: 1
---

# E17 · F25 · EN-126 — The test lane's Rust tools

`[F19.AC9]` (`tests/rust-advisories.test.ts`) runs `cargo deny check advisories` from the TypeScript tests; since EN-124 only the `Rust` job installs `cargo-deny` and fetches the RustSec database, so the `TypeScript tests` job got exit 101 (run 39138216016).

## Acceptance criteria

- Given the `TypeScript tests` job, when its tools key is missed, then it installs `cargo-deny` and fetches the advisory database before its scripts, as the `Rust` job does; a test of `ci.yml` checks it.

## Tasks

- [x] T1 — The test of `ci.yml`, failing (main thread)
- [ ] T2 — `ci.yml`'s two conditions (main thread)

One task = one commit, referenced as `EN-126.T1` in the commit footer.
