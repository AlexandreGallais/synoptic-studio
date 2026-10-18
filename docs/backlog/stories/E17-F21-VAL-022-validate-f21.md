---
id: VAL-022
epic: E17
feature: F21
title: Validate F21
status: ready
points: 1
---

# E17 · F21 · VAL-022 — Validate F21

F21 is a tooling feature: the agent validates it on its demo page and checklist, and opens its pull request into `develop` without waiting; the Product Owner may try the lines that concern them (ADR-0072).

## Acceptance criteria

- Given the demo page, when it is read, then each criterion of F21 is shown with its evidence: the tool, its test, and its time before and after.
- Given the checklist of E17, when the agent runs its F21 lines, then each passes and is ticked with its date.
- Given the release notes of 0.2.0 drafted by our tool beside release-please's (EN-112), when the page shows them, then the Product Owner can compare both.
- Given the feature's criteria, when the validation ends, then the feature's pull request into `develop` is opened with the label `autonomous`.

## Tasks

One task = one commit, referenced as `VAL-022.Tn`.

- [x] T1 — Each criterion of F21 proven again, what was seen recorded; F21.AC9 worded as AUD-023 left it (1 h)
- [x] T2 — F21's lines in E17's checklist, `?checklist=E17`, each tried (1 h)
- [x] T3 — The demo page, `docs/guide/f21-agent-tools-in-rust.md` (1 h)
- [x] T4 — The result of the validation recorded (0.5 h)

## Criteria proven again (2026-10-18)

On this Mac (Apple M1 Pro), Claude Code 2.1.302, in the worktree of `feat/val-022-validate-f21` at `c24c0f9a`, `origin/develop` merged (already up to date), the release binaries rebuilt by `git-sequences sync`. Each criterion: its tagged tests (`f21_acN_…` in Rust, `[F21.ACn]` in TypeScript), then the command a person would run; a time is the median of 20 calls by the binary's path (5 for a tool over 100 ms), a timer around each process, load 2.8 to 3.7. Criterion 9 is read as AUD-023 left it: its check now guards `[url]`, `[include]`, `[includeIf]` and `[extensions]` too (`6e093809`), and the feature file says so since this task.

| Criterion | Command | Seen | Verdict |
| --- | --- | --- | --- |
| AC1 every hook a Rust binary, called by `.claude/settings.json` | `cargo test -q --locked --workspace f21_ac1_`; `ls .claude/hooks`; the `command` lines of `.claude/settings.json`; `guard-bash`, `track-tool-call` and the status line timed | 8 tests passed (`hooks`, `stale-binaries`), among them `f21_ac1_every_hook_and_the_status_line_call_a_hooks_binary_by_its_path`; `.claude/hooks` no longer exists; the 11 hook commands and the status line name `target/release/<binary>`; `guard-bash` 3.0 ms, `track-tool-call` 3.0 ms; the status line 23 ms by its path, 37 ms through `sh -c` as the settings run it, of which `run-totals status` 15 ms: a run is open (7 h 56, 980 million tokens), which EN-104's 10 ms did not count; 2.3 % of a core at one draw a second | holds |
| AC2 every script a Rust binary but `default-font-metrics.ts` | `cargo test -q --locked --workspace f21_ac2_`; `ls scripts`; `target/release/metrics` timed | 11 tests passed (`repo-tools`, `git-sequences`, `metrics`); `scripts/` holds `default-font-metrics.ts` and its test only; `metrics` 0.51 s, its cache warm, against 0.46 s at EN-107 on fewer transcripts | holds; see the open point on `--release` |
| AC3 the library's server on `rmcp`, same tools, answers and store | `cargo test -q --locked --workspace f21_ac3_`; `grep -A1 'name = "rmcp"' Cargo.lock`; `target/release/library-mcp` fed an `initialize`; `target/release/library search sqlite fts5` | 3 tests passed (`library`: the three tools and the command line answer as the TypeScript ones did); `rmcp` 3.5.1; the answer `{"protocolVersion":"2025-06-18","capabilities":{"tools":{}},"serverInfo":{"name":"library",…}}` in 3.9 ms; a search in 4.9 ms; `.mcp.json` starts it through `cargo run`, about 86 ms once a session (ADR-0078, decision 4) | holds |
| AC4 every dev package kept with its reason or rewritten | `cargo test -q --locked --workspace f21_ac4_`; `npx vitest run tests/ci-caches.test.ts -t "F21.AC4]"`; `devDependencies` of `package.json`; `target/release/dependencies unused`; `target/release/secret-scan` timed | 49 Rust tests passed (`git-hooks`, `deps`, `secret-scan`, `format`) and 9 TypeScript; 13 dev packages, each a row of the feature's table marked kept (38 at E17's start); `dependencies` `{}`; `dependencies unused` exit 0, nothing printed; `secret-scan` on every tracked file 112 ms, exit 0, against 970 ms for secretlint | holds |
| AC5 nothing of the removed tools left | `npx vitest run tests/package.test.ts -t "F21.AC5]"`; `git ls-files` searched for `eslint`, `prettier`, `husky`, `lint-staged`, `commitlint`, `secretlint`, `knip`, `release-please`, `.ncurc` | 1 test passed; found only what EN-113 kept with its reason (its section The search): ADR-0013 and the stories of F19, `tests/eslint-rules-accounted.test.ts` and `tests/eslint-rules.txt`, which prove ESLint's rules accounted for, and the 33 fixtures of secretlint's preset under `tools/secret-scan/tests/fixtures/preset/`, the cases the scan was ported from (ADR-0076) | holds |
| AC6 the release by our own tool | `cargo test -q --locked --workspace f21_ac6_`; `npx vitest run tests/release-flow.test.ts -t "F21.AC6]"`; `target/release/release prepare --date 2026-10-18` in a scratch clone of `develop`, then `release check` | 15 Rust and 5 TypeScript tests passed; `v0.2.0 → 0.3.0 on release/0.3.0: 57 line(s) drafted in CHANGELOG.md`, the entry headed `## [0.3.0] - 2026-10-18` under `### Changed`, each line marked `[draft]` with its pull request; `release check` then names the 57 lines still to rewrite, exit 1, as meant; 0.2.0 drafted again by EN-112: 163 change lines of release-please against 14 | holds; the first real publication is 0.3.0's |
| AC7 each tool records its use, `metrics` lists them | `cargo test -q --locked --workspace f21_ac7_`; `target/release/metrics`, section Tools | 13 tests passed (`tool-use`, `metrics`, `run-totals`); `.git/metrics/tool-uses.jsonl` holds 68,596 uses; the section lists each tool and subcommand with its calls, failures, median and slowest: `guard-bash` 61,388 calls, 0 failed, 0 ms median; `run-totals status`, a poll, is not among them (`docs/tooling/metrics.md`) | holds |
| AC8 one row or one section of a page | `cargo test -q --locked --workspace f21_ac8_`; `target/release/page REF-CC-HOOKS` | 11 tests passed (`page`); the row with its table's header, 2,568 characters of the 212,137 of `docs/references.md`, in 3.0 ms | holds |
| AC9 the `.git/config` check, Git without the inherited `GIT_*` | `cargo test -q --locked --workspace f21_ac9_` | 7 tests passed (`repo-tools`, `test-support`): a lane writing `[core]`, `[user]`, `[commit]`, `[url]`, `[include]`, `[includeIf]`, `[extensions]` or a remote's URL fails naming its lines, a branch tracked passes; `GIT_DIR` set by the caller does not reach the command; no Rust test starts Git but through `test-support` | holds |

The whole workspace, `cargo test -q --locked --workspace`: 1,554 tests passed, none failed. The 16 TypeScript tests tagged `[F21.ACn]` passed. Run with `--release`, `f21_ac2_reports_the_main_checkouts_work_from_a_worktree_its_findings_included` fails: a release build of `metrics` records its own use in the test's repository, so its second report holds one row more under Tools than its first; `check:all` runs the tests in a debug build (`rust:coverage`), which records nothing (AUD-023, verifier 9), and passes. A row for the improvement log, below.

`check:all` on the branch after T2, `npm run check:all`: every check passed in 74.9 s (load 6 to 16), `test` 52.0 s, `docs:build` 59.2 s, `check:rust` 74.6 s, against 170 to 217 s at E17's start.

## F21's lines in E17's checklist

Drafted by `target/release/val-prepare . F21`, then written as 12 lines (`playground/checklists/epic-17-f21-lines.ts`, `?checklist=E17`), each tried here on 2026-10-18 with the output it names: ten commands run in this worktree, the guard's two answers seen on this story's own calls (a `git log … | head -3` after the brief's `cd` let through, a `git status; ls` refused with `A worktree refuses a chained Git command`), and the release drafted in a scratch clone of `develop` with this branch's binary, since `develop` holds no release tool until F21 is merged. Five lines carry F21's decisions to confirm; the sixth, the story worker kept on Opus, is F20's line 2 already. The page's test lists the 48 lines of E17 and names each command, output and decision of F21's; a line changed from `57 line(s) drafted` to `58 line(s) drafted` and `Decision to confirm (EN-102)` to `Decision (EN-102)` made two of its cases fail. `val-prepare` wrote the barrel's new line last, and the first `npm run fix` turned `export type * from "./checklist-line"` into `export *`: set back by hand.

## Decisions of the validation (2026-10-18)

- **Validated by the agent**: F21 is a tooling feature, so the agent validates it without waiting for the Product Owner, who may try the lines that concern them (ADR-0072, decided on 2026-10-17). The pull request into `develop` is the main thread's.
- **F21.AC9 worded as its check now is**: AUD-023 extended the `.git/config` check to `[url]`, `[include]`, `[includeIf]` and `[extensions]` (`6e093809`) and left the criterion's new wording to the main thread, which asked this story to apply it.
- **The status line read as held at 23 ms**: EN-104's 10 ms was measured with no run open; with a run open the line adds `run-totals status`, 15 ms, still 2.3 % of a core against 24.5 % before.
- **The six decisions to confirm** are shown on the demo page and in the checklist: `AGENT_LIBRARY_FILE` (ADR-0077), the binaries by their path (ADR-0078), the release branch from `develop` (ADR-0079), one Git command through read-only filters (EN-104), `run-totals status` not recorded (EN-102), the story worker on Opus (EN-066).
- **The release tool's publication not tried**: `release publish` runs on `main` after a merge; 0.3.0 will be its first run, its time read then (EN-112).

## Result of the validation (2026-10-18)

| Line | Seen by | Result |
| --- | --- | --- |
| 1 to 12 | the VAL worker | as written, each tried on this Mac; the decisions of lines 1, 2, 3, 5 and 10 left to the Product Owner |

## For the main thread

Rule-docs: none affected, a tooling validation, no rule of `docs/domain/` changed.

The `done` commit, made by this worker as the main thread asked, sets VAL-022 and F21 `done` and updates F21's line of the State table in `AGENTS.md`; the journal entry below and the improvement row are moved by `story:finish`.

### Journal

- `#run` VAL-022: F21 validated by the agent (ADR-0072): its nine criteria proven again (1,554 Rust tests, 16 TypeScript tests tagged `[F21.ACn]`), F21.AC9 worded as AUD-023 extended its check, 12 lines in E17's checklist, the demo page `docs/guide/f21-agent-tools-in-rust.md`. `guard-bash` 58.8 → 3.0 ms, the status line 180 → 23 ms with a run open (a quarter of a core → 2.3 %), the library's server start 86 → 3.9 ms, `metrics` 9.4 → 0.51 s, the secret scan 970 → 112 ms, a commit's hooks 716 → 356 ms, `check:all` 170–217 s at E17's start → 74.9 s, dev packages 38 → 13; the notes of 0.2.0 from 163 change lines to 14. Six decisions for the Product Owner to confirm: `AGENT_LIBRARY_FILE` (ADR-0077), the binaries by their path (ADR-0078), the release branch from `develop` (ADR-0079), one Git command through read-only filters (EN-104), `run-totals status` not recorded (EN-102), the story worker on Opus (EN-066).

### Improvements

| 2026-10-18 | VAL-022 | `f21_ac2_reports_the_main_checkouts_work_from_a_worktree_its_findings_included` fails under `cargo test --release`, the way VAL-021 ran its tests, since a release build of `metrics` records its own use in the test's repository and its second report differs from its first; `check:all` runs a debug build and passes. Removing `tool-uses.jsonl` before the second report, or comparing all but its Tools section, would make the test hold in both profiles | tool |
