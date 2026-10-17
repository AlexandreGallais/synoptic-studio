---
id: EN-087
epic: E17
feature: F20
title: A progress page as a claude.ai Artifact, updated during a run
status: draft
points: 3
---

# E17 · F20 · EN-087 — A progress page as a claude.ai Artifact, updated during a run

Show a run's progress on a claude.ai Artifact page, updated at the run's start and after each story, in place of the idea of a quarter-screen terminal view (improvement log, 2026-10-16). Asked by the Product Owner on 2026-10-17, during SP-021 ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), "The Product Owner's second round"); the Artifact's publishing and updating are sourced by SP-022 before the story is set `ready`.

## Acceptance criteria

- Given a run, when it starts and after each story, then a claude.ai Artifact shows the tree epic → feature → story with each status, the story in progress, the durations and the running agents.
- Given the page, when it is updated, then it keeps its address, so that the Product Owner opens it once per run.
- Given the page's content, when it is built, then it comes from the backlog files and the metrics, never from a list typed by hand.

## Tasks

One task = one commit, referenced as `EN-087.Tn`.

- [ ] T1 — The page's content read from the backlog files and the metrics (3 h)
- [ ] T2 — The page published at the run's start and updated after each story, at the same address (2 h)
- [ ] T3 — The run skill and `docs/tooling/agent.md` updated; the improvement row of 2026-10-16 marked as handled (1 h)
