---
name: run
description: Autonomous run — chain the stories of an authorized scope on its feature branch, each with every check, merged into the feature branch once checked locally, until a point that needs the Product Owner. Use only when the Product Owner explicitly starts a run, naming a feature or a scope to chain (in French they say to launch or chain it).
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run (ADR-0072)

**Silent run** (Product Owner, 2026-10-12): the chat belongs to the Product Owner, so the run writes nothing between tool calls. The commits and the status line (`.git/metrics/step`, updated at each step) show the progress; workers run as background agents; one summary at the stop, or a question when one blocks. Never write "waiting for an agent" or "message from the auditor".

Scope: $ARGUMENTS. Never without the Product Owner's explicit start in this conversation. Without argument, the scope is the feature `in-progress` (from its next unfinished story up to its `VAL`); if none is in progress, propose the next feature in order — a new feature starts with its refinement interview, never directly with code.

**Order guard** (Product Owner, 2026-10-10): a run only takes the **next feature in the Product Owner's order**, never another one, so that the backlog's order stays the one place where the Product Owner sets priorities. The next feature is the one `in-progress`, else the first feature not `done` in the **Features** table of the first epic not `done` in `docs/backlog/epics/README.md` (table order, not file order). A scope naming another feature, or a story of another feature, is refused before anything else: stop, say which feature is next and why, and do nothing. To work on another feature, the Product Owner first changes the order in the tables (a backlog commit on `develop`), then starts the run again.

## Refinement of a new feature: two rounds (Product Owner, 2026-10-14)

1. **First round**: the first questions (research done before, `functional-analyst` agent), each with options and numeric examples, then one open question asking for every remark, idea and constraint at length.
2. **Second round**: from all they said, **every** remaining question at once, again ending with room for remarks. After it the Product Owner may leave: the run goes on without asking again, except for a business rule nobody could foresee.

## Before the first story

- **First feature of an epic**: `npm run epic:check -- <epic> <first day>` before anything else, the code scanning alerts included; what it finds is fixed or listed for the Product Owner.

0. **`/run` is the go-ahead** (ADR-0072): set the feature `in-progress` and its stories `ready` yourself (a `docs(backlog)` commit on `develop`); never ask the Product Owner to do it. Ask only business questions.
1. Record the authorization in `docs/process/journal.md` (scope, date, the Product Owner's request in neutral words) — on `develop`.
2. Feature branch: `feature/f<nn>-<topic>` exists on `origin`, or create it there from `develop` without switching the main checkout: `git fetch origin`, then `git push origin origin/develop:refs/heads/feature/f<nn>-<topic>`.
3. `/triage` if issues are open: present them at the next stop, do not act on them.
4. List the scope's stories in delivery order (feature plan); stop now if the next one is `VAL`, `REV` or `RET`, or if a business question of `docs/domain/README.md` blocks it.

## Main thread kept free (ADR-0072)

The main agent dispatches and stays free for the Product Owner, who may write at any time:

- **One worker per story**, an `Agent` call in the background, in a worktree made by `npm run worktree:new -- <ID>` (EN-052), of the agent type its task kind uses (EN-065, `docs/tooling/agent.md`, Run's workers):

  | Task | Agent type |
  | --- | --- |
  | a `US`, `EN`, `NAM` or `CHK` story, the fixes of an `AUD`, the preparation of a `VAL` | `story-worker` |
  | a research spike (`SP`) | `researcher` |
  | a review, as the audit and spike skills start it | `auditor`, `verifier`, `doc-editor`, `math-expert`, `functional-analyst` |
  | a journal entry, log rows, backlog statuses, a rebase or a merge the main thread hands off | `clerk` |
  | a task none of them fits | `general-purpose` |

  Its prompt is short: the story id, the feature branch, the worktree, that every Bash call starts with `cd "<worktree>" || exit 9`. A typed worker starts with the brief and the rules' digest preloaded (`worker-brief`, `project-rules`): its prompt does not give the brief's path. A `general-purpose` worker is also given the brief's path **inside the worktree**, `<worktree>/.claude/skills/worker-brief/SKILL.md` ([worker brief](../worker-brief/SKILL.md)). A rule a typed worker missed goes into its brief or `project-rules`, never into every agent's.

- **The main checkout stays on `develop`** during a run: a rule, skill or brief read there is `develop`'s, older than the feature branch's. Read them in a worktree of the feature branch: an older brief gives older rules.
- **A background Git command of the main thread** starts with `cd <worktree> || exit 9` and checks the branch name (`git branch --show-current`) before it pushes, finishes or removes anything.
- **A rebase conflict** is resolved in the working file, never by `checkout --ours` or `--theirs`, then diffed against the story's version before `rebase --continue`: a whole side taken drops its non-conflicting lines.
- **At most three subagents at once** (the rule, ADR-0072), **one heavy command at a time** through the lock: the hook `limit-agents` of EN-047 refuses a fourth subagent, started or resumed (EN-063), and the heavy commands take the lock themselves — never by hand with `mkdir`.
- **Separate files.** Parallel workers only on stories that touch separate files; the shared files (indexes, logs, feature file, checklist) are written by the main thread at merge, one merge at a time (ADR-0065).
- **Heartbeat.** One chat line when a worker starts and when it ends, with an estimate of its duration.
- **The main agent's part.** It answers the Product Owner, says where the work is (`npm run activity`, the status line), finishes each story (`npm run story:finish`) and writes the shared files.
- **Business questions** come back from the worker. Only the main agent asks them, in the questionnaire, with room for remarks.
- **Only dispatch**, and **each comment acknowledged at once**: now, or once a named running agent has finished. A comment about tools, rules or configuration goes to another agent, on `develop`, in its own worktree, within the limit.

## For each story

1. **Pick up `develop`** (parallel sessions, `docs/tooling/agent.md`): `git fetch origin` and `git log --oneline feature/f<nn>-<topic>..origin/develop -- AGENTS.md CLAUDE.md .claude docs/domain docs/adr docs/backlog docs/conventions`. If it lists commits, the Product Owner decided something in another session: merge `origin/develop` into the feature branch in a worktree of that branch, never in the main checkout, and push it (no force push on a feature branch, even with lease, RET-003), read those commits' diffs, and apply them from this story on — a changed skill or rule replaces what was loaded at the start of the run; a change that touches the run's stories or scope is a stop.
2. Make the story's worktree and branch from the feature branch on GitHub: `git fetch origin`, then `npm run worktree:new -- <ID>` (`.claude/worktrees/<id>` on `<type>/<id>-<topic>`, EN-052).
3. The worker follows `/story` steps 2 and 4–6 (read, `ready`, tests first, one commit per task, docs), then step 6 below, pushes its branch and **stops before the `done` commit**, which writes the shared files (the stories' index, the feature's table) that only the main thread writes, at merge (ADR-0065, decision 4, enforced by `refuse-shared-files`, EN-050). The main thread makes that commit in the story's worktree — `npm run backlog:status -- <ID> done`, then `npm run git:commit -- -F <message file> docs/backlog`, footer `Closes: <ID>`, and the `Rule-docs:` line within 100 characters when the worker's report names one (`/story` step 7) — then runs step 7 below.
4. **User story** (`US`): write its **Product Owner test card** in the story file, section `## Product Owner test`: steps in the playground, expected result of each, terms explained in plain words, what to look at closely.
5. **Checkpoint** (`CHK`): update docs and methods as the project grows; check consistency of code, docs, `AGENTS.md`, `CLAUDE.md` and rules; run the light evolvability check (`docs/process/README.md`); list the test cards written so far; log what deserves the retrospective.
6. **Checks by level** (ADR-0064, from E02): no independent review per story; the worker has written the edge cases and mutation-checked each new test. The feature's `AUD` audits in depth; the epic's full audit sits beside its `NAM`.
7. `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>` (ADR-0072), run in the story's worktree: it takes the merge lock — a second story waits, then rebases on the updated feature branch and pushes with lease (ADR-0065 decision 5, EN-051) — opens the pull request with the `autonomous` label (never for `VAL`, `REV`, `RET` before the Product Owner validated them), waits at most three minutes for GitHub's merge (`story-automerge.yml`; no CI, ADR-0072), brings the feature branch up to date where it is checked out without switching branch, moves the rows its story file holds for the logs into them and pushes them (EN-050), then removes the worktree and deletes the story's branch here and on GitHub. A rebase conflict: it stops, the branch as it was. Not merged: it says so; read `gh run list --workflow story-automerge.yml --limit 1`, then `gh run view <run> --log-failed`; a branch behind its base: `gh pr update-branch <n> --rebase`.
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
2. Message to the Product Owner in French (`docs/conventions/writing.md`): what was built in plain words, the **test cards to run**, findings, questions, what the next step needs from them, and, at a session's end, the session report (`npm run metrics:session -- --all --markdown`).

## Validation (`VAL`, with the Product Owner)

The Product Owner may test over several sessions: never wait in a loop for them, and start nothing else until they have finished every step.

1. `npm run worktree:new -- VAL-<nnn>` (the `VAL` branch from the feature branch, in its worktree); there, merge `origin/develop` (no force push), `check:all` green.
2. Prepare, as `VAL` tasks: one test per feature criterion `[F01.ACn]`; the feature's lines in its epic's **checklist** (`playground/checklists/`, `?checklist=E15`, US-051), drafted by `npm run val:prepare -- <feature>` from its criteria (EN-056) — at most 12 per feature (counted by EN-056's test), one line per thing to see, each tried in a browser before the validation, the panel beside the drawing; one line per option kept and one per decision to confirm, each what to try and what to see, a check box before it, no Previous nor Next; even when no drawing changes, one line saying what to check — tested in happy-dom (every option named, none removed); the demo page in `docs/guide/` (`/review`), whose **How to try it** names the checklist's address. No new guided steps: the steps written up to E15 stay in `?guide=epic` only (Product Owner, 2026-10-15).
3. Tell the Product Owner how to launch it themselves — `npm run dev` then `http://localhost:5173/?checklist=<epic>`, and `npm run docs:dev` for the demo page — rather than starting servers for them.
4. They try the checklist's lines in any order, tick each one seen (the ticks stay in their browser), and say what they see, often spoken; compare it with what each line meant to show (`/review`); record the result and each remark in neutral words in the `VAL` story and the demo page (`docs/conventions/writing.md`); route remarks (stories, improvement log, memory).
5. Last commit: `VAL` and the feature `done` (feature file, epic table, features index); `Closes:` both. Rebase on `develop`, push (the hook runs `check:all`).
6. **Clean-up, then ask** (Product Owner, 2026-10-11): before the feature pull request, leave nothing stale — every status `done` (story, feature, indexes, epic table), the `AGENTS.md` **State** line, demo page and journal up to date, feature branch deleted once its content is in the `VAL` branch, no stash nor stray file. Then tell the Product Owner that all is clean and checked, and wait for their yes.
7. Open **one** feature pull request from the `VAL` branch into `develop` (it contains the whole feature branch plus the validation; no CI), labeled `autonomous` once the Product Owner said the validation is done: GitHub merges it with a **merge commit** (ADR-0072). A **tooling feature** is validated by the agent: it opens the pull request and labels it `autonomous` without waiting for the Product Owner, who may try the lines that concern them (ADR-0072). Its body lists the stories with their pull requests, the evidence and the result of the validation. As soon as they merged it: delete the `VAL` branch, locally and on GitHub (API), then `npm run branches:clean` — it deletes every local branch merged or gone and reports what is left; `git branch -a` shows only `main` and `develop` (Product Owner, 2026-10-13).
8. **Release, only when the Product Owner asks** (ADR-0072), typically at the end of a working day: open the pull request `develop` → `main` (title `chore(release): <feature>`, body: the features and changes since the last release). Its CI is the only one (about 2 minutes): read its checks once it should be done (`gh pr checks <n>`); a failure is read at once (`gh run view <run> --log-failed`) and fixed on `develop`. Before it, the Product Owner merges release-please's release pull request into `develop` (ADR-0067); then they merge `develop` → `main` with a **merge commit**, and on `main` release-please only tags and publishes the release ([releases](../../../docs/tooling/releases.md)).
9. After the feature's merge: journal entry, branches cleaned; after a release: bring `develop` up to `main` (`git switch develop && git pull && git merge --ff-only origin/main && git push`); run `npm run branches:clean` again; check `git branch -a` shows only `main` and `develop`; journal entry. The next `/run` starts the next feature.
