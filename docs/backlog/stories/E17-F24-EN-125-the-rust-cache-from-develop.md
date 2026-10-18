---
id: EN-125
epic: E17
feature: F24
title: The Rust cache saved from develop
status: ready
points: 2
---

# E17 · F24 · EN-125 — The Rust cache saved from develop

The Rust lane compiled every crate from nothing on GitHub's runner (over 21 min with coverage): the cache is saved only by a push to `main`, which has not happened since 0.2.0. Which branches' caches a pull request may restore is read from GitHub's documentation first.

## Acceptance criteria

- Given a push to `develop` or `main`, when its workflow runs, then the Rust build and its tests' artifacts are saved in a cache that a pull request into `main` may restore, as GitHub's documentation on cache scopes states (a `REF-` row read through the library).
- Given a pull request into `main` with a warm cache, when the Rust lane runs, then it compiles only what changed; its time warm and cold is written in `docs/tooling/ci.md`.

## Tasks

- [x] T1 — The cache's scope read and written; the job saving it on a push to `develop` and `main`, a test of the workflow's triggers failing first (1 h)
- [x] T2 — The Rust lane restoring it; the times measured (1 h)

One task = one commit, referenced as `EN-125.T1` in the commit footer.

## Decisions taken in the story

Written in `docs/tooling/ci.md`, The Rust build and its caches.

- **Which caches a pull request restores**, read through the library on GitHub's Dependency caching reference (`REF-GH-CACHE`, its row widened): a run restores the caches of its branch and of the default branch, a pull request's also those of its base branch; the page's example gives a pull request from `feature-b` into `feature-a` "the current `feature-b` branch" too, and its search order for a pull request from `feature` into `main` searches `feature` first, then `main`. So a pull request from `develop` into `main` restores a cache saved by a push to `develop`: the design asked works, with no merge ref nor other branch involved. The `actions/cache` README (Cache scopes) says only "scoped to the key, version, and branch", the default branch's available to others; it does not contradict the page, nor is it an independent source, being GitHub's too.
- **The job stays in `release.yml`**, its trigger widened to `push: branches: [develop, main]`, rather than a workflow of its own: the checklist of F22 and its guide page read `rust-cache` and `docs` as the push's jobs, which stays true. `rust-cache` runs on every push (`if: github.event_name == 'push'`); `docs` now on a push to `main` only (`&& github.ref_name == 'main'`), so a push to `develop` deploys nothing; `release` is unchanged (the button on `main`).
- **The Rust lane is unchanged**: it already restored the build by its exact key, then by the prefix `rust-build-<os>-`, which a new `Cargo.lock` needs; a test now holds that `restore-keys` (`[F24.AC3]`).
- **The cold time** is the one of run 39131147001 (attempt 1), from its log: `check:rust` not ended after 26 min 07 s, cancelled in its coverage step, after 2 min 50 s of cargo tools' install. `ci.md`'s "over 21 min" of EN-124 became "over 26 min", the time the log gives.

## Evidence

- `npx vitest run tests/ci-caches.test.ts tests/release-flow.test.ts tests/references.test.ts`: 21 passed.
- At T1, before `release.yml` changed, the four tests of the triggers failed: `[F21.AC4] [F24.AC3] saves the same caches on a push to develop or main…`, `[F22.AC2] [F24.AC3] runs on pushes to develop and main and by its button`, `[F22.AC2] deploys the site on a push to main, never on develop nor by the button`, `[F22.AC2] [F24.AC3] fetches every target's crates on develop and main…`.
- At T2, `[F24.AC3] restores in the CI the build of an older key by its prefix…` failed with the `restore-keys` line of `ci.yml` removed, and passed once it was restored.
- The warm time can only be measured on GitHub: it comes with the run of #370 after the push to `develop` that brings EN-125, once `rust-cache` has saved the caches.

## For the main thread

- **ADR-0072, The run, decision 1** (and ADR-0038's decision 4, which it supersedes) says no workflow runs on a push to `develop`; F24's criterion 3, decided at REV-006 on 2026-10-18, has a push to `develop` save the Rust caches. The job checks nothing and nobody waits for it, but the ADR's text no longer holds: a new ADR partly superseding that decision (a push to `develop` runs `rust-cache` alone, which checks nothing) is the main thread's to write, or the criterion goes back to the Product Owner. `docs/tooling/ci.md` already says what runs.
- **The warm time**: once the feature is in `develop`, its push starts `rust-cache` and #370's CI at the same time, so that run is cold; the next run of #370 (a re-run, or its next push) restores `develop`'s caches. Its Rust job's restore step names the key restored, and its `check:rust` step gives the warm time, to write in `docs/tooling/ci.md` (The Rust lane's time, cold and warm). A miss there would mean the head branch's caches are not restored as the page states: then the design falls back on `main`'s caches, saved at the next push to `main`.
- **Each push to `develop` runs `rust-cache` whole**, about the cold build when its key is new; on an exact hit it saves nothing yet still compiles. Skipping its build steps on `steps.<id>.outputs.cache-hit == 'true'` would spare those minutes; left for a review, since nobody waits for it.
- `tools/repo-tools/tests/check_all.rs` names `f24_ac3_requires_on_main_every_job_of_the_ci_and_codeql_by_its_name` (EN-124), which proves criterion 1 (the checks `main` requires), not criterion 3 (the cache); a rename to `f24_ac1_…` is left for a review.
- The `done` commit's footer: `Rule-docs: none affected, the CI's workflows, their tests and docs only, no rule of docs/domain/`.
