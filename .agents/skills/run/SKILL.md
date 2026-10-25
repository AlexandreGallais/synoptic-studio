---
name: run
description: Autonomous run — chain the stories of an authorized scope on its feature branch, one at a time, each with its checks, merged into the feature branch by a merge commit, until the scope the Product Owner set ends; a question parks its story and the run goes on. Use only when the Product Owner explicitly starts a run, naming a feature or a scope to chain (in French they say to launch or chain it).
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run

A run delivers the scope the Product Owner set while they are away, and keeps the main thread free for them when they are back: one worker at a time does the stories, in the background; the main thread dispatches, finishes and answers.

Scope: $ARGUMENTS.

## The scope

- **Only on the Product Owner's explicit start** in this conversation. They set the scope as a time (until an hour) or an end (a story, a feature, an epic); the run sets no bound of its own. Without argument, the scope is the feature `in-progress`, from its next unfinished story up to its end; with none in progress, propose the next feature; a new epic starts with its whole refinement, every question asked at once, an `SP` from `develop` (`/spike`).
- **The order guard**: the next feature is the one `in-progress`, else the first feature not `done`, by its position, of the first epic not `done` in the order of `docs/backlog/README.md`. A scope naming another feature is refused before anything else: say which feature is next and why, and do nothing.
- **A refinement takes two rounds**, so that the Product Owner can leave after the second: first the questions the research prepared, each with options and numeric examples, then one open question for every remark; second, every remaining question at once. After it the run asks nothing more; a business rule nobody foresaw parks its story.

## Before the first story

1. The first feature of an epic: `target/release/epic-check <epic> <first day>`, the code scanning alerts included; what it finds is fixed or listed for the Product Owner.
2. **`/run` is the go-ahead**: set the feature `in-progress` and its stories `ready` yourself (`target/release/backlog-status <ID> <status>`, rows of the records, no commit); ask the Product Owner only business questions. Then `target/release/run-totals start`, which counts the run's duration and tokens from now.
3. A journal row in the records: the scope set, the date, the request in neutral words.
4. The feature branch `feature/f<nn>-<topic>` is created on `origin` from `develop` once the previous feature is in `develop`: `git fetch origin`, then `git push origin origin/develop:refs/heads/feature/f<nn>-<topic>`; the main checkout stays on `develop`. It is never rebased: `develop` enters it by a merge.
5. `/triage` if issues are open: present them at the next stop, act on none.
6. List the scope's stories in delivery order; stop now if the next is a `REV`, which needs the Product Owner.

## The main thread

The chat belongs to the Product Owner, who may write at any time; so the main thread only dispatches, and the work goes to subagents.

- **Silent between tool calls**: the commits and the status line (`.git/metrics/step`) show the progress. The chat gets one line when a worker starts and when it ends; one summary at the stop; a question when one parks a story.
- **Each comment of the Product Owner acknowledged at once**, saying when it is handled: now, or once the running worker has finished. A tool that fails during the run becomes an incident card in the records, worked around and never fixed during a feature: the next review fixes it.
- **One agent at a time**: one worker per story, an `Agent` call in the background, in a worktree made by `target/release/worktree-new <ID>`, a fresh worker per story, never one agent chaining stories.

  | Task | Agent type |
  | --- | --- |
  | a `US` or `EN` story, a `REV`'s naming pass, the fixes of an `AUD` | `story-worker` |
  | a research spike (`SP`) | `researcher` |
  | an audit | `auditor` |
  | mathematics or business rules to check in a story | `math-expert`, `functional-analyst` |
  | a task none of them fits | `general-purpose` |

  Its prompt is short: the story id, the feature branch, the worktree, and that every Bash call starts with `cd "<worktree>" || exit 9`. A `general-purpose` worker is also given the brief's path inside the worktree, `<worktree>/.claude/skills/worker-brief/SKILL.md`. A rule a worker missed goes into the [worker brief](../worker-brief/SKILL.md) or `project-rules`.

- **The main checkout stays on `develop`**, so a rule or skill read there may be older than the feature branch's: read them in a worktree of the feature branch.
- **A background Git command** of the main thread starts with `cd <worktree> || exit 9` and checks `git branch --show-current` before it pushes, finishes or removes anything.
- **A rebase conflict** is resolved in the working file, never by `checkout --ours` or `--theirs`, then diffed against the story's version before `rebase --continue`.

## For each story

1. **Pick up `develop`**, since the Product Owner may decide something in another session: `git fetch origin`, then `git log --oneline feature/f<nn>-<topic>..origin/develop -- AGENTS.md CLAUDE.md .claude docs`. Commits listed: merge `origin/develop` into the feature branch in a worktree of that branch, push it without force, and apply their changes from this story on; a change to the run's stories or scope is a stop.
2. **Its worktree**: `git fetch origin`, then `target/release/worktree-new <ID>`, run in the main checkout.
3. **Its worker** works the tasks as its brief says, pushes, and stops before the pull request.
   - **A binary that runs a command given to it** (`heavy-lock`, `bench`, a wrapper): the story lands `guard-bash`'s reading of that command in a task before the one that builds the binary; its worker pushes that task and hands back; the main thread builds `guard-bash` in the story's worktree (`cargo build --release -p hooks --bin guard-bash`), copies it over the main checkout's, checks that a refused command run through it is denied, then starts a worker for the remaining tasks.
4. A **user story** (`US`) gets its Product Owner test card in its row: the steps in the playground, the expected result of each, in plain words.
5. **No checkpoint**: each story of at most 3 tasks; a larger one found while working is a stop, its split proposed to the Product Owner with an example.
6. **Finish it**: `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>` in its worktree. It rebases the story on its feature branch, pushes it with lease when the feature moved, opens the pull request with the `autonomous` label, waits for GitHub's merge by a merge commit, marks the story's rows merged and the story `done` in the records, updates the feature branch where it is checked out, and removes the worktree and the branches.
   - A rebase conflict: it stops, the branch as it was. Not merged: `gh run list --workflow <workflow> --limit 1`, the workflow `story-finish` names, then `gh run view <run> --log-failed`; a branch behind its base: `gh pr update-branch <n>`.
   - "Dependencies not up-to-date" at the push: a version passed its 7-day cooldown meanwhile; read its release notes, update it on the story branch (`build(deps)`) and on `develop`.
7. Then the next story.

## A question parks its story

A question for the Product Owner never stops the run: their time away is spent on the stories that do not need the answer.

1. The worker reports the question, what it read and the options, without choosing; it pushes what it did and stops. Its branch and worktree are kept.
2. The main thread parks the story: `target/release/backlog-status <ID> parked`, the question written in its row (`question`).
3. It asks the question in the questionnaire and pushes a notification ([Pushes](#pushes)).
4. It goes on with the next story independent of the parked one: a story that touches its files, builds on its code, or whose criteria the answer may change waits too.
5. Once answered: the answer written in the story's row, status `ready` again, a new worker started on its branch.

## Pushes

A push notification reaches the Product Owner away from the screen, so it is kept for what only they can unblock.

- **Push** a question that parks a story and a blocker the run cannot get around, once each, when either prevents finishing the scope: one line naming the story and what it waits for.
- **Never push** a merge, a stop at the scope's end, a plain stop, nor a run ending early: the message at the stop says those.
- **The chat holds the question**, the push only points to it.

## Stop when

A run stops only when it must, and says which in its message:

- **its scope ends**: the time set has passed, or the end named is reached; the next story is a `REV`, which needs the Product Owner;
- **a blocker it cannot get around** and no independent story left: a check still red after three attempts, a tool or an access missing, every story left parked, blocked, or depending on one that is;
- **the Product Owner says stop**.

## At the stop

1. `target/release/run-totals end`: it closes the run and prints its table, the duration and the tokens by kind, subagents included.
2. A journal row in the records: the stories merged with their pull requests, the findings, the stop's reason.
3. The message to the Product Owner, in French: what was built in plain words, the **test cards to run**, the findings, the questions, what the next step needs from them, the run's table as it is.

## A feature's end

A feature ends with its audit. In an epic tagged **method and tools** (E49, E50), a feature ends with its last story, and the epic ends with one audit and a full check.

1. Once its `AUD` is merged into the feature branch and the feature set `done`, open **one** pull request from the feature branch into `develop`, its body the stories with their pull requests and the evidence, labeled `autonomous`: GitHub merges it with a merge commit.
2. Then bring the main checkout to `develop` (`git switch develop`, `git merge --ff-only origin/develop`, `npm ci`), so that the Product Owner's `npm run dev` shows the latest work; delete the feature branch here and on GitHub (`gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>`).
3. The next feature's branch starts from that `develop`.
4. When the Product Owner comes back, give them the short list of what changed since their last try.

## Review (only when the Product Owner asks)

1. **A feature under way is finished first**: tell the Product Owner the run finishes it, then holds the review.
2. **A `REV` story**, its branch from `develop` (`target/release/worktree-new REV-<nnn>`), holds what `/review` lists. It is finished at the end of its session with the Product Owner, then enters `develop` by its auto-merge (`npm run story:finish -- develop …`).
3. **Then `develop` → `main`**, once the Product Owner validated the review in the chat: draft the features' text with `target/release/release notes` on an up-to-date `develop`, rewrite each `[draft]` line into a file of the scratch folder, run `target/release/release check <file>` until it passes, then open the pull request with that file for body (`gh pr create --base main --head develop --body-file <file>`), and label it `po-validated`, which `main-automerge.yml` merges once the required checks pass. Its CI is the only one: read its checks once done (`gh pr checks <n>`), a failure at once (`gh run view <run> --log-failed`). The push to `main` deploys the site and makes no release. A journal row.
4. **The release**, when the Product Owner asks: `gh workflow run release.yml --ref main`; `release publish` computes the version from the tags, its notes the bodies of the pull requests into `main` merged since the last tag; nothing is committed. A body that breaks a rule stops it, naming the pull request: fix the body on GitHub and run it again.
