# F23 — The path of a story through the tools

A story's log rows and tool cards now reach the logs in its own `done` commit, inside its bubble, so that a feature's graph shows nothing between its story bubbles; and a story outside a feature, a review or an epic's refinement, goes through the same tools as any story ([ADR-0091](../adr/0091-a-storys-log-rows-and-cards-in-its-done-commit.md)). A tooling feature, validated by the agent (VAL-024).

## What you see in the graph

| Before | After |
| --- | --- |
| after each story's merge, a commit `docs(docs): move the rows of … into the logs` on the feature's line | the feature's line goes from one story's merge commit to the next; the rows are in each story's `done` commit |
| `story:finish develop` for an epic's refinement waited 3 minutes for a merge GitHub never makes | it opens the pull request, says the Product Owner merges it, and cleans up once run again after the merge |
| `worktree-new` from `develop` could not find a story only on its feature branch | it reads the feature branches of `origin` and starts from the one holding the story |

## What guards it

- A `done` commit may only add to a log the rows and cards it removes from its own story file; any other change to a log is refused at the push (AUD-025).
- A card under `### Incidents` needs its date, its use, its failure and its workaround; a malformed one stops `story:finish` (AUD-025).
- One branch form for every story and fix, `<type>/<id>-<topic>`, the same rule for the pre-push, `worktree-new` and the auto-merge.

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. rows and cards in the `done` commit | `f23_ac1_*` (`shared-files`, `repo-tools`), `[F23.AC1]` (`tests/logs.test.ts`); F23's own graph: no commit between its bubbles | passed |
| 2. a story outside a feature by the same tools | `f23_ac2_*` (`worktree`, `repo-tools`, `git-hooks`), `[F23.AC2]` (`tests/backlog.test.ts`, `tests/story-automerge.test.ts`) | passed |
