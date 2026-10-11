---
name: run
description: Autonomous run — chain the stories of an authorized scope on its feature branch, each with every check and an independent review, auto-merged into the feature branch when green, until a point that needs the Product Owner. Use only when the Product Owner explicitly starts a run ("lance F01", "enchaîne la feature").
argument-hint: "[scope, e.g. F01 or EN-005..US-003]"
---

# Autonomous run (ADR-0028)

Scope: $ARGUMENTS. Never without the Product Owner's explicit start in this conversation. Without argument, the scope is the feature `in-progress` (from its next unfinished story up to its `VAL`); if none is in progress, propose the next feature in order — a new feature starts with its refinement interview, never directly with code.

**Order guard** (Product Owner, 2026-10-10): a run only takes the **next feature in the Product Owner's order**, never another one. The next feature is the one `in-progress`, else the first feature not `done` in the **Features** table of the first epic not `done` in `docs/backlog/epics/README.md` (table order, not file order). A scope naming another feature, or a story of another feature, is refused before anything else: stop, say which feature is next and why, and do nothing. To work on another feature, the Product Owner first changes the order in the tables (a backlog commit on `main`), then starts the run again.

## Before the first story

1. Record the authorization in `docs/process/journal.md` (scope, date, Product Owner's words quoted) — on `main`.
2. Feature branch: `feature/f<nn>-<topic>` exists on `origin`, or create it from up-to-date `main` (`git switch -c feature/f<nn>-<topic> main && git push -u origin HEAD`).
3. `/triage` if issues are open: present them at the next stop, do not act on them.
4. List the scope's stories in delivery order (feature plan); stop now if the next one is `VAL`, `REV` or `RET`, or if a business question of `docs/domain/README.md` blocks it.

## For each story

1. **Pick up `main`** (parallel sessions, `docs/tooling/agent.md`): `git fetch origin` and `git log --oneline feature/f<nn>-<topic>..origin/main -- CLAUDE.md .claude docs/domain docs/adr docs/backlog docs/conventions`. If it lists commits, the Product Owner decided something in another session: rebase the feature branch on `origin/main` (`git switch feature/f<nn>-<topic> && git pull && git rebase origin/main && git push --force-with-lease`), read those commits' diffs, and apply them from this story on — a changed skill or rule replaces what was loaded at the start of the run; a change that touches the run's stories or scope is a stop.
2. Branch from the up-to-date feature branch: `git switch feature/f<nn>-<topic> && git pull && git switch -c <type>/<id>-<topic>`.
3. Follow `/story` steps 2 and 4–6 (read, `ready`, tests first, one commit per task, docs), then the review of step 6 below, then `/story` steps 7–8 (`done` in the last commit, `check:all`), with the feature branch as base everywhere `/story` says `main`.
4. **User story** (`US`): write its **Product Owner test card** in the story file, section `## Product Owner test`: steps in the playground, expected result of each, terms explained in plain words, what to look at closely.
5. **Checkpoint** (`CHK`): update docs and methods as the project grows; check consistency of code, docs, `CLAUDE.md` and rules; run the light evolvability check (`docs/process/README.md`); list the test cards written so far; log what deserves the retrospective.
6. **Independent review**, before the `done` commit (so that it stays last): delegate to the `auditor` subagent the story's diff (`git diff feature/f<nn>-<topic>...HEAD`), its criteria, derivations and sources; fix correctness findings; record the rest in the pull request body.
7. `gh pr create --base feature/f<nn>-<topic> --label autonomous …` (never the label for `VAL`, `REV`, `RET`).
8. Wait for the merge by **reading the statuses**, never by waiting blindly (Product Owner, 2026-10-09): every 15 s, `gh pr view <n> --json state` (stop at `MERGED`) and `gh pr checks <n>` (stop at the first `fail`); give up after 12 minutes — the CI takes about 3 to 4. Branch behind its base: `gh pr update-branch <n> --rebase`.
   - A failing check: read it at once (`gh run view <run> --log-failed`), fix on the story branch, push; after three failed attempts, stop.
   - "Dependencies not up-to-date": a patch was published meanwhile — update it on the story branch (`npm install -D --save-exact <pkg>@<version>`, `build(deps)` commit) and on `main` too.
9. `git switch feature/f<nn>-<topic> && git pull --prune && git branch -D <branch>`; delete the merged story branch on GitHub too (auto-merges do not delete it): `gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>` — not `git push --delete`, which runs the pre-push hook (`check:all`) for nothing; next story.

## Stop when

- the next story is `VAL`, `REV` or `RET`;
- a business question, a research request, a guardrail of `CLAUDE.md`, or an auditor finding that changes a requirement;
- a check still red after three attempts;
- the scope is finished.

## At the stop

1. Journal entry (on `main`): stories merged with their pull requests, findings, stop reason.
2. Message to the Product Owner in French (`docs/conventions/writing.md`): what was built in plain words, the **test cards to run**, findings, questions, what the next step needs from them.

## Validation (`VAL`, with the Product Owner)

The Product Owner may test over several sessions: never wait in a loop for them, and start nothing else until they have finished every step.

1. Rebase the feature branch on `main` (`git rebase main`, `git push --force-with-lease`), `check:all` green; branch `feat/val-<nnn>-<topic>` from it.
2. Prepare, as `VAL` tasks: one test per feature criterion `[F01.ACn]` (criteria only the playground can show are tested in `playground/mount-playground.test.ts`); a **guided test** in the playground — the steps of the user stories' test cards, one at a time, what happens in plain words and what to look at, Previous / Next typing the values (`playground/guided-steps.ts`), tested end to end; the demo page in `docs/guide/` (`/review`).
3. Tell the Product Owner how to launch it themselves — `npm run dev` then `http://localhost:5173`, and `npm run docs:dev` for the demo page — rather than starting servers for them.
4. They run the guided test and give their account, often step by step and spoken; compare it with what each step meant to show (`/review`); record their account in the `VAL` story and the demo page, quoting them; route remarks (stories, improvement log, memory).
5. Last commit: `VAL` and the feature `done` (feature file, epic table, features index); `Closes:` both. Rebase on `main`, `check:all`, push.
6. **Clean-up, then ask** (Product Owner, 2026-10-11): before the feature pull request, leave nothing stale — every status `done` (story, feature, indexes, epic table), `CLAUDE.md` state line, demo page and journal up to date, feature branch deleted once its content is in the `VAL` branch, no stash nor stray file. Then tell the Product Owner that all is clean and checked, and wait for their yes.
7. Open **one** feature pull request from the `VAL` branch into `main` (it contains the whole feature branch plus the validation; no label); its body lists the stories with their pull requests, the evidence and the Product Owner's account. Wait for its checks by reading statuses. The Product Owner merges it; a release follows.
8. After the merge: delete the `VAL` and feature branches, locally and on GitHub (API); check `git branch -a` shows only `main`; journal entry; do not start the next feature until the Product Owner says so.
