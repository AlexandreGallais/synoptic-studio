---
id: VAL-021
epic: E17
feature: F20
title: Validate F20
status: ready
points: 1
---

# E17 · F20 · VAL-021 — Validate F20

Show the Product Owner F20 on one demo page, with the tokens before and after.

## Acceptance criteria

- Given the demo page, when the Product Owner reads it, then each criterion of F20 is shown with its evidence.

## Tasks

One task = one commit, referenced as `VAL-021.Tn`.

- [x] T1 — Each criterion of F20 proven again, what was seen recorded (1 h)
- [x] T2 — F20's lines in E17's checklist, `?checklist=E17`, each tried (1 h)
- [x] T3 — The demo page, `docs/guide/f20-a-context-diet.md` (1 h)
- [x] T4 — The result of the validation recorded (0.5 h)

## Criteria proven again (2026-10-18)

On this Mac (Apple M1 Pro), Claude Code 2.1.302, in the worktree of `feat/val-021-validate-f20` at `c1754793`, `origin/develop` merged (already up to date). Each criterion: its tagged tests (`[F20.ACn]` in TypeScript, `f20_acn_…` in Rust), then the command a person would run. The session measured is the run's own (`48a404b1`, F18 to F20, 95 subagents with a type and a description). Criterion 5 is struck from the feature: the progress page was dropped on 2026-10-17.

| Criterion | Command | Seen | Verdict |
| --- | --- | --- | --- |
| AC1 one agent type per task kind, its first request measured | `cargo test --release -q --locked -p hooks f20_ac1_`; `npm run context:size -- <session>/subagents` | 1 test passed; first request, median per type: `general-purpose` 38,897 (74 agents, F18 and F19, each with `CLAUDE.md`, `AGENTS.md`, `MEMORY.md` and a skill listing of 26,805 characters); `story-worker` 10,562 (12 agents, 10,386 to 11,580, no instruction file, no skill listing); `researcher` 11,971 (SP-023) against 38,339 for SP-021 on `general-purpose`; `auditor` 8,505 (AUD-022) against 15,477 and 15,655 (AUD-021, AUD-020); `verifier` 7,099 against 13,994 and 14,192; no `clerk` nor `doc-editor` started since F20, their figures EN-065's (6,901 and 6,146) | holds |
| AC2 a model per type, the Sonnet trial | `npx vitest run tests/agent-models.test.ts scripts/session-report.test.ts -t "F20.AC2]"`; `grep '^model:' .claude/agents/*.md` | 10 tests passed (each of the 8 agent files names its model and why; the session report gives the tokens per model); `clerk` on `sonnet`, the 7 others on `opus`; the trial, EN-098 on Sonnet beside EN-097, EN-096 and EN-099 on Opus, recorded in CHK-021: not comparable in size, the story worker stays on Opus | holds, the trial inconclusive |
| AC3 a worker's hand-back checked | `cargo test --release -q --locked -p hooks f20_ac3_`; the run's transcript searched for `check-handback:` | 1 test passed; 9 hand-backs let through with the line `check-handback: <story> on <branch> pushed at <hash>; next: npm run story:finish -- feature/f20-a-context-diet` (EN-066, EN-086, EN-087, EN-096 to EN-099, CHK-021, AUD-022); 2 refused once for unnamed commits (EN-087's drop and F21's refinement), both counted from the whole history, the bug AUD-022 fixed (A1) | holds |
| AC4 the run's duration and tokens | `cargo test --release -q --locked -p run-totals f20_ac4_`; `npm run metrics:session -- --run`; `.git/metrics/run.json` | 3 tests passed; `2 h 31, open \| 184,490,904 \| 2,620 \| 2,597,414 \| 181,629,911 \| 260,959` (duration, tokens, input, cache written, cache read, output); the marker `{"start":…,"end":0,"sessions":["48a404b1-…"]}`, opened at 21:56:53 on 2026-10-17 | holds |
| AC6 a run to its scope, a question parked, pushes kept for what blocks | `cargo test --release -q --locked -p run-totals f20_ac6_`; `npx vitest run tests/backlog.test.ts -t "F20.AC6]"`; `npm run run:should-stop` | 7 and 1 tests passed; `go: week 58 % used, the limit is 90 %`, exit 0; the run skill's sections A question parks its story, Pushes and Stop when; no story parked, so no push sent; the push on the phone waits for the Product Owner's setting (`/config`), as AUD-022 left it (A5-AC6) | holds |
| AC7 the Git sequences in one call, pages through the library | `cargo test --release -q --locked -p git-sequences -p hooks f20_ac7_`; `npm run git:sync`; `curl -s https://www.sqlite.org/fts5.html`; `library_get` of that page; `npm run library -- stats` | 3 tests passed; `val-021: feat/val-021-validate-f20 rebased on origin/feature/f20-a-context-diet, 0 commit(s) ahead`, 0.85 s; the `curl` refused: `The library holds this page: call library_get with url "https://www.sqlite.org/fts5.html", then library_read the sections needed`; `library_get` gave its 95 sections; Git calls per story worker (`git fetch`, `rebase`, `add`, `commit`, `push` or `npm run git:`): F19's 31 workers 1 to 32, median 12, F20's 9 story workers 1 to 7, median 4; the stats group by project, every use `(not recorded)` as yet, the running server being `develop`'s copy, which records none until F20 is merged | holds |
| AC8 the pages as principles, measured, no rule lost | `npx vitest run tests/backlog.test.ts -t "F20.AC8]"`; `wc -m` of each family today, divided by 2.30 | 1 test passed (each of EN-096 to EN-099 measured at 2.30 characters per token, its rules traced); in tokens, before, after its story, today: rules and skills 44,900, 39,500, 39,300; conventions 78,300, 66,000, 65,900; process 5,200, 4,900, 4,900; tooling 53,711, 39,330, 34,900 (the progress page's rows removed since); in all 182,100, 149,700, 144,900, −20 % | holds |

All of `hooks`, `run-totals` and `git-sequences`: 118 Rust tests passed, 15 of them tagged `f20_`; the 12 TypeScript tests tagged `[F20.ACn]` passed. Nothing failed: no fix was needed.

## F20's lines in E17's checklist

Drafted by `npm run val:prepare -- F20`, then written as 12 lines (`playground/checklists/epic-17-f20-lines.ts`, `?checklist=E17`), each tried here on 2026-10-18 with the output it names: nine commands run at the repository's root or in this worktree, the status line printed by `run-totals status` (`run 2 h 35 · 191,734,902 tokens · week 58 % used`), the hand-back line read in the run's transcript, and the run skill's sections read. `val:prepare` drafted a line for the struck criterion 5 too; it has none. The page's test lists the 36 lines of E17, names each command or output of F20's lines and nothing of the dropped progress page; a line changed from `the limit is 90 %` to `the limit is 89 %` made it fail.

## Decisions of the validation (2026-10-18)

- **Validated by the agent**: F20 is a tooling feature, so the agent validates it and opens its pull request into `develop`, labeled `autonomous`, without waiting for the Product Owner, who may try the lines that concern them (ADR-0072, decided on 2026-10-17).
- **Criterion 5 left out**: struck from the feature on 2026-10-17, the progress page being judged not useful; no line, no test, and the demo page shows it struck.
- **F20.AC2 read as held with an inconclusive trial**: the criterion asks for the comparison, which CHK-021 made and recorded; its outcome, the story worker kept on Opus, is a decision for the Product Owner to confirm, a line of the checklist.
- **F20.AC6 held without the phone push**: the stop at 90 %, the parked question and the rule of pushes are tested and written; a push on the phone waits for the Product Owner's setting in `/config`, as AUD-022 left it.
- **F20.AC7's project recorded by its test**: the library's uses read `(not recorded)` until F20 is on `develop` and the server restarted, since the running server is `develop`'s copy; `scripts/library-pages.test.ts` proves the project recorded and grouped.
- **The Git calls per story compared, EN-100's open point**: F19's 31 workers made 1 to 32 Git calls, median 12, F20's 9 story workers 1 to 7, median 4; the stories differ in size, so this is an order of magnitude, not the tool's gain alone.

## Result of the validation (2026-10-18)

| Line | Seen by | Result |
| --- | --- | --- |
| 1 to 12 | the VAL worker | as written, each tried on this Mac; line 2's decision to confirm left to the Product Owner |

## For the main thread

Rule-docs: none affected, a tooling validation, no rule of `docs/domain/` changed.

The `done` commit, made by this worker as the main thread asked, sets VAL-021 and F20 `done` and updates the State line of E17 in `AGENTS.md`; the journal entry is the main thread's to write, its text below.

### Journal

- `#run` VAL-021: F20 validated by the agent (ADR-0072): its seven criteria left proven again, criterion 5 struck; 12 lines in E17's checklist; the demo page `docs/guide/f20-a-context-diet.md`. A story worker's first request from 38,897 tokens (`general-purpose`) to 10,562 (`story-worker`); the agent pages from 182,100 tokens to 144,900 (−20 %); Git calls per story worker from a median of 12 to 4; a stored page read in 0.1 to 0.2 s instead of 6 s. The story worker stays on Opus, the Sonnet trial too small to compare: a decision for the Product Owner to confirm.

### Improvements

| 2026-10-18 | VAL-021 | `val:prepare` drafts a line for a criterion struck from the feature (F20's criterion 5) and rewrites the checklists' barrel `export type * from "./checklist-line"` as `export *`, which `npm run fix` writes back; it should skip a struck criterion and leave the barrel's other lines as they are | tool |
