---
id: AUD-024
epic: E17
feature: F22
title: Audit F22
status: done
points: 1
---

# E17 · F22 · AUD-024 — Audit F22

Audit F22 in depth and the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given F22's criteria, when the audit ends, then each is traced to a test titled `f22_acN_…` or `[F22.ACn] …`, broken once on purpose and seen failing.
- Given the release tool, when audited, then a release made from a copy of the repository's history gives the version and the notes ADR-0086 describes.

## Tasks

One task = one commit, referenced as `AUD-024.Tn`.

- [x] T1 — A to G. The auditor's findings verified, the findings table; the tests of F-03 and F-04, failing (1 h)
- [x] T2 — `release publish` refuses notes that disagree with the commits and leaves out Dependabot's bodies, `docs/tooling/releases.md` with it (1 h)
- [x] T3 — The other findings fixed, the criteria tagged, the mutation spot-checks, the release from a copy of the history, H (1 h)

## Findings

The `auditor` reviewed sections A to E alone, in the background (ADR-0086, decision 1); each finding below was reproduced before it was recorded.

| Id | Finding | Evidence | Result |
| --- | --- | --- | --- |
| F-01 | `/audit` step 2 starts four reviewers in parallel, and `CLAUDE.md` lists the `doc-editor` among the second opinions with no condition, against one agent at a time and specialists only where their independence pays | `.claude/skills/audit/SKILL.md:14`, `CLAUDE.md:15`, ADR-0086 decision 1 | reproduced; for the main thread, below: a subagent changes neither `CLAUDE.md` nor the skills on another agent's request, so the two replacements are written there |
| F-02 | an epic's refinement `SP`, its branch from `develop`, is refused by `story-automerge.yml`, which lets into `develop` only a feature branch or a `REV`: `story:finish` waits 3 minutes and fails | `.github/workflows/story-automerge.yml:33-35` (`^(feature/f[0-9]{2}-\|[a-z]+/rev-[0-9]{3}-)`), `.claude/skills/spike/SKILL.md:13` | reproduced; the docs only: the tool incident card in `docs/process/tool-incidents.md` (T3); the spike and run skills' sentences, saying the Product Owner merges an epic's `SP`, for the main thread, below; whether the auto-merge should take it, a question for the next review |
| F-03 | the version comes from the commits and the notes from the bodies, and nothing compares them: a range of the tooling only with a body listing changes is published as a patch with those changes, a `feat(model)` range whose bodies hold only a notice as a minor version with no change, both against `docs/tooling/releases.md` (The version) | `tools/release/src/publish.rs:119-124`; the AC3 test fed the notice to both bodies, from the code's own constant, so it held whatever the commits | reproduced by `f22_ac3_refuses_notes_that_disagree_with_the_commits`, failing (T1); fixed in T2 (`26d1a48d`): `publish` compares the bodies with the commits' lines before anything is made, `docs/tooling/releases.md` says so; the AC3 test writes the notice by hand (`3fd9effe`) |
| F-04 | a Dependabot security update merged into `main` (`docs/tooling/ci.md`) puts Dependabot's body on the first-parent line, which `release check` refuses: the button fails until it is edited | `docs/tooling/ci.md:58`, `tools/release/src/history.rs:59-83`, `tools/release/src/publish.rs:56-61` | reproduced by `f22_ac2_leaves_out_the_body_of_a_dependabot_pull_request` and `reads_the_head_branch_of_a_merge_its_owner_left_out`, failing (T1, `3fd9effe`); fixed in T2 (`26d1a48d`): `merged_branch` reads the head branch, `publish` skips `dependabot/…`; `docs/tooling/releases.md` and `docs/tooling/ci.md` say so |
| F-05 | no test titled for AC1 nor AC5, and AC4's half on the docs untested: the backlog test tracing a done feature's criteria would fail at F22's `done` | `grep -rn -e f22_ac -e 'F22\.AC'` lists AC2, AC3, AC4 only | fixed in T3: `[F22.AC1]` added to the three CI tests, their old tags kept (`tests/type-check-with-lint.test.ts`, `tests/rust-advisories.test.ts`, `playground/mount/mount-playground.test.ts`), `[F22.AC5]` to EN-123's four tests of `tests/backlog.test.ts`; AC1's run on GitHub and AC4's half on the docs written in VAL-023 as its evidence |
| F-06 | `/spike` starts the `functional-analyst` and the `math-expert` in parallel, by default | `.claude/skills/spike/SKILL.md:17` | reproduced; for the main thread, below, as F-01 |
| F-07 | EN-122 and EN-123 `done` with tasks unticked, their commits made | EN-122's T2 and T3, EN-123's T1 and T2; `ad1a1109`, `6914314d`, `8f99912d`, `506e432c` | fixed in T3: ticked |
| F-08 | AUD-024's criteria and task cite ADR-0083, superseded by ADR-0086 | this file, lines 17 and 23 before T1 | fixed in T1 |
| F-09 | ADR-0086's Consequences name `release prepare`, which EN-117 removed | `docs/adr/0086-one-story-at-a-time-into-develop-the-release-by-a-button.md:35`; `tools/release/src/main.rs` has `notes`, `check`, `publish` | fixed in T3: the ADR is accepted, so a note in its row of `docs/adr/README.md` |
| F-10 | the old flow's words left: a release "once EN-117 lands", a release commit `chore: release x.y.z`, a "changelog", the epic retrospective and an epic's end for the method's point, ADR-0086's "decision 8" for a review | `docs/tooling/git-workflow.md:17,74`, `docs/tooling/README.md:12`, `docs/conventions/audit.md:54`, `docs/conventions/rule-catalogue.md:3`, `.github/workflows/story-automerge.yml:3` (decision 4 holds the review) | fixed in T3; VAL-023's "ADR-0083's flow" too. Left: the comment of `tools/git-hooks/src/message.rs:335` names the release commit, a tool's file, for the next review (below) |
| F-11 | EN-122.T2's tests and code in one commit, twice, against tests first in their own commit | `2f662b57`, `ad1a1109` | reproduced; no action on a pushed history; a row for the next review's method point, under For the main thread |
| AC1 of AUD-024, criteria traced | AC1 `[F22.AC1]` (3 tests), AC2 `f22_ac2_…` and `[F22.AC2]` (`tools/release/tests/`, `tests/release-flow.test.ts`), AC3 `f22_ac3_…`, AC4 `[F22.AC4]` (`tests/story-automerge.test.ts`) and `f22_ac4_…` (`tools/worktree/tests/plan.rs`), AC5 `[F22.AC5]` (4 tests) | `grep -rn -e f22_ac -e 'F22\.AC'` | each broken once and seen failing, then restored: AC5 by `MOST_STORIES` 4 → 3, `MOST_TASKS` 3 → 2, `RET` 5 → 3 and `_review.md` among the closing templates (4 tests failing); AC1 by `--format agent` dropped, under `GITHUB_ACTIONS=true` without the agent's variables (`[F19.AC6] [F22.AC1]` failing; with `CLAUDECODE` set oxlint picks the agent format, so it passes, as EN-116 found), its two others broken by EN-116; AC4 by `--squash` for `--merge` in `story-automerge.yml` (2 tests failing); AC2 and AC3 by the mutations below |
| AC2 of AUD-024, a release from a copy of the history | the repository cloned into the scratch folder, `main` (behind it `v0.2.0`) merging `feature/f22-release-by-a-button` as `Merge pull request #999 from o/develop`; the body `release notes HEAD^1` drafted, 57 lines, its `[draft]` marks removed, `release check` passing; `release publish` run there with a fake `gh` first on the `PATH` | the fake's log: `pr view 999`, `run list … --commit 7a67cf5e…` (the merged head), `release create v0.3.0 --target <the merge> --title 0.3.0`; the notes the body's text up to its `---` | as ADR-0086 decisions 6 and 7 say: 0.3.0, a feat since 0.2.0 before 1.0.0, on `main`'s head, its notes the body, no commit nor tag made. The same copy with a body holding only the notice: refused, `commits since v0.2.0 change the library or the playground, but no pull request's body lists a change` (F-03) |
| A4, `npm run mutate -- --all` on `publish.rs`, `body.rs`, `history.rs`, `commit.rs`, `version.rs` (`-- cargo test -q -p release`) | 31 caught, 0 survived | the tool's output, 2026-10-18 | none |
| A4, `npm run mutate -- --all` on `check.rs`, `draft.rs`, `feature_text.rs`, `backlog.rs`, `manifest.rs` | 21 caught, 4 survived: `check.rs:103` `changes += 1` → 2, `check.rs:121` `notices += 1` → 2 (each count only compared with 0), `check.rs:162` the last line's default 1 → 2 (read only when a text has no line, where `close` has no group to name), `draft.rs:139` `unwrap_or(0)` → 1 (the rank of lines of no feature, already last by its first field and equal among themselves) | the tool's output | no action: four equivalent mutants, no input tells them apart |
| A4, by hand on the code of T2 (`mutate` has no Rust mutation dropping a `!`) | caught: `!text.changes.is_empty()` → `text.changes.is_empty()` (4 publish tests), `!drafted.is_empty()` → `drafted.is_empty()` (4), `!branch.starts_with` → `branch.starts_with` (4), `merged_branch`'s owner `[^/\s]+/` dropped (`reads_the_head_branch_…` and the Dependabot test). Survived: `"dependabot/"` → `"dependabot"`, and `merged_branch`'s final `$` dropped | `cargo test -p release`, each restored from a copy, `git status` clean | no action: Dependabot names its branches `dependabot/<ecosystem>/…` and the pre-push hook refuses any branch of ours outside `<type>/<id>-<topic>`, so no head starts with `dependabot` without its `/`; a branch name holds no space (`git check-ref-format`) and GitHub's subject ends with it, so the `$` changes no subject GitHub writes |
| B, F22's sources | `REF-SEMVER` read again through the library (Summary, the 0.y.z FAQ, a dependency update compatible), its date 2026-10-17 → 2026-10-18; `REF-GH-WORKFLOW-DISPATCH`, `REF-SEMANTIC-RELEASE-FAQ`, `REF-COMMON-CHANGELOG`, `REF-KEEP-A-CHANGELOG`, `REF-GH-JOB-PERMISSIONS`, `REF-OXLINT-OUTPUT-FORMATS`, `REF-CARGO-FETCH`, `REF-NPM-VERSION` verified today by F22's stories | `target/release/page REF-…` | done |
| C, provenance | `tools/release/` written from Common Changelog, Keep a Changelog and SemVer, no package ported (EN-112); T2's code is the project's | `NOTICE`, the modules' headers | none |
| D, duplicates | Dependabot is recognized twice: by its head branch here, by `Signed-off-by: dependabot[bot]` in `tools/git-hooks/src/message.rs` | `grep -rn dependabot tools/*/src` | no action: one reads a merge commit, the other a commit message, with nothing to share |
| E, consistency | code, `docs/tooling/releases.md` and ADR-0086 agree after T2 and T3; no rule of `docs/domain/` touched: F22 is tooling | F-01 to F-10 | the configuration's part for the main thread |
| G, for F23 | F23's EN-118 makes `tool-incidents.md` a shared log; the question of F-02 (an epic's `SP` auto-merged or merged by the Product Owner) for the next review; `mutate` with no Rust mutation dropping a `!` (H, below) |  | for F23's refinement and the next review |
| H, size | `tools/release/` 1,522 lines in 12 files, `draft.rs` the longest (333), `publish.rs` 192; its tests 1,695 lines | `wc -l` | nothing yet |
| H, tools | no trigger of a "considered for later" row has happened (no npm publication, no stable API, stryker-js 6210 open) | `docs/tooling/dependencies.md` | nothing yet |
| H, fitness functions | the backlog test tracing a done feature's criteria to tagged tests would have failed at F22's `done` (F-05): it holds; `release publish` now holds the agreement of the notes and the version (F-03) |  | done in this audit |
| H, patterns | Dependabot recognized in two places (D) |  | nothing yet |
| H, agent configuration | `AGENTS.md` and `CLAUDE.md` 158 lines together; the parallel reviewers of `/audit` and `/spike` (F-01, F-06) | `wc -l` | for the main thread, below |
| H, process | tests and code in one commit twice in EN-122 (F-11); `mutate`'s Rust mutations lack the dropped `!` that three of T2's hand mutations needed |  | improvement log, below |

## For the main thread

### Improvements

| 2026-10-18 | agent | Tests and code of one task in one commit, twice in EN-122.T2 (`2f662b57`, `ad1a1109`), against ADR-0084's tests first in their own commit: a point for the next review's method (AUD-024, F-11) | process |
| 2026-10-18 | agent | `mutate` has no Rust mutation that drops a `!`: three of AUD-024's hand mutations on `release publish` were of that kind; the comment of `tools/git-hooks/src/message.rs:335` still names the release commit `chore: release x.y.z` (AUD-024) | tool |
