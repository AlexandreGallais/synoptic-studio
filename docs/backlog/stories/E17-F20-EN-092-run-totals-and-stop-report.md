---
id: EN-092
epic: E17
feature: F20
title: The totals of a whole run, and a report when it stops
status: draft
points: 3
---

# E17 · F20 · EN-092 — The totals of a whole run, and a report when it stops

Asked by the Product Owner on 2026-10-17: the status line counts a "turn", whose time and tokens restart at each message, so a run of many hours shows only its last minutes (for instance "turn 21 min" while the run had lasted since the morning). They want the totals of the whole run — time, tokens of every kind, the subagents included — however long it lasts, even 48 hours across several messages and context summaries, to judge what a run costs for its length; and, when the agent stops with nothing left running, a message with the run's report.

## Acceptance criteria

- Given a run started by `/run`, when the status line is drawn, then it shows the run's time and tokens since its start, subagents included, beside the turn's, and calls it "run".
- Given a run that goes on across context summaries and new messages, when the totals are counted, then nothing restarts until the run ends.
- Given the agent stopping with no subagent and no background command left, when it stops, then it sends the Product Owner the run's report — duration, tokens by kind, stories merged, failures — through the push notification of EN-086 and in the chat.
- Given `npm run metrics:session`, when it is asked for a run, then it gives the same totals.

## Tasks

To write at F20's refinement.
