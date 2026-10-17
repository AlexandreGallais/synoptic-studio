# ADR-0067 — The release pull request on `develop`, only the release on `main`

**Status**: Accepted, decision 2 confirmed by the Product Owner at F18's validation (VAL-019, 2026-10-17). Accepted by the agent first (2026-10-17, EN-046, in the run of F18 the Product Owner started, from F18's first criterion asked on 2026-10-14). Supersedes, in ADR-0038: release-please's release pull request into `main`, which merged itself (points 1 and 5), and `develop` catching up with a release commit it lacked (point 7).

## Context

- Since ADR-0038, a release added a commit to `main` that `develop` lacked: release-please opened its release pull request (version and changelog) on `main` after the merge of `develop`, then merged it there; `develop` had to catch up each time.
- release-please splits its two steps: the release pull request (`skip-github-release`) and the release (`skip-github-pull-request`), each on the branch its `target-branch` names (`REF-RELEASE-PLEASE-UPGRADE`).
- The release step lists the pull requests merged into its target branch with the label `autorelease: pending` and tags each one's merge commit (`REF-RELEASE-PLEASE-SOURCE`, `REF-RELEASE-PLEASE-DESIGN`). While such a pull request waits for its tag, the pull request step opens nothing.
- No CI runs on a pull request into `develop` (ADR-0038), and the agent never merges a pull request.

## Decision

1. **On a push to `develop`**, release-please opens or updates the release pull request `chore(develop): release x.y.z` against `develop`, and makes no release (`target-branch: develop`, `skip-github-release: true`).
2. **The Product Owner merges it into `develop`** when they ask for a release, just before the pull request `develop` → `main`, which then holds the version and the changelog; the workflow no longer merges it by itself. Merged earlier, it would fix the version while later features wait in `develop` outside it.
3. **On a push to `main`**, release-please only tags and publishes: it looks for the release pull request merged into `develop` (`target-branch: develop`, `skip-github-pull-request: true`), tags its merge commit, now in `main`, and publishes the GitHub release; then the test report and the site, as before.
4. **After a release**, `develop` follows `main` by a fast-forward: `main` holds only the merge commit of `develop`.
5. commitlint lets the release commit through with the scope `develop` as well as `main` (the CI checks every commit of the pull request into `main`).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Release pull request on `develop`, merged by the Product Owner (chosen) | `main` gains no commit `develop` lacks; the version is fixed when the release is asked | one more merge per release |
| Release pull request on `develop`, merged by the workflow | no extra merge | the version fixed at the first push to `develop`; later features left out of its changelog, the tag behind `main` |
| Release pull request on `main` (ADR-0038) | one merge | `develop` catches up with a commit at every release |

## Consequences

- A release is two merges by the Product Owner: the release pull request into `develop`, then `develop` into `main`. Merged in the other order, no release is made; a second pull request `develop` → `main` makes it.
- The first release under this flow is 0.3 (#259), at E17's end: its tag, its GitHub release and the absence of a second pull request are checked then ([EN-046](../backlog/stories/E17-F18-EN-046-release-pr-on-develop.md)).
- `release.yml` runs on pushes to `develop` and `main`; the site published by hand runs from `main`.

## References

ADR-0038, ADR-0045, `REF-RELEASE-PLEASE-UPGRADE`, `REF-RELEASE-PLEASE-SOURCE`, `REF-RELEASE-PLEASE-DESIGN`, `docs/tooling/releases.md`
