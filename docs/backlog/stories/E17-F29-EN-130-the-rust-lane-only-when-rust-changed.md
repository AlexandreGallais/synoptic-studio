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
