---
id: EN-134
epic: E17
feature: F34
title: The auto-merge per base and the other workflows reviewed
status: ready
points: 2
---

# E17 · F34 · EN-134 — The auto-merge per base and the other workflows reviewed

`story-automerge.yml` runs on every pull request, its job choosing by the base branch in its `if:` and its script choosing again (`if [ "$BASE" = develop ]`); a pull request opened with its label starts it twice, `opened` and `labeled` (173 runs merged, 25 skipped, 2 failed among its last 200). `external-prs.yml` ran 200 times, all skipped: no outsider ever opened a pull request, the setting _Collaborators only_ keeping them out (`REF-GH-PR-ACCESS`). `links.yml` checks the outside links, which rot without a change of ours; CodeQL analyses the TypeScript and could analyse the Rust and the workflows (`REF-GH-CODEQL-TRIGGERS`). Research note 0026, §2.

## Acceptance criteria

- Given a pull request of the owner labelled `autonomous`, when it targets a feature branch, then the stories' auto-merge workflow, started by `pull_request` on `branches: ['feature/**']`, merges it by a merge commit; when it targets `develop`, the features' auto-merge workflow, on `branches: [develop]`, does; each checks its own branch pattern, without an `if:` on the base, `story-automerge.yml` leaving `[F28.AC1]`'s exceptions (EN-132), and `[F22.AC4]` of `tests/story-automerge.test.ts` reads both files.
- Given a pull request opened with its label, when its events arrive, then one run of the auto-merge merges it and no second run starts (the activity types chosen from the runs read, a draft made ready still merged).
- Given SP-024's questions answered by the Product Owner, when the story ends, then `external-prs.yml` is removed with ADR-0026's second barrier by a new ADR, or kept as decided; `codeql.yml` analyses the languages decided; `links.yml` stays weekly and by hand, blocking nothing; `docs/tooling/ci.md` gives each workflow's purpose.

## Tasks

- [x] T1 — `[F28.AC4]` and `[F22.AC4]` for the two auto-merge workflows, failing (1 h)
- [x] T2 — The auto-merge split by base, the Product Owner's decisions on the other workflows, the docs (2 h)

One task = one commit, referenced as `EN-134.T1` in the commit footer.

## Question for the Product Owner

Parked by the run on 2026-10-18 until answered (SP-024, research note 0026):

1. `external-prs.yml`: its last 200 runs were all skipped, no outside contributor ever came, and "Collaborators only" already keeps outsiders out. (a) Remove it, by a new ADR superseding ADR-0026's second barrier; (b) keep it.
2. CodeQL's languages, `Analyze` being a required check of `main` (1 min 09 s): (a) TypeScript only, as now; (b) with `actions`, the workflows; (c) with `actions` and `rust` (131,597 lines), the blocking check longer. And: does `Analyze` stay required?
3. `links.yml`: its one run (by hand, 2026-10-15) failed on 2 dead links out of 826. (a) Keep it weekly, blocking nothing; (b) remove it.
4. F30's validation: the `develop` CI can run only once its file is on `main`. (a) VAL-031 checks the tests and docs, the review after the next merge into `main` reads its first runs; (b) another way.

## Answers of the Product Owner (2026-10-18)

1. `external-prs.yml` removed, by a new ADR superseding ADR-0026's second barrier; the repository's "Collaborators only" setting stays the protection.
2. CodeQL analyses TypeScript and `actions` (the workflows), and `Analyze` stays a required check of `main`.
3. `links.yml` kept, weekly and by hand, blocking nothing; its two dead links fixed.
4. F30 is validated on its tests and docs; the review after the next merge into `main` reads the `develop` CI's first runs.

## For the main thread

- `main`'s required checks need no change: `Analyze` stays the name of `codeql.yml`'s one job, which analyses `javascript-typescript, actions` in turn; a `language` matrix, which GitHub's page recommends (`REF-GH-CODEQL-TRIGGERS`), would name its jobs `Analyze (javascript-typescript)` and `Analyze (actions)`, leaving the required `Analyze` unreported. The two auto-merges' checks (`Merge`, of `Story auto-merge` and `Feature auto-merge`) and `Link check` are required by no ruleset. `f24_ac1_…` (`tools/repo-tools/tests/check_all.rs`) still ties the ruleset's list of `ci.md` to the jobs of `ci.yml` and `codeql.yml`.
- On GitHub, once on `main`: the workflow `External pull requests` leaves the Actions tab with its file; `Feature auto-merge` appears with its first run into `develop`. The first CodeQL run with `actions` may raise alerts on the workflows, shown in the code scanning results, not failing `Analyze`.
- The task T1 named `[F28.AC4]`, which F28 does not have (three criteria, F28 done): the new tests are titled `[F34.AC1]`, the criterion of F34 they prove, beside `[F22.AC4]`, `[F28.AC1]` and `[F30.AC1]` where those hold too.
- The activity types: `opened` dropped, `labeled`, `reopened` and `ready_for_review` kept. #425 (2026-10-19) was opened at 05:04:56 and labelled at 05:04:57 by `gh pr create --label`, its timeline holding the `labeled` event, and started two runs at 05:04:59, 39202694744 merged and 39202694181 skipped; so a label given at the opening sends `labeled`, which alone now starts the run.
- The two dead links of the run of 2026-10-15: the Kanban guide (HTTP 502) answered 200 on 2026-10-19, nothing changed; Adobe's live shapes tutorial (HTTP 403) answered 403 to curl, with a browser's user agent too, and to the library (2026-10-19), while `www.adobe.com/learn/illustrator` answered 200 to curl: whether the page is gone or refuses clients could not be told. It is excluded by its exact address in `.lycheeignore`, with that reason; Adobe's help page on live shapes answers but lacks the two sentences `REF-ADOBE-LIVE-SHAPES` cites. If the Product Owner prefers, a research request can look for another source of those sentences and the exclusion go.
- `story-finish` named `story-automerge.yml` in two messages, wrong for a `REV` or an epic's `SP` into `develop` once split: `auto_merge_of` (`tools/repo-tools/src/story_finish.rs`) names the workflow of the base, `f34_ac1_names_the_auto_merge_each_base_starts` and `f23_ac2_leaves_an_epics_spike_…` holding it.
- Rule-docs: none affected, a tooling story that changes no rule of `docs/domain/`.
