# Metrics of the agent's work

Measure first, then make each epic faster than the last (Product Owner, RET-001): which commands the agent runs most, which take longest, how long each task, story, feature and session takes, and how many tokens they use.

## Two sources

| Source                                  | Written by                                                                                                               | Holds                                                                                                                                                                    |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `.git/metrics/tool-calls.jsonl`         | the hook `.claude/hooks/log-tool-call.ts`, after each tool call (`PostToolUse`, `PostToolUseFailure`, in the background) | one line per call: time, session, branch, subagent, tool, what it ran (command, file, pattern; 300 characters at most), exact duration, failure and its first error line |
| `~/.claude/projects/<repository path>/` | Claude Code: one JSON Lines transcript per session, subagents included                                                   | every request with its tokens, every tool call and its result, each line with its time, session and branch                                                               |

The hook's log sits inside the Git directory: never committed, shared by every worktree, kept when transcripts are cleaned up. Its durations are exact: Claude Code measures the tool's execution and leaves out permission prompts (`duration_ms`, `REF-CC-HOOKS`). For calls made before the hook existed, the report estimates the duration from the transcript, from the request to the result; this also counts the time spent writing the command, so a long heredoc looks slow.

## The report

`npm run metrics` prints a Markdown report (`npm run metrics > report.md` to keep it):

| Section                | What it shows                                                                                                                          |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| Commands by count      | shell commands grouped by what they run (`npm run check`, `git commit`, `gh pr checks`, `grep`): count, total, median, longest, failed |
| Commands by total time | the same groups, by total time: where waiting goes                                                                                     |
| Slowest calls          | the 20 longest tool calls, exact or estimated, with their branch                                                                       |
| Sessions               | each session from the day it started: active time, tokens, tool calls, failures — the session's timer                                  |
| Features               | the same totals per feature (stories mapped through their file names)                                                                  |
| Stories and branches   | per story (`US-018`), feature branch (`F04`) or other branch (`main`, `develop`)                                                       |
| Tasks, by commit       | per task commit (`Refs: US-018.T2`), with its short hash: read `git show <hash>` to see why a task was slow                            |

- **Active time** adds the gaps between moments of work (a request, the end of a tool call), each gap capped at 5 minutes: a pause, a night or a wait for the Product Owner is not counted as work.
- A moment belongs to the story of the branch it ran on, and to the first task commit of that story made after it. Work after a story's last task commit (pull request, wait for checks) shows as `<story> (no task commit)`.
- **Output tokens** are those the model wrote; **all tokens** add input, cache writes and cache reads, which dominate.
- **Failed** counts the tool calls that ended in an error: a failing check, a refused commit, a denied command. Their causes are read in the slowest calls, the log, or the transcript.

## Status line

The status line at the bottom of Claude Code (`.claude/hooks/status-line.ts`, `REF-CC-STATUSLINE`), refreshed every second, has three lines:

```text
E01 › RET-001 retrospective e01 · status line, documentation
● active · ⏱ turn 12 min · 3,815,104 tokens this turn (14,681 written) · conversation memory 41 % full · Opus 5.5
▶ Bash · Run the checks — 7 s
```

| Part                              | Source                                                                                                                                                                                                                                          |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| epic › feature › story            | the branch checked out, matched with the backlog's file names (`status-place.ts`); the branch itself outside a story                                                                                                                            |
| the step                          | `.git/metrics/step`, one line the agent writes at each step; `waiting` when empty                                                                                                                                                               |
| `● active` / `◌ silent for 3 min` | time since the session's transcript last changed: silent for more than 2 minutes means waiting, or stalled                                                                                                                                      |
| `⏱ turn`                          | time since the current turn started: the Product Owner's prompt after the agent last ended its turn; a message sent while the agent works, or a compaction, stays in the turn                                                                   |
| tokens this turn                  | exact sum of the turn's requests, read from the transcript, the same figures as the session report; the part the model wrote in brackets                                                                                                        |
| conversation memory               | how full the context window is (`context_window.used_percentage`): near 100 %, Claude Code compacts the conversation into a summary                                                                                                             |
| `▶`, third line                   | the oldest tool call still running, its description (or its command) and for how long: written by the `track-tool-call` hook when it starts, removed by `log-tool-call` when it ends; a call started before the turn (interrupted) is not shown |

The tokens are summed incrementally: `.git/metrics/status-line-turn.json` keeps where the reading stopped, so each refresh reads only the new lines (about 0.1 s). The turn, not the terminal: Claude Code's own figures (`cost.total_duration_ms`, the session's tokens) add up every turn since the terminal opened, which says nothing about the work in progress. Popular status lines (ccstatusline, `REF-CCSTATUSLINE`) show the same families of facts — model, branch, tokens, usage limits, session time; usage limits are left out, Claude Code already shows them. This one adds the backlog place and the call running, which no generic one knows.

## Session report

Every message that ends a piece of work closes with a session report (`CLAUDE.md`; [writing](../conventions/writing.md#messages-to-the-product-owner)):

```sh
npm run metrics:session              # the current turn, framed and colored for the terminal
npm run metrics:session -- --markdown # the same as a Markdown table, for the chat
npm run metrics:session -- --all      # the whole conversation
```

It gives one row: the duration (wall and active), the tokens, the part the model wrote, the tool calls and failures, the commits (listed when there are eight or fewer). Each report is also saved as one line in `.git/metrics/turns.jsonl`; the **Turns** section of `npm run metrics` lists every turn, so that a retrospective compares them. A turn starts at the first prompt after the agent last ended its turn; messages sent while it works stay in the turn.

## At each retrospective

`RET` reads `npm run metrics` — commands, slowest calls, sessions, features, stories, tasks, **turns** and **sources** — and answers, with figures (retro skill):

- which commands run most and which cost most time: is one worth a script, a cache or a check run less often?
- which stories and tasks took longest: read their commits (`git show <hash>`) to see why;
- how many tool calls failed, and of which kind: a repeated failure gets a check or a rule;
- how tokens grew per story compared with the last epic: what made the agent search or read more.

## Limits

- Sessions run from another folder (a worktree in the scratchpad) have their transcripts under another project folder: they are not read yet.
- Claude Code may delete old transcripts; the hook's log keeps the calls, not the tokens.
- Temporal is behind `--harmony-temporal` in Node.js 24: the hook command, `npm run metrics` and Vitest pass it.
