---
id: AUD-030
epic: E17
feature: F28
title: Audit F28
status: ready
points: 1
---

# E17 · F28 · AUD-030 — Audit F28

Audit F28 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F28's criteria, when the audit ends, then each is traced to a test titled `[F28.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [ ] T1 — The findings, and the failing tests of F3, F4 and F10 (1 h)
- [ ] T2 — The findings fixed, the criteria traced (1 h)

One task = one commit, referenced as `AUD-030.T1`.

## Findings

Reviewed by the `auditor` alone (ADR-0086), each finding reproduced before it is recorded.

| Id | Check | Result | Evidence | Action |
| --- | --- | --- | --- | --- |
| F1 | E3 — `ci.md`'s table of the removed steps gives the build inside `npm ci` as 51 s (tests) and 49 s (Rust) | reproduced: those are `cargo install`'s; cargo's `Finished` lines inside `npm ci` say 2 min 07 s and 2 min 04 s | jobs 117729253929 and 117729253867 of run 39145798246 | the numbers fixed (T2) |
| F2 | E3 — F28's criterion 3 asks the "after" times written; every cell says "the next review's" | reproduced: no run of the CI into `main` has followed EN-133 yet | `ci.md`, the jobs' table; F28 criterion 3 | criterion 3 reworded: the times before written by EN-133, those after measured at the next review's CI into `main` and written by that review; VAL-029 says so (T2) |
| F3 | E1 — nothing that runs saves the cargo tools' cache any more | reproduced: `site.yml` restores it only, `rust-caches.yml` runs on a push to `develop` alone, which only a person's push starts; the one `rust-tools-…` cache is `refs/heads/main`'s, saved by `release.yml` before EN-132, and no cache has `refs/heads/develop` | `gh api repos/…/actions/caches`, 2026-10-18 | `rust-caches.yml` on a push to `develop` or `main`, one trigger, no `if:`; `site.yml` restores both caches; tests first (T1), then the fix (T2) |
| F4 | A3 — `[F28.AC1]`'s pattern misses `github.event.pull_request.head.ref`, `github.event.ref`, `github.ref_type`, `github.event.workflow_run.head_branch`, the index syntax `github['ref']` and another case | reproduced: 8 of the 15 examples of the new test pass the old pattern | `tests/workflows.test.ts`, `[F28.AC1] reads every context …`, failing at T1 | the pattern widened, case-insensitive, one example per form (T2) |
| F5 | A3 — the purpose test only checks that a first comment and a `name:` exist | reproduced | `tests/workflows.test.ts`, `[F28.AC1] says each workflow's purpose …` | no code: whether the file name, `name:` and comment say the same purpose is a reading, which VAL-029 does (T2) |
| F6 | D — F30's EN-136 asks again what `[F28.AC3]` tests (the cargo tools and the RustSec database of the CI's lanes) | reproduced | EN-136, criteria 1 and 2; `[F28.AC3] installs each cargo tool …`, `… fetches the RustSec database …` | EN-136 narrowed to what `[F28.AC3]` does not cover (T2) |
| F7 | E3 — ADR-0088's row of the ADR index says "implemented by F28 and F30", F34 changing `story-automerge.yml`; `ci.md` names `ci.yml` the one blocking workflow while `Analyze` (CodeQL) is required too | reproduced | `docs/adr/README.md`; the ruleset's checks in `ci.md` | the row "F28, F30 and F34"; `ci.md` names `Analyze` the second required workflow; ADR-0088 unchanged (T2) |
| F8 | E3 — stale text: `docs/playbook/setup.md` (a release branch); `ci.md` (the "`rust-cache` job"); `rust-caches.yml` and `ci.md` ("compiles what `check:all` compiles", the test lane building `backlog-index` in debug by `cargo run` in `tests/backlog.test.ts`) | reproduced | as listed | the wording fixed, the debug build named as the exception (T2) |
| F9 | E3 — EN-133 gives `npm ci` 1 min 51 s to 2 min 13 s | reproduced: its steps took 1 min 51 s (lint), 2 min 13 s (tests), 2 min 33 s (docs), 2 min 28 s (format), 2 min 13 s (Rust) | `gh api …/runs/39145798246/jobs` | 2 min 33 s (T2) |
| F10 | E1 — `release publish` reads `GITHUB_REF_NAME`, which a tag named `main` gives too (`REF-GH-VARIABLES`) | reproduced by reading: `GITHUB_REF_NAME` is "the short ref name of the branch or tag" | `tools/release/src/main.rs`, `publish_release` | `GITHUB_REF` read, compared with `refs/heads/main`; tests first (T1), then the fix (T2) |
