---
id: EN-131
epic: E17
feature: F29
title: The binaries rebuilt after a pull
status: done
points: 2
---

# E17 · F29 · EN-131 — The binaries rebuilt after a pull

A pull request merged on GitHub leaves the main checkout's release binaries older than their sources: at REV-004, `target/release/guard-bash` predated EN-115, since only `story-finish` and `git-sequences sync` rebuild them (card of 2026-10-18, C9, I112). The hooks, the status line and the agent call those binaries by their path (ADR-0078).

## Acceptance criteria

- Given a checkout whose pull, by a merge or a fast-forward (`post-merge`) or by a rebase (`post-rewrite` after `rebase`), brings a change of a Rust input (EN-130's list), when the pull ends, then the release binaries are rebuilt (`npm run rust:build`) and the hook prints the binaries it rebuilt; `target/release/stale-binaries` then names none.
- Given a pull that changes no Rust input, when it ends, then the hook builds nothing and prints nothing.
- Given a build that fails, when the hook runs, then it prints the failure and the command to run again, and exits 0: the pull is done and a post hook cannot undo it.

## Tasks

- [ ] T1 — The `post-merge` and `post-rewrite` hooks of `tools/git-hooks`, their tests failing (1 h)
- [ ] T2 — The hooks in `.githooks/`, set by `git-hooks install`, `docs/tooling/git-workflow.md` updated (1 h)

One task = one commit, referenced as `EN-131.T1` in the commit footer.

## For the main thread

- ADR-0078, decision 2, lists where a stale binary is rebuilt (`prepare`, `git:sync`, `story:finish`), and ADR-0080, decision 1, names three hooks in `.githooks/`; EN-131 adds a fourth rebuild and two hooks, `post-merge` and `post-rewrite`. Neither ADR was rewritten: a new ADR superseding those two points, or none if the main thread judges them extended rather than changed, is its call.
- Technical choices made by the worker: the changed files are read with `git diff --name-only ORIG_HEAD HEAD` (`REF-GIT-REV-PARSE`), so a squash merge, which moves no `HEAD`, builds nothing; the hook rebuilds on EN-130's whole list (`RUST_INPUTS`), a change of `package.json` alone running a Cargo build that writes nothing and prints nothing; the binaries named are those whose file time changed during the build. `RUST_INPUTS`, `touches` and `touches_rust` moved from `repo-tools` to `stale-binaries`, re-exported by `check_all`: `git-hooks` depending on `repo-tools` made a cycle (`hooks` → `worktree` → `git-hooks`).
- A `git-sequences sync` in a worktree now runs the `post-rewrite` rebuild during its rebase, then its own rebuild, which finds nothing to do.
