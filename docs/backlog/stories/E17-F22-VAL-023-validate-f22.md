---
id: VAL-023
epic: E17
feature: F22
title: Validate F22
status: ready
points: 1
---

# E17 · F22 · VAL-023 — Validate F22

F22 is a tooling feature: the agent validates it on its demo page and checklist, and opens its pull request into `develop` without waiting (ADR-0072); then the Product Owner merges `develop` into `main` and publishes 0.3.0 by the button, the first release made by ADR-0086's flow, and E17 closes.

Two halves of F22's criteria no test of the repository can prove, left to this validation by AUD-024 (F-05): AC1's CI run on GitHub's runner, the green CI of `develop` → `main` its evidence; and AC4's rules on the docs, the run skill, the worker brief, `project-rules`, `AGENTS.md` and `docs/tooling/git-workflow.md` read again and shown to describe one agent at a time and a merge commit for every merge.

## Acceptance criteria

- Given the demo page, when it is read, then each criterion of F22 is shown with its evidence.
- Given the pull request `develop` → `main` of 0.3.0, when its CI ends, then it is green.
- Given the feature's criteria, when the validation ends, then the feature's pull request into `develop` is opened with the label `autonomous`.

## Tasks

One task = one commit, referenced as `VAL-023.Tn`.

- [x] T1 — Each criterion of F22 proven again, the checklist lines run, the demo page written (1 h)

## Criteria proven again (2026-10-18)

On this Mac (Apple M1 Pro), Claude Code 2.1.302, in the worktree of `feat/val-023-validate-f22` at `ec18baca`, `origin/develop` already merged (nothing new since the feature branch left it). `npm run check:all` passed in 116.2 s (load 6 to 10): `lint` 29.8 s, `test` 72.4 s, `check:rust` 115.9 s, `docs:build` and `playground:build` skipped, nothing changed in their inputs. Each criterion: its tagged tests, `cargo test -q --locked --workspace f22_ac` (18 tests) and `npx vitest run <its six files> -t "F22.AC"` (18 tests), then the command a person would run.

| Criterion | Command | Seen | Verdict |
| --- | --- | --- | --- |
| AC1 the CI of a pull request into `main` green on GitHub's runner, a failure of `cargo metadata` with its message | the 3 tests tagged `[F22.AC1]`; `env -i PATH="$PATH" HOME="$HOME" GITHUB_ACTIONS=true CI=true npx vitest run tests/type-check-with-lint.test.ts -t F22.AC1`; `CARGO_HOME="$(mktemp -d)" npx vitest run tests/rust-advisories.test.ts -t F22.AC1` | the three passed (`[F02.AC1]`'s walk 0.52 s, under its 20 s limit); the oxlint test passed with GitHub's variable and none of the agent's (`CLAUDECODE`, `AI_AGENT`), as on the runner; with an empty Cargo home the advisory test failed with `cargo metadata failed (exit 101): error: no matching package named …`, the crate `oxc_allocator` named, Cargo's message, as meant | holds on the Mac; **its run on GitHub's runner pending**: seen only in the CI of `develop` → `main` at the Product Owner's next review |
| AC2 the button, its tag and notes, no release branch nor commit; a push deploys only; `0.0.0-development`, no `CHANGELOG.md` | 12 Rust tests `f22_ac2_` (`tools/release/tests/`), 7 TypeScript `[F22.AC2]` (`tests/release-flow.test.ts`); `node -p "require('./package.json').version"`; `ls CHANGELOG.md`; `grep -n 'if: github' .github/workflows/release.yml`; `target/release/release notes`; `target/release/release check` on the draft, a good body and a bad one | the tests passed; `0.0.0-development`; no `CHANGELOG.md`; the `release` job only for `workflow_dispatch` on `main`, `rust-cache` and `docs` only for a push; 57 lines drafted from the 930 commits `origin/main` lacks, under `### Changed`, `### Added`, `### Removed` and `### Fixed`, each `[draft]`, the working tree left clean; `release check` named each `[draft]` line (exit 1), passed a body ending at `---` (exit 0), refused a `## 0.3.0` heading and a change without references, each at its line (exit 1) | holds; **the first press pending**: 0.3.0 by the button after the next review's merge into `main`; AUD-024 made the release from a copy of the history with a fake `gh` |
| AC3 tooling only a patch with its notice, nothing without a commit | 5 Rust tests `f22_ac3_`, `cargo test -q -p release f22_ac3_` | passed: tooling only gives a patch with the notice of no change, no commit since the last tag gives nothing, notes that disagree with the commits are refused | holds |
| AC4 a story from its feature branch, every merge a merge commit, a review after the feature, one agent at a time | 4 tests `[F22.AC4]` (`tests/story-automerge.test.ts`), `f22_ac4_` (`tools/worktree/tests/plan.rs`); `git log --first-parent --oneline origin/develop..origin/feature/f22-release-by-a-button --grep "Merge pull request"`; the run skill, the worker brief, `project-rules`, `AGENTS.md` and `docs/tooling/git-workflow.md` read again, with `git diff origin/develop` on them | the tests passed; five merge commits, #362 to #366, one per story; `gh pr merge … --auto --merge` in `story-automerge.yml`; the five pages describe one agent at a time (the run skill's rule, `AGENTS.md`'s row, the brief's "One story, then the end"), a merge commit for every merge and a review at each return; none keeps the rebase merges, the parallel stories nor two agents per story (the brief's "Several workers run at once" is gone, its heavy lock "across every worktree" kept for the main checkout beside a worktree) | holds |
| AC5 no epic closing, a `REV` at each return, a refinement `SP`, a feature over 4 stories split | 4 tests `[F22.AC5]` (`tests/backlog.test.ts`); `ls docs/backlog/_*.md` | passed; the templates `_audit`, `_checkpoint`, `_epic`, `_feature`, `_review`, `_spike`, `_story`: no closing's | holds |

## F22's lines in E17's checklist

Drafted by `target/release/val-prepare . F22`, then written as 12 lines (`playground/checklists/epic-17-f22-lines.ts`, `?checklist=E17`), each tried here on 2026-10-18 with the output it names, but lines 3 and 9, the CI of `develop` → `main` and the button, which only GitHub shows and which say so: "pending until that review". Six lines carry decisions to confirm: oxlint's format named and every crate fetched (EN-116), `release prepare` gone, a text ending at `---` and the version counting the library's and the playground's commits only (EN-117), Dependabot's bodies left out (AUD-024). The page's test lists E17's 60 lines and names each command, output and decision of F22's; `57 lines` changed to `58 lines`, `Decision to confirm (AUD-024)` to `Decision (AUD-024)` and one "pending until that review" removed made three of its cases fail, then the file restored passed. Tested in happy-dom only: a subagent has no browser. As at VAL-022, the first `npm run fix` turned `export type * from "./checklist-line"` into `export *`: set back by hand.

## Decisions of the validation (2026-10-18)

- **Validated by the agent**: F22 is a tooling feature (ADR-0072); the Product Owner may try the lines that concern them.
- **Two halves pending, not passed**: criterion 1's run on GitHub's runner and this story's second criterion, the CI of `develop` → `main` green, are seen only at the Product Owner's next review; so is criterion 2's first press of the button. The demo page and the checklist say so.
- **The six decisions to confirm** are on the demo page and in the checklist.

## Result of the validation (2026-10-18)

| Line | Seen by | Result |
| --- | --- | --- |
| 1, 2, 4 to 8, 10 to 12 | the VAL worker | as written, each tried on this Mac |
| 3, 9 | — | pending until the next review: the CI of `develop` → `main`, then the button |

| Criterion of VAL-023 | Result |
| --- | --- |
| the demo page shows each criterion with its evidence | done: `docs/guide/f22-release-by-a-button.md` |
| the CI of `develop` → `main` of 0.3.0 green | pending until the next review |
| the feature's pull request into `develop` opened with `autonomous` | the main thread's, after this story's `done` commit |

## For the main thread

Rule-docs: none affected, a tooling validation, no rule of `docs/domain/` changed.

- The checklist files are the epic's (`playground/checklists/epic-17-f22-lines.ts`, `epic-17-checklist.ts`), written by this story as VAL-022 wrote F21's: a `VAL` branch is not a worker's for the pre-push hook. `checklists.ts` already registered E17.
- To write at the `done` commit: VAL-023 and F22 `done` (`target/release/backlog-status`), which sets the feature file, E17's table and the indexes.
- `AGENTS.md`'s State row of E17 is updated by this story.
- `docs/guide/README.md` still says a page is written "of each epic (`REV`, epic report)", and `docs/guide/e17-fast-tooling-with-oxc-and-rust.md` counts "the four features of E17": both older than ADR-0086 and F22, for the next review.
- The worker brief and `project-rules` this worker was started with are `develop`'s, older than F22's (the brief still read "Several workers run at once"): an agent type's preloaded skills come from outside the worktree, so they follow F22 once it is merged into `develop`.

### Journal

- `#run` VAL-023: F22 validated by the agent (ADR-0072): its five criteria proven again (18 Rust tests `f22_ac…`, 18 TypeScript tests `[F22.ACn]`, `check:all` 116.2 s), 12 lines in E17's checklist, the demo page `docs/guide/f22-release-by-a-button.md`; `release notes` drafts 57 lines from the 930 commits `main` lacks. Pending until the next review: criterion 1's run on GitHub's runner, the CI of `develop` → `main` and the first press of the button, 0.3.0. Six decisions for the Product Owner to confirm: oxlint's format named and every crate fetched (EN-116), `release prepare` gone, a text ending at `---`, the version counting the library's and the playground's commits only (EN-117), notes that disagree refused and Dependabot's bodies left out (AUD-024).
