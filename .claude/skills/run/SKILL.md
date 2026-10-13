---
name: run
description: Autonomous run — chain the stories of an authorized scope on its feature branch, each with every check and an independent review, merged into the feature branch once checked locally, until a point that needs the Product Owner. Use only when the Product Owner explicitly starts a run, naming a feature or a scope to chain (in French they say to launch or chain it).
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run (ADR-0028)

**Terminal** (Product Owner, 2026-10-12): say nothing between tool calls. The commits and the status line (`.git/metrics/step`, updated at each step) show the progress; reviews run as background agents; one summary at the stop, or a question when one blocks. Never write "waiting for an agent" or "message from the auditor".

Scope: $ARGUMENTS. Never without the Product Owner's explicit start in this conversation. Without argument, the scope is the feature `in-progress` (from its next unfinished story up to its `VAL`); if none is in progress, propose the next feature in order — a new feature starts with its refinement interview, never directly with code.

**Order guard** (Product Owner, 2026-10-10): a run only takes the **next feature in the Product Owner's order**, never another one. The next feature is the one `in-progress`, else the first feature not `done` in the **Features** table of the first epic not `done` in `docs/backlog/epics/README.md` (table order, not file order). A scope naming another feature, or a story of another feature, is refused before anything else: stop, say which feature is next and why, and do nothing. To work on another feature, the Product Owner first changes the order in the tables (a backlog commit on `develop`), then starts the run again.

## Before the first story

1. Record the authorization in `docs/process/journal.md` (scope, date, the Product Owner's request in neutral words) — on `develop`.
2. Feature branch: `feature/f<nn>-<topic>` exists on `origin`, or create it from up-to-date `develop` (`git switch develop && git pull && git switch -c feature/f<nn>-<topic> && git push -u origin HEAD`).
3. `/triage` if issues are open: present them at the next stop, do not act on them.
4. List the scope's stories in delivery order (feature plan); stop now if the next one is `VAL`, `REV` or `RET`, or if a business question of `docs/domain/README.md` blocks it.

## For each story

1. **Pick up `develop`** (parallel sessions, `docs/tooling/agent.md`): `git fetch origin` and `git log --oneline feature/f<nn>-<topic>..origin/develop -- AGENTS.md CLAUDE.md .claude docs/domain docs/adr docs/backlog docs/conventions`. If it lists commits, the Product Owner decided something in another session: rebase the feature branch on `origin/develop` (`git switch feature/f<nn>-<topic> && git pull && git rebase origin/develop && git push --force-with-lease`), read those commits' diffs, and apply them from this story on — a changed skill or rule replaces what was loaded at the start of the run; a change that touches the run's stories or scope is a stop.
2. Branch from the up-to-date feature branch: `git switch feature/f<nn>-<topic> && git pull && git switch -c <type>/<id>-<topic>`.
3. Follow `/story` steps 2 and 4–6 (read, `ready`, tests first, one commit per task, docs), then the review of step 6 below, then `/story` steps 7–8 (`done` in the last commit, push through the pre-push hook).
4. **User story** (`US`): write its **Product Owner test card** in the story file, section `## Product Owner test`: steps in the playground, expected result of each, terms explained in plain words, what to look at closely.
5. **Checkpoint** (`CHK`): update docs and methods as the project grows; check consistency of code, docs, `AGENTS.md`, `CLAUDE.md` and rules; run the light evolvability check (`docs/process/README.md`); list the test cards written so far; log what deserves the retrospective.
6. **Independent review**, before the `done` commit (so that it stays last), in the background and in parallel: the `auditor` on the story's diff (`git diff feature/f<nn>-<topic>...HEAD`), its criteria, derivations and sources; the `verifier` on its tests; the `math-expert` when it adds a `math` or `geometry` function. Fix correctness findings; record the rest in the pull request body.
7. `gh pr create --base feature/f<nn>-<topic> --label autonomous …` (never the label for `VAL`, `REV`, `RET`).
8. No CI runs on the pull request (ADR-0038): the push already passed `check:all` in the pre-push hook. GitHub merges it within a minute of the label (`story-automerge.yml`); read `gh pr view <n> --json state` once or twice, never in a long loop. Branch behind its base: `gh pr update-branch <n> --rebase`. Not merged after two minutes: read the workflow's log (`gh run list --workflow story-automerge.yml --limit 1`, then `gh run view <run> --log-failed`).
   - "Dependencies not up-to-date" at the push: a version passed its 7-day cooldown meanwhile — read its release notes (`docs/tooling/versions-and-security.md`), update it on the story branch (`build(deps)` commit) and on `develop` too.
9. `git switch feature/f<nn>-<topic> && git pull --prune && git branch -D <branch>`; delete the merged story branch on GitHub too (auto-merges do not delete it): `gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>` — not `git push --delete`, which runs the pre-push hook (`check:all`) for nothing; next story.

## Stop when

- the next story is `VAL`, `REV` or `RET`;
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
7. Open **one** feature pull request from the `VAL` branch into `develop` (it contains the whole feature branch plus the validation; no label; no CI); its body lists the stories with their pull requests, the evidence and the result of the validation. The Product Owner merges it with a **merge commit** (ADR-0038). As soon as they merged it: delete the `VAL` branch, locally and on GitHub (API), then `npm run branches:clean` — it deletes every local branch merged or gone and reports what is left; `git branch -a` shows only `main` and `develop` (Product Owner, 2026-10-13).
8. **Release**: open the pull request `develop` → `main` (title `chore(release): <feature>`, body: the features and changes since the last release). Its CI is the only one (about 2 minutes): read its checks once it should be done (`gh pr checks <n>`); a failure is read at once (`gh run view <run> --log-failed`) and fixed on `develop`. The Product Owner merges it with a **merge commit**; release-please's pull request then merges itself and publishes the release.
9. After the release: empty `playground/guided-steps/current-guided-steps.ts`; bring `develop` up to `main` (`git switch develop && git pull && git merge --ff-only origin/main && git push`); run `npm run branches:clean` again; check `git branch -a` shows only `main` and `develop`; journal entry; do not start the next feature until the Product Owner says so.
