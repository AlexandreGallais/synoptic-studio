---
id: EN-068
epic: E17
feature: F19
title: The Cargo workspace at the root and the rust lane
status: ready
points: 3
---

# E17 · F19 · EN-068 — The Cargo workspace at the root and the rust lane

Gather the Rust crates of F18 into one virtual Cargo workspace at the root, with a pinned toolchain, warnings denied and a `rust` lane in `check:all`, so that every Rust tool of F19 and F21 is built and checked the same way. Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "The Rust layout" and "Answers to the spike's questions"; `REF-CARGO-WORKSPACES`, `REF-CLIPPY-USAGE`, `REF-RUSTUP-TOOLCHAIN`.

## Acceptance criteria

- Given the root of the repository, when the story ends, then a `Cargo.toml` with `[workspace]`, `members = ["tools/*"]` and `resolver = "3"` holds no package, `[workspace.package]` sets edition 2024, licence Apache-2.0 and `publish = false`, `[workspace.lints]` sets `clippy::pedantic` and `unsafe_code = "forbid"`, and `[workspace.dependencies]` pins exact versions (F19.AC9).
- Given the eight crates of `tools/` (`backlog-index`, `heavy-lock`, `hooks`, `mutate`, `rule-docs`, `shared-files`, `val-prepare`, `worktree`), when the workspace builds, then each is a member with `[lints] workspace = true`, one `Cargo.lock` and one `target/` at the root, the crates' own lock files and `target/` folders gone, `target/` ignored by Git and by every check tool (F19.AC9).
- Given `rust-toolchain.toml` with `channel = "1.99.1"` and the `rustfmt` and `clippy` components, and `.cargo/config.toml` with `[build] warnings = "deny"`, when a warning is planted in a crate, then the build fails (F19.AC9).
- Given `check:all`, when it runs, then a `rust` lane runs `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked` and `cargo test --workspace --locked`, its time printed with the other lanes, and the hooks and scripts that call a binary call `target/release/<tool>`, built from `prepare` (F19.AC9).
- Given a tool whose binary is missing, when it is called, then it fails with a message naming the build command.

## Tasks

One task = one commit, referenced as `EN-068.Tn`.

- [ ] T1 — The root `Cargo.toml`, `rust-toolchain.toml` and `.cargo/config.toml`; the eight crates moved in, their lock files and `target/` folders removed (3 h)
- [ ] T2 — The binaries' paths in `package.json`, `.claude/settings.json` and the husky hooks; `prepare` building them; the missing-binary message (2 h)
- [ ] T3 — The `rust` lane in `check-all.ts`, its explicit targets (EN-045), `target/` ignored by every check tool (2 h)
