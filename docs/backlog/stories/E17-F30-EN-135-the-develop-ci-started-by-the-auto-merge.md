---
id: EN-135
epic: E17
feature: F30
title: The develop CI, started by the auto-merge
status: ready
points: 3
---

# E17 · F30 · EN-135 — The develop CI, started by the auto-merge

The Rust caches' workflow F28 leaves on a push to `develop` never runs after the auto-merge, whose push by the `GITHUB_TOKEN` starts no workflow; `workflow_dispatch` always does, on the branch dispatched, and a dispatched run saves its caches in that branch's scope, the one the pull request `develop` → `main` restores first (`REF-GH-TOKEN-TRIGGERS`, `REF-GH-WORKFLOW-DISPATCH`, `REF-GH-CACHE`). `workflow_run` is set aside: it runs `main`'s file on `main`'s commit and cannot save its caches (research note 0026, §1). `gh pr merge --auto` merges at once only a pull request already mergeable (`REF-GH-CLI-MERGE`).

## Acceptance criteria

- Given the `develop` CI, when `[F30.AC1]` reads it, then its triggers are `push` to `develop` and `workflow_dispatch`, it has no `if:` on the event or a branch, it runs `check:rust` on Linux, the Rust tests under coverage included, and then saves the toolchain's, the cargo tools' and the build's caches under the keys `tests/ci-caches.test.ts` holds, and no ruleset nor workflow waits for it (`docs/tooling/ci.md`).
- Given the features' auto-merge into `develop`, when its merge is done (`gh pr view` reads `MERGED`, waited for after `--auto`), then it runs `gh workflow run <the develop CI's file> --ref develop` with `actions: write` and nothing else; a merge not done after its retries dispatches nothing and fails, naming the pull request.
- Given a merge into `develop` by a person, when it is pushed, then the `develop` CI runs once, by its `push`, and the auto-merge dispatches nothing.
- Given the first merge into `main` that brings the workflow (`REF-GH-WORKFLOW-DISPATCH`: a dispatch needs the file on the default branch), when the next feature enters `develop`, then one run of the `develop` CI starts, and the review's CI after it restores its caches with no `cargo install`; the review after that merge reads both runs (VAL-031).

## Tasks

- [ ] T1 — `[F30.AC1]` in `tests/workflows.test.ts` and `tests/ci-caches.test.ts`, failing (1 h)
- [ ] T2 — The `develop` CI from the Rust caches' workflow, the dispatch in the features' auto-merge, `docs/tooling/ci.md` (2 h)

One task = one commit, referenced as `EN-135.T1` in the commit footer.

## For the main thread

### Journal

- `#run` EN-135: the `develop` CI is `develop-ci.yml`, a workflow of its own (push to `develop`, `workflow_dispatch`), and `rust-caches.yml` keeps `main`'s caches alone, on a push to `main`: joined, one file would run the tests after each merge into `main` too, which ADR-0072 never re-checks, or choose by an `if:` on the branch that ADR-0088 forbids. Its job runs `check:rust` then `rust:coverage`, and saves the caches by `actions/cache/save` even after a failing step (`REF-GH-CACHE-SAVE`), so a red run on Linux leaves the review's CI warm; the auto-merge waits for `MERGED` (`REF-GH-CLI-PR-VIEW`), then dispatches into `develop` only, with `actions: write` added.
