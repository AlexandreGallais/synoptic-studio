---
id: EN-104
epic: E17
feature: F21
title: The Claude Code hooks and the status line in Rust
status: draft
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

Before, measured by SP-023 on F20's versions ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), Each tool's time before its port), median of 20 calls: `guard-bash` 56.1 ms, `track-tool-call` 86.6 ms, `log-tool-call` 105.5 ms, `session-state` 161.1 ms, the status line 169.9 ms on a transcript of 217 MB. A Rust hook binary answers in 2.7 ms by its path and in 49.9 ms through `cargo run --release -q --manifest-path …`, the way `.claude/settings.json` calls the Rust hooks today: the question below decides which the port uses.

## Questions for the Product Owner

**How are the Rust tools called?** (SP-023, 2026-10-18.) ADR-0073, decision 1, says `cargo run --release -q -p <crate>`. Measured: a binary by its path, 2.7 ms; through `cargo run`, 49.9 ms; through `npm run -s` and `cargo run`, 149 ms. Over the three days counted in F20's research note 0024, the agent made at least 9,500 Bash calls, and `guard-bash` runs before each.

- **A — keep `cargo run` everywhere**: always the binary of the current sources; `guard-bash` in Rust waits about 50 ms instead of 56 ms today, about 8 minutes per 9,500 calls instead of 9.
- **B — the hooks, the status line and the Git hooks call `target/release/<binary>`**: about 3 ms, 30 seconds per 9,500 calls; the risk, a binary older than its sources, met by `rust:build` at `prepare`, after `git:sync` and after `story:finish`, and by a check that fails naming the stale binary; a new ADR supersedes ADR-0073's decision 1 for these calls.
- **C — B, and the agent's own commands call the binaries too** (`git:sync`, `backlog:status`, `replace`, `library`…), `npm run` kept for people: about 150 ms saved on each such call, at the cost of commands named by their path.

The story waits for the answer: its criteria hold under each option, but its times and `.claude/settings.json` differ.

## Tasks

One task = one commit, referenced as `EN-104.Tn`.

- [ ] T1 — `guard-bash` in Rust, its cases carried; the folder a leading `cd` enters, the brief's form accepted, a script missing from `package.json` refused (5 h)
- [ ] T2 — `track-tool-call`, `log-tool-call` and `session-state` in Rust, on recorded inputs (4 h)
- [ ] T3 — The status line and its parts in Rust, a request counted from its last line; `settings.json` and the docs; times measured (6 h)
