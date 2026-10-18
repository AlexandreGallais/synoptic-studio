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
- [x] T2 — `ci.yml` in jobs, the reporter, `docs/tooling/ci.md`; the ruleset's required checks listed for the main thread (2 h)

One task = one commit, referenced as `EN-124.T1` in the commit footer.

## Decisions taken in the story

Written in `docs/tooling/ci.md`, The lanes as jobs.

- **The lanes are the entries of one matrix** (`REF-GH-MATRIX`), rather than five jobs repeating the same setup steps: each entry names its check and its scripts, run in turn by one step, the first failing one ending the job; `fail-fast: false` lets every lane run to its end. A step that only a lane needs is chosen by `matrix.scripts` (`REF-GH-CONTEXTS`): the cargo tools installed on a missed key and the RustSec database fetched by the Rust lane only, the `git-hooks` artifact uploaded by the lint lane, the test report by the test lane.
- **The checks are named by the lanes**, `check:all`'s grouping kept: `Lint and types` (`lint`, whose oxlint runs the type check), `TypeScript tests`, `Docs, playground and build` (`npm run build` after the docs lane, as it ran after `check:all`), `Format, secrets and dependencies`, `Rust`. The feature's criterion 1 groups the checks otherwise (format with lint, types with the tests); its text describes, the story's criterion asks one job per lane of `check:all`, which these follow.
- **The reporters**: `vite.config.ts` reads `VITEST_JUNIT` at each call and, when the CI asks the report, sets `default`, `github-actions` and `junit`, since a configured list replaces Vitest's own, which add `github-actions` on GitHub (`REF-VITEST-REPORTERS`). Outside the CI, nothing is configured, as before.
- **The history job keeps the binary built by a lane** (F21.AC4: building none), so it waits for every lane and is skipped when one fails, the failing lane's check blocking the merge anyway. Building `git-hooks` in it would give its result without the lanes, at the cost of a Rust build of its own; left as it was.

## Evidence

- `cargo test -p repo-tools --test check_all`: 12 passed, among them `f24_ac1_runs_each_lane_of_check_all_in_exactly_one_job_of_the_ci`, `f24_ac1_reads_the_jobs_of_a_matrix_to_its_end` and `f24_ac3_requires_on_main_every_job_of_the_ci_and_codeql_by_its_name`; `npx vitest run tests/ci-caches.test.ts tests/vitest-reporters.test.ts`: 12 passed.
- The three CI tests and `[F24.AC2]` failed at T1, before `ci.yml`, `docs/tooling/ci.md` and `vite.config.ts` changed.
- Each new test broken once: a lane's script run by a second job, the docs lane's scripts out of order, `fail-fast: true`, the Rust lane's entry removed, `Rust` removed from the checks of `ci.md`, the `github-actions` reporter removed; each failed, and passed again once restored.
- The lanes' times on GitHub's runner, from the log of run 39131147001 (attempt 1, its single job): `format:check` 2.5 s, `lint:secrets` 72.1 s, `deps:audit` 1.6 s, `deps:unused` 0.9 s, `deps:outdated` 3.0 s, `lint` 138.7 s, `docs:build` 265.3 s, `playground:build` 1.4 s, `test` 325.8 s, `check:rust` cancelled after 26 min; before them, the cargo tools installed in 170 s and `npm ci`, whose `prepare` builds the release binaries cold, in 133 s. Both caches were missed. The new jobs' times are measured at the first CI of a pull request into `main`.

## For the main thread

- **The ruleset of `main`, Require status checks**: remove `Format, lint, secrets, types, tests, audit, versions, docs, build`; require `Lint and types`, `TypeScript tests`, `Docs, playground and build`, `Format, secrets and dependencies`, `Rust`, `Commit messages and branch name`, `Dependency review`, `Analyze`. A check appears in the ruleset's list once a run has reported it, so the five new names may need the first CI of the pull request into `main` before they can be chosen; until the old name is removed, that pull request waits for a check no job reports.
- The `done` commit's footer: `Rule-docs: none affected, the CI's workflow, its tests and its docs only, no rule of docs/domain/`.
- `AGENTS.md` (the table of commands, `check:all`) and `docs/tooling/commands.md` say the CI runs `check:all` lane by lane now; the `project-rules` digest says nothing of it.
