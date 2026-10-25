# ADR-0087 — The CI in parallel jobs, its Rust cache saved by a push to `develop`

**Status**: Accepted (Product Owner, 2026-10-18, REV-006, F24); decision 2 superseded by ADR-0088 (2026-10-18): the auto-merge's pushes to `develop` start no workflow, so the cache is saved by the `develop` CI it dispatches. Supersedes, in ADR-0072 (The run, decision 1), "no workflow runs on … a push to `develop`" for one job: the Rust cache's, which checks nothing and which no one waits for. The rest of that decision stands: the checks of work branches run locally, and the one CI is the pull request into `main`. Implemented by EN-124 and EN-125.

## Context

- On GitHub's runner the CI's single job ran `check:all` with nothing visible until its end: `lint` 139 s, `docs:build` 265 s, `test` 326 s, then `check:rust` still compiling every crate from nothing after 26 min (run 39131147001), its cache saved only by a push to `main`, which had not happened since 0.2.0.
- A pull request's run restores the caches of its own branch, its head branch included, of its base branch and of the default branch (`REF-GH-CACHE`).

## Decision

1. **One job per lane of `check:all`**, in parallel, each its own check on the pull request into `main`, a lane failing leaving the others' results; the TypeScript tests reported file by file, a failure annotated at its line (`REF-VITEST-REPORTERS`).
2. **The Rust cache is saved by a push to `develop` as to `main`** (`release.yml`'s `rust-cache` job), so that the pull request `develop` → `main` compiles only what changed. A push to `develop` deploys nothing and releases nothing.

## Reasons

- A lane seen passing or failing on its own tells the Product Owner where the CI is, and a failure is read without waiting for the slowest lane.
- The cache costs runner time on each push to `develop`, which no one waits for, against the pull request into `main`'s 26 min of compiling, which the Product Owner waits for.

## Consequences

- `main`'s ruleset requires the five jobs' names (`docs/tooling/ci.md`).
- The warm time of the Rust lane is measured at the first CI of `develop` → `main` that restores `develop`'s cache.

## References

ADR-0072, ADR-0086, ADR-0101 (ADR-0038, removed), `REF-GH-CACHE`, `REF-VITEST-REPORTERS`, `REF-GH-CONTEXTS`
