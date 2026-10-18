---
id: EN-117
epic: E17
feature: F22
title: The release by a button, from the tags
status: ready
points: 3
---

# E17 · F22 · EN-117 — The release by a button, from the tags

ADR-0086, superseding ADR-0083: the Product Owner runs `release.yml` on `main` from GitHub's Actions tab; `release publish` computes the version from the last tag and the commits since, tags `main`'s head and makes the GitHub release, its notes the bodies of the pull requests into `main` merged since the last tag; no release branch, no commit, no `CHANGELOG.md`. A push to `main` deploys the site only.

## Acceptance criteria

- Given `release.yml` run by its button on `main`, when it ends, then the tag `vx.y.z` is on `main`'s head and the GitHub release holds the bodies of the pull requests into `main` merged since the last tag and the CI's test report, and no commit was made.
- Given a push to `main`, when `release.yml` runs, then it deploys the site and makes no tag nor release.
- Given a feature's pull request into `main`, when `release check` reads its body, then it passes only when it follows the rules of `docs/tooling/releases.md`.
- Given `package.json`, when the tests run, then its version is `0.0.0-development`; `CHANGELOG.md` no longer exists and nothing links to it.
- Given a release with no change of the library nor the playground, or no commit since the last tag, when `release publish` runs, then the first is a patch with its notice and the second does nothing.

## Tasks

- [ ] T1 — The interface and its failing tests: `release notes` drafts a feature's text without a branch nor a file written, `release check` reads a body, `release publish` computes, tags and joins the notes (2 h)
- [ ] T2 — The code that passes them; `release.yml` by its button, the push deploying only; `package.json` at `0.0.0-development`; `CHANGELOG.md` deleted; `docs/tooling/releases.md` on ADR-0086 (2 h)

One task = one commit, referenced as `EN-117.T1` in the commit footer.
