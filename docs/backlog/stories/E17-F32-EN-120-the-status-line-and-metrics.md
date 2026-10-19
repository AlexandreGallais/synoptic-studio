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
- [ ] T2 — The status line and `metrics` fixed, `docs/tooling/metrics.md` (2 h)

One task = one commit, referenced as `EN-120.T1` in the commit footer.
