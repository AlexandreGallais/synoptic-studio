---
id: VAL-024
epic: E17
feature: F23
title: Validate F23
status: done
points: 1
---

# E17 · F23 · VAL-024 — Validate F23

F23 is a tooling feature: the agent validates it on the graph of its own feature branch and the stories' tools (ADR-0072).

## Acceptance criteria

- Given F23's criteria, when the validation ends, then each is proven again and shown in a demo page, with the graph of F23's branch showing no commit between its stories' bubbles after EN-137.

## Tasks

- [x] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-024.T1`.

## Result (2026-10-19)

Validated by the agent (ADR-0072): F23's tagged tests pass; F23's own graph has nothing between its story bubbles; the demo page `docs/guide/f23-the-path-of-a-story.md`.
