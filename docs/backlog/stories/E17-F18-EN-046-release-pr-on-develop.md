---
id: EN-046
epic: E17
feature: F18
title: Release pull request on develop, only the release on main
status: ready
points: 2
---

# E17 · F18 · EN-046 — Release pull request on develop, only the release on main

Run release-please's release pull request on `develop` and only its GitHub release on `main`, so that a release adds no commit to `main` that `develop` lacks (F18.AC1; asked by the Product Owner on 2026-10-14). Done before the release that closes E17: the release pull request 0.3 (#259) waits until the end of E17 and is merged by the Product Owner then (2026-10-16), with this flow in place.

SP-020 ([research note 0022](../../research/notes/0022-f18-parallel-run.md)): the action's `target-branch`, `skip-github-release` and `skip-github-pull-request` split the release pull request from the release (`REF-RELEASE-PLEASE-ACTION`); a release tags the merge commit of the release pull request found by its label (`REF-RELEASE-PLEASE-DESIGN`), so the step on `main` looks for pull requests merged into `develop`, checked on the next release.

## Acceptance criteria

- Given release-please's documentation read (its `release-pr` and `github-release` steps, the target branch option), when the workflow is changed, then `release.yml` runs the release pull request on `develop` and only the tag and the GitHub release on `main`.
- Given a release pull request merged into `develop`, then merged into `main` by the Product Owner, when the workflow runs on `main`, then it tags and publishes the release from that commit and opens no second pull request.
- Given the change, when `docs/tooling/releases.md` (or the page that describes releases) is read, then it describes the new flow and `develop` follows `main` by a fast-forward.

## Tasks

One task = one commit, referenced as `EN-046.Tn`.

- [ ] T1 — Read release-please's documentation for the two steps and the target branch; record the source (1 h)
- [ ] T2 — Change the workflow and its configuration; describe the flow in the tooling docs (1 h)

## What the sources say (T1)

| Finding                                                                                                                                            | Source                                                   |
| -------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| `target-branch` is the branch the release pull request is opened against; a workflow per branch sets it                                            | `REF-RELEASE-PLEASE-UPGRADE`                             |
| The old `release-pr` step is `skip-github-release: true`; the old `github-release` step is `skip-github-pull-request: true`                        | `REF-RELEASE-PLEASE-UPGRADE`                             |
| The release step lists the pull requests merged into `target-branch` with the label `autorelease: pending`, and tags each one's merge commit       | `REF-RELEASE-PLEASE-SOURCE`, `REF-RELEASE-PLEASE-DESIGN` |
| While a merged release pull request waits for its tag, the pull request step opens nothing ("untagged, merged release PRs outstanding - aborting") | `REF-RELEASE-PLEASE-SOURCE`                              |
| The previous release is found among the last releases, then in the history of `target-branch` (500 commits at most by default)                     | `REF-RELEASE-PLEASE-DESIGN`, `REF-RELEASE-PLEASE-SOURCE` |

So the step on `main` runs with `target-branch: develop` and `skip-github-pull-request: true`: it finds the release pull request merged into `develop`, and tags its merge commit, which `main` holds once the Product Owner has merged `develop`. Checked on the repository on 2026-10-17: `v0.2.0`'s commit is an ancestor of `develop`, 354 commits behind it.

## The change (T2)

- `.github/workflows/release.yml`: a `release-pr` job on pushes to `develop` (`target-branch: develop`, `skip-github-release: true`); the `release-please` job on pushes to `main` (`target-branch: develop`, `skip-github-pull-request: true`); the automatic merge of the release pull request removed ([ADR-0067](../../adr/0067-release-pull-request-on-develop.md), decision 2: the Product Owner merges it into `develop` when they ask for a release).
- `commitlint.config.ts`: the release commit `chore(develop): release x.y.z` is let through, since the CI of the pull request into `main` checks every commit it brings.
- `tests/release-flow.test.ts`: each job's branch, `target-branch` and `skip-` input, no `gh pr merge`, the release commit of either branch; broken once on purpose (`skip-github-pull-request: false`, then the `develop` scope removed), each run failed.
- Docs: [releases](../../tooling/releases.md), [Git workflow](../../tooling/git-workflow.md) (`develop` follows `main` by a fast-forward), [CI](../../tooling/ci.md), ADR-0038's status.

The release itself cannot run before the merge of `develop` into `main`: the criteria are proven by the sources read (T1) and the test, then checked at the release of 0.3.

## To check at the next release (0.3, #259)

1. Once F18 is merged into `develop`, the Release workflow runs on `develop` and opens `chore(develop): release 0.3.0` against `develop`, from the `release-please--branches--develop` branch; its changelog starts after `v0.2.0`.
2. The Product Owner merges it into `develop`; the next run on `develop` logs "There are untagged, merged release PRs outstanding - aborting" and opens nothing.
3. The pull request #259 gains the release commit; its CI, commit messages included, is green.
4. After the merge of #259, the run on `main` logs no pull request opened, creates the tag `v0.3.0` on the release pull request's merge commit (`git merge-base --is-ancestor v0.3.0 origin/main`), the GitHub release with `test-report.xml`, and the site; the release pull request carries `autorelease: tagged`.
5. `git merge --ff-only origin/main` on `develop` succeeds.

**Rollback** if step 1 or 4 fails: in `release.yml`, remove the `release-pr` job and `develop` from the push branches, remove `target-branch` and `skip-github-pull-request` from the `release-please` job and give back its `gh pr merge --auto --merge` step (the file before EN-046); close the release pull request left on `develop`. If the tag is missing after step 4 with the release pull request merged, run the workflow by hand from `main` (_Actions → Release → Run workflow_): its release step looks again for the merged pull request.

## For the main thread

### Journal

- `#run` EN-046: the release pull request now lives on `develop` and `main` only tags it (ADR-0067); the Product Owner merges the release pull request into `develop` before #259, checked at the release of 0.3.

### Improvements

| 2026-10-17 | agent | EN-046: release-please's commit scope follows its branch, so the release commit `chore(develop): release x.y.z` would have failed the commit-message check of the pull request into `main`; found by reading `ci.yml` before the release (commitlint now lets it through) | tooling |
