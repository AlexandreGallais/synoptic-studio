---
name: run
description: Autonomous run — chain the stories of an authorized scope on its feature branch, each with every check, merged into the feature branch once checked locally, until the scope the Product Owner set ends; a question parks its story and the run goes on. Use only when the Product Owner explicitly starts a run, naming a feature or a scope to chain (in French they say to launch or chain it).
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run (ADR-0072)

A run delivers the scope the Product Owner set while they are away, and keeps the main thread free for them when they are back: workers do the stories, the main thread dispatches, merges and answers.

Scope: $ARGUMENTS.

## The scope

- **Only on the Product Owner's explicit start** in this conversation. They set the scope at the start, as a time (until an hour) or an end (a story, a feature, an epic), so that they decide how much is delivered; the run sets no bound of its own (2026-10-17). Without argument, the scope is the feature `in-progress`, from its next unfinished story up to its `VAL`; with none in progress, propose the next feature: a new feature starts with its refinement, never with code.
- **The order guard**: the backlog's order is the one place where the Product Owner sets priorities (2026-10-10), so a run takes only the next feature: the one `in-progress`, else the first feature not `done` in the **Features** table of the first epic not `done` in `docs/backlog/epics/README.md` (table order, not file order). A scope naming another feature, or one of its stories, is refused before anything else: say which feature is next and why, and do nothing. To change it, the Product Owner reorders the tables on `develop` and starts the run again.
- **A refinement takes two rounds** (2026-10-14), so that the Product Owner can leave after the second: first the questions the research prepared (the `functional-analyst`), each with options and numeric examples, then one open question for every remark, idea and constraint; second, every remaining question at once, again ending with room for remarks. After it the run asks nothing more; a business rule nobody foresaw parks its story.

## Before the first story

1. The first feature of an epic: `target/release/epic-check <epic> <first day>`, the code scanning alerts included; what it finds is fixed or listed for the Product Owner.
2. **`/run` is the go-ahead**: set the feature `in-progress` and its stories `ready` yourself, a `docs(backlog)` commit on `develop`; never ask the Product Owner to, and ask them only business questions. Then `target/release/run-totals start` (EN-092), which counts the run's duration and tokens from now; a run continued in a new session runs it again, which adds the session and keeps the start.
3. Record the authorization in `docs/process/journal.md` on `develop`: the scope set, the date, the request in neutral words.
4. The feature branch `feature/f<nn>-<topic>` exists on `origin`, or is created there from `develop` without switching the main checkout: `git fetch origin`, then `git push origin origin/develop:refs/heads/feature/f<nn>-<topic>`.
5. `/triage` if issues are open: present them at the next stop, act on none.
6. List the scope's stories in delivery order; stop now if the next is `VAL`, `REV` or `RET`. A story a business question of `docs/domain/README.md` blocks is parked, and the run starts with the next one independent of it.

## The main thread

The chat belongs to the Product Owner (2026-10-12), who may write at any time; so the main thread only dispatches, and the work goes to subagents.

- **Silent between tool calls**: the commits and the status line (`.git/metrics/step`, updated at each step) show the progress. The chat gets one line when a worker starts and when it ends, with an estimate of its duration; one summary at the stop; a question when one parks a story. Never "waiting for an agent" nor "message from the auditor".
- **Each comment of the Product Owner acknowledged at once**, saying when it is handled: now, or once a named running agent has finished. A comment about tools, rules or configuration goes to another agent, on `develop`, in its own worktree, within the limit.
- **One worker per story**, an `Agent` call in the background, in a worktree made by `target/release/worktree-new <ID>`, of the type its task kind uses (EN-065):

  | Task | Agent type |
  | --- | --- |
  | a `US`, `EN`, `NAM` or `CHK` story, the fixes of an `AUD`, the preparation of a `VAL` | `story-worker` |
  | a research spike (`SP`) | `researcher` |
  | a review, as the audit and spike skills start it | `auditor`, `verifier`, `doc-editor`, `math-expert`, `functional-analyst` |
  | a journal entry, log rows, backlog statuses, a rebase or a merge handed off | `clerk` |
  | a task none of them fits | `general-purpose` |

  Its prompt is short: the story id, the feature branch, the worktree, and that every Bash call starts with `cd "<worktree>" || exit 9`. A typed worker has its brief preloaded; a `general-purpose` worker is also given the brief's path **inside the worktree**, `<worktree>/.claude/skills/worker-brief/SKILL.md`. A rule a typed worker missed goes into the [worker brief](../worker-brief/SKILL.md) or `project-rules`, never into every agent's.

- **The main checkout stays on `develop`**, so a rule, skill or brief read there may be older than the feature branch's: read them in a worktree of the feature branch.
- **A background Git command** of the main thread starts with `cd <worktree> || exit 9` and checks `git branch --show-current` before it pushes, finishes or removes anything.
- **A rebase conflict** is resolved in the working file, never by `checkout --ours` or `--theirs`, then diffed against the story's version before `rebase --continue`: a whole side taken drops its other lines.
- **Parallel work** within the limits of `AGENTS.md` (Work) and `CLAUDE.md`, and only on stories that touch separate files; the shared files (indexes, logs, feature file, checklist) are written by the main thread at merge, one merge at a time (ADR-0065).
- **The main thread's own part**: answer the Product Owner, say where the work is (`target/release/activity`, the status line), finish each story (`npm run story:finish`), write the shared files, and ask the business questions workers bring back, in the questionnaire, with room for remarks.

## For each story

1. **Run `target/release/run-totals should-stop`** in a worktree of the feature branch ([Stop when](#stop-when)).
2. **Pick up `develop`**, since the Product Owner may decide something in another session: `git fetch origin`, then `git log --oneline feature/f<nn>-<topic>..origin/develop -- AGENTS.md CLAUDE.md .claude docs/domain docs/adr docs/backlog docs/conventions`. Commits listed: merge `origin/develop` into the feature branch in a worktree of that branch, never the main checkout, push it without force, read their diffs and apply them from this story on; a changed skill or rule replaces what was loaded at the start, and a change to the run's stories or scope is a stop.
3. **Its worktree**: `git fetch origin`, then `target/release/worktree-new <ID>` (`.claude/worktrees/<id>` on `<type>/<id>-<topic>`, from the feature branch on GitHub).
4. **Its worker** works the tasks as its brief says, pushes, and **stops before the `done` commit**, which writes the shared files only the main thread writes (ADR-0065, enforced by `refuse-shared-files`, EN-050). The main thread makes it in the story's worktree: `target/release/backlog-status <ID> done`, then `target/release/git-sequences commit -F <message file> docs/backlog`, footer `Closes: <ID>`, and the `Rule-docs:` line when the report names one (`/story` step 7).
5. A **user story** (`US`) gets its Product Owner test card in its file, `## Product Owner test`: the steps in the playground, the expected result of each, terms in plain words, what to look at closely.
6. A **checkpoint** (`CHK`) brings docs and methods up to date as the project grows, checks that code, docs, `AGENTS.md`, `CLAUDE.md` and rules agree, runs the light evolvability check (`docs/process/README.md`), lists the test cards so far and logs what deserves the retrospective.
7. **Checks by level** (ADR-0064): no review per story; the worker wrote the edge cases and mutation-checked each new test; the feature's `AUD` audits in depth, the epic's full audit sits beside its `NAM`.
8. **Finish it**: `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>` in its worktree (the `story-finish` binary of `tools/repo-tools/`, EN-103). It takes the merge lock (a second story waits, then rebases and pushes with lease, EN-051; the status rows the first story's done commit wrote are left to the base and the second's written again after the rebase, EN-103), opens the pull request with the `autonomous` label (never for `VAL`, `REV`, `RET` before the Product Owner validated them), waits at most three minutes for GitHub's merge (`story-automerge.yml`, no CI), updates the feature branch where it is checked out, moves the story file's rows into the logs (EN-050), and removes the worktree and the branches.
   - A rebase conflict outside the status rows: it stops, the branch as it was. Not merged: `gh run list --workflow story-automerge.yml --limit 1`, then `gh run view <run> --log-failed`; a branch behind its base: `gh pr update-branch <n> --rebase`.
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

- **its scope ends**: the time set has passed, or the end named is reached; the next story is `VAL`, `REV` or `RET`, which needs the Product Owner; or the feature's `VAL` is done and it was the epic's last feature (the next run's scope is then the epic's `NAM` naming pass, ADR-0049, a story like the others, its branch from `develop`);
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
5. Last commit: the `VAL` and the feature `done` (feature file, epic table, features index), `Closes:` both; rebase on `develop`, push.
6. **Clean up, then ask** (2026-10-11): before the feature pull request nothing is stale — every status `done`, the **State** table of `AGENTS.md`, the demo page and the journal up to date, the feature branch deleted once its content is in the `VAL` branch, no stash nor stray file. Then tell the Product Owner all is clean and checked, and wait for their yes.
7. Open **one** feature pull request from the `VAL` branch into `develop`, its body the stories with their pull requests, the evidence and the validation's result, labeled `autonomous` once the Product Owner said the validation is done: GitHub merges it with a merge commit. A **tooling feature** is validated by the agent, which labels it without waiting; the Product Owner may try the lines that concern them. Once merged: delete the `VAL` branch here and on GitHub, then `target/release/branches-clean`, until `git branch -a` shows only `main` and `develop`.
8. **Release, only when the Product Owner asks**, typically at a working day's end (ADR-0079, [releases](../../../docs/tooling/releases.md)): on an up-to-date `develop`, `target/release/release prepare` makes `release/x.y.z` and the entry's draft; rewrite each `[draft]` line from the commits it lists, run `target/release/release check` until it passes, commit `chore: release x.y.z`, push, and open the pull request into `main`, its title the commit's, its body `target/release/release notes`. Its CI is the only one: read its checks once done (`gh pr checks <n>`), a failure at once (`gh run view <run> --log-failed`), fixed on the release branch. The Product Owner merges it with a merge commit; `release.yml` tags and publishes.
9. After the feature's merge: a journal entry. After a release: `develop` brought up to `main` (`git switch develop && git pull && git merge --ff-only origin/main && git push`, a merge if `develop` moved), the release branch deleted, `target/release/branches-clean`, `git branch -a` showing only `main` and `develop`, a journal entry. The next `/run` starts the next feature.
