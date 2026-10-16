---
id: F18
epic: E17
title: A safe and cheap parallel run, and E15's leftovers
status: in-progress
---

# E17 · F18 — A safe and cheap parallel run, and E15's leftovers

**Benefit hypothesis**: once the tools enforce the agent limit, one heavy command at a time and conflict-free shared files, a run with parallel workers no longer freezes the computer nor leaves merge conflicts, and costs fewer tokens per story than E15's 24 M.

Written at RET-003 (2026-10-16) from the Product Owner's decisions (ADR-0063, ADR-0064, ADR-0065, ADR-0066), as F11 was at RET-001. It also holds the leftovers of E15 decided there (US-061, EN-057, EN-062). The Product Owner refines and orders it with the rest of E17.

## Acceptance criteria

1. Release pull request on `develop`, only the release on `main`.
2. A fourth subagent refused, naming those running.
3. One heavy command at a time across worktrees, a stale lock taken over.
4. Tables without padding; append-only logs merged by union, rows kept unique.
5. The stories index generated.
6. A story branch editing a shared file refused at the push.
7. One merge at a time; `story:finish` from a worktree; a branch deleted only once merged.
8. Worktrees named after their story; worktree commands answered at once; `golden:write` refused in a worktree.
9. The whole session reported at its end; background subagents on the status line.
10. Timing tests hold under load; `[F15.AC4]` at 1 000 texts in under 0.4 s.
11. `epic:check` names failing tests, refusals and lost calls; `golden:check` at the push.
12. At most 12 checklist lines per feature, the panel beside the drawing; `val:prepare` writes them.
13. Every exception registered and checked; a 192 MB font collection read.
14. The font list held while the default font is read again; a refused face not measured.
15. `npm run mutate` reports caught and surviving mutations.
16. A done commit that changes a rule brings its Q row, decisions to confirm and guide page.
17. E15's long pages shortened.
18. Modules per folder counted; no spread in a fold.
19. The CI reads a real font file.
20. Every check tool given explicit targets, no tracked file left out; Vitest on half the cores.

## Stories

In delivery order.

| ID                                                                                   | Type       | Title                                                              | Status |
| ------------------------------------------------------------------------------------ | ---------- | ------------------------------------------------------------------ | ------ |
| [SP-020](../stories/E17-F18-SP-020-research-f18.md)                                  | Spike      | Research for F18, the sources of RET-003 recorded                  | done   |
| [EN-045](../stories/E17-F18-EN-045-explicit-tool-scopes.md)                          | Enabler    | Every check tool given explicit targets                            | done   |
| [EN-047](../stories/E17-F18-EN-047-heavy-lock-and-agent-limit.md)                    | Enabler    | A lock for heavy commands and a limit on subagents                 | done   |
| [EN-063](../stories/E17-F18-EN-063-subagent-limit-in-settings.md)                    | Enabler    | The subagent limit set in Claude Code's settings                   | ready  |
| [EN-048](../stories/E17-F18-EN-048-tables-without-padding-and-union-logs.md)         | Enabler    | Tables without padding, append-only logs merged by union           | ready  |
| [EN-049](../stories/E17-F18-EN-049-stories-index-generated.md)                       | Enabler    | The stories index generated from the story files                   | done   |
| [EN-050](../stories/E17-F18-EN-050-shared-files-written-at-merge.md)                 | Enabler    | Shared files written only at merge                                 | done   |
| [EN-051](../stories/E17-F18-EN-051-story-finish-one-merge-at-a-time.md)              | Enabler    | `story:finish` one merge at a time, from a worktree too            | ready  |
| [CHK-019](../stories/E17-F18-CHK-019-checkpoint-f18.md)                              | Checkpoint | Checkpoint in the middle of F18                                    | ready  |
| [EN-052](../stories/E17-F18-EN-052-worktrees-named-after-their-story.md)             | Enabler    | Worktrees named after their story, commands that fit them          | ready  |
| [EN-053](../stories/E17-F18-EN-053-session-figures-and-background-subagents.md)      | Enabler    | The session's figures at its end, background subagents seen        | done   |
| [EN-054](../stories/E17-F18-EN-054-timing-tests-that-hold-under-load.md)             | Enabler    | Timing tests that hold under load                                  | done   |
| [EN-055](../stories/E17-F18-EN-055-epic-check-says-what-failed.md)                   | Enabler    | `epic:check` says what failed; the golden check at push            | ready  |
| [EN-056](../stories/E17-F18-EN-056-a-short-checklist.md)                             | Enabler    | A short checklist, prepared by `val:prepare`                       | ready  |
| [EN-057](../stories/E17-F18-EN-057-register-of-exceptions-and-a-byte-buffer.md)      | Enabler    | The register of exceptions, and a byte buffer for font collections | done   |
| [US-061](../stories/E17-F18-US-061-font-list-held-while-the-default-font-is-read.md) | User story | Font list held while the default font is read again                | ready  |
| [EN-058](../stories/E17-F18-EN-058-a-mutation-runner.md)                             | Enabler    | A mutation runner as a tool                                        | ready  |
| [EN-059](../stories/E17-F18-EN-059-done-commit-checks-the-docs.md)                   | Enabler    | The done commit checks the docs a rule touches                     | ready  |
| [EN-060](../stories/E17-F18-EN-060-documentation-pass-on-e15-pages.md)               | Enabler    | A documentation pass on E15's long pages                           | ready  |
| [EN-061](../stories/E17-F18-EN-061-modules-per-folder-counted.md)                    | Enabler    | Modules per folder counted, no spread in a fold                    | ready  |
| [EN-062](../stories/E17-F18-EN-062-inter-as-a-test-font.md)                          | Enabler    | Inter Regular as a test font file                                  | done   |
| [EN-046](../stories/E17-F18-EN-046-release-pr-on-develop.md)                         | Enabler    | Release pull request on develop, only the release on main          | ready  |
| [AUD-020](../stories/E17-F18-AUD-020-audit-f18.md)                                   | Audit      | Audit F18                                                          | ready  |
| [VAL-019](../stories/E17-F18-VAL-019-validate-f18.md)                                | Validation | Validate F18                                                       | ready  |

## Feature plan

| Acceptance criterion | Realized by    | Verified by           |
| -------------------- | -------------- | --------------------- |
| F18.AC1              | EN-046         | AUD-020, `[F18.AC1]`  |
| F18.AC2              | EN-047, EN-063 | AUD-020, `[F18.AC2]`  |
| F18.AC3              | EN-047         | AUD-020, `[F18.AC3]`  |
| F18.AC4              | EN-048         | AUD-020, `[F18.AC4]`  |
| F18.AC5              | EN-049         | AUD-020, `[F18.AC5]`  |
| F18.AC6              | EN-050         | AUD-020, `[F18.AC6]`  |
| F18.AC7              | EN-051         | AUD-020, `[F18.AC7]`  |
| F18.AC8              | EN-052         | AUD-020, `[F18.AC8]`  |
| F18.AC9              | EN-053         | AUD-020, `[F18.AC9]`  |
| F18.AC10             | EN-054         | AUD-020, `[F18.AC10]` |
| F18.AC11             | EN-055         | AUD-020, `[F18.AC11]` |
| F18.AC12             | EN-056         | AUD-020, `[F18.AC12]` |
| F18.AC13             | EN-057         | AUD-020, `[F18.AC13]` |
| F18.AC14             | US-061         | AUD-020, `[F18.AC14]` |
| F18.AC15             | EN-058         | AUD-020, `[F18.AC15]` |
| F18.AC16             | EN-059         | AUD-020, `[F18.AC16]` |
| F18.AC17             | EN-060         | AUD-020, `[F18.AC17]` |
| F18.AC18             | EN-061         | AUD-020, `[F18.AC18]` |
| F18.AC19             | EN-062         | AUD-020, `[F18.AC19]` |
| F18.AC20             | EN-045         | AUD-020, `[F18.AC20]` |
