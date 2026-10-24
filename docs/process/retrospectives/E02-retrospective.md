# E02 retrospective — Select and move shapes on one drawing

Held on 2026-10-24 with the Product Owner at REV-008, after E02's merge into `develop` (#490). E02 ran from 2026-10-22 to 2026-10-24: thirteen features, F35 to F47, each ended by its audit, F43 to F47 corrections from three tries of the playground. The decisions: ADR-0100.

## What the agent got wrong

| What happened | What it should have done | Now |
| --- | --- | --- |
| The features were stacked, each branch from the one before, none in `develop` until the end; the main checkout was left on old branches | merge each feature into `develop` at its audit's end, the main checkout on `develop` | ADR-0100, decision 3; `AGENTS.md` (Work); the `run` skill (A feature's end) |
| The epic held several subjects; the agent judged at its start that they fitted | one subject per epic, small if need be | ADR-0100, decision 1; the epics regrouped before the next one |
| Each feature opened with its own spike, its questions asked feature by feature, its size learned while built | the whole research and every question at the epic's start | ADR-0100, decision 2 |
| The time estimates ran two to three times the real time | estimate from the measures by kind (a spike 17 to 32 minutes, a story 14 to 37, an audit's fixes 44 to 86) | ADR-0100, decision 7 |
| Questions put in terms the Product Owner could not follow (the sign of a value typed), and F47 opened without asking which key was pressed (Option, not Cmd) | ask by the plain rule, and the exact gesture before opening a fix | the feedback log |
| Shared files written on story branches, refused at the push; several pushes failing unseen, the last output lines only read | the shared files in the done commit or a branch of their own; the whole output and `git ls-remote` read | the feedback log, the worker brief |
| The main thread's docs committed straight on the feature branches: flat runs in the history | every commit through a branch of its own, merged | ADR-0100, decision 5 |
| The whole list of checks given again after each correction | the changed items alone | ADR-0100, decision 4; the `run` skill |

## What went well

- The audits found what the tests did not: the history's cost with 1 000 shapes, a field doubling from a negative value, a key sequence the test helper hid, the keys lost under Cmd on macOS.
- The Product Owner found the playground fluid: drawing, undo back to the start, the zoom, the selection with Alt.

## The telemetry, read at the review

`target/release/metrics --since 2026-10-22`: 9 252 requests, 10 143 tool calls, 510 failed (5 %), 2 127 minutes active.

| Seen | What it costs | Done |
| --- | --- | --- |
| `until` polling loops by workers waiting for their own commands: 45 calls, 219 minutes in all, up to 10 minutes each | a worker's time spent waiting, the Bash call's 10-minute limit reached | the worker brief: a long command in the background, its notification awaited, never a polling loop |
| `npm run mutate` 46 minutes in 24 calls, `npm run fix` 24 minutes in 126 | the method's weight, kept (ADR-0100, decision 6) | — |
| `check:all` at every push, a branch deletion included, tests timing out under load (load averages of 26 to 38) | pushes failing on unrelated timeouts, three retries | REV-008's tooling task: a deletion runs no checks, a story branch runs the tests its change reaches |
| `python3` and `perl` written to files, refused by the guard | refused calls, rewritten | the brief already forbids them; kept |

## Decisions of the Product Owner

ADR-0100 holds them. Also: the two permission rules `Bash(git switch:*)` and `Bash(git push origin --delete:*)` to add to `.claude/settings.json`; a real browser for the playground's tests when the agent needs one, with its ADR and its row; the two layers' remaining helps (the frames and handles still in the drawing's SVG) to an epic of their own when the epics are regrouped.
