# ADR-0078 — The hooks, the Git hooks and the agent call the Rust tools by their binary's path

**Status**: Accepted (the agent, 2026-10-18, under ADR-0069, option C of SP-023's question, to be told to the Product Owner; written by EN-104). Supersedes ADR-0073, decision 1, for the calls it names: the Claude Code hooks, the status line, the Git hooks and the agent's own commands. Decision 1's npm script kept for each of the agent's tools, and the page mapping them, superseded by ADR-0082 (2026-10-18, EN-115): the scripts of a person and of a caller only.

## Context

- ADR-0073, decision 1, runs every Rust tool by `cargo run --release -q -p <crate>`: always the binary of the current sources, Cargo rebuilding what changed.
- SP-023 measured what a call costs before any work ([research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md), Each tool's time before its port): a Rust binary by its path, 2.7 ms; the same through `cargo run`, 49.9 ms; through `npm run -s` and `cargo run`, 149 ms. The launcher costs more than the language: a hook ported to Rust but run by `cargo run` waits about 47 ms of Cargo before its first line.
- The hooks run on every tool call: `guard-bash` before each of the 9,500 Bash calls of three days of F20 (research note 0024), `track-tool-call` before every call, `log-tool-call` after it, the status line every second. EN-104 measured them in Rust: 3 ms each by the binary's path, 47 ms through `cargo run`; the status line 10 ms against 51 ms (EN-104, Measures).
- A binary called by its path is the one last built: built before its sources changed, it runs the old code, and nothing says so.

## Decision

1. **The Claude Code hooks, the status line, the Git hooks and the agent's own commands call `target/release/<binary>`** directly: `.claude/settings.json` in exec form, `${CLAUDE_PROJECT_DIR}/target/release/<hook>` (`REF-CC-HOOKS`); the status line `"$(git rev-parse --show-toplevel)/target/release/status-line"`; `.husky/` by the path from the checkout's root; the agent `target/release/<binary> <arguments>` from the root of its checkout, as `docs/tooling/agent-tools.md` maps each script. `npm run <script>` stays the form for people and the scripts' own composition (`check:all`, `story:finish` under their locks).
2. **A stale binary is met by rebuilds** of the checkout's release binaries, `cargo build --release --workspace --locked`: at `prepare` (`npm install`, a new worktree), after `git:sync`, and after `story:finish` in the checkout of the base it brought up to date.
3. **A check fails naming a stale binary**: `stale-binaries` (`npm run rust:fresh`, the last step of `check:rust`) prints each binary missing or older than a file Cargo built it from, read from its dep-info `target/release/<binary>.d`, and exits 1; the `session-state` hook names them at each session's start.
4. **The library's server keeps `cargo run`**: it starts once a session, where Cargo's 50 ms do not count, and Cargo builds it fresh.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| A — `cargo run` everywhere | always the current sources; nothing to rebuild | about 47 ms on every call: `guard-bash` in Rust 50 ms instead of 56 |
| B — the hooks, the status line and the Git hooks by the path | about 3 ms a hook | a binary older than its sources; the agent's commands still pay `npm run` and `cargo run` |
| C — B, and the agent's own commands by the path (chosen) | about 150 ms saved again on each of the agent's calls (`git:sync`, `page`, `replace`, `backlog:status`) | commands the agent learns by path; a stale binary, met by decisions 2 and 3 |

## Consequences

- `.claude/settings.json`, `.husky/pre-push` and `.husky/commit-msg` name the binaries; `tools/hooks/tests/settings.rs` and `tests/check-targets.test.ts` fail on a hook run otherwise.
- `git-sequences sync` ends with the rebuild and says `release binaries rebuilt`; `story-finish` rebuilds in the base's checkout after its merge; a failed build stops each with Cargo's message.
- A worktree's binaries are its own (`target/` is per checkout, built by `worktree:new`); the hooks run the main checkout's, rebuilt when `story:finish` brings its base up to date. Between a binary's removal and its new link by Cargo, a hook starting finds no file and its call goes through: a rebuild lasts seconds, a hook's start milliseconds.
- `docs/tooling/agent-tools.md` holds the table of the binaries the agent calls; the worker brief and the project rules name them.

## References

ADR-0021, ADR-0069, ADR-0072, ADR-0073, `REF-CC-HOOKS`, [research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md), [EN-104](../backlog/stories/E17-F21-EN-104-claude-code-hooks-in-rust.md), `docs/tooling/agent-tools.md`
