# F30 — A CI on develop that blocks nothing

After each pull request merged into `develop`, a workflow runs the Rust tests and the coverage on Linux and saves the Rust caches that the review's CI into `main` reads; it is never a required check and blocks nothing ([ADR-0088](../adr/0088-one-workflow-per-purpose-the-ci-into-main-the-only-blocking-one.md), [ADR-0090](../adr/0090-the-develop-ci-beside-the-rust-caches-of-main.md)). A tooling feature, validated by the agent (VAL-031).

## How it starts

A merge made by the story auto-merge with GitHub's own token starts no workflow, so the auto-merge itself dispatches _Develop CI_ once the pull request reads `MERGED` (`workflow_dispatch`, `REF-GH-TOKEN-TRIGGERS`). A person's merge into `develop` starts it by its push. GitHub accepts a dispatch only once the workflow's file is on `main`: until the next merge into `main`, the auto-merge prints a warning instead of failing.

## What it guards

| Case | What happens |
| --- | --- |
| a Rust test fails on Linux only (F26's `kill`) | _Develop CI_ is red the day of the merge, not at the review |
| an install of the cargo tools fails | the tools are not saved, so the review never reads a cache without them (AUD-032) |
| a pull request into `develop` cannot be merged within the attempts | its auto-merge is disabled, a person merges it, and the push starts the CI |
| a job of any workflow calls a tool no earlier step gives | `[F30.AC2]` names the file, the job and the tool |

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. a non-blocking CI after each merge into `develop` | `[F30.AC1]` (the dispatch script run with a stub `gh`, the saves' conditions, no required check) | passed; its first real runs read at the review after the next merge into `main`, as the Product Owner decided |
| 2. each job finds its tools | `[F30.AC2]`, five hand breaks of real workflows each named | passed |

The F28 page's row for `rust-caches.yml` described it before F30: it now runs on a push to `main` only.
