---
id: EN-124
epic: E17
feature: F24
title: The CI in parallel jobs, its tests shown as they run
status: ready
points: 3
---

# E17 · F24 · EN-124 — The CI in parallel jobs, its tests shown as they run

`ci.yml` runs `check:all` as one job: on GitHub's runner its lanes took 139 s (`lint`), 265 s (`docs:build`), 326 s (`test`) and over 21 min (`check:rust`), nothing visible until the end (run 39131147001, REV-006). `check:all` stays what `pre-push` runs; the CI calls its lanes as jobs.

## Acceptance criteria

- Given a pull request into `main`, when its CI runs, then each lane of `check:all` runs in a job of its own, in parallel, each a check on the pull request, and one lane failing leaves the others' results; every lane `check:all` runs is run by exactly one job, as a test checks.
- Given the TypeScript tests in the CI, when they run, then they report file by file as they go, and a failing test is annotated on the pull request at its line (Vitest's GitHub Actions reporter).
- Given `main`'s ruleset, when the jobs change, then the checks it requires are the new jobs' names, written in `docs/tooling/ci.md`.

## Tasks

- [x] T1 — The test that every lane of `check:all` is run by one job of `ci.yml`, failing (1 h)
- [ ] T2 — `ci.yml` in jobs, the reporter, `docs/tooling/ci.md`; the ruleset's required checks listed for the main thread (2 h)

One task = one commit, referenced as `EN-124.T1` in the commit footer.
