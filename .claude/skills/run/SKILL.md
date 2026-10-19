---
name: run
description: Autonomous run — chain the stories of an authorized scope on its feature branch, one at a time, each with every check, merged into the feature branch by a merge commit once checked locally, until the scope the Product Owner set ends; a question parks its story and the run goes on. Use only when the Product Owner explicitly starts a run, naming a feature or a scope to chain (in French they say to launch or chain it).
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run (ADR-0072)

A run delivers the scope the Product Owner set while they are away, and keeps the main thread free for them when they are back: one worker at a time does the stories, in the background, the main thread dispatches, merges and answers (ADR-0086). A slower run is accepted for its lower usage.

Scope: $ARGUMENTS.

## The scope

- **Only on the Product Owner's explicit start** in this conversation. They set the scope at the start, as a time (until an hour) or an end (a story, a feature, an epic), so that they decide how much is delivered; the run sets no bound of its own (2026-10-17). Without argument, the scope is the feature `in-progress`, from its next unfinished story up to its `VAL`; with none in progress, propose the next feature: a new feature starts with its refinement, never with code; a new epic with its own, an `SP` from `develop` (`/spike`, ADR-0086), which the Product Owner merges: the auto-merge lets into `develop` only a feature branch or a `REV` (AUD-024), so `npm run story:finish -- develop …` opens its pull request without the label, says it waits for that merge and stops; run again once merged, it cleans up (EN-118).
- **The order guard**: the backlog's order is the one place where the Product Owner sets priorities (2026-10-10), so a run takes only the next feature: the one `in-progress`, else the first feature not `done` in the **Features** table of the first epic not `done` in `docs/backlog/epics/README.md` (table order, not file order). A scope naming another feature, or one of its stories, is refused before anything else: say which feature is next and why, and do nothing. To change it, the Product Owner reorders the tables on `develop` and starts the run again.
- **A refinement takes two rounds** (2026-10-14), so that the Product Owner can leave after the second: first the questions the research prepared (with the `functional-analyst` only when business rules are to check), each with options and numeric examples, then one open question for every remark, idea and constraint; second, every remaining question at once, again ending with room for remarks. After it the run asks nothing more; a business rule nobody foresaw parks its story.

## Before the first story

1. The first feature of an epic: `target/release/epic-check <epic> <first day>`, the code scanning alerts included; what it finds is fixed or listed for the Product Owner.
2. **`/run` is the go-ahead**: set the feature `in-progress` and its stories `ready` yourself, a `docs(backlog)` commit on `develop`; never ask the Product Owner to, and ask them only business questions. Then `target/release/run-totals start` (EN-092), which counts the run's duration and tokens from now; a run continued in a new session runs it again, which adds the session and keeps the start.
3. Record the authorization in `docs/process/journal.md` on `develop`: the scope set, the date, the request in neutral words.
4. The feature branch `feature/f<nn>-<topic>` exists on `origin`, or is created there from `develop`, once the previous feature merged (ADR-0084), without switching the main checkout; while it is open nothing is pushed to `develop`, the main thread's docs and log rows going to the feature branch: `git fetch origin`, then `git push origin origin/develop:refs/heads/feature/f<nn>-<topic>`. It is never rebased: `develop` enters it by a merge (ADR-0086).
5. `/triage` if issues are open: present them at the next stop, act on none.
6. List the scope's stories in delivery order; stop now if the next is a library feature's `VAL` or a `REV`, which need the Product Owner (a tooling feature's `VAL` is the agent's, ADR-0072). A story a business question of `docs/domain/README.md` blocks is parked, and the run starts with the next one independent of it.

## The main thread

The chat belongs to the Product Owner (2026-10-12), who may write at any time; so the main thread only dispatches, and the work goes to subagents.

- **Silent between tool calls**: the commits and the status line (`.git/metrics/step`, updated at each step) show the progress. The chat gets one line when a worker starts and when it ends, with an estimate of its duration; one summary at the stop; a question when one parks a story. Never "waiting for an agent" nor "message from the auditor".
- **Each comment of the Product Owner acknowledged at once**, saying when it is handled: now, or once a named running agent has finished. A comment about rules or configuration is handled once the running worker has ended, in its own worktree; a comment about a tool, and a tool that fails during the run, become a card in the tool incidents (how it was used, why it failed, what was done instead), worked around and never fixed during a feature's work: the cards are read at the next review, which fixes the tools (ADR-0085, ADR-0086).
- **One agent at a time** (ADR-0086): one worker per story, an `Agent` call in the background, never two stories, features, researchers or reviews at once, in a worktree made by `target/release/worktree-new <ID>`, of the type its task kind uses (EN-065).
- **A fresh worker per story**, never one agent chaining stories nor resumed for the next: an agent's cost is its growing context re-read at each call, not its start (EN-122's worker ended near 255,000 tokens, a start costs about 10,600).
- **Specialists only where their independence pays**: the `auditor`, and the `verifier` when the tests need a second look, at a feature's audit; the `math-expert` and the `functional-analyst` only for a story holding mathematics or business rules to check, never by default. **No agent for small tasks**: the statuses, the journal, the `done` commits and the merges are the main thread's own; the `clerk` is no longer started.

  | Task | Agent type |
  | --- | --- |
  | a `US` or `EN` story, a `REV`'s naming pass, the fixes of an `AUD`, the preparation of a `VAL` | `story-worker` |
  | a research spike (`SP`) | `researcher` |
  | a feature's audit | `auditor`, then `verifier` when the tests need a second look |
  | mathematics or business rules to check in a story | `math-expert`, `functional-analyst` |
  | a task none of them fits | `general-purpose` |

  Its prompt is short: the story id, the feature branch, the worktree, and that every Bash call starts with `cd "<worktree>" || exit 9`. A typed worker has its brief preloaded; a `general-purpose` worker is also given the brief's path **inside the worktree**, `<worktree>/.claude/skills/worker-brief/SKILL.md`. A rule a typed worker missed goes into the [worker brief](../worker-brief/SKILL.md) or `project-rules`, never into every agent's.

- **The main checkout stays on `develop`**, so a rule, skill or brief read there may be older than the feature branch's: read them in a worktree of the feature branch.
- **A background Git command** of the main thread starts with `cd <worktree> || exit 9` and checks `git branch --show-current` before it pushes, finishes or removes anything.
- **A rebase conflict** is resolved in the working file, never by `checkout --ours` or `--theirs`, then diffed against the story's version before `rebase --continue`: a whole side taken drops its other lines.
- **The main thread writes the small things itself**: merges, `done` commits, statuses, the journal and log moves, once the story's worker has ended (ADR-0086). The shared files (indexes, logs, feature file, checklist) are written by the main thread only (ADR-0065), a story's log rows and cards in its `done` commit (ADR-0091).
- **The main thread's own part**: answer the Product Owner, say where the work is (`target/release/activity`, the status line), finish each story (`npm run story:finish`), write the shared files, and ask the business questions workers bring back, in the questionnaire, with room for remarks.

## For each story

1. **Run `target/release/run-totals should-stop`** in a worktree of the feature branch ([Stop when](#stop-when)).
2. **Pick up `develop`**, since the Product Owner may decide something in another session: `git fetch origin`, then `git log --oneline feature/f<nn>-<topic>..origin/develop -- AGENTS.md CLAUDE.md .claude docs/domain docs/adr docs/backlog docs/conventions`. Commits listed: merge `origin/develop` into the feature branch in a worktree of that branch, never the main checkout, push it without force, read their diffs and apply them from this story on; a changed skill or rule replaces what was loaded at the start, and a change to the run's stories or scope is a stop.
3. **Its worktree**: `git fetch origin`, then `target/release/worktree-new <ID>` (`.claude/worktrees/<id>` on `<type>/<id>-<topic>`, from the feature branch on GitHub), run in the main checkout: a story whose file is only on its feature branch is found there (EN-118).
4. **Its worker** works the tasks as its brief says, pushes, and **stops before the `done` commit**, which writes the shared files only the main thread writes (ADR-0065, enforced by `refuse-shared-files`, EN-050). The main thread makes it in the story's worktree: `target/release/move-rows <branch>`, which moves the rows and cards under the story file's `## For the main thread` into the logs and prints the files it wrote (ADR-0091), `target/release/backlog-status <ID> done`, then `target/release/git-sequences commit -F <message file> docs/backlog <the files move-rows printed>`, footer `Closes: <ID>`, and the `Rule-docs:` line when the report names one (`/story` step 7).
   - **A binary that runs a command given to it** (`heavy-lock`, `bench`, a wrapper): the guard judges every hook call by the main checkout's `target/release/guard-bash`, so the story lands `guard-bash`'s reading of that command in a task before the one that builds the binary. Its worker pushes that task and hands back; the main thread builds `guard-bash` in the story's worktree (`cargo build --release -p hooks --bin guard-bash`), copies it over the main checkout's, checks there that a refused command run through the new binary is denied, then starts a worker for the remaining tasks. Until the feature reaches `develop`, a rebuild of the main checkout (`prepare`, `post-merge`) brings back the older guard: copy it again before the next call of that binary (AUD-035, F2: EN-141's `bench` was built before the main checkout's guard read it).
5. A **user story** (`US`) gets its Product Owner test card in its file, `## Product Owner test`: the steps in the playground, the expected result of each, terms in plain words, what to look at closely.
6. **No checkpoint**: a feature holds at most 4 stories, each of at most 3 tasks (ADR-0084, EN-123); a larger one found while working is a stop, its split proposed to the Product Owner with an example.
7. **Checks by level** (ADR-0064): no review per story; the worker wrote the edge cases and mutation-checked each new test; the feature's `AUD` audits in depth, and the naming pass over the code changed runs at each review.
8. **Finish it**: `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>` in its worktree (the `story-finish` binary of `tools/repo-tools/`, EN-103). It rebases the story on its feature branch and pushes it with lease when the feature moved (the status rows written there meanwhile are left to the feature and the story's written again after the rebase, EN-103), opens the pull request with the `autonomous` label (a `VAL` is finished only once its feature is validated: by the agent for a tooling feature, by the Product Owner for a library feature, ADR-0072; a `REV` at the end of its session with the Product Owner, ADR-0086; each then labelled as any story), waits at most three minutes for GitHub's merge by a merge commit (`story-automerge.yml`, no CI), updates the feature branch where it is checked out, commits nothing on it, and removes the worktree and the branches; a story file still holding rows for the logs stops it before the pull request (ADR-0091).
   - A rebase conflict outside the status rows: it stops, the branch as it was. Not merged: `gh run list --workflow story-automerge.yml --limit 1`, then `gh run view <run> --log-failed`; a branch behind its base: `gh pr update-branch <n>`.
   - "Dependencies not up-to-date" at the push: a version passed its 7-day cooldown meanwhile; read its release notes (`docs/tooling/versions-and-security.md`), update it on the story branch (`build(deps)`) and on `develop`.
9. Statuses change with `target/release/backlog-status <ID> <status>` (the file, the index and the feature's table at once), edits across files with `target/release/replace`; every agent tool by its binary from the checkout's root, as `docs/tooling/agent-tools.md` lists them (ADR-0078, ADR-0082), `npm run` kept for the scripts `package.json` keeps: `check:all`, `story:finish`, `mutate`, `golden:check`, `fix` and `test` among them. Then the next story.

## A question parks its story

A question for the Product Owner never stops the run (2026-10-17): their time away is spent on the stories that do not need the answer. Such a question is a business question, a research request, a stop of `AGENTS.md`, or an audit finding that changes a requirement.

1. The worker reports the question, what it read and the options, without choosing; it pushes what it did and stops. Its branch and worktree are kept.
2. The main thread parks the story in a worktree of the feature branch: `target/release/backlog-status <ID> parked`, the question written in the story file under `## Question for the Product Owner`, a `docs(backlog)` commit pushed. The file, not the chat, keeps the question across sessions.
3. It asks the question in the questionnaire and pushes it ([Pushes](#pushes)).
4. It goes on with the next story independent of the parked one: a story that touches its files, builds on its code, or whose criteria the answer may change waits too.
5. Once answered: the answer written in the story, status `ready` again, its worker resumed or a new one started on its branch.

## Pushes

A push reaches the Product Owner away from the screen, so it is kept for what only they can unblock (2026-10-17). `PushNotification` sends a desktop notification, and a phone push when Remote Control is connected (`REF-CC-TOOLS-REFERENCE`); Claude decides when to push, no setting chooses the events (`REF-CC-REMOTE-CONTROL`).

- **Push** a question that parks a story and a blocker the run cannot get around, once each, when either prevents finishing the scope: one line naming the story and what it waits for. The run goes on with independent work meanwhile.
- **Never push** a merge, a stop at the scope's end, a plain stop, nor a run ending early: the message at the stop says those.
- **The chat holds the question**, the push only points to it: a push is skipped while the Product Owner types in or focuses the terminal, and phone pushes stay off until they turn them on ([agent configuration](../../../docs/tooling/agent.md#pushes)). A push refused or off changes nothing.

## Stop when

A run stops only when it must, and says which in its message:

- **its scope ends**: the time set has passed, or the end named is reached; the next story is a library feature's `VAL` or a `REV`, which needs the Product Owner;
- **a blocker it cannot get around** and no independent story left: a check still red after three attempts, a tool or an access missing, every story left parked, blocked, or depending on one that is;
- **the Product Owner says stop**;
- **the week's usage reaches 90 %**, so that the margin stays the Product Owner's own work until the week resets: `target/release/run-totals should-stop` prints `stop` and exits 1 once `sevenDay.usedPercent` of `.git/metrics/usage.json` reaches it, and lists the parked stories of the checkout it runs in.

## At the stop

1. `target/release/run-totals end` (EN-092): it closes the run and prints its table, the duration and the tokens by kind, subagents included.
2. A journal entry on `develop`: the stories merged with their pull requests, the findings, the stop's reason.
3. The message to the Product Owner, in French (`docs/conventions/writing.md`): what was built in plain words, the **test cards to run**, the findings, the questions, what the next step needs from them, the run's table of step 1 as it is, and at a session's end the session report.

## Validation (`VAL`, with the Product Owner)

The Product Owner may test over several sessions: never wait for them in a loop, and start nothing else until they have finished.

1. `target/release/worktree-new VAL-<nnn>`; there, merge `origin/develop` (no force push) and see `check:all` green.
2. **Prepare**, as the `VAL` tasks:
   - one test per feature criterion `[F01.ACn]`;
   - the feature's lines in its epic's checklist (`playground/checklists/`, `?checklist=E15`), drafted by `target/release/val-prepare . <feature>` from its criteria (EN-056): at most 12 per feature, one per thing to see, one per option kept and per decision to confirm, each what to try and what to see after a check box, the panel beside the drawing, no Previous nor Next; when no drawing changes, still one line saying what to check; each tried in a browser first, and tested in happy-dom (every option named, none removed);
   - the demo page in `docs/guide/` (`/review`), whose **How to try it** names the checklist's address. No new guided steps: those written up to E15 stay in `?guide=epic`.
3. Tell the Product Owner how to launch it themselves, rather than starting servers for them: `npm run dev` then `http://localhost:5173/?checklist=<epic>`, and `npm run docs:dev` for the demo page.
4. They try the lines in any order, tick each one seen, and say what they see; compare it with what each line meant to show (`/review`), record the result and each remark in neutral words in the `VAL` story and the demo page, and route the remarks (stories, improvement log, memory).
5. Last commit: the `VAL` and the feature `done` (feature file, epic table, features index), `Closes:` both, pushed.
6. **Clean up, then ask** (2026-10-11): before the feature's pull request nothing is stale — every status `done`, the **State** table of `AGENTS.md`, the demo page and the journal up to date, no stash nor stray file. Then tell the Product Owner all is clean and checked, and wait for their yes.
7. Finish the `VAL` into the feature branch (`npm run story:finish`), then open **one** pull request from the feature branch into `develop`, its body the stories with their pull requests, the evidence and the validation's result, labeled `autonomous` once the Product Owner said the validation is done: GitHub merges it with a merge commit. A **tooling feature** is validated by the agent, which labels it without waiting; the Product Owner may try the lines that concern them. Once merged: delete the feature branch here and on GitHub, its bubble and its name staying in the merge commit, then `target/release/branches-clean`, until `git branch -a` shows only `main` and `develop`, and a journal entry. When the scope goes on, the run starts the next feature.

## Review (at each return of the Product Owner)

A review is held each time the Product Owner comes back, once or twice a day, no longer at an epic's end (ADR-0086).

1. **A feature under way is finished first**: tell the Product Owner the run finishes it, then holds the review.
2. **A `REV` story**, its branch from `develop` (`target/release/worktree-new REV-<nnn>`, no feature branch), holds the review of the features finished since the last merge into `main`, the naming pass over the code changed since then, the tool incident cards fixed, the improvements of the method decided with the Product Owner, and the docs brought up to date. It is finished at the end of its session with the Product Owner, never before, then enters `develop` by a merge commit (`npm run story:finish -- develop …`, labelled `autonomous` and awaited as any story).
3. **Then `develop` → `main`**: draft the features' text with `target/release/release notes` on an up-to-date `develop`, rewrite each `[draft]` line into a file of the scratch folder, the evidence and the attribution after a line `---`, run `target/release/release check <file>` until it passes, then open the pull request with that file for body (`gh pr create --base main --head develop --body-file <file>`). Its CI is the only one: read its checks once done (`gh pr checks <n>`), a failure at once (`gh run view <run> --log-failed`). The Product Owner merges it by a merge commit, the review's mark on `main`; the push to `main` deploys the site and makes no release. A journal entry.
4. **The release is a button** ([releases](../../../docs/tooling/releases.md)): the Product Owner runs `release.yml` on `main` from GitHub's Actions tab whenever they choose; `release publish` computes the version from the tags, its notes the bodies of the pull requests into `main` merged since the last tag; nothing is committed and no number is typed. A body that breaks a rule stops it, naming the pull request: fix the body on GitHub, and the Product Owner presses again.
