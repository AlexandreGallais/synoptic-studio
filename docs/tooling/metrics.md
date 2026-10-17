# Metrics of the agent's work

Measure first, then make each epic faster than the last (Product Owner, RET-001): which commands the agent runs most, which take longest, how long each task, story, feature and session takes, and how many tokens they use.

## Where the data lives

| Source | Written by | Holds |
| --- | --- | --- |
| `.git/metrics/tool-calls.jsonl` | the hook `.claude/hooks/log-tool-call.ts`, after each tool call (`PostToolUse`, `PostToolUseFailure`, in the background) | one line per call: see [each call's record](#the-record-of-each-call) |
| `~/.claude/projects/<repository path>/` | Claude Code: one JSON Lines transcript per session, subagents included | every request with its tokens, every tool call and its result, each line with its time, session and branch |
| `.git/metrics/turns.jsonl` | `npm run metrics:session` without `--all` | one line per turn: start, session, duration, active time, tokens, tokens written, calls, failures, commits |
| `~/.local/share/agent-library/library.sqlite` | the library server and `npm run library` ([sources](./sources.md)) | the pages read, their sections indexed, and every search and read (`uses` table) |

The hook's log sits inside the Git directory: never committed, shared by every worktree, kept when transcripts are cleaned up. Its durations are exact: Claude Code measures the tool's execution and leaves out permission prompts (`duration_ms`, `REF-CC-HOOKS`). For calls made before the hook existed, the report estimates the duration from the transcript, from the request to the result; this also counts the time spent writing the command, so a long heredoc looks slow.

## The record of each call

Each line of `tool-calls.jsonl` holds what the hook knows when the call ends, and what the status line last saw (`.git/metrics/now.json`, written at each refresh, so that reading it costs the hook nothing; RET-002):

| Field | Holds |
| --- | --- |
| `time`, `startedAt`, `durationMs` | when it ended and started (epoch milliseconds, from `track-tool-call`), how long it ran |
| `session`, `branch`, `step` | where: the session, the branch checked out, the step the agent wrote |
| `agent`, `agentId` | who: `main` or the subagent's type, and its own id |
| `tool`, `summary`, `description`, `isBackground` | what: the tool, what it acted on, its description, whether it ran in the background |
| `inputChars`, `outputChars` | the size of what the agent wrote to make it and of what it read back |
| `isOk`, `error`, `permissionMode` | how it ended, the first line of its error, the permission mode |
| `turnStart`, `turnElapsedMs`, `turnTokens`, `turnOutputTokens` | the turn: its start, the time since, its tokens so far and the part written |
| `contextPercent`, `model`, `pendingAgents`, `pendingCommands` | how full the conversation memory was, the model, the background tasks running |
| `snapshotAgeMs` | how old the status line's snapshot was |

## The report

`npm run metrics` prints a Markdown report (`npm run metrics > report.md` to keep it); `npm run metrics -- --since 2026-10-16` covers only the calls, requests and turns from that day on, 00:00 UTC (EN-055). The transcripts are read from the main checkout's folder, also when the report runs in a worktree:

| Section | What it shows |
| --- | --- |
| Commands by count | shell commands grouped by what they run (`npm run check`, `git commit`, `gh pr checks`, `grep`): count, total, median, longest, failed |
| Commands by total time | the same groups, by total time: where waiting goes |
| Slowest calls | the 20 longest tool calls, exact or estimated, with their branch |
| Sessions | each session from the day it started: active time, tokens, tool calls, failures — the session's timer |
| Features | the same totals per feature (stories mapped through their file names) |
| Stories and branches | per story (`US-018`), feature branch (`F04`) or other branch (`main`, `develop`) |
| Tasks, by commit | per task commit (`Refs: US-018.T2`), with its short hash: read `git show <hash>` to see why a task was slow |
| Turns | each turn saved by the session report |
| Where the time of the latest turns went | for the main agent's timed calls: the tools running, the model between two calls (reading, thinking, writing the next one), the characters written |

- **Active time** adds the gaps between moments of work (a request, the end of a tool call), each gap capped at 5 minutes: a pause, a night or a wait for the Product Owner is not counted as work.
- A moment belongs to the story of the branch it ran on, and to the first task commit of that story made after it. Work after a story's last task commit (pull request, wait for checks) shows as `<story> (no task commit)`.
- **Output tokens** are those the model wrote; **all tokens** add input, cache writes and cache reads, which dominate.
- **Failed** counts the tool calls that ended in an error: a failing check, a refused commit, a denied command. Their causes are read in the slowest calls, the log, or the transcript.

## Status line

The status line at the bottom of Claude Code (`.claude/hooks/status-line.ts`, `REF-CC-STATUSLINE`), refreshed every second, has two lines, then one per call running and per background task:

```text
E14 › RET-002 retrospective e14 · status line, documentation
● working · 1 agent in the background · ⏱ turn 12 min 05 s · 3,815,104 tokens this turn (14,681 written) · conversation memory 41 % full · Opus 5.5
▶ Bash · Run the checks — 7 s
↳ agent auditor: Review EN-030 — 4 min · now Bash · List the files
```

At each refresh it also writes the subscription's usage to `.git/metrics/usage.json`, for a run to stop before the weekly limit (EN-092): `sevenDay` and `fiveHour`, each with `usedPercent` (0 to 100) and `resetsAt` (ISO 8601), and `time`, when it was read (epoch milliseconds). Claude Code sends these windows only to Pro and Max subscribers, after the session's first response (`rate_limits`, `REF-CC-STATUSLINE`); a missing window is left out, and without either the last file is kept. No hook receives them.

Its four states tell the Product Owner when the chat is free (RET-002):

| State | Means |
| --- | --- |
| `● working` | the agent works on the main thread: a message waits until its turn ends |
| `● working · 2 agents in the background` | the same, with subagents or background commands running |
| `◌ chat free · 1 agent, 1 command in the background` | the agent's turn is over; only background tasks run: a message is answered at once |
| `◌ your turn` | nothing runs |

A working agent silent for more than 2 minutes shows `◌ silent for 3 min`: waiting, or stalled. A background task is a subagent, a monitor, a call run in the background, or a subagent resumed by a message; it ends with its notification, or with an error result when it was refused before it started (a denied permission, the safety check unavailable): such a call sends no notification and once stayed counted for three hours (2026-10-14). A subagent run in the foreground ends with its result, which is not a launch in the background (AUD-020).

A message to a subagent that had stopped (`SendMessage`, its result `Resuming agent …`) runs it again in the background, its end notified under the message's call id; a message to a subagent still running is only queued and runs nothing. The status line did not count the first kind: on 2026-10-15 it showed `◌ your turn` from 12:07 to 12:22 UTC while a resumed worker fixed its audit findings, 43 such messages in E15 (EN-053). The limit of three subagents (`tools/hooks`) reads the tasks the same way.

| Part | Source |
| --- | --- |
| epic › feature › story | the branch checked out, matched with the backlog's file names (`status-place.ts`); the branch itself outside a story |
| the step | `.git/metrics/step`, one line the agent writes at each step; when empty, the place alone (it once read `waiting`, taken for a state: RET-003) |
| the state | see the table above; read from the transcript and its background tasks (`status-tasks.ts`) |
| `⏱ turn` | time since the current turn started, to the second so that it moves at each refresh; once the agent ends its turn, `last turn` and the clock stopped at its last answer: the Product Owner's prompt after the agent last ended its turn; a message sent while the agent works, or a compaction, stays in the turn |
| tokens this turn | exact sum of the turn's requests, read from the transcript, the same figures as the session report; the part the model wrote in brackets. They grow by steps: Claude Code writes a request's tokens into the transcript only once its answer is complete, so a long answer (a large file written) shows its tokens when it ends |
| conversation memory | how full the context window is (`context_window.used_percentage`): near 100 %, Claude Code compacts the conversation into a summary |
| `▶` | the oldest call of the main agent still running, its description (or its command) and for how long: written by the `track-tool-call` hook when it starts, removed by `log-tool-call` when it ends; in red after 5 minutes |
| `↳` | one line per background task: a subagent with its type, task, time and the call it makes now (the latest started by a subagent of its type), a command with its description |

The tokens are summed incrementally: `.git/metrics/status-line-turn.json` keeps where the reading stopped, so each refresh reads only the new lines (about 0.1 s). The turn, not the terminal: Claude Code's own figures (`cost.total_duration_ms`, the session's tokens) add up every turn since the terminal opened, which says nothing about the work in progress. Popular status lines (ccstatusline, `REF-CCSTATUSLINE`) show the same families of facts — model, branch, tokens, usage limits, session time; usage limits are left out, Claude Code already shows them. This one adds the backlog place and the call running, which no generic one knows.

## Session report

The session report closes the last message of a session, once, with `--all` (`CLAUDE.md`; [writing](../conventions/writing.md#messages-to-the-product-owner)): without it, the report counts the current turn only, from the last prompt, and 10 h 40 of work once showed as 2.8 s (2026-10-15, RET-003).

```sh
npm run metrics:session -- --all --markdown # the whole session, as a Markdown table, for the chat
npm run metrics:session -- --all            # the same, framed and colored for the terminal
npm run metrics:session                     # the current turn only, saved for the comparison of turns
```

It gives one row: the duration (wall and active), the tokens, the part the model wrote, the tool calls and failures, the commits (listed when there are eight or fewer); the subagents' transcripts count with the session's. A report of the current turn (without `--all`) is also saved as one line in `.git/metrics/turns.jsonl`; the **Turns** section of `npm run metrics` lists every turn, so that a retrospective compares them. A turn starts at the first prompt after the agent last ended its turn; messages sent while it works stay in the turn.

## At each retrospective

`RET` reads `npm run metrics` — commands, slowest calls, sessions, features, stories, tasks, **turns** and **sources** — and answers, with figures (retro skill):

- which commands run most and which cost most time: is one worth a script, a cache or a check run less often?
- which stories and tasks took longest: read their commits (`git show <hash>`) to see why;
- how many tool calls failed, and of which kind: a repeated failure gets a check or a rule;
- how tokens grew per story compared with the last epic: what made the agent search or read more.

## Watching a long task

A command expected to last more than a minute runs in the background (`run_in_background`), never behind a waiting loop; the agent watches it with Claude Code's Monitor and `scripts/watch-task.ts`, which prints a line only when the task needs a look:

```sh
node --harmony-temporal scripts/watch-task.ts <output file> --stall 120 --until "done" --max 1800
```

| Line printed | When |
| --- | --- |
| `stalled: no output for 125 s — last line: …` | the output has not changed for `--stall` seconds |
| `resumed: …` | it changes again after a stall |
| `done: …` | a new line starts with `--until` (plain text), or the task exited (`[exited with code N]`, the line Claude Code appends) |
| `watch over: …` | `--max` seconds passed (Monitor's longest watch is 30 minutes: start another) |

The status line shows a tool call running for more than 5 minutes in red. A long task writes one progress line per step, and stops itself when a step changes nothing ([tooling pitfalls](https://github.com/AlexandreGallais/synoptic-studio/blob/main/.claude/rules/tooling.md)).

## Limits

- Sessions started in another folder, such as a worktree of `.claude/worktrees/`, have their transcripts under another project folder: they are not read yet.
- Claude Code deletes transcripts older than `cleanupPeriodDays` (30 by default): `.claude/settings.json` sets it to 3650, so that every retrospective still reads the tokens of its epic.
- `.git/metrics/` lives in this clone only: a fresh clone starts empty. `npm run records:save` keeps a copy of it, of the transcripts and of the library in the private records repository `records/` (ADR-0041, ADR-0044), from which a new machine restores them.
- Tokens are counted per model request, not per command: a command's cost shows as the request that wrote it and the one that read its result.
- Temporal is behind `--harmony-temporal` in Node.js 24: the hook command, `npm run metrics` and Vitest pass it.
