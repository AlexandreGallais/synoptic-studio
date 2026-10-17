---
id: EN-087
epic: E17
feature: F20
title: A progress page as a claude.ai Artifact, updated during a run
status: ready
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

## Decisions

- **Which features**: those with a branch `feature/f<nn>-…` on `origin`, read from that branch, where the `done` commits land; the main checkout stays on `develop`, whose statuses are older.
- **A story's agents**: the first story id in the agent's description (its `.meta.json`); an agent naming none counts in the run's tokens only, shown as those of the main thread and of agents outside the stories.
- **Tokens**: counted by `run-totals`' library from the same cache (`run-cache.json`), so the page and `metrics:session -- --run` agree; `projects_dir` and the cache refresh moved into that library.
- **An agent running**: its last assistant line neither hands back nor ends its turn, and its transcript was written within 30 minutes.

## For the main thread

- The improvement row of 2026-10-16 (a progress view in the terminal beside Claude Code) is answered by this page: mark it handled by EN-087 in `docs/process/improvements.md` when writing the logs (a shared file).
- Measured on the live run of 2026-10-17 (4 workers, 17 stories): the page is 6 KB, written in 0.1 s, 0.3 s through npm.
