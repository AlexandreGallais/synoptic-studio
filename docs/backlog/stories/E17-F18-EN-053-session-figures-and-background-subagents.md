---
id: EN-053
epic: E17
feature: F18
title: The session's figures at its end, background subagents seen
status: done
points: 2
---

# E17 · F18 · EN-053 — The session's figures at its end, background subagents seen

Report the whole session's figures at its end, and show each background subagent on the status line (rows 9, 11, 12 of RET-003; I50, I52, I53; F18.AC9).

## Acceptance criteria

- Given a session of several turns, when `npm run metrics:session -- --all --markdown` runs at its end, then it counts every turn of the session.
- Given a worker running in the background, when the status line refreshes, then it shows the worker and its duration, never "waiting" nor "return"; the cause of the wrong states is found in the call records first and a test written from it.

## Tasks

One task = one commit, referenced as `EN-053.Tn`.

- [x] T1 — The cause found in the call records; a test from that line (1 h)
- [x] T2 — Background subagents and their duration on the status line; the session report rule (1 h)

## Cause

Found in the call records (transcript `5332193d…`, 2026-10-15): at 12:07:38 UTC a `SendMessage` to the stopped EN-039 worker answered `Resuming agent a93498c`; the worker ran in the background until its notification at 12:22:26, which named the message's call id. The status line counted only `Agent`, `Monitor` and background calls, so it showed `◌ your turn` for those 15 minutes; 43 messages resumed a subagent in E15, among them 10:01 to 10:14 and 11:17 to 11:22 the same day. The first line's `waiting` was the step's fallback when `.git/metrics/step` was empty, and the free chat read `waiting for 1 agent`.

## For the main thread

- The agent's memory still says to end each turn with `npm run metrics:session -- --markdown` (`feedback-session-report.md`): it now runs once, at a session's end, with `--all` (`CLAUDE.md`, `writing.md`, `metrics.md`).
- The subagent limit (`tools/hooks`, EN-047) now counts a resumed subagent as running, but its hook is on `Agent` only: a `SendMessage` that resumes a fourth subagent is not refused. A matcher on `SendMessage` for `limit-agents` would close it (`.claude/settings.json`, EN-063's scope).
- A resumed subagent's line shows the message's summary (`↳ agent resumed: EN-039 audit findings to fix — 4 min`), without the call it makes now: the running-call records name a subagent by its type, not by its id.
