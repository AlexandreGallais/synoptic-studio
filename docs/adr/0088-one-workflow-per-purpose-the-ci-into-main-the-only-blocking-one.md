# ADR-0088 — One workflow per purpose, its trigger its only condition; the CI into `main` the only blocking one

**Status**: Accepted (Product Owner, 2026-10-18, after 0.3.0's merge into `main`). Supersedes ADR-0087's decision 2: the Rust cache is no longer saved by `release.yml` on a push to `develop`, which the auto-merge's merges never start, but by the `develop` CI below. Supersedes, in ADR-0072 (The run, decision 1), "no workflow runs on … a push to `develop`", already narrowed by ADR-0087: a workflow may run on `develop`, never blocking. The rest of ADR-0072's decision 1 stands: the checks of work branches run locally, before the push. Implemented by F28 and F30 ([research note 0026](../research/notes/0026-the-tooling-features-of-2026-10-18.md)).

## Context

- `release.yml` holds three purposes under three conditions: the site on a push to `main`, the Rust caches on a push to `develop` or `main`, the release by its button on `main`, each job choosing itself by an `if:` on the event and the branch. `ci.yml` runs on the pull request into `main` and by hand, two of its jobs skipping themselves by `if: github.event_name == 'pull_request'`. Reading the Actions tab, the Product Owner cannot tell what each workflow is for.
- Every merge into `develop` is the auto-merge's, made with the built-in `GITHUB_TOKEN`, and a push by that token starts no workflow; only `workflow_dispatch` and `repository_dispatch` do (`REF-GH-TOKEN-TRIGGERS`). On 2026-10-18 eight merges into `develop` started no run, so `release.yml`'s `rust-cache` job never ran there, and the next CI into `main` installed the cargo tools again, 2 min 47 s in the Rust lane (note 0026, §1).
- `workflow_run` runs the default branch's file on the default branch's commit (`REF-GH-WORKFLOW-DISPATCH`) and cannot save a cache in the default branch's scope (`REF-GH-CACHE`); `workflow_dispatch` runs on the branch dispatched and saves its caches there (`REF-GH-WORKFLOW-DISPATCH`, `REF-GH-CACHE`).

## Decision

1. **One workflow per purpose**, its file and its name saying that purpose. A workflow that does half of another's work is a file of its own; the site's deployment and the release are two workflows.
2. **Its trigger is its only condition.** Whether a workflow runs, and which of its jobs run, is chosen by its `on:` (events, activity types, branches), never by an `if:` reading the event or a branch. A condition the trigger cannot express stays: a step chosen by its matrix lane, a guard on the pull request's label, author or draft state.
3. **The CI of `develop` → `main` is the one blocking check**: the jobs `main`'s ruleset requires, holding nothing its checks do not need. A security update of Dependabot into `main` runs it too.
4. **A workflow on `develop` never blocks**: nothing waits for it and no ruleset requires it. The `develop` CI runs the Rust tests on Linux after each pull request merged into `develop`, and saves the Rust caches the pull request into `main` restores.
5. **The auto-merge starts the `develop` CI by `workflow_dispatch`** once its merge into `develop` is done (`gh workflow run <file> --ref develop`, `actions: write`, `REF-GH-CLI-WORKFLOW-RUN`, `REF-GH-REST-WORKFLOW-DISPATCH`); a merge made by a person starts it by its `push`. No personal token is kept for it.

## Reasons

- A workflow per purpose, chosen by its trigger, reads at the top of its file and in the Actions tab: one name, one condition, no job shown skipped.
- The dispatch is the one way the docs give for a run started from the built-in token; it runs on `develop`'s head and saves the caches under `develop`, the scope the review's pull request reads first (`REF-GH-CACHE`). `workflow_run` would test and cache `main`.
- A blocking check only into `main` keeps the run's pace: the stories and features are checked before their push, and a red `develop` run says what the review's CI will meet, without stopping the run.

## Consequences

- F28 splits `release.yml` into the site's deployment and the release, moves the Rust caches into a workflow of their own, splits `story-automerge.yml` by its base, removes `ci.yml`'s `workflow_dispatch` and its two conditions, and proposes the fate of `external-prs.yml` and CodeQL's languages to the Product Owner; a test reads every workflow against decision 2.
- F30 makes the Rust caches' workflow the `develop` CI, started by the auto-merge's dispatch, and adds a test that each CI job installs the tools its scripts call (AUD-027).
- `workflow_dispatch` runs only once the file is on the default branch (`REF-GH-WORKFLOW-DISPATCH`): the `develop` CI runs from the first merge into `main` that brings it.
- `docs/tooling/ci.md` describes the workflows as F28 and F30 leave them.

## References

ADR-0026, ADR-0072, ADR-0086, ADR-0087, `REF-GH-TOKEN-TRIGGERS`, `REF-GH-WORKFLOW-DISPATCH`, `REF-GH-CACHE`, `REF-GH-CLI-WORKFLOW-RUN`, `REF-GH-REST-WORKFLOW-DISPATCH`, `REF-GH-CLI-MERGE`
