# 0024 — Sources of F20: a context diet for every agent

Spike SP-022, 2026-10-17, for F20 (E17). Each page below was read in the local library before it was cited; the rows are in [references](../../references.md), under _A context diet for agents_, and in the rows `REF-CC-ENV-VARS`, `REF-CC-HOOKS` and `REF-CC-SUBAGENTS`, read again the same day. Claude Code 2.1.302 runs on the machine.

The measures come from the project's own records: the hook log `.git/metrics/tool-calls.jsonl` from 2026-10-14 18:00 UTC to 2026-10-17 (26,794 calls, 22,956 with their sizes), and the transcripts of the session of 2026-10-17 (1,884 requests of the main thread, 15,379 of its 74 subagents). The scripts that counted them stay in the scratch folder; F20's stories rebuild the measures that last as Rust tools.

## What a call costs

| Measure | Main thread | Subagents |
| --- | --- | --- |
| Tokens per request, median (almost all cache reads) | 356,156 | 165,667 |
| Seconds from a tool's result to the next answer, median | 3.5 | 2.6 |
| Characters per token of a tool's result, median | 2.4 | 1.9 |
| The model's share of a turn's time, latest turns | 50 to 99 %, mostly over 90 % | — |

So the time of a tool itself hardly counts: a call through Bash runs in about 70 ms, the built-in `Read` in 3 ms, the `library` server in 12 ms, while the model takes 2.6 to 3.5 s before the next call. **What a tool of our own can save is round trips and characters, not milliseconds**: each call avoided saves about 3 s and, in a subagent, about 166,000 tokens read from the cache; each character of a result stays in the context and is read again by every later request.

Priced at the rates of Claude Opus 5.5 (`REF-CLAUDE-PROMPT-CACHING`), the subagents of 2026-10-17 cost the equivalent of about $903 (cache reads $703, cache writes $176, output $25) and the main thread about $187: cache reads are 78 % of a subagent's cost, so the size of what an agent carries, times its number of requests, is the lever. The Product Owner works on a subscription, where these figures are a share of the usage limits rather than a bill; the proportions hold.

## EN-064 — The one-hour prompt cache

| Finding | Source |
| --- | --- |
| `CLAUDE_CODE_PROMPT_CACHE_TTL` sets the main conversation's cache, `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` the subagents' (`5m` or `1h`); `experimental.cacheTtl` does it for one agent type | `REF-CC-ENV-VARS`, `REF-CC-SUBAGENTS` |
| A subscription within its included usage already gets the one-hour cache on the main conversation | `REF-CC-ENV-VARS` |
| A one-hour write costs 2 times the input price, a five-minute write 1.25 times, a hit 0.05 times on Opus 5.5 and Sonnet 5.5 | `REF-CLAUDE-PROMPT-CACHING` |

**Measured** on 2026-10-17: the main thread wrote 2,397,275 tokens to the one-hour cache and none to the five-minute one; the subagents wrote 35,181,446 to the five-minute cache and none to the one-hour one. So the main thread already has what EN-064 asked for.

Over every subagent transcript kept (106.2 million tokens written), 52 requests came after a pause of more than 5 minutes and rewrote 9.2 million tokens (8.6 %); 38,658 came within 5 minutes. A one-hour cache for subagents would have turned those rewrites into hits but billed every write at 2 times instead of 1.25: about $779 instead of $531 at Opus 5.5's rates, **47 % more**. It pays only when more than 38 % of the writes are rewrites after a pause (`(1 − f) × 8 + f × 0.2 < 5`).

**Answer**: EN-064 has nothing left to set. The story becomes a record of this measure, unless a worker type is found that waits more than 5 minutes often (the heavy lock), for which `experimental.cacheTtl: 1h` on that type alone would be measured.

## EN-065 — Workers started from a digest

| Finding | Source |
| --- | --- |
| `omitClaudeMd: true` starts a subagent "without the user, project, and local CLAUDE.md files"; a subdirectory's `CLAUDE.md` and the path-scoped rules still load when it works there | `REF-CC-SUBAGENTS` |
| `skills` preloads a skill's full content; `tools` narrows the tools, whose definitions are part of the initial context; `mcpServers` gives a server to one agent type only | `REF-CC-SUBAGENTS` |
| "The main conversation's auto memory isn't loaded" in a subagent | `REF-CC-SUBAGENTS` |

**Measured**, first request of each subagent on 2026-10-17:

| Agent type | Agents | First request, tokens | Requests per agent | Tokens per agent |
| --- | --- | --- | --- | --- |
| `general-purpose` (the run's workers) | 68 | 38,905 | 213 | 50.7 million |
| `auditor` | 2 | 15,566 | 195 | 25.6 million |
| `verifier` | 2 | 14,093 | 129 | 14.0 million |
| `doc-editor` | 2 | 13,867 | 87 | 10.0 million |

The project's agent types, which list their tools, start about 24,000 tokens lighter than a `general-purpose` worker, although both load `CLAUDE.md` and `AGENTS.md`: most of the difference is the tools' definitions (every MCP server and every tool) and the longer task message. Carried over 213 requests, 24,000 tokens are 5.1 million tokens read per worker, about 10 % of a worker's tokens. `AGENTS.md`, `CLAUDE.md` and the memory index are 22,938 characters together; the memory index alone 6,753.

**Contradiction found**: this spike's own worker received the memory index (`MEMORY.md`) in its context, although the page says a subagent does not load the auto memory. EN-065 counts it with `context:size` and reports it rather than trusting either.

**Answer**: a `worker` agent type of our own, with `omitClaudeMd`, the worker brief preloaded as a skill (or its digest), and a `tools` list of what a story needs, is the sourced way; how far each task type can be cut is measured per type (story worker, research, audit, log writer) with `context:size` before and after, as the Product Owner asked (first round, remark 1).

## EN-066 — A model per agent type

| Finding | Source |
| --- | --- |
| `model` in an agent's file takes `sonnet`, `opus`, `haiku`, `fable`, a full id or `inherit`; the `Agent` call's own `model` comes first, then the file, then `CLAUDE_CODE_SUBAGENT_MODEL`, then the main conversation's | `REF-CC-SUBAGENTS`, `REF-CC-ENV-VARS` |
| Sonnet 5.5 costs half of Opus 5.5 on every kind of token | `REF-CLAUDE-PROMPT-CACHING` |
| A subagent's context window is sized by its own model; thinking follows the main conversation's | `REF-CC-SUBAGENTS` |

Every agent file of the project says `model: inherit`; on 2026-10-17, 71 subagents ran on Opus 5.5 and 3 on Haiku. A worker costs the equivalent of about $12 on Opus 5.5 and would cost about $6 on Sonnet 5.5, if it needs no more requests. The session report already reads each request's `model`, so the tokens per model need only a column.

## EN-086 — A push at a question or a blocking failure

| Finding | Source |
| --- | --- |
| `PushNotification` sends a desktop notification, and a phone push when Remote Control is connected | `REF-CC-TOOLS-REFERENCE` |
| Phone pushes need the Claude app signed in with the same account, Remote Control active and **Push when Claude decides** on in `/config`; Claude decides when to push, with no setting per event | `REF-CC-REMOTE-CONTROL` |
| Remote Control starts with `/remote-control`, or for every session through `remoteControlAtStartup: true` in the user's own settings; a project's settings can turn it off, never on | `REF-CC-REMOTE-CONTROL` |
| Pushes are skipped while the user types in or focuses the terminal; `CLAUDE_CLIENT_PRESENCE_FILE` extends that to any time at the machine | `REF-CC-REMOTE-CONTROL`, `REF-CC-ENV-VARS` |
| A hook's `terminalSequence` raises a desktop notification only | `REF-CC-HOOKS` |

`PushNotification` was called once in the whole log. **Answer**: since only the agent decides when to push, the run skill states when: a question waiting for the Product Owner, or a failure that blocks the run, sent while the run goes on with independent work (first round). The phone push needs one setting on the Product Owner's side: Remote Control on for every session in their user settings, and the push option in `/config`; the repository cannot set it.

## EN-087 — A progress page as an Artifact

| Finding | Source |
| --- | --- |
| Publishing again from the same file updates the page at the same address, seen in place by anyone who has it open; each publish is a version | `REF-CC-ARTIFACTS` |
| A new session makes a new page unless it is given the address | `REF-CC-ARTIFACTS` |
| The page is static: it reaches no local file, only its own origin and Google Fonts; 16 MiB at most | `REF-CC-ARTIFACTS` |
| A styled page costs more output tokens than terminal text | `REF-CC-ARTIFACTS` |
| A publish from a turn started by a scheduled task asks first | `REF-CC-ARTIFACTS` |

**Answer**: the page is written by a Rust tool from the backlog files and the metrics, then published from that file, so a publish costs one call and no page text written by the model; its address is kept in `.git/metrics/` so that a run continued in a new session updates the same page. Its content, from the first round: the tree with statuses, durations per story and feature, the agents running, tokens per story and per run, and the commits of each story and feature.

## EN-088 — The SubagentStop hook

| Finding | Source |
| --- | --- |
| `SubagentStop` gets `agent_id`, `agent_type`, `agent_transcript_path`, `last_assistant_message`, `stop_hook_active`, and the parent's `background_tasks` | `REF-CC-HOOKS` |
| With `SubagentHandback` the report is the call's `tool_input.message`, seen by a `PostToolUse` hook on that tool; `last_assistant_message` is only the closing text | `REF-CC-HOOKS`, `REF-CC-TOOLS-REFERENCE` |
| `decision: "block"` with a `reason` keeps the worker running, the reason its next instruction; at most 8 continuations in a row | `REF-CC-HOOKS` |
| To add context to the main thread after a subagent returns, a `PostToolUse` hook on `Agent` | `REF-CC-HOOKS` |

**Measured**: a worker's transcript names the main checkout as `cwd` and the main checkout's branch as `gitBranch`, never its worktree. So the hook finds the worker's branch from what the worker did, not from its input: the worktree named in its first `cd` (the hook log ties each call to its `agentId`), or a line the brief makes the report start with. Then: branch pushed and up to date with `origin`, else `block` once with what is missing; pushed, then a line for the main thread naming the story and `story:finish`.

## EN-092 — The totals of a run

| Finding | Source |
| --- | --- |
| A compaction keeps the session's file and id: the session of 2026-10-17 holds 2 compactions in one file, another 12 | measured |
| Each subagent writes its own transcript under the session's `subagents/` folder | `REF-CC-HOOKS`, measured |
| The compaction itself is recorded (`compactMetadata`: 471,456 tokens before it, 54 s) | measured |

**Answer**: a run's totals are the sums of the session's transcript and its subagents' from the run's start, which `/run` writes to `.git/metrics/run.json` with the session's id; a session started later while the run is open adds its id. The status line and `metrics:session -- --run` read the same file. The report at the stop holds only the run's duration and tokens (first round); measured gains and incidents go to the agent's logs and the retrospective.

## The agent's most frequent calls: shell or a tool of our own

Counted from 2026-10-14 18:00 UTC to 2026-10-17:

| Calls | Count | Median time | Mean result, characters |
| --- | --- | --- | --- |
| reading a file through Bash (`cat`, `sed`, `head`, `tail`, `awk`) | 4,707 | 73 ms | 4,341 |
| searching through Bash (`grep`, `rg`) | 2,535 | 65 ms | 2,590 |
| `Read` | 429 | 3 ms | 8,417 |
| Git | 2,297 | 22 ms to 2.5 s (`commit`, with its hooks) | 1,014 to 11,740 (`diff`) |
| `library_search`, `library_read`, `library_get` | 315, 369, 211 | 11, 12, 265 ms | 1,118, 4,567, 2,774 |
| `WebFetch`, `WebSearch`, `curl` | 58, 34, 32 | 6.0 s for `WebFetch` | 1,484 (a summary) |
| the memory files | 27 | — | — |

Timed on the machine, 20 runs each: `git status` 46 ms, `git log` 28 ms, both from one Rust binary 72 ms; `cat` of a 13 kB file 5 ms; `node` starting 52 ms, the `library` CLI 137 ms; `zsh` 17 ms without the profile.

| Kind | What the measure says | Verdict |
| --- | --- | --- |
| **Git** | 793 of the 2,297 calls followed another Git call of the same agent, because a worktree refuses chained Git commands: `add` then `commit` 59 times, `commit` then `push` 46, `fetch` then `rebase` 39, `fetch` then `diff` 18. Each saved call is about 2.6 s and 166,000 tokens: about 34 minutes and 132 million tokens over the three days | **worth a tool**: Rust commands for the frequent sequences (`fetch` and `rebase` on the feature branch; `add` and `commit` from a message file), each one call |
| **The `cd` prefix** | every Bash call of a worker starts with `cd "<absolute worktree>" \|\| exit 9`: 61 characters on average, 480,442 characters written on 2026-10-17, about 6 % of the output tokens of the day, the slowest and dearest tokens | **no tool**: a relative `cd .claude/worktrees/<id> \|\| exit 9` halves it, since a worker's calls start in the main checkout |
| **Reading files** | `Read` is already 3 ms and needs no `cd`, but adds a line number to each line, about 9 % more characters for a 13 kB file, kept in the context for every later request; `cat` and `sed` read as fast for the model. The files read most are `docs/references.md` (121 times, 725,000 characters) and `docs/process/improvements.md` (98, 532,000) | **no server**: reading less matters more than reading faster — F20's digests, and a Rust command that prints one row of the references or one section of a page by its id or heading, as `library_read` does for the web |
| **Web** | `WebFetch` takes 6 s and returns a summary written by a small model, which cannot be cited; the library serves a stored page in 12 to 265 ms and keeps its text; 58 `WebFetch` and 32 `curl` calls went around it (74 % of the pages were served by the library) | **worth a tool**: a `PreToolUse` hook that sends `WebFetch` and `curl` of a page to `library_get`, `WebFetch` kept only for a page the library returns empty; the library's `uses` table gains the project that asked, so that the library can serve several projects |
| **Memory** | 27 calls in three days, the index loaded once per session | **no tool** |

Each tool of our own records its own use — the call, its time, its outcome — into `.git/metrics/tool-uses.jsonl`, through one function shared by the Rust crates, and `npm run metrics` lists it; the retrospective reviews the list as a tools team would.

## Drafted stories

| # | Title | Criteria (draft) |
| --- | --- | --- |
| 1 | Git sequences as Rust commands | Given a sequence seen more than 30 times in the log (`fetch` and `rebase`, `add` and `commit`), when the agent needs it, then one call does it; the count of Git calls per story is compared before and after |
| 2 | Every page fetched through the library, and the project recorded | Given a `WebFetch` or a `curl` of a page, when it is called, then a hook sends it to `library_get` unless the library returned the page empty; each use records its project |
| 3 | Each tool of our own records its use | Given a call of a Rust tool, when it ends, then one line holds its name, time, duration and outcome, and `npm run metrics` lists it |
| 4 | One row or section of a repository page | Given an id such as `REF-CC-HOOKS` or a heading, when asked, then the command prints that row or section only |

Story 2 touches the `library` server, which F21 rewrites in Rust: written in its Rust version if F21 comes first.

## The Product Owner's first round (2026-10-17)

| Subject | Decision |
| --- | --- |
| Progress page (EN-087) | the tree and statuses, durations per story and feature, the agents running, tokens per story and per run, and the commits of each story and feature; the agent may add other useful views |
| Notifications (EN-086) | only when a question waits for the Product Owner and when a failure blocks, sent while the run keeps going on other independent work, so that the Product Owner can unblock it on return; never at a plain stop nor at each merge |
| Run size | the Product Owner keeps control: a run delivers no more than they can test on return, never two epics in a working day; a bound is asked in the second round |
| Run report (EN-092) | only the run's duration and tokens; measured gains and incidents go to the agent's logs and the retrospective |
| Context (EN-065) | each agent receives what its task needs, smaller and better made, not everything |
| Agent-facing docs (EN-096) | reworded as principles, so that the idea behind a decision is kept |
| Tools of `package.json` | their move to Rust (commitlint and the others) is F21's, not F20's |

## The second round, for the Product Owner

Only what the first round and the measures leave open.

1. **How big may a run be?** A: a number of stories, for example 8 stories at most. B: a number of features waiting for the Product Owner's test, for example 1 feature with its `VAL` ready per run. C: a time, for example 8 hours of active time. D: B, with C as a ceiling.
2. **The one-hour cache (EN-064)**: the main thread already has it; for subagents it would cost 47 % more on cache writes for 8.6 % of writes saved. A: close EN-064 with this measure. B: try `1h` for one run on the workers only and compare. C: try it only on a type that waits for the heavy lock.
3. **How far to cut a worker's start (EN-065)**: today 38,905 tokens at its first request, about 24,000 more than the project's typed agents. A: a `worker` type with the tools a story needs and the brief preloaded, `CLAUDE.md` kept. B: the same without `CLAUDE.md` (`omitClaudeMd`), the brief carrying every rule a worker needs. C: B, and one type per task kind (story, research, audit, logs), each measured.
4. **Which model for which agent (EN-066)**: a worker costs about $12 of tokens on Opus 5.5, about $6 on Sonnet 5.5. A: Sonnet for logs, backlog, merges and step texts; Opus for everything else. B: A, and Sonnet tried on one story worker, measured and its result checked by the audit. C: Opus everywhere, as today.
5. **Phone pushes (EN-086)**: they need Remote Control on for every session in your own settings and the push option in `/config`; a desktop notification needs nothing. A: both, set once on your side. B: desktop only.
6. **The tools of our own**: A: the four drafted stories in F20. B: Git sequences and the library in F20, the rest in F21. C: the shorter `cd` only, no new tool. A read of `docs/references.md` prints about 6,000 characters today, 121 times in three days; one row is a few hundred.
7. **The agent-facing docs as principles (EN-096)**: A: one pass over every page, in one story. B: one story per family (rules and skills, conventions, process), each measured in tokens before and after. C: only the pages a worker reads, the others when touched.
8. Any remark, idea or constraint for F20.

## The Product Owner's second round (2026-10-17), questions 1 to 4

| Subject | Decision |
| --- | --- |
| Run size | no bound set by the agent: the Product Owner sets a run's scope at its start, as a time ("until I am back tonight") or an end ("stop at epic E02"). A question never stops a run: it parks its story and the run goes on with independent work. A run stops only on a problem it cannot get around, at the end of its scope, or when the Product Owner says so |
| Notifications (EN-086) | only for a question or a blocker that prevents finishing the scope asked; never to say the run ended early |
| One-hour cache (EN-064) | closed with the measure above: nothing to set |
| Workers' start (EN-065) | one worker type per task kind, each measured before and after (option C) |
| Model per agent (EN-066) | Sonnet for logs, backlog, merges and step texts, and one measured trial of Sonnet on a real story worker compared with Opus (option B) |

Questions 5 to 8 are still open.
