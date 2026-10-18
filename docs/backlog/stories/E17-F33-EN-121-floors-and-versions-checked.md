---
id: EN-121
epic: E17
feature: F33
title: The floors, versions and paths checked
status: ready
points: 2
---

# E17 · F33 · EN-121 — The floors, versions and paths checked

From RET-004's "Checks to add" (I37, I54, I55, I56, I58, I60, I81, I99, P8). Moved from F23 on 2026-10-18 (SP-024): its builds and lanes went to F29 (EN-129, EN-130, EN-131), the checks of the docs and the agent pages to EN-140, the `bench` binary to EN-141. Today `rust:coverage` fails under 93 % of lines and functions (`package.json`), and `lint` and `fix` pass `--type-check` to oxlint on their command line.

## Acceptance criteria

- Given `rust:coverage`, when the workspace's coverage falls under 95 % of lines or of functions, then it fails, its message naming the floor; `.claude/rules/tooling.md` and `docs/tooling/commands.md` say 95 %.
- Given `.oxlintrc.json`, when `tests/oxlint-config.test.ts` reads it, then `typeCheck` is set there, and no script passes `--type-check` beside it.
- Given the cargo tools' versions (`CARGO_LLVM_COV_VERSION`, `CARGO_DENY_VERSION` in the workflows, the commands of `docs/tooling/commands.md`), when `deps:outdated` runs, then each is checked against crates.io and `docs/tooling/dependencies.md` as a crate is, a newer release 7 days old reported.
- Given `.vscode/settings.json`, when a test reads it, then its TypeScript path names the installed TypeScript 7 folder, which exists.

## Tasks

- [ ] T1 — Each criterion's test, failing (1 h)
- [ ] T2 — The floor, the configuration, the versions' check and the path, the docs (2 h)

One task = one commit, referenced as `EN-121.T1` in the commit footer.
