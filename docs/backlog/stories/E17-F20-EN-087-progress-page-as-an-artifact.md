---
id: EN-087
epic: E17
feature: F20
title: A progress page as a claude.ai Artifact, updated during a run
status: draft
points: 3
---

# E17 · F20 · EN-087 — A progress page as a claude.ai Artifact, updated during a run

Show a run's progress on a claude.ai Artifact page, updated at the run's start and after each story, in place of the idea of a quarter-screen terminal view (improvement log, 2026-10-16). Asked by the Product Owner on 2026-10-17; content decided at F20's refinement the same day.

Publishing again from the same file updates the page at the same address, seen in place by anyone who has it open; a new session makes a new page unless given the address; the page is static and reads no local file (`REF-CC-ARTIFACTS`). So a Rust tool writes the page from the backlog and the metrics, and the main thread publishes that file: one call, no page text written by the model ([research note 0024](../../research/notes/0024-f20-context-diet.md)).

## Acceptance criteria

- Given a run, when it starts and after each story, then the page shows the tree epic → feature → story with each status, the story in progress, the durations per story and per feature, the agents running, the tokens per story and per run, and the commits of each story and feature.
- Given the page, when it is updated, then it keeps its address, kept in `.git/metrics/`, also when the run goes on in a new session; the Product Owner opens it once per run.
- Given the page's content, when it is built, then it comes from the backlog files, Git and the metrics, never from a list typed by hand; other views are added when useful and named in the story.

## Tasks

One task = one commit, referenced as `EN-087.Tn`.

- [ ] T1 — The Rust tool writing the page from the backlog, Git and the metrics, with its tests (3 h)
- [ ] T2 — The page published at the run's start and after each story, at the same address (2 h)
- [ ] T3 — The run skill and `docs/tooling/agent.md` updated; the improvement row of 2026-10-16 marked as handled (1 h)
