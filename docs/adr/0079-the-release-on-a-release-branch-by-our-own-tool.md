# ADR-0079 — The release prepared on a release branch from `develop`, published by our own tool

**Status**: Accepted (the agent, 2026-10-18, EN-112, from F21's decisions of 2026-10-17; the option chosen to be shown to the Product Owner). Supersedes ADR-0067, decisions 1 to 3: release-please's release pull request on `develop`, its merge by the Product Owner, and release-please's tag on `main`; decision 5, release-please's commit let through, goes with them. Decision 4 stays: after a release, `develop` follows `main`. Supersedes too ADR-0072's lines that restate ADR-0067 on the release.

## Context

- F21's decisions (2026-10-17): the project's own Rust tool replaces release-please entirely, the GitHub release included; the version and the changelog are written inside the pull request into `main`, before it merges, so that the CI makes no commit after the merge; `main` holds nothing `develop` lacks; the notes are better than release-please's ([EN-112](../backlog/stories/E17-F21-EN-112-release-tool-in-rust.md)).
- Under ADR-0067, a release is two merges by the Product Owner, release-please's pull request into `develop` then `develop` into `main`, a personal access token (`RELEASE_PLEASE_TOKEN`) so that release-please's pull request starts the CI, and a third-party action.
- The version and the changelog are one commit, made by `release prepare` (`docs/tooling/releases.md`). It must be in the pull request into `main` before the CI runs, so that the CI checks it and the merge commit holds it.
- `develop` moves often: 32 commits reached it in the four days after 0.2.0 (2026-10-14 to 2026-10-17), the main thread's log rows and docs among them. The CI of a pull request into `main` runs again on each push to its head, the previous run cancelled (`ci.yml`, `cancel-in-progress`); its `quality` job took 5 to 8 minutes in October (EN-111).
- Git-flow names this step: a release branch "may branch off from `develop`", gets its version number "at the start", is merged into `master` and tagged, then merged back into `develop`; "the `develop` branch is cleared to receive features for the next big release" (`REF-GITFLOW`).

## Decision

1. **The release is prepared on a release branch from `develop`**: the agent, asked for a release, runs `target/release/release prepare` on an up-to-date `develop`; it computes the version, makes the branch `release/x.y.z` there, and writes `package.json`, `package-lock.json` and the entry of `CHANGELOG.md`. The agent rewrites the entry's lines, `release check` passes, and the agent commits `chore: release x.y.z`, pushes the branch and opens its pull request into `main`, its body the entry (`release notes`).
2. **The Product Owner merges that pull request into `main`** by a merge commit, once its CI is green: one merge per release. A change needed in the release goes on the release branch; anything else waits in `develop` for the next release.
3. **On the push to `main`, `release publish` publishes without a commit**: it reads the version from `package.json`, makes the GitHub release `vx.y.z` on the merge commit with the entry as its notes and the test report of the pull request's CI attached; the same workflow publishes the site. It runs on `GITHUB_TOKEN`: no personal token, no third-party action.
4. **After the release**, `develop` follows `main` by a fast-forward, or by a merge of `main` when it moved meanwhile, and the release branch is deleted (ADR-0067, decision 4, kept).

## Reasons

The two ways the commit of the version gets into the pull request, beside ADR-0067's, for one release:

|  | A — the commit on `develop`, then `develop` → `main` | B — a release branch from `develop` (chosen) | ADR-0067 |
| --- | --- | --- | --- |
| merges by the Product Owner | 1 | 1 | 2 |
| commits by the CI after the merge | 0 | 0 | 0 |
| the agent's pushes to `develop` for the release | 1, the commit of the version | 0 | 0 |
| branches made | 0 | 1, deleted after | 1, release-please's |
| a push to `develop` while the pull request is open | joins the release outside its changelog, and starts the CI again (5 to 8 minutes) | stays in `develop` for the next release | joins the release outside its changelog |
| `develop` caught up after the merge | fast-forward | fast-forward, or a merge of `main` when `develop` moved | fast-forward |
| commits of `main` that `develop` lacks, once caught up | 0 | 0 | 0 |
| token | `GITHUB_TOKEN` | `GITHUB_TOKEN` | a personal access token |

- **A** needs no branch, but it freezes `develop` while the pull request is open: at the rate of October, about eight pushes a day, each would enter the release without its line in the changelog, or wait for the merge.
- **B** costs one branch and, when `develop` moved, a merge commit on `develop` at the catch-up; the release holds exactly what its changelog describes, and the version is fixed when the release is asked, as git-flow fixes it (`REF-GITFLOW`).

## Consequences

- `BRANCH_PATTERN` accepts `release/x.y.z` and no longer `release-please--…`; the commit-message check no longer lets release-please's `chore(develop): release …` through, and the release commit `chore: release x.y.z` passes as any other.
- `release-please-config.json`, `.release-please-manifest.json`, release-please's jobs and the `RELEASE_PLEASE_TOKEN` secret go; the Product Owner deletes the secret in the repository's settings.
- `release.yml` runs on a push to `main` only, and by hand for the site.
- `docs/tooling/releases.md` and `docs/tooling/git-workflow.md` describe the flow; git-flow's release branches, left out by ADR-0072, come back for this one step.

## References

ADR-0038, ADR-0067, ADR-0072, `REF-GITFLOW`, `REF-COMMON-CHANGELOG`, `REF-KEEP-A-CHANGELOG`, `REF-SEMVER`, `REF-GH-RELEASE-NOTES`, `docs/tooling/releases.md`
