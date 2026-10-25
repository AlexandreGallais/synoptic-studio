# ADR-0086 — One story at a time, a merge commit for every merge, a review at each return, the release by a button

**Status**: Accepted (Product Owner, 2026-10-18, during F22's run, revised the same day); its feature frame (a `VAL` per feature, a feature into `develop` once validated) superseded by ADR-0100 on 2026-10-24. Supersedes ADR-0083, removed since (ADR-0101): the release is no longer made by the merge into `main`, and `CHANGELOG.md` goes. Supersedes, in ADR-0084, decision 3 for the rebase of a story into its feature and of a feature onto `develop`, and for `develop` → `main` at the epic's end; decision 4 (two agents per story); and the parallel researchers of decision 5. ADR-0084's decisions 1 and 2, small features and tests first in their own commit, stand, and so does its feature branch from `develop`. Supersedes, since the review now holds them: ADR-0085's tools changed only at an epic's end (now at each review), ADR-0049's naming pass per epic and ADR-0023's retrospective per epic (now in each review). Implemented by [EN-122](../backlog/stories/E17-F22-EN-122-one-story-at-a-time-into-develop.md), [EN-117](../backlog/stories/E17-F22-EN-117-release-from-the-merge-into-main.md) and EN-123.

## Context

- During F21 and F22's start the parallel work cost more than it saved: two stories open at once meant rebases and merges done again, each by an agent that starts cold, and a week's usage that a run of two days could spend whole. The Product Owner prefers a slower run they can afford each day.
- A story rebased into its feature leaves no trace of itself in the graph, and a feature rebased onto `develop` is rewritten under new hashes; a merge commit keeps each piece of work visible where it joined.
- Under ADR-0083 (removed since, ADR-0101) the merge into `main` is the release, so the site cannot follow a reviewed feature without making a version.
- The Product Owner comes back once or twice a day; an epic's closing (`NAM`, `REV`, `RET`) came weeks apart, so the naming, the tool fixes and the method's changes waited that long.
- semantic-release reads the version from the tags and writes it only into the published package, never in a commit (`REF-SEMANTIC-RELEASE-FAQ`); GitHub runs a workflow on demand from a button of its Actions tab (`workflow_dispatch`, `REF-GH-WORKFLOW-DISPATCH`); `release.yml` already accepts it.

## Decision

1. **One agent at a time, in the background.** A run works one story at a time, one worker for it, in the background, so that the main thread stays free for the Product Owner, who may ask and try ideas while it works; a review or a research runs alone too. No two stories, features or researchers at once. A slower run is accepted for its lower usage. Added the same day: a **fresh worker per story**, never one agent chaining stories, since an agent's cost is its growing context re-read at each call (EN-122's worker ended near 255,000 tokens), not its start (about 10,600); **specialists only where their independence pays**, the `auditor` (and the `verifier` when tests need a second look) at a feature's audit, the `math-expert` and the `functional-analyst` only for a story holding mathematics or business rules to check; **no agent for small tasks**, the main thread writing the statuses, the journal, the `done` commits and the merges itself, the `clerk` unused (EN-123).
2. **A feature branch per feature, from `develop`.** `feature/f<nn>-<topic>` is made from `origin/develop` when the feature starts, and deleted once merged: its bubble and its name in the merge commit stay. Each story's branch `<type>/<id>-<topic>` is made from its feature branch.
3. **Every merge is a merge commit.** A story enters its feature branch by a merge commit, by GitHub's auto-merge in a run; a feature enters `develop` by a merge commit. The graph shows a small bubble per story on the feature's line and a big bubble per feature on `develop`.
4. **A review at each return of the Product Owner**, once or twice a day, no longer at an epic's end. A feature under way is finished first, and the agent says so. Then a `REV` story, its branch from `develop` with no feature branch, holds the review of the features finished since the last merge into `main`, the naming pass over the code changed since then, the tool incident cards fixed, the improvements of the method decided with the Product Owner, and the docs brought up to date; it enters `develop` by a merge commit.
5. **`main` receives each review.** After the `REV`'s merge the agent opens `develop` → `main`, its body the features' texts for the release (decision 7) checked by `release check`; its CI is the only one (ADR-0072). The Product Owner merges it by a merge commit, the review's mark on `main`; the push to `main` deploys the site and makes no release.
6. **The release is a button.** The Product Owner runs `release.yml` on `main` from GitHub's Actions tab. `release publish` computes the version from the last tag `vx.y.z` and the commits since, tags `main`'s head and makes the GitHub release; nothing is committed and no number is typed. The version lives only in the tag, the release and, later, a published package, written into it at publish time; `package.json` holds `0.0.0-development`.
7. **The notes are the features' texts.** The release's notes join the bodies of the pull requests into `main` merged since the last tag, each holding the texts of the features it brings, written at their review; `CHANGELOG.md` is deleted, 0.1.0's and 0.2.0's entries staying in their GitHub releases.
8. **An epic is a list of features**, with no closing of its own: no `NAM`, `REV` nor `RET` at its end, their work done at each review.

## Reasons

- One agent at a time trades speed for usage: no merge nor rebase is done twice, and each worker's cold start is paid once per story.
- A merge commit for every merge keeps the graph the Product Owner reads: each story and each feature stays one bubble, with its name, and no commit is rewritten.
- A review at each return keeps the naming, the tools and the method a day behind the code at most, instead of an epic.
- A button computes the number, so the Product Owner chooses when to release without choosing, or mistyping, the number (the reason of ADR-0083, recorded in ADR-0101, against a tag typed by hand).
- The features' texts are written once, by the agent who knows the feature, and read by the Product Owner at the review; commits would give a raw list (0.2.0's draft had 163 change lines before sorting).

## Consequences

- `story-automerge.yml` merges by a merge commit a story into its feature branch, and into `develop` a feature branch or a review; `worktree-new` keeps starting a story from its feature branch, and a `REV` from `develop`; the run skill, the worker brief, `project-rules`, `AGENTS.md` (Work, Git) and `docs/tooling/git-workflow.md` describe the flow, the run skill saying that a review asked mid-feature waits for that feature's end (EN-122).
- `release prepare` and `release notes` draft a feature's text, `release check` reads it, `release publish` runs from the button; `release.yml`'s push to `main` deploys the site only (EN-117).
- The epics without closing, the review story's template and skill, E02's `NAM-003`, `REV-005` and `RET-005`, and `docs/process/agile.md` are rewritten by EN-123, with the run skill, `AGENTS.md`, `CLAUDE.md` and `docs/tooling/agent.md` for the fresh worker per story, the specialists and the main thread's small tasks.
- F22 is the first feature made so, on `feature/f22-release-by-a-button`.

## References

ADR-0023, ADR-0049, ADR-0072, ADR-0084, ADR-0101 (ADR-0083, removed), ADR-0085, `REF-SEMANTIC-RELEASE-FAQ`, `REF-GH-WORKFLOW-DISPATCH`, `REF-SEMVER`
