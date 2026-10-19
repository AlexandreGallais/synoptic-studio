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

- [x] T1 — Each criterion's tests in `epic-check`, `session-report`, `val-prepare` and `context-size`, from the recorded cases, failing (1 h)
- [x] T2 — The four tools fixed, `docs/tooling/agent-tools.md` (2 h)

One task = one commit, referenced as `EN-139.T1` in the commit footer.

## For the main thread

Rule-docs: none affected, a tooling story: no rule of `docs/domain/` changes.

Decisions taken: `context-size` counts a file named at 2.30 characters per token, the ratio EN-097 measured from its requests' input and EN-098 and EN-099 used, since no tokenizer is available offline (I72); `format_duration` chooses its unit from the duration rounded as shown, so 59.6 s is `1 min 00 s` and 999.6 ms `1.0 s`, in every report of `tools/metrics`, not only `session-report`; `val-prepare` tags a criterion with its own number, so F20's criterion 6 stays `F20.AC6` once the fifth is struck; `epic-check` reads the last pull request into `main` whatever its state (`gh pr list --base main --state all --limit 1`), a job skipped or neutral passing, one still running to look at. I88's other half, the barrel `export type *` rewritten as `export *`, is outside the story's criteria and left as it was.

### Improvements

| 2026-10-19 | EN-139 | The rows a retrospective numbers (I72, I88, I110, I111) are found nowhere once moved: their text was read back from `git log -p` of `docs/process/improvements.md`, by their order in the removed table; the record's table of logged rows could carry each row's number beside its text | process |
