---
id: EN-130
epic: E17
feature: F29
title: The Rust lane only when Rust changed, its coverage a lane
status: ready
points: 2
---

# E17 · F29 · EN-130 — The Rust lane only when Rust changed, its coverage a lane

`check:rust` is `check:all`'s longest lane (56.7 s at a load average of 19.6, I97), run on every push, a story of Markdown alone included; its coverage runs inside it, after rustfmt, Clippy and the advisories (I106). `check:all` already leaves the docs lane out when the branch changed none of its inputs against its base (`tools/repo-tools/src/check_all.rs`, `DOCS_INPUTS`, EN-085): the Rust lane follows the same rule.

## Acceptance criteria

- Given a branch whose changes against its base touch none of the Rust inputs (`tools/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `deny.toml`), when `check:all` runs, then the Rust lanes are skipped with the line `- check:rust, … skipped: nothing changed in their inputs against <base>`; with no base known (the CI, a detached head), they run.
- Given a change of one Rust input, when `check:all` runs, then the Rust coverage, `cargo llvm-cov` with its floor, runs as a lane of its own beside rustfmt, Clippy and the advisories, and `ci.yml`'s matrix holds a job for it (`[F24.AC1]` reads every lane against the matrix).
- Given a new crate in `tools/`, when the Rust inputs are read, then it is one of them without a change of the list; a file added at the root that Cargo reads is named by a test that fails until it joins the list.

## Tasks

- [ ] T1 — The Rust inputs and the lanes' tests in `tools/repo-tools/tests/check_all.rs`, failing (1 h)
- [ ] T2 — The Rust lanes skipped without a Rust change, the coverage a lane, `ci.yml`'s matrix and `docs/tooling/commands.md` updated (2 h)

One task = one commit, referenced as `EN-130.T1` in the commit footer.

## For the main thread

- The new CI check `Rust coverage` must join the required checks of `main`'s ruleset on GitHub, a setting the owner changes ([CI](../../tooling/ci.md#repository-settings-done-once-by-the-owner)); `docs/tooling/ci.md` already lists it, which `[F24.AC1]` reads against the jobs.
- Technical choices made by the worker: the Rust inputs are Cargo's (`tools/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `deny.toml`) and the files the crates' tests read outside `tools/` — `package.json`, `package-lock.json`, `.oxfmtrc.json`, `.github/workflows/`, `docs/tooling/ci.md`, `docs/conventions/files.md`, `.claude/settings.json`, `playground/checklists/` and every `*.test.ts` — since skipping is allowed only when the lane has nothing to check ([lanes](../../tooling/commands.md#the-lanes-of-the-full-check)). The last one, read by `[F19.AC9]`'s scan of the binaries a test starts, makes every story changing a TypeScript test run the Rust lanes: a story of Markdown or of a script skips them, a library story does not.
- The CI's Rust lane, whose `check:rust` no longer runs the tests, installs no npm package: a field `node-packages` chooses `npm ci` for the five other lanes and `release-build` its own `npm run rust:build` (`[F28.AC3]`).
- `[F19.AC9]` reads "a `rust` lane runs … the tests, a coverage floor": they now run in a second lane beside it, as `[F29.AC2]` asks.

### Journal

- `#run` EN-130: `check:all` skips `check:rust` and `rust:coverage` together, saying so, when the branch changed no Rust input against its base (Cargo's inputs and the files the crates' tests read outside `tools/`, every `*.test.ts` among them); the coverage is a lane of its own and the CI's job `Rust coverage`; every root entry is named a Rust input or not, so a `clippy.toml` added fails until classified.

### Improvements

| 2026-10-18 | EN-130 | Narrow `[F19.AC9]`'s scan of the test files starting a release binary to `tests/` (the only folder holding one, `tests/declarations.test.ts`), so that `*.test.ts` leaves the Rust inputs and a library story skips the Rust lanes too | tool |
