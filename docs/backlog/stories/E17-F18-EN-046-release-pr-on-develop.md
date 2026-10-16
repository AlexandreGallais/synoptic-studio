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
