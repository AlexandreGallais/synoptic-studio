# ADR-0083 — The release made by the merge into `main`: the version from the tags, no release branch nor version commit

**Status**: Accepted (Product Owner, 2026-10-18, RET-004, option B of the retrospective's P2). Supersedes ADR-0079: its release branch, its commit `chore: release x.y.z` and the version and changelog committed before the merge. ADR-0079's decision 3 stays in substance: `release publish` makes the GitHub release on `main` with `GITHUB_TOKEN`, without a commit; and its decision 4, `develop` following `main` after a release. Implemented by [EN-117](../backlog/stories/E17-F22-EN-117-release-from-the-merge-into-main.md).

## Context

- Under ADR-0079 a release is a branch `release/x.y.z` from `develop`, a commit of the version in `package.json` and `package-lock.json` and of the entry of `CHANGELOG.md`, its pull request into `main`, and a catch-up of `develop` by a merge when `develop` moved meanwhile.
- Nothing in the library, the playground nor the site reads `package.json`'s version; only the release tool does (`tools/release/`). The library is not published to npm.
- semantic-release tracks the version by Git tags and writes it into `package.json` only in the published package, never pushed; a committed value such as `0.0.0-development` says the field is not kept up to date; committing during a release "adds significant complexity", and a committed changelog repeats the GitHub release's notes, a changelog holding a link to the releases being a middle ground (`REF-SEMANTIC-RELEASE-FAQ`).
- `npm version --no-git-tag-version` writes a version without a commit nor a tag, and `from-git` reads it from the latest tag (`REF-NPM-VERSION`): a publication to npm, later, can write the tag's version into the package at publish time.
- The options weighed at RET-004, for one release: A, ADR-0079 (1 commit, 1 branch, 1 merge); B, the merge into `main` makes the tag (0 commit, 0 branch, 1 merge); C, a tag pushed by hand makes the release (0 commit, 0 branch, 1 merge and 1 tag).

## Decision

1. **The Product Owner merges `develop` into `main`** by a merge commit when they ask for a release: one action, no release branch.
2. **The pull request's body is the release's entry.** The agent opens `develop` → `main` with the draft `target/release/release notes` makes from the commits since the last tag, rewrites each line as the rules of `docs/tooling/releases.md` ask, and `release check` checks the body.
3. **On the push to `main`, `release publish` computes the version** from the last tag `vx.y.z` and the commits since, as `release prepare` computes it today, tags the merge commit `vx.y.z` and makes the GitHub release, its notes the pull request's body, the test report of its CI attached; no commit, `GITHUB_TOKEN` alone.
4. **`package.json` holds `0.0.0-development`**, kept by a test, so that no file claims a version; the current version is read from the tags and GitHub's releases. `CHANGELOG.md` keeps the entries of 0.1.0 and 0.2.0 and links to the releases for the later ones. A publication to npm, when it comes, writes the tag's version into the package at publish time, without a commit.
5. **After the release**, `develop` fast-forwards to `main`; with no release commit, `develop` lacks only the merge commit.
6. **0.3.0 is the first release made so**, once the CI of `develop` → `main` is green ([EN-116](../backlog/stories/E17-F22-EN-116-ci-green-on-github-runners.md)); E17 closes at its publication.

## Reasons

- Option B is one action for the Product Owner, like A, and makes no commit nor branch for the release; C asks a second action, and a tag typed by hand can misname the version.
- A version written in `package.json` and committed is a second place to keep true; with nothing reading it, the tags suffice.

## Consequences

- `release prepare` no longer makes a branch nor writes files; `release check` reads the pull request's body; `release publish` computes and tags; `BRANCH_PATTERN` no longer needs `release/x.y.z`.
- A checkout of `v0.3.0` reads `0.0.0-development` in `package.json`.
- `docs/tooling/releases.md`, `docs/tooling/git-workflow.md`, the run skill's release step and `AGENTS.md` (Git) describe the flow once EN-117 is merged; until then ADR-0079's tool stays in place, unused.

## References

ADR-0027, ADR-0072, ADR-0079, `REF-SEMANTIC-RELEASE-FAQ`, `REF-NPM-VERSION`, `REF-GH-WORKFLOW-DISPATCH`, `REF-COMMON-CHANGELOG`, `REF-SEMVER`, `docs/process/retrospectives/E17-retrospective.md`
