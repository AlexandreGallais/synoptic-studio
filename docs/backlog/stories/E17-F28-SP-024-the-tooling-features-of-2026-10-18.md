---
id: SP-024
epic: E17
feature: F28
title: The tooling features of 2026-10-18
status: ready
points: 3
---

# E17 · F28 · SP-024 — The tooling features of 2026-10-18

The Product Owner asked on 2026-10-18 for the tooling left after 0.3.0's merge to be cut into small features, each one real theme of a few stories, done now in E17, the review and the method's work held at the session's end before the merge into `main` (ADR-0086). This spike reads what the workflows need, writes the features and their stories, and the rule on workflows as an ADR.

## Acceptance criteria

- Given GitHub's documentation, when read through the library, then the notes say how a non-blocking workflow runs on `develop` after each merged pull request although a merge made with `GITHUB_TOKEN` starts no workflow (`workflow_dispatch` from the auto-merge, `workflow_run`), and what each current workflow is for.
- Given the decisions of 2026-10-18, when the spike ends, then each tooling theme is a feature of E17 of 2 to 4 stories, each story of at most 3 tasks with its criteria, F23 moved from E02 into E17, and an ADR states: one workflow per purpose, its trigger its only condition, no blocking workflow on `develop`.

## Tasks

- [x] T1 — The sources read, the research note (1 h)
- [x] T2 — The ADR, the features and their stories, F23 moved (2 h)

One task = one commit, referenced as `SP-024.T1` in the commit footer.
