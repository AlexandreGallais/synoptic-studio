---
id: EN-119
epic: E17
feature: F31
title: The guard reads a command by its words
status: ready
points: 3
---

# E17 · F31 · EN-119 — The guard reads a command by its words

From the card C2 and the rows I12, I22, I40, I87, I93 and I103 (RET-004), with AUD-023's table of the guard's cases. Moved from F23 on 2026-10-18 (SP-024). Its former criterion on a feature branch pushed with lease after its rebase onto `develop` (ADR-0084, decision 3) is dropped: ADR-0086 merges a feature into `develop` by a merge commit and supersedes that rebase.

## Acceptance criteria

- Given a worktree, when a worker calls `target/release/git-sequences state`, then it prints the branch, its base, `status --short`, `diff --stat` and the commits ahead, in one call (I93).
- Given the guard, when it reads a command, then it splits it by its words as the shell does: a `|` or a `;` inside quotes is not a chain, a chain of read-only Git commands after `cd <worktree> || exit 9` goes through, a multi-line commit message and a URL in it go through; `mkdir …heavy.lock` and a `python3` writing a file in a worktree are refused, each with the command to use instead (C2, I22, I40, I87, I103).
- Given a worker whose prompt names its worktree, when it writes a file into the main checkout, then the write is refused, naming the worktree's path for the same file (I12).

## Tasks

- [x] T1 — Each criterion's tests in `tools/hooks` and `tools/git-sequences`, AUD-023's table of cases among them, failing (1 h)
- [x] T2 — The guard on its word reader, the writes outside the worktree refused (2 h)
- [x] T3 — `git-sequences state`, the worker brief and `docs/tooling/agent-tools.md` (1 h)

One task = one commit, referenced as `EN-119.T1` in the commit footer.

## For the main thread

The configuration the story needs, left to the main thread: a subagent does not change `.claude/settings.json`, `CLAUDE.md`, `AGENTS.md`, `.claude/rules/` nor `.claude/skills/` on another agent's request, which is no consent of the Product Owner's.

- `.claude/settings.json`, `hooks.PreToolUse`: a handler `{ "matcher": "Write|Edit|NotebookEdit", "hooks": [{ "type": "command", "command": "${CLAUDE_PROJECT_DIR}/target/release/guard-bash", "args": [] }] }`, without which the third criterion's code never runs; `onFailure: "block"` as on `Bash` would block every write while the binary is missing, to decide. `tools/hooks/tests/settings.rs` could check the handler as it checks `Bash`'s.
- `.claude/settings.json`, `permissions.allow`: `"Bash(target/release/git-sequences state *)"`, and `"git-sequences state"` in `ALLOWED_BINARIES` of `tools/hooks/tests/settings.rs` (21 entries); then `target/release/git-sequences state` in the list "The binaries the agent calls" of `docs/tooling/agent-tools.md`, whose paragraph says the settings allow each of them.
- `.claude/skills/worker-brief/SKILL.md`, Setup item 3, its first sentence to read: "**One Git command per call**, never chained to another by `&&`, `;` or a pipe, Git reads aside: after the brief's `cd`, a chain of Git commands that only read goes through (`git status; git log -3`), and `target/release/git-sequences state` prints the branch, its base, `status --short`, `diff --stat HEAD` and the commits ahead in one call (EN-119)."
- `AGENTS.md`, the agent tools row: `git-sequences state` after `git-sequences commit`.
- `CLAUDE.md`, the `guard-bash` sentence: add "and the heavy lock taken by hand (`mkdir …/heavy.lock`)" to what it denies.
- `.claude/rules/tooling.md`, the `guard-bash` bullet: add "In a worktree, after the brief's `cd`, a chain of Git reads goes through, a Python program writing a file is refused; on `Write` and `Edit`, a worker is refused a file outside its prompt's worktree (EN-119)."

### Incidents

#### 2026-10-19 — `guard-bash`, a `$'…'` text holding `\'`

- Used: a loop over the story's cases written as `$'…\'EOF\'…'` texts, after the brief's `cd`, to probe the release binary.
- Failed: the separator reader closed the quote at `\'`, read the rest as commands and refused a chained Git command where none was.
- Instead: the cases written to a JSON file and read with `jq`; the reader fixed in EN-119.T2, a backslash escaping inside `$'…'`.
