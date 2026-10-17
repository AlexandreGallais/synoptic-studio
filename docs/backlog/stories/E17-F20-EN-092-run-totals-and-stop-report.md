---
id: EN-092
epic: E17
feature: F20
title: The duration and tokens of a whole run
status: done
points: 3
---

# E17 · F20 · EN-092 — The duration and tokens of a whole run

Asked by the Product Owner on 2026-10-17: the status line counts a "turn", whose time and tokens restart at each message, so a run of many hours shows only its last minutes. They want the run's duration and tokens, subagents included, however long it lasts, to see what a run consumes. Decided at F20's refinement: the run's report holds only its duration and tokens; measured gains and incidents go to the agent's own logs and the retrospective.

Measured by SP-022 ([research note 0024](../../research/notes/0024-f20-context-diet.md)): a compaction keeps the session's file and id (up to 12 in one file), and each subagent writes its own transcript under the session's `subagents/` folder. So a run's totals are the sums of those transcripts from the run's start.

## Acceptance criteria

- Given `/run`, when it starts, then `.git/metrics/run.json` holds its start and its session's id; a session started while the run is open adds its id; the run's end closes it.
- Given a run, when the status line is drawn, then it shows the run's duration and tokens since its start, subagents included, beside the turn's, and calls it "run".
- Given a run that goes on across compactions and new messages, when the totals are counted, then nothing restarts until the run ends.
- Given the run's end, when the agent stops, then its message to the Product Owner holds the run's duration and its tokens by kind, nothing else.
- Given `npm run metrics:session -- --run`, when asked, then it gives the same totals.

## Tasks

One task = one commit, referenced as `EN-092.Tn`.

- [ ] T1 — The run's file written by `/run` and closed at its end (1 h)
- [ ] T2 — The run's totals in the session report, read incrementally, with their tests (3 h)
- [ ] T3 — The run's totals in the status line, and the report at the run's end (2 h)

## For the main thread

- The status line's turn tally (`.claude/hooks/status-tokens.ts`) keeps a request's first line, as `run-totals` did at first: on the main transcript every line of a request carries the same usage, so the turn is right; a subagent's lines carry a growing `output_tokens` (0.75 million counted first-line against 1.83 million last-line on the session of 2026-10-17), which F21's port of the status line should take from the last line.
- Not tested live on the real `.git/metrics/run.json`, to leave the open run's marker alone: the next `/run` writes it with `npm run run:start`.
