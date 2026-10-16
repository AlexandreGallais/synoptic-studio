---
id: EN-053
epic: E17
feature: F18
title: The session's figures at its end, background subagents seen
status: ready
points: 2
---

# E17 · F18 · EN-053 — The session's figures at its end, background subagents seen

Report the whole session's figures at its end, and show each background subagent on the status line (rows 9, 11, 12 of RET-003; I50, I52, I53; F18.AC9).

## Acceptance criteria

- Given a session of several turns, when `npm run metrics:session -- --all --markdown` runs at its end, then it counts every turn of the session.
- Given a worker running in the background, when the status line refreshes, then it shows the worker and its duration, never "waiting" nor "return"; the cause of the wrong states is found in the call records first and a test written from it.

## Tasks

One task = one commit, referenced as `EN-053.Tn`.

- [ ] T1 — The cause found in the call records; a test from that line (1 h)
- [ ] T2 — Background subagents and their duration on the status line; the session report rule (1 h)
