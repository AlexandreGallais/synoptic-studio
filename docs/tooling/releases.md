# Releases

release-please (`.github/workflows/release.yml`) writes the versions and the changelog from the Conventional Commits. Its pull request lives on `develop`, and `main` only receives the release and tags it, so that `main` never holds a commit `develop` lacks (ADR-0067).

| Step | Branch | Who | What happens |
| --- | --- | --- | --- |
| 1. a push to `develop` | `develop` | release-please (`release-pr` job) | opens or updates the release pull request `chore(develop): release x.y.z`: the next version in `package.json`, the new section of `CHANGELOG.md`; no release |
| 2. a release asked | `develop` | the Product Owner | merges the release pull request into `develop`; no new one opens until it is tagged |
| 3. the release | `main` | the agent opens, the Product Owner merges | `develop` → `main`, CI green, merge commit ([Git workflow](./git-workflow.md)) |
| 4. a push to `main` | `main` | release-please (`release-please` job) | finds the release pull request merged into `develop`, tags its merge commit `vx.y.z` and publishes the GitHub release |
| 5. after the release | `main` | the same workflow | attaches the test report of the `develop` → `main` CI (ADR-0027), publishes the site and the playground to GitHub Pages |
| 6. catch-up | `develop` | the agent | `develop` fast-forwards to `main` |

- Docs and tooling wait in `develop` and travel with the next release.
- The order matters: merged `develop` → `main` first, step 4 finds nothing to tag. Merge the release pull request into `develop`, then open a second `develop` → `main`. The site can also be published by hand: _Actions → Release → Run workflow_.
- release-please runs with the fine-grained token `RELEASE_PLEASE_TOKEN` (Contents, Pull requests, Issues: read and write), because a pull request made with `GITHUB_TOKEN` triggers no other workflow. It **expires 2026-12-31**: renew it before.
- The library stays below `1.0.0` until the Product Owner calls it stable (`initial-version` `0.1.0`). It is not published to npm yet.

| Commits since the last release | Next version before 1.0.0 | After 1.0.0 |
| --- | --- | --- |
| only `fix` | patch | patch |
| at least one `feat` | minor | minor |
| `!` or `BREAKING CHANGE:` | minor (`bump-minor-pre-major`) | major |
| only `chore`, `ci`, `test`, `style` | no release | no release |

Changelog sections: Features, Bug fixes, Performance, Refactoring, Documentation, Build and dependencies (`release-please-config.json`).

**Rolling back** to ADR-0038's flow: in `release.yml`, remove the `release-pr` job and `develop` from the push branches, remove `target-branch` and `skip-github-pull-request` from the `release-please` job, and restore its step `gh pr merge --auto --merge`; close by hand a release pull request still open on `develop`.
