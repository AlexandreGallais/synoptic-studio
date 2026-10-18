---
id: EN-139
epic: E17
feature: F32
title: The reports of a review say what happened
status: ready
points: 2
---

# E17 · F32 · EN-139 — The reports of a review say what happened

From the rows I36, I72, I82, I88, I110 and I111 (RET-004) and its finding that `epic-check` is blind to the CI; split from EN-120 on 2026-10-18 (SP-024). RET-004's other finding, a page's visit recorded with its project, is already done (`project_uses`, EN-106).

## Acceptance criteria

- Given `epic-check`, when an epic's story is `dropped`, then it counts as closed; when the epic's last pull request into `main` has a CI, then its result and its failed jobs are reported (I110).
- Given `session-report`, when a duration is 59.6 s, then it shows `1 min 00 s`, never `0 min 60 s` (I111).
- Given `val-prepare`, when a criterion of the feature is struck through, then it writes no checklist line for it (I88).
- Given `context-size`, when it is given a file, then it prints that file's tokens, as it counts a subagent's first request.

## Tasks

- [ ] T1 — Each criterion's tests in `epic-check`, `session-report`, `val-prepare` and `context-size`, from the recorded cases, failing (1 h)
- [ ] T2 — The four tools fixed, `docs/tooling/agent-tools.md` (2 h)

One task = one commit, referenced as `EN-139.T1` in the commit footer.
