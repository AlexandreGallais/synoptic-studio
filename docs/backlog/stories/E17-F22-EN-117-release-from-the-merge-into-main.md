---
id: EN-117
epic: E17
feature: F22
title: The release made by the merge into main, from the tags
status: draft
points: 3
---

# E17 · F22 · EN-117 — The release made by the merge into main, from the tags

ADR-0083: the Product Owner merges `develop` into `main`; `release publish` computes the version from the last tag and the commits since, tags the merge commit and makes the GitHub release with the pull request's body as its notes; no release branch, no commit.

## Acceptance criteria

- Given a merge of `develop` into `main`, when `release.yml` runs, then the tag `vx.y.z` is on the merge commit and the GitHub release holds the pull request's body and the CI's test report, and no commit was made.
- Given the pull request's body, when `release check` reads it, then it passes only when it follows the rules of `docs/tooling/releases.md`.
- Given `package.json`, when the tests run, then its version is `0.0.0-development`; `CHANGELOG.md` keeps 0.1.0 and 0.2.0 and links to the releases.
- Given a release with no change of the library nor the playground, or a version already released, when `release publish` runs, then the first is a patch with its notice and the second does nothing.

## Tasks

- [ ] T1 — `release prepare` and `notes` draft the pull request's body without a branch nor a file written; `release check` reads the body (2 h)
- [ ] T2 — `release publish` computes the version, tags the merge commit and makes the release; `package.json` at `0.0.0-development`; the branch pattern without `release/x.y.z` (2 h)
- [ ] T3 — `docs/tooling/releases.md`, `docs/tooling/git-workflow.md`, the run skill's release step and `AGENTS.md` (Git) on ADR-0083 (1 h)

One task = one commit, referenced as `EN-117.T1` in the commit footer.
