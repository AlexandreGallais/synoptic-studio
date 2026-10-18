---
id: REV-004
epic: E17
title: Review E17 with the Product Owner
status: in-progress
points: 1
---

# E17 · REV-004 — Review E17 with the Product Owner

Review E17 with the Product Owner: the epic's report, its checklist, their feedback (ADR-0023).

## Acceptance criteria

- Given E17's features validated, when the review is held, then the Product Owner's feedback is recorded and the backlog adjusted.

## Tasks

One task = one commit, referenced as `REV-004.Tn`.

- [x] T1 — The epic's report, `docs/guide/e17-fast-tooling-with-oxc-and-rust.md`, listed in the guide's index (1 h)
- [x] T2 — E17's checklist run again by the agent on `develop`, line by line (1 h)
- [ ] T3 — The questions for the Product Owner: the closure criteria, what was built, the next epic (30 min)
- [ ] T4 — Session with the Product Owner: their answers recorded, the backlog adjusted (1 h)

## E17's checklist run again (2026-10-18)

Each of the 48 lines of `?checklist=E17` run by the agent on `develop` at `e8f360ef`, in this story's worktree, or its claim checked by a command, a figure or a file where a line needs Chrome or a run. Its tests pass: `npx vitest run playground/checklist/checklist-page.test.ts playground/checklists`, 138 tests. A line that no longer works as written is a finding; no tool was changed (decided on 2026-10-18: tools are not fixed during the work).

### F18

| Line | Result | What was seen |
| --- | --- | --- |
| 1 `npm run test` twice at once | ok | the second: `heavy lock held by process 17056 (npm run test in rev-004, for 8 s): waiting`, then ran; both passed, 30.9 s |
| 2 four subagents | ok, by its tests | not started live; `AGENT_LIMIT` 3 and its message in `tools/hooks/src/limit.rs`; `cargo test -p hooks f18_ac2`: 7 passed |
| 3 `npm run worktree:new` | **finding** | script removed by EN-115, refused by `guard-bash`; `target/release/worktree-new US-61`: `` `US-61` is not a story id such as US-061 ``, exit 1 |
| 4 `npm run backlog:index`, `git status` | ok | no file changed |
| 5 `git check-attr merge` | ok | `docs/process/journal.md: merge: union`; its tables are written without padding since EN-073, so the line's "stay padded" is out of date |
| 6 `npm run metrics:session -- --all --markdown` | **finding** | script removed; `target/release/session-report --all --markdown` gives the whole session, but writes `active 1617 min 60 s` |
| 7 `npm run epic:check` | **finding** | script removed; `target/release/epic-check E17 2026-10-16`: every check ok, `check:all passed in 105.9 s`, but the backlog row lists EN-064 as **to look at**, though it is `dropped` |
| 8 `npm run mutate` | ok | `3 caught, 0 survived, 0 file(s) skipped` |
| 9 Chrome, Installed fonts | not run, by its tests | needs Chrome; `playground/default-font` tests passed |
| 10 Weight, Embed fonts | not run, by its tests | needs Chrome; same tests |
| 11 the font tests | ok | `src/io/font-face`, `src/io/font-collection`, `playground/fonts`: every test passed, none skipped |
| 12 ADR-0067, release-please's pull request | **finding** | obsolete: ADR-0079 replaced release-please by the project's tool; release-please is in no workflow |

### F19

| Line | Result | What was seen |
| --- | --- | --- |
| 1 `npm run lint` | ok | `lint: 0 finding(s) in 1945 file(s)`, 1.9 s |
| 2 a type error planted | ok | `src/probe.ts:1:14: error typescript(TS2322)…` |
| 3 a directive silencing nothing | ok | `error: Unused oxlint-disable directive (no problems were reported).` |
| 4 a table row of three cells | ok | `docs/probe.md:5:9 markdown/table-column-count …` |
| 5 `npm run format:check`, `npm run format` | **finding** | `format` removed by EN-115 (`npm run fix` formats); `format:check`: `Markdown: 0 of 564 files not formatted.`; `npm run fix` wrote the padded table `\| A \| B \|` |
| 6 `npx tsc --version`, `typescript6` | ok | `Version 7.0.2`, then `0` |
| 7 `npm run build`, the declarations | ok | 2.1 s, then 589 |
| 8 `npm run docs:api` | ok | `api-pages: 537 pages written to docs/api`, 0.3 s |
| 9 `check:all`, nothing of the site changed | not run as written | no such branch at hand: both runs of this review had a change in `docs/`; the skip is proven by `tools/repo-tools/tests/check_all.rs` (F19.AC8) |
| 10 `check:all`, a page of `docs/` changed | ok, slower | the site built: 112.6 s (`docs:build` 77.7 s, `check:rust` 112.3 s, load average 9 to 12), then 105.9 s inside `epic-check`; against about 55 to 60 s in the line |
| 11 `npm run check:rust` | ok, inside `check:all` | its lane passed in both runs |
| 12 the rules accounted, the linter's tests | ok | `tests/eslint-rules-accounted.test.ts` passed; `cargo test --release -q -p lint`: 652 passed |

### F20

| Line | Result | What was seen |
| --- | --- | --- |
| 1 `npm run context:size` | **finding** | script removed; `target/release/context-size` on this session's subagents: `general-purpose` 38,897 (74 agents), `story-worker` 10,562 (30), `auditor` 8,993, `verifier` 7,304 |
| 2 `grep '^model:'` | ok | `clerk` on `sonnet`, the seven others on `opus` |
| 3 the hand-back line | by its tests | seen at this story's hand-back; `cargo test -p hooks f20_ac3`: passed |
| 4 the status line during a run | by its tests | not in a run; `cargo test -p run-totals`: 30 passed |
| 5 `npm run metrics:session -- --run` | **finding** | script removed; `target/release/session-report --run`: `9 h 01 · 1,053,702,389` tokens by kind |
| 6 `npm run run:should-stop` | **finding** | script removed; `target/release/run-totals should-stop`: `go: week 71 % used, the limit is 90 %`, exit 0 |
| 7 the run skill's three sections | ok | `A question parks its story`, `Pushes`, `Stop when` in `.claude/skills/run/SKILL.md` |
| 8 `npm run git:sync` | **finding** | script removed; `target/release/git-sequences sync`: `rev-004: docs/rev-004-review-e17 rebased on origin/develop, 0 commit(s) ahead …; release binaries rebuilt` |
| 9 `npm run git:commit` | **finding** | script removed; `target/release/git-sequences commit -F - <paths>` made this story's commits |
| 10 `curl` of a stored page | ok | refused: `The library holds this page: call library_get with url "https://www.sqlite.org/fts5.html" …` |
| 11 `npm run library -- stats` | **finding** | script removed; `target/release/library stats`: `SVG editor` listed (21 searches), the uses before it under `(not recorded)` |
| 12 the conventions' size, `F20.AC8]` | ok, figure grown | 155,195 characters (67,500 tokens) against about 151,600 in the line; the test passed |

### F21

| Line | Result | What was seen |
| --- | --- | --- |
| 1 `ls .claude/hooks`, the hooks' commands | ok | no such folder; each hook runs `target/release/<binary>`: `guard-bash`, `limit-agents`, `library-pages`, `check-handback`, `track-tool-call`, `session-state`, `log-tool-call` |
| 2 one Git command piped, then two commands | ok | `git log --oneline -3 \| head -3` went through; `git status; ls` refused, `A worktree refuses a chained Git command` |
| 3 `npm run metrics`, section Tools | **finding** | script removed; `target/release/metrics`: `guard-bash` 62,008 calls, 0 failed, 0 ms median; no row `run-totals status` |
| 4 `ls scripts`, `grep -n '"node ' package.json` | **finding** | `default-font-metrics.ts` and its test; the `grep` prints nothing: `font:metrics` was removed by EN-115 (`node scripts/default-font-metrics.ts`) |
| 5 `npm run -s library -- search fts5` | **finding** | script removed; `target/release/library search fts5`: FTS5's page first, 25 ms |
| 6 dev packages, `npm run deps:unused` | ok | 13; nothing printed, exit 0 |
| 7 `git-hooks commit-msg` | ok | `subject may not be empty [subject-empty]`, `type may not be empty [type-empty]`, `found 2 problems, 0 warnings`, exit 1 |
| 8 `time npm run -s lint:secrets` | ok | nothing printed, exit 0, 0.30 s |
| 9 files of the removed packages | ok | `0` |
| 10 `npm run release:prepare` in a fresh clone | **finding** | script removed; in a clone of `develop` in the scratch folder, `target/release/release prepare`: `v0.2.0 → 0.3.0 on release/0.3.0: 57 line(s) drafted in CHANGELOG.md`, then `release check` named each `[draft]` line |
| 11 `npm run -s page -- REF-CC-HOOKS` | **finding** | script removed; `target/release/page REF-CC-HOOKS`: 2,574 characters of the 214,258 of `docs/references.md`, 11 ms |
| 12 `check:all`, `f21_ac9_` | ok, slower | `check:all` green, 112.6 s and 105.9 s with the site built, against about 75 s in the line; `cargo test -q -p repo-tools f21_ac9_`: 5 passed |

### What the findings come to

- **14 lines name an npm script EN-115 removed** (F18 3, 6, 7; F19 5; F20 1, 5, 6, 8, 9, 11; F21 3, 5, 10, 11) and F21 4 expects one; each tool works by its binary. EN-115 left the validated checklists and their test as records, and `checklist-page.test.ts` asserts the old commands, so the lines are rewritten with their test.
- **One line is obsolete**: F18 12, release-please's pull request (ADR-0079).
- **Two tools print a wrong detail**: `epic-check` takes a `dropped` story for one to look at; `session-report` writes `60 s` instead of carrying the minute.
- **`check:all` was slower than the lines say** in this review's two runs, the site built and the machine loaded: 112.6 s and 105.9 s. Its median over the 64 runs `target/release/metrics` records is 73 s.
- **Outside the checklist**: the main checkout's hooks run a `guard-bash` built at 06:55, before EN-115 was merged, so a removed script is refused with the reason before EN-115's criterion 4 (the binary not named); the F18 demo page still reads "Not yet validated (`VAL-019`)" though F18 was validated on 2026-10-17; `.claude/settings.json` still allows `Bash(npm run library *)`, a script removed; RET-004's criterion names `npm run epic:check`.
