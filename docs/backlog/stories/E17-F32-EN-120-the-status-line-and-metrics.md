---
id: EN-120
epic: E17
feature: F32
title: The status line and metrics say what ran
status: ready
points: 2
---

# E17 · F32 · EN-120 — The status line and metrics say what ran

From the card C5 and the rows I83, I91 and I92 (RET-004). Moved from F23 on 2026-10-18 (SP-024), its other criteria gone to EN-139. On 2026-10-18 the status line showed a story worker running for 9 min 20 although no agent ran: 169 files in `.git/metrics/running/`, left by subagents stopped before their `PostToolUse` hook, were removed by hand.

## Acceptance criteria

- Given a subagent stopped before its `PostToolUse`, when the status line refreshes, then its calls are no longer shown running, the files of an ended agent removed by the hook that sees it end (C5).
- Given `metrics`, when it counts commands, then a call starting with `cd … || exit 9;` or `cd … || exit 9` and a new line counts the command after it, and `--since` cuts the Findings to the period, as it cuts the totals (I91, I92).
- Given a Sonnet subagent's transcript, when `metrics` reads its tokens, then they come from its `modelUsage`, counted under its model (I83).

## Tasks

- [x] T1 — Each criterion's tests in `tools/hooks` and `tools/metrics`, from the recorded cases, failing (1 h)
- [x] T2 — The status line and `metrics` fixed, `docs/tooling/metrics.md` (2 h)

One task = one commit, referenced as `EN-120.T1` in the commit footer.

## For the main thread

Rule-docs: none affected, a tooling story: no rule of `docs/domain/` changes.

The `SubagentStop` hook takes effect in a session started after `.claude/settings.json` is merged and the release binaries rebuilt; the status line's own cleanup removes, at its first refresh, the files left before EN-120 (no session named) once no subagent older than them runs.

### Journal

- `#run` EN-120: the status line drops the calls of ended subagents (removed on `SubagentStop` by `log-tool-call`, and by the status line once no running subagent can have made them); `metrics` names the command after `|| exit 9`; `session-report --all` reads each model's tokens from the session's `modelUsage`. The 174 files then in `running/` were all calls refused by `guard-bash` in subagents, never logged; the Findings were already cut by `--since` since EN-107.

### Improvements

| 2026-10-19 | EN-120 | Whether `SubagentStop` fires for a subagent stopped by `TaskStop` or by the person is not documented (`REF-CC-HOOKS`: "when a subagent has finished responding"); the status line's own cleanup covers it, but a stopped subagent's calls stay while an older subagent runs | tool |
| 2026-10-19 | EN-120 | The `cost-state` lines that carry `modelUsage` are written only when a session is left, so `session-report --all` on a session still open counts from the requests, which understate Sonnet; and `target/release/metrics` still sums its Sessions, Features and Stories tables from the requests, since `modelUsage` cannot be split by branch | tool |
