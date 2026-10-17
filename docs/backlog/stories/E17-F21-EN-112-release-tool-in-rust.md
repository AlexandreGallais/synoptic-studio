---
id: EN-112
epic: E17
feature: F21
title: The release made by our own tool, in place of release-please
status: draft
points: 8
---

# E17 · F21 · EN-112 — The release made by our own tool, in place of release-please

Decided on 2026-10-17 (F21's decisions): our release tool replaces release-please entirely, the GitHub release included; the version bump and the changelog are written inside the pull request `develop` → `main`, before it merges, so that no commit is made by the CI after the merge; the release notes are to be better than release-please's, from what the documentation on good release notes says; the tool may improve anything else it finds (F21.AC6). Today release-please opens a release pull request on `develop` that the Product Owner merges before the pull request into `main`, then tags and publishes from `main` (ADR-0067, `docs/tooling/releases.md`): two merges per release, a personal access token, and a third-party action.

## What good release notes are, from their sources

Read through the library on 2026-10-17:

| Rule | Source |
| --- | --- |
| Written for the people who use the library, not a copy of the commit log: "full of noise", each commit documents "a step in the evolution of the source code", a release entry "the noteworthy difference, often across multiple commits" | `REF-KEEP-A-CHANGELOG` (Commit log diffs), `REF-COMMON-CHANGELOG` (4.1) |
| One entry per version, the latest first, headed `## [x.y.z] - YYYY-MM-DD`, the date in ISO 8601, the version linked to its GitHub release, which holds the same content | `REF-KEEP-A-CHANGELOG` (Guiding principles, Confusing dates), `REF-COMMON-CHANGELOG` (2.2) |
| Changes grouped by their impact, in the order `Changed`, `Added`, `Removed`, `Fixed`; deprecations, removals and breaking changes always listed | `REF-COMMON-CHANGELOG` (2.4), `REF-KEEP-A-CHANGELOG` (Types of changes, Ignoring deprecations) |
| A breaking change first in its group, marked `**Breaking:**` | `REF-COMMON-CHANGELOG` (2.4.4) |
| Each change one line, in the imperative mood, self-describing without its heading, followed by a link to its pull request or commit | `REF-COMMON-CHANGELOG` (2.4.1, 2.4.2, 3.6) |
| Noise removed — dotfiles, development-only dependencies, formatting — but refactorings and new documentation kept | `REF-COMMON-CHANGELOG` (3.2) |
| Related changes merged into one line, changes that undo each other left out | `REF-COMMON-CHANGELOG` (3.4, 3.5) |
| At most one notice, a single sentence, for a first release or an upgrade guide | `REF-COMMON-CHANGELOG` (2.3) |
| Conventional Commits only draft the categories; the lines are then rephrased: a draft generated, then edited | `REF-COMMON-CHANGELOG` (3.1, 4.2) |
| The changelog says it follows Semantic Versioning; before 1.0.0 a release raises the minor version | `REF-KEEP-A-CHANGELOG` (Guiding principles), `REF-SEMVER` (Summary, FAQ) |

Where release-please's 0.2.0 falls short of them (`CHANGELOG.md`): one line per commit, 163 lines in all, 29 under Features and 96 under Documentation; the commits of a feature listed beside the feature's own merge commit (`F10 round shape…`, then `model: the round shape…`); the tooling's commits (`eslint`, `tooling`) among the library's; scopes in bold instead of words; a reference to a commit where a pull request exists; the breaking change in a block of its own, far from its group. GitHub's generated notes (`REF-GH-RELEASE-NOTES`) list merged pull requests by label, which here would be stories and features without their meaning for a user.

## Acceptance criteria

- Given the history since the last tag, when the release is prepared, then the version follows the table of `docs/tooling/releases.md` (`REF-SEMVER`, before 1.0.0 a breaking change or a `feat` raises the minor version, a `fix` the patch) and `package.json`, `package-lock.json` and `CHANGELOG.md` are written by one Rust command.
- Given the commits of the release, when the tool drafts the changelog's entry, then it follows every rule of the table above: one line per user-visible change of the library and the playground, grouped by feature, the tooling, tests, CI and backlog left out; the agent then rewrites each line in the imperative mood before committing; a check refuses an entry that breaks a rule it can check (heading, date, group order, a reference per line, the `**Breaking:**` mark first).
- Given the release asked, when the pull request into `main` is opened, then it holds the commit of the version and the changelog before it merges, its body is the changelog's entry, and the new ADR records how the commit reaches it (on `develop` before the pull request, or on a release branch from `develop`), superseding ADR-0067's decisions 1 to 3.
- Given the merge into `main`, when the release workflow runs, then a Rust command tags the merge commit `vX.Y.Z`, publishes the GitHub release with the changelog's entry as its notes, attaches the test report, and the site is published; no commit is made, and `develop` follows `main` by a fast-forward.
- Given the history from `v0.1.0` to `v0.2.0`, when the tool drafts its entry, then the draft and release-please's section are compared line for line in the story, with the count of lines and words of each.
- Given the story's end, when the repository is searched, then release-please's workflow steps, `release-please-config.json`, `.release-please-manifest.json`, the `release-please--` exceptions of the CI, the branch pattern and the commit-msg hook, and `RELEASE_PLEASE_TOKEN` in the docs are gone, and the docs ask the Product Owner to delete the token's secret.
- Given the release command, when it is timed, then its time to prepare and to publish is stated.

## Files

`tools/release/` (new crate: `prepare`, `check`, `publish`), `package.json` (`release:prepare`), `.github/workflows/release.yml`, `.github/workflows/ci.yml`, `release-please-config.json` and `.release-please-manifest.json` (removed), `CHANGELOG.md` (its header: the format and Semantic Versioning named), `tools/git-hooks/` (the release commit and branch), `docs/tooling/releases.md`, `docs/tooling/git-workflow.md`, `docs/adr/` (a new ADR superseding ADR-0067's decisions 1 to 3), `docs/references.md`.

## Measures (ADR-0069)

`release:prepare` and `release publish` timed on a copy of the repository at `v0.2.0`'s parent (10 runs, release build); the lines and words of the 0.2.0 entry, release-please's against ours.

## Tasks

One task = one commit, referenced as `EN-112.Tn`.

- [ ] T1 — The ADR of the release flow, the rules of good notes in `docs/tooling/releases.md` from their sources (3 h)
- [ ] T2 — The version and the changelog's draft from the history, the entry's check; tested on recorded histories (6 h)
- [ ] T3 — The tag, the GitHub release and the test report from `main` by a Rust command; `release.yml` without release-please (4 h)
- [ ] T4 — 0.2.0 drafted again and compared; release-please's files and exceptions removed; docs; times measured (3 h)
