# ADR-0086 — One story at a time into `develop`, `main` at each reviewed feature, the release by a button

**Status**: Accepted (Product Owner, 2026-10-18, during F22's run). Supersedes ADR-0083: the release is no longer made by the merge into `main`, and `CHANGELOG.md` goes. Supersedes, in ADR-0084, decision 3 (the feature branch, its rebase with lease, `develop` → `main` at the epic's end), decision 4 (two agents per story) and the parallel researchers of decision 5. ADR-0084's decisions 1 and 2, small features and tests first in their own commit, stand. Implemented by [EN-122](../backlog/stories/E17-F22-EN-122-one-story-at-a-time-into-develop.md) and [EN-117](../backlog/stories/E17-F22-EN-117-release-from-the-merge-into-main.md).

## Context

- During F21 and F22's start the parallel work cost more than it saved: two stories open at once meant rebases and merges done again, each by an agent that starts cold, and a week's usage that a run of two days could spend whole. The Product Owner prefers a slower run they can afford each day.
- A feature branch adds a level to the graph and a rebase whenever `develop` moves; the merge commits of `develop`, one per pull request, already show each piece of work and where a session ended.
- Under ADR-0083 the merge into `main` is the release, so the site cannot follow a reviewed feature without making a version.
- semantic-release reads the version from the tags and writes it only into the published package, never in a commit (`REF-SEMANTIC-RELEASE-FAQ`); GitHub runs a workflow on demand from a button of its Actions tab (`workflow_dispatch`, `REF-GH-WORKFLOW-DISPATCH`); `release.yml` already accepts it.

## Decision

1. **One agent at a time, in the background.** A run works one story at a time, one worker for it, in the background, so that the main thread stays free for the Product Owner, who may ask and try ideas while it works; a review or a research runs alone too. No two stories, features or researchers at once. A slower run is accepted for its lower usage.
2. **Every story starts from `develop` and returns to it.** Its branch `<type>/<id>-<topic>` is made from `origin/develop`; finished, it opens its pull request into `develop`, merged by a merge commit (one loop per story in the graph), by GitHub's auto-merge in a run. No feature branch.
3. **A run goes on past a feature's end, a review waits for the feature under way.** When the scope set is an epic, a finished feature waits for the Product Owner's review while the run starts the next one; no branch marks it. `develop` → `main` is opened only when `develop`'s head is a finished feature, its `VAL` merged, so that `main` only ever receives whole features: a review asked while the next feature is under way waits for that feature's end, and the agent says so.
4. **`main` receives the features reviewed by the Product Owner.** At the review the agent brings the features' docs up to date and opens one pull request `develop` → `main` for every feature finished since the last merge into `main`, its body their texts for the release (decision 6) checked by `release check`; its CI is the only one (ADR-0072). The Product Owner merges it by a merge commit; the push to `main` deploys the site and makes no release.
5. **The release is a button.** The Product Owner runs `release.yml` on `main` from GitHub's Actions tab. `release publish` computes the version from the last tag `vx.y.z` and the commits since, tags `main`'s head and makes the GitHub release; nothing is committed and no number is typed. The version lives only in the tag, the release and, later, a published package, written into it at publish time; `package.json` holds `0.0.0-development`.
6. **The notes are the features' texts.** The release's notes join the bodies of the pull requests into `main` merged since the last tag, each holding the texts of the features it brings, written at their review; `CHANGELOG.md` is deleted, 0.1.0's and 0.2.0's entries staying in their GitHub releases.

## Reasons

- One agent at a time trades speed for usage: no merge nor rebase is done twice, and each worker's cold start is paid once per story.
- A story merged into `develop` by a merge commit keeps the graph the Product Owner reads, without a feature branch to rebase.
- A review taken when `develop`'s head is a finished feature needs no branch to mark it, and one pull request brings every feature finished since the last merge into `main`.
- A button computes the number, so the Product Owner chooses when to release without choosing, or mistyping, the number (ADR-0083's reason against a tag typed by hand).
- The features' texts are written once, by the agent who knows the feature, and read by the Product Owner at the review; commits would give a raw list (0.2.0's draft had 163 change lines before sorting).

## Consequences

- `story-automerge.yml` merges a story into `develop` by a merge commit; `worktree-new` starts a story from `origin/develop`; `story:finish` takes `develop` as its base; the run skill, the worker brief, `project-rules`, `AGENTS.md` (Work, Git) and `docs/tooling/git-workflow.md` describe the flow, the run skill saying that a review asked mid-feature waits for that feature's end (EN-122).
- `release prepare` and `release notes` draft a feature's text, `release check` reads it, `release publish` runs from the button; `release.yml`'s push to `main` deploys the site only (EN-117).
- F22 is the first feature made so: its feature branch, opened at the run's start, is deleted unused.

## References

ADR-0072, ADR-0083, ADR-0084, ADR-0085, `REF-SEMANTIC-RELEASE-FAQ`, `REF-GH-WORKFLOW-DISPATCH`, `REF-SEMVER`
