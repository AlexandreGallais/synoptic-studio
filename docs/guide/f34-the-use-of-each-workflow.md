# F34 — The use of each workflow

GitHub's Actions tab now keeps only the workflows the project uses, as the Product Owner decided on 2026-10-18: the auto-merge split by base, each run once per pull request, the outside pull requests' workflow gone ([ADR-0094](../adr/0094-outside-pull-requests-kept-out-by-the-setting-alone.md)), CodeQL on the workflows too. A tooling feature, validated by the agent (VAL-035).

## What you see in the Actions tab

| Workflow | Starts on | Does |
| --- | --- | --- |
| Story auto-merge (`story-automerge.yml`) | a story's pull request into a feature branch, labelled `autonomous` | merges it by a merge commit |
| Feature auto-merge (`feature-automerge.yml`) | a feature's or a review's pull request into `develop`, labelled `autonomous` | merges it by a merge commit, then starts _Develop CI_ |
| CodeQL (`codeql.yml`) | a pull request into `main`, a push to `main`, weekly | analyses the TypeScript and the workflows in one job, `Analyze` |
| Links (`links.yml`) | weekly, or by hand | checks the pages' links, blocking nothing |

_External pull requests_ is gone: the repository's setting lets only collaborators open a pull request, and its 275 runs were all skipped. Whether its name leaves the Actions tab once the file reaches `main` is read at the next review.

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. the auto-merge per base, the workflows as decided | `[F34.AC1]` (`tests/story-automerge.test.ts`, `tests/workflows.test.ts`, `story_finish.rs`); `[F28.AC1]` with no exception | passed |
