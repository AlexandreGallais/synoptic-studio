# ADR-0090 — The `develop` CI in its own workflow, beside the Rust caches of `main`

**Status**: Accepted (the agent, 2026-10-19, AUD-032, from F30's audit; to be told to the Product Owner). Supersedes ADR-0088's second consequence, "F30 makes the Rust caches' workflow the `develop` CI": F30 kept `rust-caches.yml` for `main` and added `develop-ci.yml`. The rest of ADR-0088 stands.

## Context

- ADR-0088's second consequence says F30 turns the Rust caches' workflow into the `develop` CI.
- EN-135 kept `rust-caches.yml`, run on a push to `main`, and added `develop-ci.yml`, run on a push to `develop` and by the auto-merge's dispatch (`docs/tooling/ci.md`, The develop CI).
- `rust-caches.yml` compiles what the CI compiles and checks nothing, on `main`, after the review's CI checked that code (ADR-0072); it saves `main`'s caches for the site and the release. The `develop` CI runs the Rust checks and tests on `develop` and saves `develop`'s caches, the scope the review's pull request reads first (`REF-GH-CACHE`).

## Decision

1. **The `develop` CI is a workflow of its own**, `develop-ci.yml`, on `develop` only; `rust-caches.yml` keeps the Rust caches of `main`, on a push to `main` only, running no test.

## Reasons

- One workflow per purpose (ADR-0088, decision 1): the tests after each merge into `develop`, and the caches of `main` without a test, are two purposes.
- One file for both would run on `develop` and on `main`, and choosing its tests by the branch is an `if:` that ADR-0088, decision 2, forbids; without it, every merge into `main` would run the tests again, where nothing is checked again after the review's CI (ADR-0072).

## Consequences

- `docs/tooling/ci.md` describes both workflows, and `tests/ci-caches.test.ts` holds their caches to the same keys and paths as `ci.yml`'s.
- The F28 guide's row of `rust-caches.yml`, written before EN-135, names a push to `develop` too: the F30 guide, written by VAL-031, says so.

## References

ADR-0072, ADR-0088, `REF-GH-CACHE`, EN-135, AUD-032, `docs/tooling/ci.md`
