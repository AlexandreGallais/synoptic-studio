---
id: AUD-031
epic: E17
feature: F29
title: Audit F29
status: done
points: 1
---

# E17 · F29 · AUD-031 — Audit F29

Audit F29 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F29's criteria, when the audit ends, then each is traced to a test titled `[F29.ACn] …` or `f29_acn_…`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — The findings, and the tests of F3, F4, F7 and F8, seen failing (1 h)
- [x] T2 — The findings fixed, the criteria traced (1 h)
- [x] T3 — ADR-0089: the five Git hooks and the rebuild after a pull (30 min)

One task = one commit, referenced as `AUD-031.T1`.

## Findings

Reviewed by the `auditor` alone (ADR-0086), each finding reproduced before it is recorded.

| Id | Check | Result | Evidence | Action |
| --- | --- | --- | --- | --- |
| F1 | E3 — `main`'s ruleset lacks the `Rust coverage` check that `ci.md` lists | reproduced: its required checks are the five other lanes, the history job, `Dependency review` and `Analyze` | `gh api repos/…/rulesets/24687556`, 2026-10-18 | no file change: the main thread adds the check to the ruleset when F29 merges into `develop` |
| F2 | E1 — ADR-0080, decision 1, lists three hooks, `.githooks/` holds five; ADR-0078, decision 2 and its third consequence, name three places that rebuild, the post hooks a fourth; ADR-0087's consequence says "the five jobs", the matrix holds six lanes | reproduced | `ls .githooks`; `ci.yml`'s matrix | ADR-0089 supersedes those points, its consequences correcting ADR-0087's count; the ADR index's statuses (T3) |
| F3 | A4 — `[F29.AC1]`'s factor never shown to fail: `LIMIT_FACTOR` 8 passes, every limit of the repository being over 8 times its measure; `timingLimits` and `limitFault` never given a made-up case | reproduced | `tests/fitness.test.ts`, `LIMIT_FACTOR = 8`, the suite green | a made-up file of five limits (three times its measure, no `alone:`, four times exactly, an undeclared constant, a constant under four times), each fault expected with its `file:line`; seen failing on `LIMIT_FACTOR = 8`, on `<` made `<=` and on the constant left unread (T1) |
| F4 | E3 — four limits of `playground/` (20 s each) have no measure, and run in the same loaded lane, outside `[F29.AC1]`'s scan | reproduced: `timedTestFiles` reads `src/` and `tests/` only | `playground/mount/mount-playground.test.ts:80`, `playground/fonts/import-font.test.ts:230`, `playground/guided-steps/epic-15/f15-steps.test.ts:280`, `:456` | `playground` read by `timedTestFiles`, the count 21: `[F29.AC1]` fails on the four (T1); each measured alone and written (T2) |
| F5 | E3 — the first pull bringing F29 runs the old `git-hooks`, which has no `post-merge`: it prints its usage and builds nothing, so that pull leaves the binaries stale | reproduced by reading: `.githooks/tool.sh` runs `target/release/git-hooks` whenever it exists, and the old `main.rs` answers an unknown command with its usage | `.githooks/post-merge`, `.githooks/tool.sh` | `git-workflow.md` says to run `npm run rust:build` once after the pull that brings these hooks; VAL-030 notes it (T2) |
| F6 | E3 — `post-merge` does not run after a merge stopped on a conflict and finished by `git commit` | reproduced by reading: "not executed if the merge failed due to conflicts" (`REF-GITHOOKS`); `git commit` runs the commit hooks only | `REF-GITHOOKS` | documented in `git-workflow.md`, with what still names the stale binaries: `npm run rust:fresh` and the `session-state` hook (T2) |
| F7 | A3 — nothing proves the hook "exits 0" on a failed build: Git ignores a post hook's exit code, so `f29_ac3_a_failed_build_…`'s successful `git pull` passes whatever the hook returns | reproduced: `run_after_pull` made to return 1 on a failure, every test passes | `tools/git-hooks/tests/rebuild.rs` | `f29_ac3_exits_0_from_the_binary_after_a_failed_build`: the binary run by hand after a Rust pull, a failing `cargo` stub, `post-merge` and `post-rewrite rebase`, exit 0 and the message asserted; seen failing on that mutation (T1) |
| F8 | A3 — `check-all`'s `CI` guard never run with a base known: every test of the CI has no remote | reproduced: the guard `env::var_os("CI").is_some()` made `false`, every test passes | `tools/repo-tools/src/bin/check-all.rs` | `f29_ac2_runs_every_lane_in_the_ci_even_with_a_base_known`: a story of docs alone, its base on `origin`, `CI=1`: every lane reported, no `skipped` line; seen failing on that mutation (T1) |
| F9 | D1 — the release build's arguments written four times (`git-hooks`, `git-sequences`, `story_finish`, `worktree-new`), and `git_hooks::rebuild::built_times` repeating `stale_binaries`' path of a binary and its file time | reproduced | `tools/git-hooks/src/rebuild.rs`, `tools/git-sequences/src/main.rs`, `tools/repo-tools/src/story_finish.rs`, `tools/worktree/src/bin/worktree-new.rs` | one `RELEASE_BUILD` and one public `binary_written` in `stale-binaries`, used by the four (T2) |
| F10 | E3 — `docs/tooling/agent.md` gives the binaries' rebuilds without the post hooks | reproduced | `docs/tooling/agent.md`, the hooks' paragraph | the post hooks named (T2) |
| F11 | E3 — `AGENTS.md` says a crate is "checked by `check:rust`", the coverage a lane of its own since EN-130 | reproduced; the `project-rules` skill does not repeat it | `AGENTS.md`, Never | "by `check:rust` and `rust:coverage`" (T2) |
| F12 | E3 — the RustSec database is read by `check:rust`'s advisories and is no Rust input: a new advisory against an unchanged `Cargo.lock` passes a story's push that skipped the lane | reproduced by reading | `RUST_INPUTS`; `npm run rust:advisories` | the exception written in `commands.md`: the database lives outside the repository, and the CI into `main` runs the lane every time (T2) |
| F13 | E4 — EN-129, EN-130 and EN-131 done with their tasks unticked | reproduced | the three story files | ticked (T2) |
| F14 | E4 — EN-129's criterion 2 asks the load average written in the story's report; its five runs are only summed up in PR #394 | reproduced | PR #394's body; EN-129's story | the five runs and their load averages copied into EN-129's story (T2) |
| C1 | A1, A2 — F29 holds no formula: its code is a test helper, `check-all`'s lanes and the post hooks | no finding | `tests/fitness.test.ts`, `tools/repo-tools/src/check_all.rs`, `tools/git-hooks/src/rebuild.rs` | — |
| C2 | A4 — `rebuilt`: `earlier != time` made `==` | caught | `names_the_binaries_written_again` and the three `f29_ac3_…_rebuilds_…` fail | restored |
| C3 | A4 — `after_pull` without its check of `Cargo.toml` | caught | `a_checkout_without_a_workspace_builds_nothing` fails | restored |
| C4 | A4 — `skipped_groups`: `!touches_rust` made `touches_rust` | caught | `f29_ac2_skips_the_rust_lanes_only_when_…` and `f29_ac2_skips_the_rust_lanes_for_a_story_…` fail | restored |
| C5 | A4 — `RUST_INPUTS`: `tools/` made `tool/` | caught | five `f29_ac2_…` fail, among them `…_counts_a_new_crate_…` | restored |
| C6 | A5 — properties: F29 adds no `math` nor `geometry` function | no finding | `src/` unchanged by F29 | — |
| C7 | B — `REF-GITHOOKS` read again through the library for `post-merge` after a conflict, 2026-10-18 | done | githooks, `post-merge`: "is not executed, if the merge failed due to conflicts"; the row already says so | — |
| C8 | C, D — no third-party code; F9 the only duplicate | no finding | the crates F29 changed | — |
| C9 | `[F29.AC1]` traced | caught | `[F29.AC1] names the file and the line …` fails on F3's three mutations; `[F29.AC1] gives every limit …` fails on the four limits of `playground/` (T1) | fixed by T2: each measured alone, the median of 5 runs of its file (421, 788, 773 and 895 ms; 20 s is 22 to 47 times them) |
| C10 | `[F29.AC2]` traced | caught | C4, C5 and F8's mutation | restored |
| C11 | `[F29.AC3]` traced | caught | C2, C3 and F7's mutation | restored |
| C12 | F — `check:all` green on the head | run by the pre-push hook | the push of T3 | — |

## G. Next feature

F30 runs the Rust tests on Linux in a `develop` CI. That CI sets `CI`, so `check-all`'s lanes there run whatever the branch changed (F8), and no post hook runs on a runner's fresh checkout: it builds its binaries itself. The RustSec database, outside the Rust inputs (F12), is fetched by each run, as `[F28.AC3]` checks. Sources: those of F30's spike, `REF-GH-WORKFLOW-DISPATCH`, `REF-GH-CACHE`, `REF-GH-TOKEN-TRIGGERS`, already read; none new from F29.

## H. Light evolvability check

- Size: `tests/fitness.test.ts` holds the timing check's helpers and a made-up file beside the fitness checks; the timing check may become a test file of its own when it grows again.
- Tools: none failed; `cargo build` without `--locked` once wrote the two new path dependencies into `Cargo.lock`.
- Fitness functions: `[F29.AC1]` now reads `playground/` and shows each fault on a made-up file; the post hooks' exit code and `check-all`'s `CI` guard each have a test that fails on a mutation.
- Patterns: the release build is one constant of `stale-binaries`, the four places that rebuild using it.
- Agent configuration: `AGENTS.md` names the coverage lane; the `project-rules` skill did not repeat the sentence.
- Process: the auditor found two claims of the stories (F7's exit code, F8's `CI` guard) that their tests passed whatever the code did: a test of a post hook asserts what the hook returns, not what Git does.

## For the main thread

- F1: `Rust coverage` joins the required checks of `main`'s ruleset (24687556) when F29 merges into `develop`; no file changes for it.
- F29's criterion 1 names the timing tests of `src/` and `tests/`; since T1 `[F29.AC1]` reads `playground/` too (F4). The feature file is unchanged: widening the criterion's words to the three folders is the main thread's call.
- `docs/domain/` is not affected: F29 is tooling. Footer `Rule-docs: none affected, a tooling feature, no business rule changed`.

### Improvements

| 2026-10-18 | AUD-031 | A test of a Git post hook asserts the hook's own exit code and output, run by hand, besides Git's: Git ignores a post hook's code, so a test of the pull alone passes whatever the hook returns (F7) | process |
