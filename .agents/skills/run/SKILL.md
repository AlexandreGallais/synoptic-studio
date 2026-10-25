---
name: run
description: Autonomous run — chain the stories of an authorized scope, one worker per story in the background, each merged into its feature branch by a merge commit, until the scope the Product Owner set ends; a question parks its story and the run goes on. Use only when the Product Owner explicitly starts a run, naming a feature or a scope (in French they say to launch or chain it).
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run

Scope: $ARGUMENTS. The main thread dispatches, finishes and answers; the chat stays free for the Product Owner.

## The scope

- **Only on the Product Owner's start**, which is the go-ahead: no other question but business ones. The scope is a time or an end (a story, a feature, an epic); without argument, the feature in progress to its end, else the next one.
- **The order guard**: the next feature is the one `in-progress`, else the first not `done`, by position, of the first epic not `done` by position (`knowledge sql "SELECT … FROM epics, features …"`). A scope naming another feature is refused: say which is next and why.
- **A new epic** opens with its refinement (`/spike`), every question asked at once, so that the Product Owner can leave after it.

## Before the first story

1. Set the feature `in-progress` and its stories `ready` (`target/release/backlog-status <ID> <status>`); `target/release/run-totals start`.
2. The feature branch, once the previous feature is in `develop`: `git fetch origin`, then `git push origin origin/develop:refs/heads/feature/f<nn>-<topic>`. The main checkout stays on `develop`; `develop` enters a feature branch by a merge, never a rebase.
3. A journal row (`knowledge add journal -`): the scope, the date, the request in neutral words.

## For each story

1. **Pick up `develop`**: `git fetch origin`, then `git log --oneline feature/f<nn>-<topic>..origin/develop -- AGENTS.md CLAUDE.md .agents .claude`; anything listed is merged into the feature branch in a worktree of it, and applied from this story on.
2. **Its worktree**: `target/release/worktree-new <ID>` from the main checkout.
3. **Its worker**, one `Agent` call in the background, a fresh one per story, never one agent chaining stories:

   | Task | Agent type |
   | --- | --- |
   | a `US` or `EN` story, a review's naming pass, an audit's fixes | `story-worker` |
   | a research spike (`SP`) | `researcher` |
   | an audit | `auditor` |
   | mathematics or business rules to check | `math-expert`, `functional-analyst` |
   | anything else | `general-purpose`, given `<worktree>/.agents/skills/worker-brief/SKILL.md` |

   Its prompt: the story id, the feature branch, the worktree, and that every Bash call starts with `cd "<worktree>" || exit 9`. A rule a worker missed goes into the worker brief or `project-rules`.

4. **A binary that runs a command given to it**: once its worker hands back the guard's task, build `guard-bash` in the worktree (`cargo build --release -p hooks --bin guard-bash`), copy it over the main checkout's, check a refused command is denied, then start a worker for the rest.
5. **Finish it**: `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>` in its worktree (`/story`, step 6). Not merged: `gh run view <run> --log-failed`; behind its base: `gh pr update-branch <n>`; "Dependencies not up-to-date": read the release notes, update on the story branch and on `develop`.

## The chat

- One line per commit of the worker (a Monitor on its worktree's commits); at each story's end, the epic as a path of its features, each step marked ✅ done, ⏳ now, ⬜ to do, with one progress bar in plain words, no bare ratio.
- Each message of the Product Owner acknowledged at once, saying when it is handled.
- A tool that fails is worked around and recorded as an incident (`knowledge add incident -`), fixed at the next review.

## A question parks its story

The worker pushes what it did and reports the question; the main thread sets the story `parked` with the question in its row (`question`), asks it in the questionnaire, pushes a phone notification only when it blocks the scope, and goes on with the stories that do not depend on it. Once answered: the answer in the row, `ready` again, a new worker on its branch.

## Stop when

The scope ends; a blocker with no independent story left (a check red three times, an access missing, every story left parked); or the Product Owner says stop. Then `target/release/run-totals end`, a journal row, and a short message in French: what was built, the test cards to try, the questions, what the next step needs.

## A feature's end

1. A feature ends with its audit (`/audit`); in an epic tagged **method and tools**, with its last story, the epic ending with one audit and the full check.
2. One pull request from the feature branch into `develop`, labeled `autonomous`, its body the stories with their pull requests; GitHub merges it by a merge commit after the full check.
3. The main checkout brought to `develop` (`git switch develop`, `git merge --ff-only origin/develop`, `npm ci`) so that the Product Owner's `npm run dev` shows the latest work; the feature branch deleted (`gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>`).
