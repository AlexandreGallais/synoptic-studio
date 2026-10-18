---
id: EN-104
epic: E17
feature: F21
title: The Claude Code hooks and the status line in Rust
status: done
points: 5
---

# E17 · F21 · EN-104 — The Claude Code hooks and the status line in Rust

The hooks still run by Node.js — `guard-bash`, `track-tool-call`, `log-tool-call`, `session-state` — and the status line with its parts (`status-json`, `status-place`, `status-run`, `status-tasks`, `status-tokens`, `status-usage`) rewritten in the `hooks` crate, their output and exit codes unchanged (F21.AC1). Measured by SP-021 ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), Hooks and agent tools): `guard-bash` 59.9 ms, waited for before every Bash call; `track-tool-call` 94.2 ms before every call; the status line 135.7 ms every second; a Rust binary reading the hook's input 3.0 ms.

Frictions of F20's run folded in (2026-10-17):

- `guard-bash` judges a subagent's Git commands against the session's working folder, the main checkout, not the worktree its command enters with `cd`; and its refusal of a chained Git command also refuses the brief's own form, `cd "<worktree>" || exit 9` then one Git command.
- `npm run -s git:sync` in a branch made before EN-100 fails with no output: the script is missing from that branch's `package.json`, and `-s` hides npm's message.
- EN-092's note: a subagent's transcript repeats a request's usage on each of its lines with a growing `output_tokens`, so the status line's tally takes a request's last line, not its first.

## Acceptance criteria

- Given each hook and the status line, when its Rust binary runs on the inputs of its TypeScript tests and on recorded inputs of a session, then its output and exit code are the same, and `.claude/settings.json` calls the binary; the TypeScript file and its test are removed in the same task, and `tools/hooks`' temporary twin of `status-tasks.ts` becomes the only one.
- Given a command that starts with `cd "<folder>"`, when `guard-bash` judges it, then it judges the rest of the command in that folder, not in the session's.
- Given `cd "<worktree>" || exit 9` followed by one Git command, when `guard-bash` judges it, then it lets it through; two Git commands chained after it are still refused, with the same hint.
- Given `npm run <script>`, `-s` or not, when the script is missing from the `package.json` of the folder the command runs in, then `guard-bash` refuses it, naming the script and the branch, and saying that a branch made before the script gets it by a rebase on its base, `git fetch origin` then `git rebase origin/<base>`, one call each.
- Given a subagent's transcript, when the status line counts its tokens, then each request is counted once, from its last line.
- Given each binary, when it is timed, then its time per call before and after is stated, and the status line's CPU share over one minute.

## Files

`tools/hooks/` (new binaries and modules), `.claude/settings.json`, `.claude/hooks/*.ts` and their tests (removed), `docs/tooling/agent.md` (Hooks), `docs/tooling/agent-tools.md`, `.claude/skills/worker-brief/SKILL.md`.

## Measures (ADR-0069)

Before (0023's table, measured again by SP-023) and after, 20 to 30 calls each on this Mac, the release build: `guard-bash`, `track-tool-call`, `log-tool-call`, `session-state`, the status line; the status line's CPU time over 60 draws.

Before, measured by SP-023 on F20's versions ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), Each tool's time before its port), median of 20 calls: `guard-bash` 56.1 ms, `track-tool-call` 86.6 ms, `log-tool-call` 105.5 ms, `session-state` 161.1 ms, the status line 169.9 ms on a transcript of 217 MB. A Rust hook binary answers in 2.7 ms by its path and in 49.9 ms through `cargo run --release -q --manifest-path …`, the way `.claude/settings.json` calls the Rust hooks today: the port uses the binary by its path (Decisions).

After, measured on 2026-10-18 at 01:55 on this Mac, load 2.0 to 2.4, the same method (a process started and waited for, 20 calls after one to warm, median and mean in milliseconds), each hook in a scratch repository of its own on the inputs it meets: a Bash call `cd "<worktree>" || exit 9; git status`, its `PostToolUse` with a result of 2,000 characters, the status line on this session's transcript of 223 MB, its cache warm.

| Hook | Before, TypeScript as `settings.json` ran it | After, the binary by its path | After, through `cargo run --release -q --manifest-path …` |
| --- | --- | --- | --- |
| `guard-bash` | 58.8 (58.9) | 2.9 (3.0) | 46.9 (47.6) |
| `track-tool-call` | 89.3 (90.5) | 3.0 (3.0) | 46.6 (47.9) |
| `log-tool-call` | 106.1 (107.1) | 3.0 (3.1) | 46.9 (47.6) |
| `session-state` | 120.6 (122.5) | 25.4 (26.6) | 69.8 (71.3) |
| the status line, through `sh -c` as `settings.json` runs it | 180.4 (181.3) | 10.0 (10.3) | 50.7 (52.4) |

The status line's CPU time over 60 draws, one a second: 14,682 ms before, 24.5 % of one core, since each draw read the whole transcript before slicing it at the cache's offset; 1,156 ms after, 1.9 % of one core, the transcript read from its offset. `track-tool-call`, `log-tool-call` and the status line read the Git folder and the branch from the files Git writes instead of starting Git; `session-state` still starts it twice, for `git status` and `git log`.

## Decisions

- The agent, 2026-10-18, under ADR-0069, to tell the Product Owner: option C of SP-023's question. The hooks, the status line, the Git hooks and the agent's own commands call `target/release/<binary>` directly, `npm run` kept for people. A stale binary is met by a rebuild at `prepare`, after `git:sync` and after `story:finish`, and by a check that fails naming it. A new ADR, written by this story, supersedes ADR-0073 decision 1 for these calls.
- The agent, 2026-10-18, under ADR-0069, since a refusal costs a call: after `cd "<worktree>" || exit <n>;`, one Git command piped into read-only filters goes through — `head`, `tail`, `grep`, `wc`, `cut`, `sort`, `uniq`, `sed -n`, `awk` without `system` nor a redirection — with or without `2>&1`; a second Git command, or any other command chained after it by `;`, `&&` or `||`, is still refused. Without that `cd`, a pipe after a Git command in a worktree is refused as before.

## Tasks

One task = one commit, referenced as `EN-104.Tn`.

- [ ] T1 — `guard-bash` in Rust, its cases carried; the folder a leading `cd` enters, the brief's form accepted, a script missing from `package.json` refused (5 h)
- [ ] T2 — `track-tool-call`, `log-tool-call` and `session-state` in Rust, on recorded inputs (4 h)
- [ ] T3 — The status line and its parts in Rust, a request counted from its last line; `settings.json` and the docs; times measured (6 h)
- [ ] T4 — The new ADR superseding ADR-0073 decision 1 for these calls; the binaries called by path, the rebuilds at `prepare`, after `git:sync` and after `story:finish`, and the stale-binary check (3 h)

## For the main thread

- The equivalence, on recorded inputs: the 29,804 distinct Bash commands of the transcripts give the TypeScript `guard-bash` logic and its port the same output, byte for byte; 20,000 tool calls rebuilt from the transcripts give the same `tool-calls.jsonl` lines and running files; the 337 versions of the feature files in the history the same session lines; the 13 main transcripts the same status lines and files, times masked. The tallies of 63 transcripts match but for the subagents' tokens, now counted from a request's last line: their written tokens were undercounted up to tenfold.
- Judged in the folder a leading `cd` enters (criterion 2), `guard-bash` refuses 2,356 of the 29,804 recorded commands it let through: a worker's `cd "<worktree>" || exit 9` followed by a Git command and another command (`git status; ls`, `git log | head`). Of the 2,047 calls whose result the transcripts hold, Claude Code ran 2,033 without refusal, since the session's folder was the main checkout. 662 other refusals keep their reason with another worktree named or the binary's form in their hint, 34 are lifted (a command entering the main checkout from a worktree), 8 are new for a script missing from `package.json` and 1 for `golden:write` in a worktree entered by `cd`. The criterion and the brief's "one Git command per call" ask for the refusal; a Git command piped into read-only filters is let through since (Decisions).
- `.claude/hooks/status-json.ts` stays, with the folder's entries in `tsconfig.json`, `package.json`, `.oxlintrc.json`, `knip.json` and `tools/lint`: `scripts/activity.ts`, `metrics-time.ts` and `metrics-tools.ts`, EN-107's, import it. EN-107 or EN-113 removes it with them.
- The agent's commands by path (ADR-0078) are named in the worker brief, the project rules, `AGENTS.md` and `docs/tooling/agent-tools.md`; the `run`, `story`, `audit` and `retro` skills and the `clerk` agent still write `npm run <script>`, which works, to change when each is next edited. `story-finish` still starts `move-rows` and `format` through `cargo run`, and `.mcp.json` the library's server (ADR-0078, decision 4).
- `tools/test-support` was merged by EN-103 during this story: the hooks' new tests start their commands through `test_support::command`.
