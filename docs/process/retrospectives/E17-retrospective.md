# E17 — Retrospective

**Dates**: 2026-10-16 to 2026-10-18. **Story**: RET-004. **Participants**: Product Owner, Claude Code.

E17 made the agent's checks and tools fast: four features in three days (a safe and cheap parallel run, the checks on Oxc, TypeScript 7 and Rust, a context diet for every agent, the agent's tools and the library server in Rust). ESLint, Prettier, TypeDoc, secretlint, Husky and release-please left; `check:all` went from 170–217 s to a median of about 73 s; the dev packages from 38 to 13; a story worker's first request from 38,897 to 10,562 tokens. The tooling it wrote is now larger than the library: 131,597 lines of Rust in 27 crates, against 15,096 lines of library code. This record holds the facts, the agent's account, every logged row and tool incident with a recommendation, and the Product Owner's method wishes of 2026-10-18 as proposals, all adopted that day: the release from the merge into `main` (ADR-0083) with a CI green first (F22), small features with tests first and a linear history from E02 (ADR-0084), and the tools' failures as cards fixed at an epic's end (ADR-0085), E17's own fixed by F23, E02's first feature.

## 1. Facts

| Measure | E15 (RET-003) | E17 |
| --- | --- | --- |
| Days | 3 | 3 (2026-10-16 evening to 2026-10-18) |
| Features | 5 | 4 (F18, F19, F20, F21) |
| Stories | 67 | 92: 71 EN, 2 US, 4 SP, 4 CHK, 4 AUD, 4 VAL, NAM-002, REV-004, RET-004; 263 points; 1 dropped (EN-064) |
| Stories per feature | 9 to 17 | F18 26, F19 28, F21 18, F20 17 |
| Commits on `develop` since the epic's first day | 336 | 873 (since 2026-10-16 16:00 UTC) |
| ADRs | 11 (0052–0062) | 16 (0067–0082) |
| Library | 517 modules | unchanged by E17: 589 files of `src/` without tests, 15,096 lines; 460 test files, 39,446 lines |
| Tooling | 8 std-only crates at the epic's start | 27 crates, 581 files, 131,597 lines of Rust (`tools/lint` 73,418), 1,566 Rust tests; `scripts/` holds one TypeScript tool |
| Dev packages | 38 | 13 |
| `check:all` | 170 s passed, 217 s failed under load | median about 73 s (REV-004); 61 recorded runs, median 1 min 15 s under the lock, 11 failed |
| Active time by feature | ≈ 32 h 40 | F19 10 h 37, F21 5 h 40, F18 3 h 28, F20 2 h 42 (≈ 22 h 27), and 10 h 45 on `develop` and detached heads, shared with E15's last day |
| Tokens by feature (cache included) | ≈ 1,566 M | F19 1,967 M, F21 799 M, F20 325 M, F18 132 M (≈ 3,223 M), and 577 M on `develop` and detached heads |
| Tokens per feature story | ≈ 24 M | ≈ 36 M (+51 %): F19 70 M, F21 44 M, F20 19 M; F18's work ran mostly on `develop` after a resume from a plain terminal |
| Main session | — | 1,645 min active, 17,453 calls, 3.0 M output tokens |
| Failed calls | 675 of 13,127 (5.1 %) | 232 of 20,032 (1.2 %), 142 worktree refusals counted apart |
| Sources | 897 pages, 77 % from the library | 1,321 pages, 73 % from the library |
| CI | 0 of 3 runs failed | 6 of 6 runs of the pull request `develop` → `main` (#259, closed 2026-10-18) failed, the same three tests each time; CodeQL failed 4 of 6, green on the last two; 2 story auto-merges refused a branch without a story id (I50) |

Figures from `target/release/metrics --since 2026-10-16`, which counts from 2026-10-16 00:00 UTC and so holds E15's last stories (F17, NAM-001, VAL-018); per-feature lines leave them out.

What the metrics say:

- **The model writing is still the cost.** The model's share of the latest main-thread turns is 72 to 99 %. `python3` fell from 2,172 calls and 590 min in E15 to 337 calls and 51 min, but four workers (EN-098, AUD-022, SP-023, EN-105) still edited files with Python scripts against the project's rules. `cat` 1,031 calls and `sed` 634 read files a `page` call or `Read` would have read.
- **Waiting loops stayed.** `until` 76 calls, 71 min (E15: 70, 61 min), `sleep` gone; `git push` 86 calls, 47 min.
- **`npm run fix` became slow**: 36 calls, median 37.3 s, 31 min in all, since it also runs Clippy and `cargo fmt` (EN-113).
- **The brief's prefix hides the commands.** `exit` is the first command by count (10,573), the `cd "<worktree>" || exit 9` every worker call starts with (I91).
- **The Rust tools themselves are fast** (Tools section): `guard-bash` 62,105 calls, median 0 ms; the library's server 1 to 6 ms a request. The slow ones wait on something else: `story-finish` 1 min 50 s (GitHub's merge), `worktree-new` 33.1 s (a full build, 15.7 s since EN-113), `records-save` 42 s (a 1.9 GB copy, I86).
- **Rarely used**: `context-size` 1 call, `val-prepare` 1, `session-state` 4, `mutate` 6 (2 failed), `records-save` 4 (2 failed, the refused push that led to ADR-0081). `git-hooks commits` failed 14 of 14, all in EN-108's worktree while it compared verdicts over the history.
- **Findings**: `A worktree refuses a chained Git command` 176 times (144 in E17), then the TypeScript tools' lint and type errors while they were edited before their ports (`naming-convention` 91, `TS2345` 44), then the pedantic Clippy lints of the ports (`assert_is_empty` 45, `doc_markdown` 42). On 2026-10-18 (UTC) the TypeScript findings fell to a handful: 2 type errors on `develop`, none of `naming-convention` (I94 checked).
- **Sources**: the pages read most are still E15's (OpenType OS/2 38 times, CSS Fonts 4 27 times); the library records no project for its 1,321 pages ("(not recorded)"), so a page's visits are not counted per project.

### Full check of the epic

`target/release/epic-check E17 2026-10-16`, run 2026-10-18 at a load average of about 30 (about 4 minutes):

| Check | Result | Detail |
| --- | --- | --- |
| check:all (code, site, playground, dependencies) | to look at | test lane: `src/io/font-file/parse-font-metrics.test.ts` > `[F18.AC19] reads Inter Regular of the test fixtures as inter-metrics.json holds it` |
| branches left | ok | only `main`, `develop` and the retrospective's |
| pull requests open | ok | none |
| issues open | ok | 0 |
| code scanning alerts open | ok | none |
| calls failed during the epic (2 % at most) | ok | 232 of 20,032; 142 refused by the worktree isolation, apart |
| calls lost at a force-quit, over an hour old | ok | 3 lost, 2026-10-18 07:42 to 07:47 |
| backlog of E17 | to look at | `E17-F20-EN-064-one-hour-prompt-cache` |
| private records | ok | last save 2026-10-18 07:32 UTC |
| golden file of refactors | ok | present |

The lines to look at, and one the check does not read:

| Line | Cause | Fix |
| --- | --- | --- |
| `check:all` failed on `[F18.AC19]` | load: the file alone passes, 16 tests in 3.05 s; the test reads Inter's file and went over Vitest's 5 s limit, as it did at REV-004's push (12.3 s at a load near 60, incident card C8) | a finding, not fixed during the epic (2026-10-18): the timing of the tests that read Inter under load, in P8's tooling feature |
| EN-064 listed in the backlog | EN-064 is `dropped`; `epic-check` counts only `done` as closed (I110) | P8's tooling feature; nothing to change in the backlog |
| the CI of `develop` → `main` (not read by `epic-check`) | three tests fail on GitHub's runner and pass on the Mac: `[F19.AC6]` expects oxlint's plain output, which prints GitHub annotations when `GITHUB_ACTIONS` is set; `[F19.AC9]` parses `cargo metadata`'s output, empty after a crate download failed; `[F02.AC1]`, the guided test's walk, over 5 s on the runner | P1: a story before 0.3.0; `epic-check` to read the last CI of the pull request into `main` (P8) |

The published site and the playground answer (HTTP 200, 2026-10-18); their pass by hand is left to the main thread with the Product Owner, a subagent having no browser.

## 2. Logged items, decided

Every row of the improvement log (I1–I113, in the file's order on 2026-10-18), of the feedback log (F1–F6), every tool incident card of the private records' `tool-incidents.md` (C1–C9, 2026-10-18, now in [tool incidents](../tool-incidents.md)) and the run's notes. Decided by the Product Owner on 2026-10-18, each proposal's recommendation adopted; the treated rows left the logs, 32 rows kept in the improvement log with their reason.

| Theme | Items | Recommendation | Decision | Where it went |
| --- | --- | --- | --- | --- |
| Release | CI red on #259 (§1) | P1: a story that makes the three tests hold on GitHub's runner, before 0.3.0 | adopted: the CI first | EN-116 (F22) |
| Release | the Product Owner's tag idea; I17 | P2: the version from a tag; I17 closed, release-please gone | adopted: option B, 0.3.0 its first release; I17 closed | ADR-0083, EN-117 (F22) |
| Stories outside a feature | C1, C3, C4, C6; I50 | P8, story 1: `worktree-new`, `story-finish`, the backlog test and the shared rows accept a story attached to `develop`; the auto-merge itself stays parked (C3's merge, C7) | adopted (P8) | EN-118 (F23), ADR-0085 |
| Guard in a worktree | C2; I12, I22, I40, I87, I93, I103 | P8, story 2: one call printing a worktree's state (`git-sequences state`), read-only chains let through, the guard's text matchers on its word reader with AUD-023's cases, `mkdir …heavy.lock` refused, a URL in a commit message let through, a worker's write into the main checkout refused once its prompt names its worktree | adopted (P8), with the push with lease of a rebased feature | EN-119 (F23), ADR-0084 |
| Reports | C5; I36, I72, I82, I83, I88, I91, I92, I110, I111; `epic-check` blind to the CI; the library's visits per project | P8, story 3: the status line drops the calls of ended agents, `metrics` names the command after `\|\| exit 9` and cuts its Findings per epic, `epic-check` closes `dropped` stories and reads the CI of the pull request into `main`, `session-report` carries the minute, `val-prepare` skips a struck criterion | adopted (P8) | EN-120 (F23) |
| Builds and lanes | C8, C9; I51, I90, I97, I100, I106, I107, I112 | P8, story 4: the main checkout's binaries rebuilt after a pull, `check:rust` only when Rust changed and its coverage a lane of its own, the tests reading Inter held under load, a `bench` binary for the stories' measures | adopted (P8) | EN-121 (F23) |
| Checks to add | I37, I54, I55, I56, I58, I60, I81, I99 | into P8's stories where they touch the same tool: `backlog:index --check` reaching the index tool and not the formatter, the cargo tools' versions, the editor's TypeScript path, the Rust coverage floor 93 → 95 %, `typeCheck` in `.oxlintrc.json`, `move-rows` naming the notes it leaves, one anchor slug for GitHub and the site, a test refusing `npm run <binary>` in the agent's pages | adopted (P8) | EN-121 (F23) |
| Mutations | I16, I38, I46, I47 | `mutate` fails when nothing was tried, names the failing baseline, finds a file's own test; when a story next needs it | not now, as recommended | improvement log |
| Code debt of the tools and tests | I32, I68, I69, I101, I104, I105, I108 | one story-id reader, one JSON parser, one Semantic Versioning reader, a shared Git test helper, one reader of the tracked sources for the tests: in the next tooling epic's naming pass | not now, as recommended | improvement log |
| Records | I86 | `records-save` copies only what changed since its last save (42 s for 1.9 GB today) | not now, as recommended | improvement log |
| Library and product | I1–I10, I35, I41–I43, I109 | kept with their trigger, as at RET-003 | kept (P11) | improvement log |
| Oxc | I48, I66 | kept: checked at each epic's tooling pass, an upgrade of oxlint reads its new `correctness` and `suspicious` rules | kept (P11) | improvement log |
| Library server, worktrees | I44, I89 | not now: a library call takes 6 ms; no story ran `npm install` in a worktree since EN-073 | not now, as recommended | improvement log |
| GitHub union merge | I13 | kept until a pull request shows it | kept (P11) | improvement log |
| Process lessons | I20, I27, I63, I64, I67, I70, I71 | closed: each is a rule or a habit already (`.claude/rules/tooling.md`, ADR-0070, the context diet of F20) | closed (P10) | — |
| Done during E17 | I11, I14, I15, I18, I19, I21, I23–I26, I28–I31, I33, I34, I39, I45, I49, I52, I53, I57, I59, I61, I62, I65, I69, I73–I80, I84, I85, I94–I96, I98, I102, I113 | closed: each row names the story that settled it, or the tool it named is gone (ESLint, Prettier, release-please, npm scripts of EN-115); I62's Stryker issue 6210 still open on 2026-10-14, its row kept in `dependencies.md` | closed (P10) | — |
| Method | the Product Owner's wishes a to d, the run's notes | P2 to P7 (§6) | adopted: P3 to P7 | ADR-0084, ADR-0085, `AGENTS.md`, the run skill, `docs/process/agile.md`, the backlog's templates |
| Parked until the next review | story auto-merge into `develop` for agent stories (C3, C7); the permissions of the `story-finish` and `mutate` binaries; ADR-0082's option B; a configuration file for local paths such as the records' root | not proposed now (Product Owner, 2026-10-18) | parked | the next review (RET-005 removed by EN-123) |
| Habits | F1–F6 | written as habits (§4), the rows leave the log | — | habits sheet |

## 3. What helped, what hurt

| Side | Helped | Hurt |
| --- | --- | --- |
| Product Owner | nothing added beyond their remarks of 2026-10-18 | features too large to validate in a day, tools fixed during the run, the main thread's context grown too large: their wishes of 2026-10-18, P3 to P9 |
| Agent | the heavy lock and the agent limit held by the tools (no freeze in E17); the Rust tools by their binary (3 ms a hook); `page` and the library for one row or one section; `git-sequences` (Git calls per worker from a median of 12 to 4); the worker brief with the project rules preloaded | 176 refusals of chained Git commands; features of 17 to 28 stories, F19 alone 10 h 37 and 1,967 M tokens; tools fixed during the run, each fix a worktree, a merge and a rebuild; a CI red on `develop` → `main` unseen until the release; the main thread's context, grown by every merge and log move |

## 4. The agent's own review

| Mistake | Times in E17 | What prevents it now, or what would |
| --- | --- | --- |
| A Git command chained after `cd` in a worktree | 144 refusals | the brief; P8's `git-sequences state` would answer the reads in one call |
| An edit made by a Python script against the rules | 4 workers | the rule is written; a check is missing: `guard-bash` could refuse `python3` writing a file in a worktree (P8, story 2) |
| A worker reading the main checkout's brief, older | 1, a deadlock of 16 min | the prompt names the worktree's brief (F2) |
| A test inheriting the pre-push `GIT_DIR` and writing the shared `.git/config` | 2 | tests remove every `GIT_*` variable; `check:all` compares `.git/config` (EN-103, EN-113) |
| A background Git command started outside its worktree | 1 | every background command starts with `cd <worktree> \|\| exit 9` and checks the branch (F3) |
| A conflict resolved by taking a whole side | 1, a paragraph lost then restored | resolved in the file, then diffed (F4) |
| Personal data in a request header | 1 (EN-110, a `User-Agent` sent to crates.io) | the rule in `AGENTS.md` and the project rules (EN-113) |
| A tool built before its view was shown | 1 (EN-087's progress page, dropped) | a sample shown first (F6) |
| A CI failure not read until the release | 6 runs | P1, and `epic-check` reading the CI (P8) |

Rules bent: "every new tool in Rust" by EN-047's first draft in TypeScript (F1); "never a Python edit script" by four workers; "no two agents on the same files" held by the shared-files check (EN-050), never bent.

Context lost: none by a force-quit; the main thread's context grew through the run, every merge, log move and worker report read into it, and the Product Owner asked that more of that work go to agents (P7).

The feedback log's six rows are written as habits in [Working with the agent](../../playbook/agent.md#lessons-from-the-product-owner):

| Row | What happened | Habit |
| --- | --- | --- |
| F1 | EN-047's lock started as a TypeScript script | say in every worker prompt that a new tool is written in Rust |
| F2 | a worker followed the main checkout's older brief and blocked every heavy command 16 minutes | point a worker to the rules inside its own worktree |
| F3 | a background push and `story:finish` ran in the main checkout; a worktree removed too early | start a background Git command inside the story's worktree and check the branch first |
| F4 | `checkout --ours` dropped a paragraph that did not conflict | resolve a conflict in the file, then diff it with the story's version |
| F5 | zsh's `set -- $var` did not split and made a branch `origin/feature/…` | pass each value quoted on its own, or split words in `bash` |
| F6 | the run's progress page was built, used, then judged not useful | show the Product Owner a sample of a view before building it |

## 5. Evolvability review

| Question | Answer | Recommendation |
| --- | --- | --- |
| 1. Size | the library unchanged (589 files, nine folders at 15 or 16 modules, each listed in `docs/conventions/files.md` and held by `tests/folders.test.ts`); the tooling 131,597 lines of Rust, 8.7 times the library's code, `tools/lint` alone 73,418; `check:all` median about 73 s, `check:rust` its longest lane (56.7 s at load 19.6, I97) | no new tool without a measured signal (`docs/tooling/agent-tools.md`); the next tooling work as fixes of cards, not new tools (P8) |
| 2. Tools for later | `size-limit`: not published yet. `@microsoft/api-extractor`: the public API changes in E02. Stryker: issue 6210 still open (updated 2026-10-14) | keep the three rows |
| 2b. Overrides | one left, `katex ^0.19.0`: `mermaid` 12.1.0 asks `^0.16.47` and its latest asks `^0.18.0`; `deepmerge-ts` left with ESLint | keep `katex` |
| 3. Fitness functions | added in E17: the modules per folder (EN-061), the shared files on a story branch (EN-050), `.git/config` unchanged by the tests (EN-103), stale binaries (EN-104), the Rust coverage floor and advisories (EN-069), the binaries named in `.claude/settings.json` (AUD-023); still only written: the agent's pages naming binaries (I99), the CI's result before a release | P1, P8 |
| 4. Patterns | five story-id readers, two JSON parsers, two Semantic Versioning readers, 15 Git test helpers (I101, I104, I105, I108); the Unicorn, functional and JSDoc rules carried into one Rust linter | the next tooling epic's naming pass |
| 5. Agent configuration | `AGENTS.md` 125 lines and `CLAUDE.md` 29 (154 of 200); eight agent types; the run skill 114 lines; the run skill still sent a comment about tools to a worker during a run, against the decision of 2026-10-18 | changed in RET-004.T3; nothing else past its limit |
| 6. Process | features of 17 to 28 stories with 4 stories of overhead each (SP, CHK, AUD, VAL); one validation for 9 to 12 criteria; the tooling validated by the agent (ADR-0072) | P3 to P5 |

The trial of three subagents (ADR-0063) was measured: three at once, the heavy lock on every heavy command, no freeze in E17; the load average reached 20 to 60 under other applications (Figma, Spotlight indexing the records), where a timing test failed (C8). ADR-0072 already made three the rule on 2026-10-17: kept.

## 6. Method and tools

The agent's account (ADR-0074). E17 was the agent's own epic: it wrote, tested and validated its tools, and most of its time went into features far larger than a story worker can see whole. F19 took 28 stories and 10 h 37 to carry 265 lint rules into Rust; its stories were right-sized, the feature was not, and the Product Owner waited for one validation at its end. Tools fixed during the run cost more than their fixes: each needed a worktree, a merge, a rebuild of the main checkout's binaries and a rebase of every running story. What would go faster: smaller features, one at a time, the tools fixed together at the epic's end from cards; what would spend less: fewer reads by the main thread, merges and log moves done by agents; what would do better: tests written apart from the code, and a CI read before a release.

The proposals, grouped for the questionnaire (at most 12 decisions). Each says what changes for the Product Owner. The Product Owner adopted all eleven on 2026-10-18 as recommended: P6 as a trial on two stories of E02, measured against one worker; P8 as E02's first feature, F23; P1 and P2 as E17's last feature, F22, before 0.3.0.

| # | Proposal | Recommendation | Evidence | What changes, for example |
| --- | --- | --- | --- | --- |
| P1 | The CI of `develop` → `main` green before 0.3.0 | yes: one story, before the release | §1: 6 of 6 runs failed on three tests that pass on the Mac | the pull request of 0.3.0 opens green; today it would show three red tests and wait |
| P2 | The release made from a tag, no version commit | B (below) | `REF-SEMANTIC-RELEASE-FAQ`, `REF-NPM-VERSION`, `REF-GH-WORKFLOW-DISPATCH` (a workflow on a pushed tag); nothing in the code reads `package.json`'s version but the release tool (`tools/release/`) | you merge `develop` into `main`, and the release `v0.3.0` appears with its notes; no `release/0.3.0` branch, no `chore: release` commit |
| P3 | Small real features | yes, from E02: 2 to 4 stories, the research done as a spike before the refinement, no checkpoint (`CHK`) under 5 stories | F19 28 stories, 10 h 37, 1,967 M tokens; F20 17 stories in 2 h 42; `REF-ANTHROPIC-HARNESSES`: one feature at a time was "critical" | a color picker is one spike, three stories (the model, the panel, the export), one validation of 4 to 6 lines; you validate it the same day |
| P4 | Tests first, in their own commit | yes: per story, `T1` the tests from the criteria, failing, `T2` the code that makes them pass | ADR-0022 already asks tests first; a failing first commit shows each test can fail, which ADR-0064 asks the worker to prove by breaking each test once | each story shows two commits, `test(model): …` then `feat(model): …`; the history shows the tests existed before the code |
| P5 | One feature at a time, a linear history | yes: a feature branched from `develop` only after the previous one merged; nothing pushed to `develop` while it is open (the main thread's docs and log rows go to the feature branch); merged by a merge commit; if `develop` moved anyway, `git-sequences sync` rebases the feature and pushes with lease, an exception to RET-003's "no force push on a feature branch"; `develop` → `main` and the release at the epic's end | `REF-GITFLOW` (`--no-ff` keeps a feature visible), `REF-GH-MERGE-METHODS`; F21 was already branched from `develop` after F20 merged | `git log --graph` shows `develop` as one line with one loop per feature, none crossing |
| P6 | Two agents per story, in parallel: one writes the tests, one the code | a trial on two stories of E02's first feature, measured against P4 done by one worker; the story names its interface at the refinement (files, function names, signatures); the reconciliation by the implementer, a disputed value settled by the derivation or stopped as a business question | `REF-CC-BEST-PRACTICES` ("have one Claude write tests, then another write code"), `REF-CC-AGENT-TEAMS` (a test file is a right-sized task; tokens scale with the agents; each owns its files) | for `colorToHex`, one agent writes `color-to-hex.test.ts` from the criteria and the hand-computed values, the other `color-to-hex.ts`; a value they disagree on is shown to you only if no derivation settles it; a story costs about twice the tokens, and only one story runs at a time within the three agents |
| P7 | Researchers in parallel on separate topics only, and a lighter main thread | yes: one note per researcher; the main thread hands merges, done commits and log moves to the `clerk` agent | `REF-CC-AGENT-TEAMS` (research is the strongest parallel use; two agents on one file overwrite each other); the main session's 1,645 active minutes and its context | a spike on fonts and one on colors run at once, each writing its own note; the main thread answers you sooner |
| P8 | E17's tool rows and cards fixed together, in a small tooling feature at E02's start | yes: four stories (stories outside a feature; the guard in a worktree; the reports; the builds and lanes), the rows of §2 assigned to them; the rest not now | 9 cards in one day; 176 refusals; the three CI tests; I110, I112 | before E02's first feature, one short feature whose validation you leave to the agent, as ADR-0072 allows for tooling |
| P9 | Where the incident cards live, and how they are written | in the repository, `docs/process/tool-incidents.md`, a log like the others (written by the main thread at merge, a worker's under `### Incidents` moved by `move-rows`, merged by union); no Rust command for now, a test of the cards' format instead; read at each epic's "method and tools" point | the cards hold no personal data; the records folder is outside the repository, so a worker cannot read it nor write one there; 9 cards written by hand in a day, each a few lines | you read the cards on GitHub beside the improvement log; a worker's failed tool reaches the retrospective without the main thread copying it |
| P10 | The rows already settled closed | yes: the "Done during E17" and "Process lessons" lines of §2 | each row names its story or the tool it named is gone | the improvement log shrinks from 113 rows to about 40 |
| P11 | The long-kept library and product rows kept | yes: with their trigger, as at RET-003 | I1–I10 and I41–I43 wait for features not started | nothing changes until a feature needs them |

P2's options, for one release:

|  | A — ADR-0079 now | B — the merge into `main` makes the tag (recommended) | C — a tag pushed by hand makes the release |
| --- | --- | --- | --- |
| commits made for the release | 1 (`chore: release x.y.z`) | 0 | 0 |
| branches made | 1 (`release/x.y.z`) | 0 | 0 |
| merges by the Product Owner | 1 | 1 (`develop` → `main`) | 1, then a tag pushed |
| `package.json`'s `version` | the released version | `0.0.0-development`, as semantic-release advises when the version is not committed | same as B |
| where the version comes from | `release prepare`, committed | `release publish` on `main`: the last tag and the commits since, as `release prepare` computes it today | the tag's name |
| the release notes | the entry in `CHANGELOG.md`, committed | the pull request's body, rewritten by the agent and checked by `release check`, copied into the GitHub release; `CHANGELOG.md` keeps 0.1.0 and 0.2.0 and links to the releases | same as B |
| `develop` caught up after the release | a fast-forward, or a merge when `develop` moved | a fast-forward | a fast-forward |
| tool changes | none | `release prepare` writes nothing, `publish` computes and tags; a new ADR supersedes ADR-0079; about one story | as B, plus a workflow on `push: tags` (`REF-GH-WORKFLOW-DISPATCH`) |

The cost of B and C: a checkout of `v0.3.0` reads `0.0.0-development` in `package.json`, and the current version is read from the tags or GitHub's releases, not from a file. When the library is published to npm, the publishing job writes the tag's version into the package before `npm publish`, without a commit (`npm version --no-git-tag-version`, `REF-NPM-VERSION`). The Rust crates' `0.1.0` are not released and stay as they are. B over C: one action for the Product Owner instead of two, and no tag pushed by hand that a typo could misname.

## 7. Changes adopted

Decided by the Product Owner on 2026-10-18.

- [ADR-0083](../../adr/0083-the-release-made-by-the-merge-into-main-from-a-tag.md), superseding ADR-0079: the merge of `develop` into `main` makes the tag and the GitHub release, the version from the tags, no release branch nor version commit; 0.3.0 its first release, E17 closing at its publication.
- [F22](../../backlog/features/E17-F22-the-release-from-the-merge-into-main.md), E17's last feature: EN-116 (the CI green on GitHub's runner), EN-117 (the release of ADR-0083), AUD-024, VAL-023; no checkpoint.
- [ADR-0084](../../adr/0084-small-features-tests-first-and-a-linear-history.md), from E02: features of 2 to 4 stories, one at a time; `T1` the interface and its failing tests, `T2` the code; a linear history with the feature rebased with lease when `develop` moved; two agents per story as a trial on two stories; researchers on separate topics; a lighter main thread. Written into `AGENTS.md` (Work, Never), the project rules, the run skill, the worker brief, `docs/process/agile.md`, `docs/backlog/README.md`, the templates and `tests/backlog.test.ts`.
- [ADR-0085](../../adr/0085-tool-incident-cards-tools-changed-at-an-epics-end.md), amending ADR-0074: a tool that fails is a card in [tool incidents](../tool-incidents.md), tools changed only at an epic's end; the nine cards moved there.
- [F23](../../backlog/features/E17-F23-the-path-of-a-story.md), E02's first feature, draft: EN-118 to EN-121, AUD-025, VAL-024; E02's closing stories NAM-003, REV-005 and RET-005 drafted, RET-005 holding what is parked until it.
- The limit of three subagents kept (ADR-0072).
- The improvement log from 113 rows to 32, the feedback log emptied into the habits sheet.

## 8. Playbook updates

- Six habits added to [Working with the agent](../../playbook/agent.md#lessons-from-the-product-owner), from F1 to F6.
- The starter kit (`docs/playbook/setup.md`): the tools called by their binary, a tool failure written as a card and fixed at the epic's end.
- [The agent's tools](../../tooling/agent-tools.md): the tools E17 asked for, with their signal.

## 9. Best practices read for agent-built projects

Read on 2026-10-18 through the local library.

| Source | What it says that we use |
| --- | --- |
| Claude Code docs, [Best practices](https://code.claude.com/docs/en/best-practices) (changed 2026-10-06) | one Claude writes the tests, another the code that passes them (P6); a reviewer in a fresh context flags only gaps of correctness or of the requirements, or it over-engineers (ADR-0064's audits) |
| Claude Code docs, [Agent teams](https://code.claude.com/docs/en/agent-teams) (changed 2026-10-16) | research and review are the strongest parallel uses; a test file is a right-sized task; each agent owns its files; tokens scale with the agents; 3 to 5 at once (P6, P7; the limit of 3 kept) |
| Anthropic Engineering, [Effective harnesses for long-running agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) | the agent tries to do too much at once; one feature at a time was critical; each session leaves a clean, committed state; specialized testing agents an open question (P3, P6) |
| semantic-release, [FAQ](https://semantic-release.gitbook.io/semantic-release/support/faq) | the version tracked by tags, written only into the published package; committing during a release adds complexity; a changelog repeating the GitHub release may be a link to it (P2) |
| npm Docs, [npm version](https://docs.npmjs.com/cli/v11/commands/npm-version) | `from-git` and `--no-git-tag-version`: a version written at publish time without a commit (P2) |
| GitHub Docs, [Events that trigger workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows) | a workflow run on a pushed tag, filtered by name (P2, option C) |
| V. Driessen, [A successful Git branching model](https://nvie.com/posts/a-successful-git-branching-model/), note of 2020 | explicitly versioned software keeps git-flow; `--no-ff` keeps each feature one visible group (P5) |
| GitHub Docs, [About merge methods](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/about-merge-methods-on-github) | a rebase merge rewrites the commits with new hashes; a rebased branch must be force-pushed with care (P5's push with lease) |
