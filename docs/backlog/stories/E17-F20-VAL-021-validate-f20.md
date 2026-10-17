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
- [ ] T2 — F20's lines in E17's checklist, `?checklist=E17`, each tried (1 h)
- [ ] T3 — The demo page, `docs/guide/f20-a-context-diet.md` (1 h)
- [ ] T4 — The result of the validation recorded (0.5 h)

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
