# Metrics of the agent's work

Measure first, then make each epic faster than the last (Product Owner, RET-001): which commands run most, which take longest, how long each task, story, feature and session takes, and how many tokens they use. A change to how the agent works is judged by these figures, before and after.

## Where the data lives

| Source | Written by | Holds |
| --- | --- | --- |
| `.git/metrics/tool-calls.jsonl` | the hook `log-tool-call` (Rust), after each tool call, in the background | one line per call ([the record of each call](#the-record-of-each-call)) |
| `.git/metrics/tool-uses.jsonl` | each Rust tool of the workspace, at its end (`tools/tool-use`) | one line per use of a tool ([the uses of each tool](#the-uses-of-each-tool)) |
| `~/.claude/projects/<main checkout's path>/` | Claude Code: one JSON Lines transcript per session, its subagents' beside it | every request with its tokens, every tool call and result, each line with its time, session and branch |
| `.git/metrics/run.json`, `run-cache.json` | `npm run run:start` and `run:end`; the cache by `run-totals` | the open or last run: start, end (0 while open), sessions; the cache, where each transcript was read up to ([a run's totals](#a-runs-totals)) |
| `.git/metrics/turns.jsonl` | `npm run metrics:session` without `--all` | one line per turn: start, session, duration, active time, tokens, calls, failures, commits |
| `.git/metrics/transcripts-cache.jsonl` | the metrics' Rust tools (`tools/metrics`, EN-107) | each transcript's events as last read and the offset reached ([reading the transcripts](#reading-the-transcripts)) |
| `.git/metrics/usage.json` | the status line, at each refresh | the subscription's usage ([status line](#status-line)) |
| `~/.local/share/agent-library/library.sqlite` | the source library ([sources](./sources.md)) | the pages read, their sections, every search and read with its project |

`.git/metrics/` is never committed, is shared by every worktree, and outlives the transcripts. The hook's durations are exact: Claude Code measures the tool's run, permission prompts left out (`duration_ms`, `REF-CC-HOOKS`). For a call made before the hook existed, the report estimates the duration from the request to the result, which also counts the time spent writing the command.

## The record of each call

Each line of `tool-calls.jsonl` holds what the hook knows when the call ends, and what the status line last saw (`.git/metrics/now.json`, written at each refresh, so that the hook reads it for free):

| Field | Holds |
| --- | --- |
| `time`, `startedAt`, `durationMs` | when it ended and started (epoch milliseconds), how long it ran |
| `session`, `branch`, `step` | where: the session, the branch checked out, the step the agent wrote |
| `agent`, `agentId` | who: `main` or the subagent's type, and its id |
| `tool`, `summary`, `description`, `isBackground` | what: the tool, what it acted on, its description, whether it ran in the background |
| `inputChars`, `outputChars` | the size of what the agent wrote and of what it read back |
| `isOk`, `error`, `permissionMode` | how it ended, the first line of its error, the permission mode |
| `turnStart`, `turnElapsedMs`, `turnTokens`, `turnOutputTokens` | the turn: its start, the time since, its tokens so far and the part written |
| `contextPercent`, `model`, `pendingAgents`, `pendingCommands` | how full the context was, the model, the background tasks running |
| `snapshotAgeMs` | how old the status line's snapshot was |

## The uses of each tool

Each tool of our own records its own use (EN-102), whoever calls it — the agent, a hook, a script, a person — so that the retrospective reviews them as a tools team would: which tool runs, how often it fails, how long it takes. A binary's `main` hands its work to `tool_use::run`, which times it and appends one line to `tool-uses.jsonl` in the main checkout's `.git/metrics/`, a worktree's uses included:

| Field | Holds |
| --- | --- |
| `tool` | the binary: `git-sequences`, `format`, `check-handback` |
| `subcommand` | what the call asked of it (`sync`, `--check`; for `heavy-lock`, the command it runs), empty for a tool of one mode |
| `worktree` | the linked worktree's name (`en-102`), `main` for the main checkout |
| `start`, `durationMs` | when it started (ISO 8601, UTC) and how long its work took, the record left out |
| `exitCode` | the code it exited with |

- **Never in the tool's way**: a line that cannot be written — no `.git` above the folder, a read-only folder — is dropped without a word, the tool's output and exit code unchanged. The `metrics` folder is created when missing, never `.git` itself.
- **One write per line**: the line goes to a file opened to append in a single write, so that two tools ending at once never mix their lines.
- **A poll is not recorded**: a subcommand another program calls on a timer is a poll, which its tool marks by calling `tool_use::run_or_poll` instead of `run`; its line is never written, its output and exit code unchanged. Today one: `run-totals status`, which the status line calls once a second during a run, about 18,000 lines a run of 5 hours that would say nothing of the tool's use; every other subcommand of `run-totals` is recorded. `tools/tool-use/tests/record.rs` proves both.
- **Its cost**: 0.13 to 0.17 ms a call, about 121 bytes a line ([EN-102's measures](../backlog/stories/E17-F21-EN-102-each-tool-records-its-use.md#measures-adr-0069)).
- **Release builds only**: `package.json`, the Git hooks and `.claude/settings.json` run every tool with `--release`; a debug build is a test's, whose calls are not uses, so it records nothing.
- No crate: the standard library appends the line ([research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md)).

## The report

`npm run metrics` prints a Markdown report; `-- --since <day>` keeps only what happened from that day on, 00:00 UTC. It reads the main checkout's transcripts, also when run in a worktree.

| Section | What it shows |
| --- | --- |
| Commands by count | shell commands grouped by what they run: count, total, median, longest, failed |
| Commands by total time | the same groups, by total time: where waiting goes |
| Slowest calls | the 20 longest tool calls, with their branch |
| Sessions | each session: active time, tokens, tool calls, failures |
| Features | the same totals per feature |
| Stories and branches | per story, feature branch or other branch |
| Tasks, by commit | per task commit, with its short hash: `git show <hash>` says why a task was slow |
| Turns | each turn the session report saved |
| Sources | the source library's uses ([sources](./sources.md#uses)) |
| Where the time of the latest turns went | the main agent's tools running, the model between two calls, the characters written |
| The agent's tools | each tool of [the agent's tools](./agent-tools.md) run through npm, from the hook's log: count, total, median, longest, failed |
| Tools | each Rust tool of the workspace by tool and subcommand, from [its own record](#the-uses-of-each-tool), whoever called it: calls, failed (an exit code other than 0), median, slowest |
| Findings | each rule's findings met in the tool results, per epic ([below](#findings)) |

- **Active time** adds the gaps between moments of work, each capped at 5 minutes, so that a pause, a night or a wait for the Product Owner is not counted as work.
- A moment belongs to the story of its branch, and to that story's first task commit made after it; work after the last task commit shows as `<story> (no task commit)`.
- **Output tokens** are those the model wrote; **all tokens** add input, cache writes and cache reads, which dominate.
- **Failed** counts the calls that ended in an error: a failing check, a refused commit, a denied command.

It is the binary `metrics` of `tools/metrics` (EN-107): on this session's transcripts and logs, its output is the TypeScript report's, the Findings section added, but for one row of the Tools section, `library stats`, which the script ran as a command, recording one use more, where the binary reads the library in its own process. Measured on 1.6 GB of transcripts (2026-10-18): 6.4 s without the cache, then 0.5 s, against 8.2 s for `metrics-report.ts` ([research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md#the-tools-story-by-story)).

### Findings

The count EN-093 made by hand for [every rule and why](../conventions/rules.md) (Friction), made at each report: a rule's findings are its work and its cost, and the next review of the rules reads them here.

| Kind | Read from | Rule named |
| --- | --- | --- |
| `hook` | an error result opening `<event>:<tool> hook error: `, any tool | the event and tool, then the reason up to its first `: ` or `. ` (`PreToolUse:Bash: A worktree refuses a chained Git command`) |
| `lint` | a line of a shell command's output: ESLint's `  12:5  error  Message  rule`, oxlint's `path:1:2: error plugin(rule):` or `× plugin(rule):`, the project linter's `path:1:2 plugin/rule`, markdownlint's `path:1:2 error MD013/line-length` | oxlint's as `.oxlintrc.json` names them (`unicorn/no-null`, `typescript/no-explicit-any`, `id-length`); the others as their linter writes them, so ESLint's era keeps `@typescript-eslint/…` and `local/…` |
| `type` | `error TS2322:` in a shell command's output | the code |
| `clippy` | Clippy's link `…/rust-clippy/…/index.html#<lint>` | `clippy::<lint>` |
| `check` | `check:all`'s line `✗ <check> <seconds> s` | the check |

A result counts each rule once per line that reports it; a result's epic is that of its branch's story or feature, from the backlog's file names, else the branch itself (`develop`, `HEAD`). Lines without a time are left out, as everywhere in the report. The findings are kept in the transcripts' cache with the rest.

## Status line

The status line (the `status-line` binary of `tools/hooks`, `REF-CC-STATUSLINE`), refreshed every second, tells the Product Owner where the work is and whether the chat is free: two lines, then one per call running and per background task.

```text
E14 › RET-002 retrospective e14 · status line, documentation
● working · 1 agent in the background · ⏱ turn 12 min 05 s · 3,815,104 tokens this turn (14,681 written) · run 2 h 05 · 43,100,000 tokens · week 34 % used · conversation memory 41 % full · Opus 5.5
▶ Bash · Run the checks — 7 s
↳ agent auditor: Review EN-030 — 4 min · now Bash · List the files
```

| State | Means |
| --- | --- |
| `● working` | the agent works on the main thread: a message waits until its turn ends |
| `● working · 2 agents in the background` | the same, with background tasks running |
| `◌ chat free · 1 agent, 1 command in the background` | the turn is over, only background tasks run: a message is answered at once |
| `◌ your turn` | nothing runs |
| `◌ silent for 3 min` | the agent has been silent for over 2 minutes: waiting, or stalled |

A background task is a subagent, a monitor or a call run in the background, and a stopped subagent that a message resumes; it ends with its notification, or with an error result when it was refused before it started. A message to a subagent still running only queues and starts nothing. `limit-agents` counts the tasks the same way.

| Part | Source |
| --- | --- |
| epic › feature › story | the branch checked out, matched with the backlog's file names; the branch itself outside a story |
| the step | `.git/metrics/step`, one line the agent writes at each step; when empty, the place alone, so that no word is taken for a state |
| the state | the transcript and its background tasks (`tools/hooks/src/tasks.rs`, which `limit-agents` reads too) |
| `⏱ turn` | the time since the turn started; once the agent ends its turn, `last turn`, stopped at its last answer. A turn starts at the first prompt after the agent ended its turn; a message sent while it works, or a compaction, stays in the turn |
| tokens this turn | the exact sum of the turn's requests, as in the session report, the part written in brackets; they grow by steps, as a request's tokens reach the transcript only once its answer is complete. A request written over several lines is counted once, from its last line: a subagent's transcript repeats its usage with a growing `output_tokens` (EN-104) |
| `run` | while a run is open: its duration, its tokens with the subagents', then the week's usage ([a run's totals](#a-runs-totals)) |
| conversation memory | how full the context window is; near 100 %, Claude Code compacts the conversation |
| `▶` | the main agent's oldest call running, its description and for how long; in red after 5 minutes |
| `↳` | one line per background task: a subagent with its type, task, time and current call, or a command with its description |

The tokens are summed incrementally (`.git/metrics/status-line-turn.json` keeps where the reading stopped), so a refresh reads only the new lines. The figures are the turn's, not the terminal's: Claude Code's own add up every turn since the terminal opened, which says nothing of the work in progress. It shows the backlog place and the call running, which no generic status line knows (`REF-CCSTATUSLINE`).

At each refresh it also writes the subscription's usage to `.git/metrics/usage.json`, for a run to stop before the weekly limit: `sevenDay` and `fiveHour`, each with `usedPercent` and `resetsAt`, and `time`, when it was read. Claude Code sends these windows to Pro and Max subscribers only, after the session's first answer (`rate_limits`, `REF-CC-STATUSLINE`); a missing window is left out, and without either the last file is kept. No hook receives them.

## A run's totals

A turn restarts at each message, so a run's totals count from its start to its end instead, whatever happens between:

| Moment | Command | Does |
| --- | --- | --- |
| `/run` starts, or continues in a new session | `npm run run:start` | opens `.git/metrics/run.json` with the time and the session's id; a run already open keeps its start and adds the session |
| each status line refresh | `run-totals status` | the `run …` part, only while a run is open |
| before each story | `npm run run:should-stop` | `stop` once the week's usage reaches 90 %, else `go`; then the parked stories |
| any time | `npm run metrics:session -- --run` | the open or last run's table |
| the run's stop | `npm run run:end` | closes the run, prints the table for the message to the Product Owner |

The table holds the duration and the tokens by kind, nothing else (F20's refinement, 2026-10-17):

```text
| Duration | Tokens | Input | Cache written | Cache read | Output |
| --- | --- | --- | --- | --- | --- |
| 2 h 05 | 43,100,000 | 1,234 | 2,000,000 | 41,000,000 | 98,766 |
```

It counts every assistant request of each session's transcript and its `subagents/` folder from the run's start to its end, once per message id, with the usage of its last line, since a request's output grows over its lines. A compaction keeps the session's file, so nothing restarts. `tools/run-totals` reads each transcript from the offset kept in `run-cache.json`, so a later count reads only the new lines.

## Session report

The last message of a session closes with the session report, made with `--all` (`CLAUDE.md`, [writing](../conventions/writing.md#messages-to-the-product-owner)): without it, the report counts the current turn only, which can be seconds of a day's work.

```sh
npm run metrics:session -- --all --markdown # the whole session, as a Markdown table, for the chat
npm run metrics:session -- --all            # the same, for the terminal
npm run metrics:session                     # the current turn only, saved for the comparison of turns
```

One row: duration (wall and active), tokens, tokens per model (`opus-5-5 2,580,122,562 · haiku-5-5 237,698`), the part written, tool calls and failures, commits (listed up to eight); the subagents count with the session. A report of the current turn is also saved in `.git/metrics/turns.jsonl`, which the **Turns** section lists for a retrospective to compare. A session id as argument (its first characters suffice) reports that session instead of the one changed last.

It is the binary `session-report` of `tools/metrics` (EN-107), the tokens per model included (EN-066's `model-tokens.ts` folded in), and reads the main checkout's transcripts from a worktree too: `scripts/session-report.ts` took the transcripts' folder from the folder it ran in, and failed in a worktree.

## Reading the transcripts

The metrics' tools read the transcripts as `run-totals` does: each transcript from the byte offset reached last time, its complete lines only. `.git/metrics/transcripts-cache.jsonl` keeps, per transcript, the offset and what was read before it — each request with its last line's model, tokens, time, branch and session; each tool call asked; each result with its time and outcome; the rules' [findings](#findings); whether the agent waits for a prompt, and when the current turn started — so that a report reads only what was written since. A transcript shorter than its offset was rewritten and is read again from its start; a cache of another version is dropped and rebuilt. The file is written aside and renamed, so a report reading it meanwhile sees a whole one.

- **A request is counted once**, by its message id, with the values of its last line: a subagent's lines carry a growing `output_tokens`, the last line the final count (EN-092's measure).
- **A call is counted once answered**, from its request's time to its result's; a later result of the same call replaces the first.
- **Measured on one session of 1.1 GB** (2026-10-18): 3.8 s to read it without a cache, 0.16 s after. The lines are parsed by `serde_json`, a fifth faster than the hooks' own reader there.

## At each retrospective

`RET` reads `npm run metrics` and answers with figures (retro skill):

- which commands run most and cost most time: is one worth a script, a cache or a check run less often?
- the **Tools** section, as a tools team reviews its tools: one never called is a candidate for removal, one failing often a bug or a misleading usage, one slow a target for the next story;
- which stories and tasks took longest: their commits say why;
- how many calls failed, and of which kind: a repeated failure gets a check or a rule;
- how tokens grew per story against the last epic: what made the agent search or read more.

## Watching a long task

A command expected to last more than a minute runs in the background (`run_in_background`), never behind a waiting loop, so that the agent stays free and the wait costs no tokens. Claude Code's Monitor watches it with the binary `watch-task` of `tools/metrics` (EN-107, the lines of `watch-task.ts` unchanged), which prints a line only when the task needs a look:

```sh
cargo run --release -q -p metrics --bin watch-task -- <output file> --stall 120 --until "done" --max 1800
```

| Line printed | When |
| --- | --- |
| `stalled: no output for 125 s — last line: …` | the output has not changed for `--stall` seconds |
| `resumed: …` | it changes again after a stall |
| `done: …` | a line starts with `--until`, or the task exited (`[exited with code N]`) |
| `watch over: …` | `--max` seconds passed (Monitor watches 30 minutes at most: start another) |

A long task writes one progress line per step, and stops itself when a step changes nothing.

## Limits

- Sessions started in another folder, such as a worktree, keep their transcripts under another project folder, not read yet.
- Claude Code deletes transcripts older than `cleanupPeriodDays`: `.claude/settings.json` sets it to 3650, so that every retrospective reads its epic's tokens.
- `.git/metrics/` lives in this clone only; `npm run records:save` keeps a copy of it, the transcripts and the library in the private records (ADR-0041, ADR-0044), from which a new machine restores them.
- Tokens are counted per model request, not per command: a command costs the request that wrote it and the one that read its result.
- Temporal is behind `--harmony-temporal` in Node.js 24: the hook command and Vitest pass it.
