---
id: EN-046
epic: E17
feature: F18
title: Release pull request on develop, only the release on main
status: draft
points: 2
---

# E17 · F18 · EN-046 — Release pull request on develop, only the release on main

Run release-please's release pull request on `develop` and only its GitHub release on `main`, so that a release adds no commit to `main` that `develop` lacks (F18.AC1; asked by the Product Owner on 2026-10-14). Done before the release that closes E17: the release pull request 0.3 (#259) waits until the end of E17 and is merged by the Product Owner then (2026-10-16), with this flow in place.

## Acceptance criteria

- Given release-please's documentation read (its `release-pr` and `github-release` steps, the target branch option), when the workflow is changed, then `release.yml` runs the release pull request on `develop` and only the tag and the GitHub release on `main`.
- Given a release pull request merged into `develop`, then merged into `main` by the Product Owner, when the workflow runs on `main`, then it tags and publishes the release from that commit and opens no second pull request.
- Given the change, when `docs/tooling/releases.md` (or the page that describes releases) is read, then it describes the new flow and `develop` follows `main` by a fast-forward.

## Tasks

One task = one commit, referenced as `EN-046.Tn`.

- [ ] T1 — Read release-please's documentation for the two steps and the target branch; record the source (1 h)
- [ ] T2 — Change the workflow and its configuration; describe the flow in the tooling docs (1 h)
