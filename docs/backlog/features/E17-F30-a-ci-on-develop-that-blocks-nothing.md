---
id: F30
epic: E17
title: A CI on develop that blocks nothing
status: done
---

# E17 · F30 — A CI on develop that blocks nothing

**Benefit hypothesis**: after each pull request merged into `develop`, GitHub runs the Rust tests on Linux and saves the Rust caches, so a fault the Mac's checks cannot see shows up the same day without stopping the run, and the review's CI into `main` compiles only what changed.

Decided by the Product Owner on 2026-10-18 (ADR-0088, decisions 4 and 5). The auto-merge merges with the built-in token, whose push starts no workflow: eight merges into `develop` that day started none, so `release.yml`'s `rust-cache` job never ran there and the review's CI installed the cargo tools again (2 min 47 s in the Rust lane). The docs give one way: a `workflow_dispatch` sent by the auto-merge once its merge is done, which runs on `develop` and saves its caches there (`REF-GH-TOKEN-TRIGGERS`, `REF-GH-WORKFLOW-DISPATCH`, `REF-GH-CACHE`). With it, the test AUD-027 asked for: a lane's tests needed `cargo-deny`, which only another lane installed (#370). A tooling feature, validated by the agent (ADR-0072); sources in [research note 0026](../../research/notes/0026-the-tooling-features-of-2026-10-18.md), §1.

## Acceptance criteria

1. After each pull request merged into `develop`, by the auto-merge or by a person, the `develop` CI runs once on `develop`'s head: the Rust tests on Linux, then the Rust caches saved; no ruleset requires it and nothing waits for it (EN-135).
2. Each job of each workflow installs, or receives, every tool its scripts and their tests call; a job that lacks one fails a test naming the job and the tool (EN-136).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-135](../stories/E17-F30-EN-135-the-develop-ci-started-by-the-auto-merge.md) | EN | The develop CI, started by the auto-merge | done |
| [EN-136](../stories/E17-F30-EN-136-each-job-finds-its-tools.md) | EN | Each CI job finds the tools its scripts call | done |
| [AUD-032](../stories/E17-F30-AUD-032-audit-f30.md) | AUD | Audit F30 | done |
| [VAL-031](../stories/E17-F30-VAL-031-validate-f30.md) | VAL | Validate F30 | done |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-135 | AUD-032, VAL-031 |
| 2 | EN-136 | AUD-032, VAL-031 |
