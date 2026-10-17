---
id: EN-070
epic: E17
feature: F19
title: Rust tools on portable standard calls
status: ready
points: 2
---

# E17 · F19 · EN-070 — Rust tools on portable standard calls

Replace the Unix-only parts of the F18 crates by Rust's portable standard calls where one exists, and check the workspace for Windows without linking, a goal and not a blocker (Product Owner, 2026-10-17). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Platforms"; `REF-RUST-FILE-LOCK` (`File::lock` and `File::try_lock`, stable since Rust 1.89.0).

## Acceptance criteria

- Given `heavy-lock`, when it takes and releases the lock, then it uses `File::lock` and `File::try_lock` and holds no `extern "C"`, no `std::os::unix` and no `unsafe` block; its tests pass unchanged (F19.AC10).
- Given `mutate`, `worktree` and `rule-docs`, when they stop a process, link a folder or set a file's permissions, then they use a standard call where one exists, and each call left Unix-only is behind `cfg(unix)` with its reason in a comment (F19.AC10).
- Given the `rust` lane, when it runs, then `cargo check --workspace --target x86_64-pc-windows-msvc` compiles the workspace without linking, and a Unix-only call planted outside `cfg(unix)` fails it on the Mac; the cost of the target and of the pass is measured and recorded (F19.AC10).

## Tasks

One task = one commit, referenced as `EN-070.Tn`.

- [x] T1 — `heavy-lock` on `File::lock`, its `unsafe` block removed (2 h) — the lock itself moved to `File::try_lock` with `unsafe_code` forbidden in EN-068; left here: the file identity check (`dev`, `ino`), now behind `cfg(unix)`, stable Rust having no file index on Windows
- [ ] T2 — `mutate`'s `kill`, the symlinks of `mutate` and `worktree`, `rule-docs`'s test permissions on standard calls or behind `cfg(unix)` (3 h)
- [ ] T3 — The Windows target installed, `cargo check` for it in the `rust` lane, a planted call caught, the cost recorded (1 h)
