---
id: EN-120
epic: E02
feature: F23
title: Reports that say what happened
status: draft
points: 3
---

# E02 · F23 · EN-120 — Reports that say what happened

From the card C5 and the rows I36, I72, I82, I83, I88, I91, I92, I110 and I111 (RET-004), and the two findings of its §1: `epic-check` blind to the CI, the library's visits recorded without their project.

## Acceptance criteria

- Given a subagent stopped before its `PostToolUse`, when the status line refreshes, then its calls are no longer shown running (C5).
- Given `metrics`, when it counts commands, then a call starting with `cd … || exit 9;` counts the command after it, and `--since` cuts the Findings to the period (I91, I92); a Sonnet subagent's tokens are read from `modelUsage` (I83).
- Given `epic-check`, when an epic's story is `dropped`, then it is closed, and the last CI of the pull request into `main` is read and reported (I110).
- Given `session-report`, `val-prepare`, `context-size` and the library, when they run, then 59.6 s rounds to `1 min 00 s`, a struck criterion gets no line, a file's tokens can be counted, and a page's visit records its project (I111, I88, I72).

## Tasks

- [ ] T1 — Each criterion's tests first, failing, from the recorded cases; then the tools (3 h)

One task = one commit, referenced as `EN-120.T1` in the commit footer.
