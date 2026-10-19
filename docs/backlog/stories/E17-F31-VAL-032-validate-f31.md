---
id: VAL-032
epic: E17
feature: F31
title: Validate F31
status: done
points: 1
---

# E17 · F31 · VAL-032 — Validate F31

F31 is a tooling feature: the agent validates it on the guard's and the commands' cases (ADR-0072).

## Acceptance criteria

- Given F31's criteria, when the validation ends, then each is proven again and shown in a demo page.

## Tasks

- [x] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-032.T1`.

## Result (2026-10-19)

Validated by the agent (ADR-0072): F31's tagged tests and the 109 audit cases pass; the demo page `docs/guide/f31-the-guard-and-the-agents-commands.md`. EN-119's open point, whether Claude Code itself accepts a chain of Git reads after the brief's `cd` once the guard lets it through, is read in the next run's workers.
