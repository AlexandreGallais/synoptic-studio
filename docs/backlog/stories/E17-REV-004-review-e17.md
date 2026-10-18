---
id: REV-004
epic: E17
title: Review E17 with the Product Owner
status: done
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
- [x] T3 — The questions for the Product Owner: the closure criteria, what was built, the next epic (30 min)
- [x] T4 — Session with the Product Owner: their answers recorded, the backlog adjusted (1 h)

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

## Questions for the Product Owner

Asked before the session (T4); answered on 2026-10-18 (Result, below).

### The closure criteria

| Criterion | Met? | Evidence |
| --- | --- | --- |
| ESLint and its plugins removed | met | no `eslint` package in `package.json` nor `package-lock.json`, no ESLint configuration; the 854 rules accounted for (`tests/eslint-rules-accounted.test.ts`, ADR-0075) |
| `check:all` under a duration set at the refinement | met most of the time, to confirm | the target set at SP-021 (2026-10-17): about 70 to 90 s most of the time. 74.6 s (EN-113), 74.9 s (VAL-022), 83.9 s with the site built (NAM-002); median 73 s over the 64 runs `target/release/metrics` records; this review's two runs 112.6 s and 105.9 s, the site built, load average 9 to 49 |
| tokens per agent input measured lower | met | a story worker's first request 38,897 → 10,562 tokens, a research spike 38,339 → 11,971, an audit 15,477 → 8,505 (VAL-021; `target/release/context-size` on 30 story workers this day) |
| the release that closes the epic made with the flow of EN-046 | not met | no tag since `v0.2.0`; EN-046's flow (release-please's pull request into `develop`) was replaced by ADR-0079's release branch and the project's tool, which drafts 0.3.0 (57 lines) but has not published yet |

### Questions

1. Is `check:all` at a median of 73 s, and 106 to 113 s on a loaded machine with the site built, the criterion met?
2. Is the closing release 0.3.0, made with ADR-0079's flow instead of EN-046's, and is E17 closed once it is published, or before?
3. The agent's decisions to be told, each kept unless refused: the tools called by their binary (ADR-0078), the release branch from `develop` (ADR-0079), the Git hooks in `.githooks/` (ADR-0080), 26 npm scripts with `story:finish` and `mutate` kept since their binaries take no lock (ADR-0082); and the decisions VAL-021 and VAL-022 left to confirm: `AGENT_LIBRARY_FILE` (ADR-0077), one Git command through read-only filters (EN-104), `run-totals status` not recorded (EN-102), the story worker on Opus (EN-066).
4. Anything built that was not meant? The progress page was built, then dropped (EN-087); the private records' GitHub repository is left for deletion (EN-114).
5. E17's checklist: rewrite its 16 out-of-date lines on the binaries and ADR-0079, with their test, or keep the validated lines as records and add a note?
6. Permissions only the Product Owner sets: allowing `target/release/story-finish` and `mutate`, so that their scripts can go (ADR-0082); the story auto-merge for a tooling story outside a feature (EN-115's feedback row).
7. Is E02, editing shapes with precision, still the next epic after RET-004, and do NAM-002's logged tooling items (the `rust` lane's split, the guard's matchers, the hooks' JSON reader, the version and story-id readers) wait for a later tooling pass, or come first?

## Result (2026-10-18)

The Product Owner answered the review on 2026-10-18; each answer is in the [E17 report](../../guide/e17-fast-tooling-with-oxc-and-rust.md), Review.

- **`check:all`**: the criterion is met; its median, about 73 s, is within the 70 to 90 s target, and the runs of 106 to 113 s come from the load.
- **The closing release**: after RET-004, which weighs the Product Owner's idea of a tag that creates the version without a new commit; 0.3.0 is then published with the flow chosen, and E17 closes at that publication. ADR-0079 stands until then.
- **Next epic**: E02, editing shapes with precision, after RET-004.
- **Tooling items**: NAM-002's logged items and this review's incident cards wait for the tools point at an epic's end; tools are not fixed during an epic's features (decided on 2026-10-18).
- **What was built**: nothing built that was not meant; no remark on the report. The records' private GitHub repository is left for the Product Owner to delete.
- **Decisions confirmed** the same day: the tools called by their binary's path (ADR-0078), the story worker on Opus (EN-066), `AGENT_LIBRARY_FILE` (ADR-0077), one Git command let through when followed by read-only filters (EN-104), a tooling feature validated by the agent (ADR-0072). Told: ADR-0080, ADR-0081, ADR-0082.
- **Permissions parked** until RET-005, the next epic's retrospective: the `story-finish` and `mutate` binaries allowed, and the story auto-merge into `develop` for a tooling story outside a feature.
- **E17's checklist**: the 16 lines naming removed scripts or release-please stay as records of what was validated; the report notes that EN-115 moved those tools to their binaries. The checklist and its test are unchanged (the agent's decision).

**Closure criteria**: three met (ESLint removed, `check:all`, tokens per agent input); the closing release follows RET-004, and E17 closes at its publication.

**Backlog changes**: none in this story. RET-004 weighs the tag idea and the release flow; RET-005 takes up the parked permissions.

## For the main thread

Incident cards (tools are not fixed during the work, decided on 2026-10-18):

- **The main checkout's hooks ran stale binaries.** Seen: `npm run worktree:new` refused with the reason before EN-115 (`gets it by a rebase`, no binary named), from `target/release/guard-bash` of the main checkout, built at 06:55, which lacks EN-115's text; the worktree's binary, built at 10:16, has it. Cause, likely: EN-115 (#359) merged on GitHub, not by `story:finish`, so nothing rebuilt the main checkout's binaries. Remedy: `npm run rust:build` in the main checkout; then a rebuild after any pull of `develop`.
- **`epic-check` takes a dropped story for an open one.** Seen: the backlog row lists `E17-F20-EN-064-one-hour-prompt-cache`, `status: dropped`, as **to look at**, and exits 1. Cause: `tools/repo-tools/src/epic_check.rs` keeps every story without the line `status: done`.
- **`session-report` writes `1617 min 60 s`.** Seen in `target/release/session-report --all --markdown`: the seconds rounded to 60 without carrying the minute.
- **E17's checklist names removed scripts**: 14 lines, one more expecting `font:metrics`, one obsolete (release-please); `playground/checklist/checklist-page.test.ts` asserts the old commands. Out of date too: `docs/guide/f18-safe-and-cheap-parallel-run.md`, Validation, still "Not yet validated"; `.claude/settings.json` allows `Bash(npm run library *)`; RET-004's criterion names `npm run epic:check`.

`check:all` measured on `develop` at `e8f360ef` in this worktree: 112.6 s (`time` 115.3 s; the site built, the worktree's first run, load average 12 at its end), then 105.9 s inside `epic-check`.

Rule-docs: none affected, a review of a tooling epic, no rule of `docs/domain/` changed.

### Journal

- `#run` REV-004: E17 reviewed with the Product Owner on 2026-10-18; the report `docs/guide/e17-fast-tooling-with-oxc-and-rust.md`. The checklist run again: 26 lines ok, 6 by their tests, 16 out of date since EN-115 or ADR-0079, kept as records. Closure: ESLint removed, `check:all` (median about 73 s) and tokens per agent input met; E17 closes at the publication of 0.3.0, after RET-004 weighs the tag idea. Next: RET-004, then E02.
- `#decision` REV-004: confirmed ADR-0078, ADR-0077, the story worker on Opus, the guard's read-only filters (EN-104) and tooling features validated by the agent; told ADR-0080 to ADR-0082. The permissions for the `story-finish` and `mutate` binaries and the story auto-merge into `develop` parked until RET-005; tools are not fixed during an epic's features.

### Improvements

| 2026-10-18 | REV-004 | `epic-check` lists a `dropped` story among the backlog to look at: a story `dropped` or `done` is closed | tool |
| 2026-10-18 | REV-004 | `session-report` rounds seconds to 60 without carrying the minute (`1617 min 60 s`): round the whole duration first | tool |
| 2026-10-18 | REV-004 | The main checkout's release binaries are not rebuilt when `develop` is pulled after a merge made on GitHub: the hooks ran a `guard-bash` older than EN-115 | tool |
| 2026-10-18 | REV-004 | E17's checklist and its test name 14 npm scripts EN-115 removed, `font:metrics` and release-please's pull request: rewrite them on the binaries and ADR-0079, or mark them as records | process |
