---
id: CHK-020
epic: E17
feature: F19
title: Checkpoint in the middle of F19
status: ready
points: 1
---

# E17 · F19 · CHK-020 — Checkpoint in the middle of F19

Bring the docs and methods up to date in the middle of F19, with a light evolvability check (ADR-0028).

## Acceptance criteria

- Given the stories merged so far, when the checkpoint ends, then the docs, rules and tools describe what was built.

## Tasks

One task = one commit, referenced as `CHK-020.Tn`.

- [x] T1 — Docs and rules in step; light evolvability check (1 h)

## For the main thread

- Rule-docs: none affected, a checkpoint of tooling docs that changes no rule of `docs/domain/`.
- Test cards: none so far in F19, whose stories are all enablers and spikes.
- Still for the Product Owner: the strict form of EN-076 (a named type proven by its own declaration) and the `limit-agents` command of EN-068 (a settings change).
