# Tool incidents

One card per time a tool failed or did not do what was needed: how it was used, why it failed, what was done instead (ADR-0085). A tool is never fixed during a feature's work: the cards are read at the next review, held at each return of the Product Owner, at its "method and tools" point, and the tools change in that review or in the stories it decides (ADR-0086). Moved by the main thread in the story's `done` commit (ADR-0091); a worker's card goes into its story file under `### Incidents`, its heading `#### <date> — <title>`.

The cards of E17 were decided at RET-004 ([record](./retrospectives/E17-retrospective.md), C1 to C9): their fixes are the stories of [F23](../backlog/features/E17-F23-the-path-of-a-story.md), moved into E17 and cut by theme on 2026-10-18 into F23, [F29](../backlog/features/E17-F29-local-checks-stable-and-fast.md), [F31](../backlog/features/E17-F31-the-guard-and-the-agents-commands.md), [F32](../backlog/features/E17-F32-reports-that-say-what-happened.md) and [F33](../backlog/features/E17-F33-the-checks-ret-004-listed.md) (SP-024), C3's auto-merge and C7 parked until the next review (ADR-0086).

## 2026-10-18 — `worktree-new EN-114`

- Used: `target/release/worktree-new EN-114` to make the story's worktree before writing its story file.
- Failed: it requires the story file in `docs/backlog/stories/`, and bases the worktree on the story's feature branch; a story outside a feature has neither.
- Instead: `git worktree add -b build/en-114-… .claude/worktrees/en-114 origin/develop`, `node_modules` linked and `target/release` cloned by hand.

## 2026-10-18 — `guard-bash` in a worktree, chained Git commands

- Used: `cd <worktree> && git status`, `git checkout -p … ; sed …`, `git add -A && git commit …` in one call.
- Failed: the hook refuses any Git command chained after a `cd` into a worktree, even read-only ones (`git log`, `git diff --stat`); five refusals in one story.
- Instead: one Bash call to `cd`, then each Git command alone.

## 2026-10-18 — `story-finish develop …` for a story outside a feature

- Used: `target/release/story-finish develop "<title>" <body>` for EN-114.
- Failed: it waited 3 minutes for a merge that never comes: `story-automerge.yml` merges into `develop` only `feat/val-…`, `…/nam-…`, `…/rev-…`, `…/ret-…` heads; the Product Owner merged #358 by hand.
- Instead: for EN-115, `gh pr create --label autonomous` directly; the Product Owner merges.

## 2026-10-18 — `tests/backlog.test.ts`, a story attached to the epic only

- Used: `E17-EN-114-….md` with `epic: E17` and no feature, listed in the epic's Closing table.
- Failed: the pre-push `check:all` refused it: only NAM, REV and RET may sit under an epic without a feature.
- Instead: attached to F21 (`E17-F21-EN-114-…`, `feature: F21`, a row in F21's table).

## 2026-10-18 — status line, a story worker "running 9 min 20"

- Seen: the status line showed a story worker running for 9 min 20 although no agent ran.
- Cause: 169 files in `.git/metrics/running/` left by subagents stopped before their PostToolUse hook; nothing removes a call whose agent ended.
- Instead: the 169 files removed by hand.

## 2026-10-18 — pre-push, shared files in a story branch

- Used: EN-114's commit holding a row of `docs/process/journal.md`.
- Failed: the pre-push hook refuses a shared file on a story branch (ADR-0065), but nothing writes those rows on `develop` for a story merged straight into it.
- Instead: the row kept under `## For the main thread` in the story; the journal not written yet.

## 2026-10-18 — permission check, `.github/workflows/story-automerge.yml`

- Used: the EN-115 worker widened the auto-merge condition to story branches into `develop`, as the Product Owner asked.
- Failed: the permission system refused the edit as a CI bypass.
- Instead: not done; left to the Product Owner.

## 2026-10-18 — `story-finish develop` for REV-004, the log rows' push

- Used: `story-finish` after #360 merged; it moved the rows into the logs, committed on `develop` in the main checkout and pushed.
- Failed: the push's `check:all` failed on a time limit: `[F18.AC19] reads Inter Regular …` took 12.3 s against 5 s, the load average near 60 (Spotlight indexing the 2.3 GB of records just moved into `Documents`, and Figma).
- Instead: `story-finish` run again from the worktree.

## 2026-10-18 — the main checkout's binaries after a merge on GitHub

- Seen (REV-004): the main checkout's `target/release/guard-bash` predated EN-115: a PR merged on GitHub (#359) leaves the main checkout's binaries stale, since only `story-finish` and `git-sequences sync` rebuild them.
- Instead: `npm run rust:build` in the main checkout.

## 2026-10-18 — `replace` without an argument

- Used: `target/release/replace` alone, to see its usage, by the main thread and by EN-117's worker.
- Failed: it prints nothing and waits on its standard input until the call's 120 s timeout.
- Instead: the Edit tool, or a short script for a multi-file change.

## 2026-10-18 — `worktree-new` from the main checkout for a story only on its feature branch

- Used: `target/release/worktree-new EN-123` in the main checkout, on `develop`, while EN-123's file was only on `feature/f22-release-by-a-button`.
- Failed: `no file E…-EN-123-<topic>.md in docs/backlog/stories/`: it reads the story files of the checkout it runs in.
- Instead: run from a worktree of the feature branch.

## 2026-10-18 — `story-automerge.yml` for an epic's refinement `SP`

- Used (found by AUD-024, not yet met in a run): an epic's refinement, an `SP` from `develop` (`/spike`, ADR-0086), finished by `story:finish develop …` with the `autonomous` label.
- Fails: the workflow lets into `develop` only `feature/f<nn>-…` and `…/rev-<nnn>-…` heads (`.github/workflows/story-automerge.yml`, its branch check), so the `SP`'s pull request is refused and `story:finish` waits 3 minutes for a merge that never comes.
- Instead: the spike and run skills say the Product Owner merges an epic's `SP`, its pull request opened by `gh pr create`; whether the auto-merge should take it is a question for the next review.

## 2026-10-18 — `npm run fix` on a checklist barrel

- Used: `npm run fix` after VAL-023's checklist lines, as at VAL-022.
- Failed: it rewrote `export type * from "./checklist-line"` into `export *` in `playground/checklists/index.ts`, a type-only re-export turned into a value one.
- Instead: the line set back by hand; `tests/barrels.test.ts` passes.

## 2026-10-19 — `mutate` on a Rust file, a number inside a comment

- Used: `npm run mutate -- --all tools/worktree/src/lib.rs -- cargo test -q -p worktree`.
- Failed: it changed `025` into `26` inside the doc comment `(AUD-025)` and reported it `SURVIVED`, a mutation of no code; its report of a Rust return also reads `→ undefined`.
- Instead: the survivor read and recorded as a comment, no action.

## 2026-10-19 — `guard-bash`, a `$'…'` text holding `\'`

- Used: a loop over the story's cases written as `$'…\'EOF\'…'` texts, after the brief's `cd`, to probe the release binary.
- Failed: the separator reader closed the quote at `\'`, read the rest as commands and refused a chained Git command where none was.
- Instead: the cases written to a JSON file and read with `jq`; the reader fixed in EN-119.T2, a backslash escaping inside `$'…'`.

## 2026-10-22 — epic-check, to read its usage

- Used: `target/release/epic-check --help`, by the auditor of AUD-037, to read its options.
- Failed: it ignores `--help` and runs the whole epic check, `check:all` included, in the worktree it was called from.
- Instead: the check was left to end; it left no file in the worktree, as `git status --ignored` showed at the fixes' start.

## 2026-10-22 — mutate, for the mutation spot-checks of F35

- Used: `npm run mutate -- <files>`, the audit's new modules not yet committed.
- Failed: the copy takes tracked files only, so every test importing a new module failed in it, and each file was skipped as "its related tests fail, or none was found", the cause unsaid.
- Instead: the new modules staged with `git add` before the run, which the copy then took.

## 2026-10-22 — `worktree-new`, to make each worktree of F36

- Used: `target/release/worktree-new <ID>` for the stories of F36, a feature started on F35's branch (`feature/f36-draw-pick-and-remove-shapes` stacked on `feature/f35-several-shapes-on-one-drawing`).
- Failed: it refuses a story whose file is on two feature branches, which every story file of F36 is while F35's branch is not merged.
- Instead: each worktree was made by hand, `git worktree add` on the story's branch from F36's, then `npm ci`, about 2 min each.

## 2026-10-23 — `mutate`, for the audit's mutation spot-checks

- Used: `npm run mutate -- <27 files> -- npx vitest run {test}` in the background, before the audit's new modules were committed.
- Failed: the copy holds tracked files only (`git stash create`), so the two new modules, untracked, were missing and every file was skipped as "its related tests fail"; `{test}`, unquoted in the background call, reached the tool dropped, so each skip ran the whole suite; the run, stopped, left its temporary worktree registered; `git add -N` on the new files made `git stash create` fail ("not uptodate").
- Instead: the run stopped and its worktree removed (`git worktree remove --force`), the new files staged by `git add`, then `npm run mutate` again with `'{test}'` quoted.
