# Releases

Versions and changelog are produced from the commit history by **release-please** (`.github/workflows/release.yml`), from Conventional Commits.

The release pull request lives on `develop`; `main` only receives the release and tags it, so a release adds no commit to `main` that `develop` lacks (ADR-0067):

| Step                   | Branch    | Who                                       | What happens                                                                                                                                                                                                                                                |
| ---------------------- | --------- | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1. a push to `develop` | `develop` | release-please (`release-pr` job)         | opens or updates the release pull request `chore(develop): release x.y.z`: the next version in `package.json` and the new section of `CHANGELOG.md`; no release (`skip-github-release`)                                                                     |
| 2. a release asked     | `develop` | the Product Owner                         | merges the release pull request into `develop` (merge commit); until it is tagged, no new release pull request opens                                                                                                                                        |
| 3. the release         | `main`    | the agent opens, the Product Owner merges | the pull request `develop` → `main`, CI green, merge commit (ADR-0038)                                                                                                                                                                                      |
| 4. a push to `main`    | `main`    | release-please (`release-please` job)     | finds the release pull request merged into `develop` (`target-branch: develop`, `skip-github-pull-request`), tags its merge commit `vx.y.z` and publishes the GitHub release; opens no pull request                                                         |
| 5. after the release   | `main`    | the same workflow                         | attaches the **test report** of the CI of the `develop` → `main` pull request (ADR-0027; no new test run), then **publishes the documentation site and the playground** to GitHub Pages (`docs` job, `https://alexandregallais.github.io/synoptic-studio/`) |
| 6. catch-up            | `develop` | the agent                                 | `develop` follows `main` by a fast-forward: `git merge --ff-only origin/main` ([Git workflow](./git-workflow.md))                                                                                                                                           |

- Documentation and tooling changes wait in `develop` and travel with the next release; the release pull request on `develop` grows with each push until it is merged.
- Merged in the other order (`develop` → `main` first), step 4 finds nothing and makes no release: merge the release pull request into `develop`, then open a second pull request `develop` → `main`. The site can also be published by hand from `main`: _Actions → Release → Run workflow_.
- release-please runs with the personal access token `RELEASE_PLEASE_TOKEN` (repository secret, fine-grained: Contents, Pull requests, Issues read and write; **expires 2026-12-31**, renew it before): a pull request or a release made with a `GITHUB_TOKEN` triggers no other workflow. Since ADR-0067 no CI runs on its pull request into `develop`; the token stays, for a rollback to the previous flow.
- The first version is `0.1.0` (`initial-version`): the library stays below `1.0.0` until the Product Owner decides it is stable.

| Commits since the last release      | Next version (before 1.0.0)    | After 1.0.0 |
| ----------------------------------- | ------------------------------ | ----------- |
| only `fix`                          | patch                          | patch       |
| at least one `feat`                 | minor                          | minor       |
| `!` or `BREAKING CHANGE:`           | minor (`bump-minor-pre-major`) | major       |
| only `chore`, `ci`, `test`, `style` | no release                     | no release  |

Changelog sections: Features, Bug fixes, Performance, Refactoring, Documentation, Build and dependencies (`release-please-config.json`). Publishing to npm is not set up: the library is not published yet.

**Rolling back** to the previous flow (ADR-0038): in `release.yml`, remove the `release-pr` job and `develop` from the push branches, remove `target-branch` and `skip-github-pull-request` from the `release-please` job and restore its step `gh pr merge --auto --merge` on the pull request it opens. A release pull request still open on `develop` is closed by hand.
