# 0022 — Sources of F18: a safe and cheap parallel run

Spike SP-020, 2026-10-16, for F18 (E17). It records the sources read for RET-003 on the same day, and answers what each F18 story still needed from a source. Every page below was read in the local library (`npm run library -- read`) before it was cited; the rows are in [references](../../references.md), under _Parallel agents and shared files_, and in the updated rows `REF-CC-HOOKS` and `REF-CC-SUBAGENTS`.

## Counting the running subagents (EN-047, EN-063)

| Finding                                                                                                                                                                                      | Source                                |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| Claude Code refuses an `Agent` call once `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` subagents run (default 20), and its error tells Claude not to retry; a resume takes a slot without the check | `REF-CC-SUBAGENTS`, `REF-CC-ENV-VARS` |
| The variable is set for the whole project under `env` in `.claude/settings.json`; it can lower the cap, never remove it                                                                      | `REF-CC-ENV-VARS`                     |
| A `PreToolUse` hook on `Agent` receives `session_id`, `transcript_path`, `tool_input` and `tool_use_id`, **no count of running subagents** and no list of tasks in flight                    | `REF-CC-HOOKS`                        |
| Only `Stop` and `SubagentStop` receive `background_tasks`, the tasks in flight with `type: "subagent"`                                                                                       | `REF-CC-HOOKS`                        |
| `SubagentStart` (also fired on a resume) and `SubagentStop` give `agent_id`; `SubagentStart` cannot block; `SubagentStop` also fires for internal agents, with an empty `agent_type`         | `REF-CC-HOOKS`                        |
| A background `Agent` call returns at once with `status: "async_launched"` and its `agentId` (`PostToolUse`)                                                                                  | `REF-CC-HOOKS`                        |
| The transcript is written asynchronously and "may lag the in-memory conversation"                                                                                                            | `REF-CC-HOOKS`                        |

**Answer.** The hook must keep or rebuild the count itself. Two ways are sourced:

1. **From the transcript**, as `.claude/hooks/status-tasks.ts` already does for the status line: an `Agent` call whose end notification has not arrived is running. Free, and it names each subagent by its description; but the transcript may lag, so several `Agent` calls sent in one message may each see the same count.
2. **From a state file** written by the hooks: the `PreToolUse` hook on `Agent` records each call it allows by its `tool_use_id`, `PostToolUse` ties it to the `agentId`, `SubagentStop` with a non-empty `agent_type` removes it; an entry older than `LOST_TASK_MS` is dropped (a session stopped by force). Exact for calls sent together, provided the file is updated under a lock.

The built-in limit set to 3 (EN-063) stays a second guard: it holds even if the hook fails, but its message names no subagent, and resumes escape it. Both limits count per session; ADR-0063 does not say whether a parallel session (`docs/tooling/agent.md`) counts too — EN-047 states which.

## The heavy lock from any worktree (EN-047, EN-051)

| Finding                                                                                                                                                                 | Source                  |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| In a linked worktree, `$GIT_DIR` is `<main>/.git/worktrees/<name>` and `$GIT_COMMON_DIR` points back to `<main>/.git`; refs are shared                                  | `REF-GIT-WORKTREE`      |
| `git rev-parse --git-common-dir` prints `$GIT_COMMON_DIR`, else `$GIT_DIR`, relative to the current directory when relative; `--path-format=absolute` makes it absolute | `REF-GIT-REV-PARSE`     |
| `process.kill(pid, 0)` throws when the process does not exist                                                                                                           | `REF-NODE-PROCESS-KILL` |
| macOS `lockf` holds a `flock(2)` lock that the system frees with its process, so a lock is never stale; it is macOS only and does not name its holder                   | `REF-MACOS-LOCKF`       |
| Vitest uses every core by default; `maxWorkers: '50%'` halves it                                                                                                        | `REF-VITEST-MAXWORKERS` |

**Answer.** `git rev-parse --path-format=absolute --git-common-dir` gives the same `.git` from the main checkout and from every worktree. Checked on Git 2.54 on 2026-10-16: without the option the main checkout prints `.git`, a worktree prints the absolute path. The lock of ADR-0063 is a directory created with `mkdir` (atomic) holding the owner's process id; a lock whose process `process.kill(pid, 0)` finds absent is taken over. The brief's lock today has no process id, so a force-quit leaves it for good.

**Open.** A process id reused by another process after a force-quit keeps the lock held; no source read gives a portable check. EN-047 may also record the start time and say so.

## Tables without padding (EN-048)

| Finding                                                                                                                                                       | Source                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| The documentation of `proseWrap` speaks only of prose: `"never"` puts each prose block on one line                                                            | `REF-PRETTIER-PROSEWRAP`  |
| In the code, a table is always printed padded, except with `proseWrap: "never"` **and** a padded table wider than `printWidth`, which is then printed compact | `REF-PRETTIER-TABLE-CODE` |

Tried with Prettier 3.9.10 on the repository, 2026-10-16:

| With `proseWrap: "never"`                                                               | Count            |
| --------------------------------------------------------------------------------------- | ---------------- |
| Markdown files under `docs/` changed                                                    | 281 of 453       |
| Tables printed compact (header wider than 80 characters)                                | about 546 of 607 |
| Tables still padded (narrow)                                                            | about 61         |
| Lines of the diff outside tables (paragraphs written on several lines, joined into one) | about 2 485      |

A Markdown override with `printWidth: 1` and `embeddedLanguageFormatting: "off"` printed every table of a sample compact and left a code block untouched; it was not tried on the whole repository.

**Open, for the Product Owner** (ADR-0065 and its source disagree): decision 1 expects `proseWrap: "never"` to remove the padding, which it does only for tables wider than 80 characters; a narrow table that grows is still rewritten whole, and every wrapped paragraph is joined once. Options: keep decision 1 as it is and accept the narrow tables; add the Markdown override above; or another formatter for Markdown (E17's F19).

## Logs merged by union (EN-048, EN-050)

| Finding                                                                                                  | Source                            |
| -------------------------------------------------------------------------------------------------------- | --------------------------------- |
| `merge=union` keeps the lines of both sides, in an order that may be random, and asks the user to verify | `REF-GIT-UNION`                   |
| One file per entry and an index built from them is the pattern of two release tools                      | `REF-CHANGESETS`, `REF-TOWNCRIER` |
| Parallel agents collided until each worked on its own files                                              | `REF-ANTHROPIC-C-COMPILER`        |
| `pre-push` reads one line per ref pushed on its standard input and stops the push on a non-zero exit     | `REF-GITHOOKS`                    |

**Open.** Whether GitHub's merge of a pull request applies `merge=union`: no GitHub documentation says so; the only reports found are a 2016 pull request of RuboCop, where GitHub's web merge did not, and tickets of other hosts (level 6, never alone). Under ADR-0065 decision 4 only the main thread writes the logs, at merge, so union guards local merges (`develop` into a feature branch); EN-048 tries a GitHub merge once on a throwaway branch before relying on it there.

## Worktrees (EN-051, EN-052)

| Finding                                                                                                                                                                                     | Source             |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ |
| `.claude/worktrees/` belongs in `.gitignore` (already there)                                                                                                                                | `REF-CC-WORKTREES` |
| `worktree.baseRef` is `"fresh"` (the remote's default branch) or `"head"`, never a branch name: a worker's worktree on a feature branch is created with Git directly, as the run skill does | `REF-CC-WORKTREES` |
| `git worktree add` run by Bash fires no `WorktreeCreate` hook; a `PreToolUse` hook on Bash sees it                                                                                          | `REF-CC-HOOKS`     |
| `git worktree remove` and `prune` clean a worktree and its private directory                                                                                                                | `REF-GIT-WORKTREE` |

## Timing tests (EN-054)

| Finding                                                                                        | Source                      |
| ---------------------------------------------------------------------------------------------- | --------------------------- |
| Quarantine a non-deterministic test and fix it quickly; poll rather than sleep; wrap the clock | `REF-FOWLER-NONDETERMINISM` |
| `vi.useFakeTimers()` and `vi.advanceTimersByTime` move timers without waiting                  | `REF-VITEST-TIMERS`         |
| `retry` is `0` by default and may be bounded by a `condition` on the error                     | `REF-VITEST-RETRY`          |

The lint already forbids `performance.now()` and `Date.now()` in tests (`.claude/rules/tooling.md`); a retry is not adopted, since it hides real failures.

## The release flow (EN-046)

| Finding                                                                                                                                                     | Source                      |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| The action's `target-branch` sets the branch of the release pull request; `skip-github-release` and `skip-github-pull-request` split the two steps          | `REF-RELEASE-PLEASE-ACTION` |
| A release is created once the release pull request is merged into the release branch; the release tags that pull request's merge commit, found by its label | `REF-RELEASE-PLEASE-DESIGN` |

**Open.** With the release pull request merged into `develop`, its merge commit is on `develop`: the step on `main` must look for pull requests merged into `develop` (`target-branch: develop`, `skip-github-pull-request: true`) and tag that commit once `main` holds it. EN-046 checks this on the next release rather than assuming it.

## The other stories

| Story                  | Sources                                                                  | Open                                                                                                                                           |
| ---------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| EN-049                 | `REF-CHANGESETS`, `REF-TOWNCRIER`                                        | none                                                                                                                                           |
| EN-053                 | `REF-CC-STATUSLINE`, `REF-CC-HOOKS` (`background_tasks`, `SubagentStop`) | the cause of the wrong states, from the call records (the story's own task)                                                                    |
| EN-055                 | `REF-GITHOOKS`                                                           | none                                                                                                                                           |
| EN-056, EN-059, EN-060 | the project's own rules (`docs/conventions/writing.md`)                  | none                                                                                                                                           |
| EN-057                 | `REF-OT-OTFF` (font collections)                                         | none                                                                                                                                           |
| EN-058                 | `REF-GOOGLE-MUTATION`, `REF-STRYKER-6210`                                | none                                                                                                                                           |
| EN-061                 | `REF-OXLINT-ACCUMULATING-SPREAD`, `REF-ESLINT-RESTRICTED-SYNTAX`         | the fix the Oxlint page shows mutates the accumulator, which the project forbids (ADR-0014): use `map`, `flatMap` or a loop over a local array |
| EN-062                 | `REF-INTER-LICENSE`                                                      | none                                                                                                                                           |
| US-061                 | `REF-CSS-FONT-LOADING`, `REF-WICG-LOCAL-FONT-ACCESS`                     | none                                                                                                                                           |
| EN-045                 | done before the spike                                                    | —                                                                                                                                              |

## Not adopted

`git rerere` (`REF-GIT-RERERE`) and a merge queue (`REF-GH-MERGE-QUEUE`, organizations only) stay out, as ADR-0065 decides.

**Read by EN-046 (2026-10-17).** release-please's source confirms it: the release step lists the pull requests merged into its `target-branch` with the pending label (`REF-RELEASE-PLEASE-SOURCE`), so the step on `main` runs with `target-branch: develop`; while such a pull request waits for its tag, no new release pull request is opened. The tag itself is checked on the next release ([EN-046](../../backlog/stories/E17-F18-EN-046-release-pr-on-develop.md)).
