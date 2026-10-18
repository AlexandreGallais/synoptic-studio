---
id: EN-068
epic: E17
feature: F19
title: The Cargo workspace at the root and the rust lane
status: done
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

## For the main thread

- **Proposed to the Product Owner, not applied** (settings change): the `limit-agents` hook in `.claude/settings.json` keeps its approved form, `cargo run --release -q --manifest-path ${CLAUDE_PROJECT_DIR}/tools/hooks/Cargo.toml --bin limit-agents`, which works inside the workspace. The proposed form, the same as the scripts and Git hooks: `"command": "cargo"`, `"args": ["run", "--release", "-q", "--manifest-path", "${CLAUDE_PROJECT_DIR}/Cargo.toml", "-p", "hooks", "--bin", "limit-agents"]` — the workspace's manifest instead of the crate's, so that renaming or moving the crate changes one name; same speed, same build.
- The scripts and Git hooks run each tool with `cargo run --release -q -p <crate> [--bin <binary>] --`, no launcher: measured on a built workspace, `cargo run` adds 0.03 to 0.05 s per call over the binary itself (`backlog-index` 0.03 to 0.04 s against 0.00 s; `heavy-lock -- true` 0.04 to 0.05 s), under the 0.3 s allowed. `prepare` keeps `rust:build`, so the first call is warm; `cargo run` rebuilds a tool whose crate changed. A nested `cargo build` under `cargo run -p heavy-lock` does not wait on Cargo's build lock (checked). The story's missing-binary criterion no longer arises: `cargo run` builds a missing binary.
- `unsafe_code = "forbid"` needed `heavy-lock` off its `unsafe` `flock` call: it takes the lock with `File::try_lock` now, the first task of EN-070 done here; its `std::os::unix` use for `is_at` stays for EN-070.
- `backlog-index` inherits `clippy::pedantic`, which it lacked: two findings fixed (an extension compared through `Path::extension`, an empty assertion through `assert_eq!`).
- `rust-toolchain.toml` makes rustup install the `1.99.1` toolchain beside `stable` at its first use in a checkout (about 6 components, once).
- `check:rust` measured on this machine: 11.2 s warm and 17.6 s cold before (eight crates one by one, no release build), 9.4 s warm and 14.1 s cold after (the workspace, release build included).
