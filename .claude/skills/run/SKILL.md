---
name: run
description: Autonomous run — chain the stories of an authorized scope on its feature branch, each with every check and an independent review, merged into the feature branch once checked locally, until a point that needs the Product Owner. Use only when the Product Owner explicitly starts a run, naming a feature or a scope to chain (in French they say to launch or chain it).
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run (ADR-0028)

**Terminal** (Product Owner, 2026-10-12): say nothing between tool calls. The commits and the status line (`.git/metrics/step`, updated at each step) show the progress; reviews run as background agents; one summary at the stop, or a question when one blocks. Never write "waiting for an agent" or "message from the auditor".

Scope: $ARGUMENTS. Never without the Product Owner's explicit start in this conversation. Without argument, the scope is the feature `in-progress` (from its next unfinished story up to its `VAL`); if none is in progress, propose the next feature in order — a new feature starts with its refinement interview, never directly with code.

**Order guard** (Product Owner, 2026-10-10): a run only takes the **next feature in the Product Owner's order**, never another one. The next feature is the one `in-progress`, else the first feature not `done` in the **Features** table of the first epic not `done` in `docs/backlog/epics/README.md` (table order, not file order). A scope naming another feature, or a story of another feature, is refused before anything else: stop, say which feature is next and why, and do nothing. To work on another feature, the Product Owner first changes the order in the tables (a backlog commit on `develop`), then starts the run again.

## Refinement of a new feature: two rounds (Product Owner, 2026-10-14)

1. **First round**: the first questions (research done before, `functional-analyst` agent), each with options and numeric examples, then one open question asking for every remark, idea and constraint at length.
2. **Second round**: from all they said, **every** remaining question at once, again ending with room for remarks. After it the Product Owner may leave: the run goes on without asking again, except for a business rule nobody could foresee.

## Before the first story

- **First feature of an epic**: `npm run epic:check -- <epic> <first day>` before anything else, the code scanning alerts included; what it finds is fixed or listed for the Product Owner.

0. **`/run` is the go-ahead** (ADR-0045): set the feature `in-progress` and its stories `ready` yourself (a `docs(backlog)` commit on `develop`); never ask the Product Owner to do it. Ask only business questions.
1. Record the authorization in `docs/process/journal.md` (scope, date, the Product Owner's request in neutral words) — on `develop`.
2. Feature branch: `feature/f<nn>-<topic>` exists on `origin`, or create it from up-to-date `develop` (`git switch develop && git pull && git switch -c feature/f<nn>-<topic> && git push -u origin HEAD`).
3. `/triage` if issues are open: present them at the next stop, do not act on them.
4. List the scope's stories in delivery order (feature plan); stop now if the next one is `VAL`, `REV` or `RET`, or if a business question of `docs/domain/README.md` blocks it.

## Main thread kept free (ADR-0051, trial during E15)

The main agent dispatches and stays free for the Product Owner, who may write at any time:

- **One worker per story.** It is an `Agent` call (`general-purpose`, `isolation: "worktree"`), in the background, from the up-to-date feature branch. Its prompt holds the story id, this skill's steps 2 to 4 below, `AGENTS.md`'s rules, and two limits: stop before the pull request, and stop with the question when a business rule is missing.
- **One worker at a time per feature branch.** A feature's stories build on each other. Work on other files may run beside it: the research of the next feature, documentation, reviews.
- **The main agent's part.** It answers the Product Owner while the worker works, with `npm run activity` and the status line to say where the work is. When the worker reports, it runs the reviews of step 6 in the background, sends the correctness findings back to a worker, then finishes the story (`npm run story:finish`).
- **Business questions** come back from the worker. Only the main agent asks them, in the questionnaire, with room for remarks.
- **Measured.** At the end of each run, the turn's tokens and the time the chat was free go into the journal, for E15's retrospective.
- **Only dispatch.** Every piece of work goes to a subagent: research, stories, tool or rule changes. The main thread does none itself, so the chat is always free.
- **Each comment acknowledged at once.** The main agent says the comment is taken into account and when: now, or once a named running agent has finished, if the comment concerns that agent's work.
- **Comments one after another.** The Product Owner may send several without waiting; each gets its own acknowledgement.
- **Tool and rule comments in parallel.** A comment about tools, rules, skills or configuration, unrelated to the running story, goes at once to another agent, on `develop`, in its own worktree.
- **No shared files.** The main agent knows each running agent's topic and files, so that two agents never edit the same files.

## For each story

1. **Pick up `develop`** (parallel sessions, `docs/tooling/agent.md`): `git fetch origin` and `git log --oneline feature/f<nn>-<topic>..origin/develop -- AGENTS.md CLAUDE.md .claude docs/domain docs/adr docs/backlog docs/conventions`. If it lists commits, the Product Owner decided something in another session: rebase the feature branch on `origin/develop` (`git switch feature/f<nn>-<topic> && git pull && git rebase origin/develop && git push --force-with-lease`), read those commits' diffs, and apply them from this story on — a changed skill or rule replaces what was loaded at the start of the run; a change that touches the run's stories or scope is a stop.
2. Branch from the up-to-date feature branch: `git switch feature/f<nn>-<topic> && git pull && git switch -c <type>/<id>-<topic>`.
3. Follow `/story` steps 2 and 4–6 (read, `ready`, tests first, one commit per task, docs), then the review of step 6 below, then `/story` steps 7–8 (`done` in the last commit, push through the pre-push hook).
4. **User story** (`US`): write its **Product Owner test card** in the story file, section `## Product Owner test`: steps in the playground, expected result of each, terms explained in plain words, what to look at closely.
5. **Checkpoint** (`CHK`): update docs and methods as the project grows; check consistency of code, docs, `AGENTS.md`, `CLAUDE.md` and rules; run the light evolvability check (`docs/process/README.md`); list the test cards written so far; log what deserves the retrospective.
6. **Independent review**, before the `done` commit (so that it stays last), in the background and in parallel: the `auditor` on the story's diff (`git diff feature/f<nn>-<topic>...HEAD`), its criteria, derivations and sources; the `verifier` on its tests; the `math-expert` when it adds a `math` or `geometry` function. Fix correctness findings; record the rest in the pull request body.
7. `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>` (ADR-0050): it opens the pull request with the `autonomous` label (never for `VAL`, `REV`, `RET` before the Product Owner validated them), waits at most three minutes for GitHub's merge (`story-automerge.yml`; no CI, ADR-0038), brings the feature branch up to date and deletes the story's branch here and on GitHub. Not merged: it says so; read `gh run list --workflow story-automerge.yml --limit 1`, then `gh run view <run> --log-failed`; a branch behind its base: `gh pr update-branch <n> --rebase`.
   - "Dependencies not up-to-date" at the push: a version passed its 7-day cooldown meanwhile — read its release notes (`docs/tooling/versions-and-security.md`), update it on the story branch (`build(deps)` commit) and on `develop` too.
8. Statuses: `npm run backlog:status -- <ID> <status>` changes the story's file, index and feature table at once; edits across files: `npm run replace` (`docs/tooling/agent-tools.md`).
9. Next story.

## Stop when

- the next story is `VAL`, `REV` or `RET`;
- the feature's `VAL` is done and it was the epic's last feature: the next run's scope is the epic's `NAM` naming pass (ADR-0049), a story run like the others, with its own branch from `develop`;
- a business question, a research request, a stop of `AGENTS.md`, or an auditor finding that changes a requirement;
- a check still red after three attempts;
- the scope is finished.

## At the stop

1. Journal entry (on `develop`): stories merged with their pull requests, findings, stop reason.
2. Message to the Product Owner in French (`docs/conventions/writing.md`): what was built in plain words, the **test cards to run**, findings, questions, what the next step needs from them, and the session report (`npm run metrics:session -- --markdown`).

## Validation (`VAL`, with the Product Owner)

The Product Owner may test over several sessions: never wait in a loop for them, and start nothing else until they have finished every step.

1. Rebase the feature branch on `develop` (`git rebase origin/develop`, `git push --force-with-lease`), `check:all` green; branch `feat/val-<nnn>-<topic>` from it.
2. Prepare, as `VAL` tasks: one test per feature criterion `[F01.ACn]`; a **guided test** — even when no drawing changes, one or two steps saying what to check (Product Owner, 2026-10-13) — of a few short steps in `playground/guided-steps/current-guided-steps.ts` — each one line to do and one to look at, Next typing the values, tested end to end (the local playground shows it while the feature is validated); the same steps appended to the epic's list (`playground/guided-steps/guided-steps.ts`, `?guide=epic`) for the review; the demo page in `docs/guide/` (`/review`).
3. Tell the Product Owner how to launch it themselves — `npm run dev` then `http://localhost:5173`, and `npm run docs:dev` for the demo page — rather than starting servers for them.
4. They run the guided test and say what they see, often step by step and spoken; compare it with what each step meant to show (`/review`); record the result and each remark in neutral words in the `VAL` story and the demo page (`docs/conventions/writing.md`); route remarks (stories, improvement log, memory).
5. Last commit: `VAL` and the feature `done` (feature file, epic table, features index); `Closes:` both. Rebase on `develop`, push (the hook runs `check:all`).
6. **Clean-up, then ask** (Product Owner, 2026-10-11): before the feature pull request, leave nothing stale — every status `done` (story, feature, indexes, epic table), the `AGENTS.md` **State** line, demo page and journal up to date, feature branch deleted once its content is in the `VAL` branch, no stash nor stray file. Then tell the Product Owner that all is clean and checked, and wait for their yes.
7. Open **one** feature pull request from the `VAL` branch into `develop` (it contains the whole feature branch plus the validation; no CI), labeled `autonomous` once the Product Owner said the validation is done: GitHub merges it with a **merge commit** (ADR-0045). Its body lists the stories with their pull requests, the evidence and the result of the validation. As soon as they merged it: delete the `VAL` branch, locally and on GitHub (API), then `npm run branches:clean` — it deletes every local branch merged or gone and reports what is left; `git branch -a` shows only `main` and `develop` (Product Owner, 2026-10-13).
8. **Release, only when the Product Owner asks** (ADR-0045), typically at the end of a working day: open the pull request `develop` → `main` (title `chore(release): <feature>`, body: the features and changes since the last release). Its CI is the only one (about 2 minutes): read its checks once it should be done (`gh pr checks <n>`); a failure is read at once (`gh run view <run> --log-failed`) and fixed on `develop`. The Product Owner merges it with a **merge commit**; release-please's pull request then merges itself and publishes the release.
9. After the feature's merge: empty `playground/guided-steps/current-guided-steps.ts` on `develop`, journal entry, branches cleaned; after a release: empty `playground/guided-steps/current-guided-steps.ts`; bring `develop` up to `main` (`git switch develop && git pull && git merge --ff-only origin/main && git push`); run `npm run branches:clean` again; check `git branch -a` shows only `main` and `develop`; journal entry. The next `/run` starts the next feature.
