---
id: VAL-035
epic: E17
feature: F34
title: Validate F34
status: done
points: 1
---

# E17 · F34 · VAL-035 — Validate F34

F34 is a tooling feature: the agent validates it on GitHub's Actions tab (ADR-0072).

## Acceptance criteria

- Given F34's criteria, when the validation ends, then each is proven again and shown in a demo page.

## Tasks

- [x] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-035.T1`.

## Result (2026-10-19)

Validated by the agent (ADR-0072): `[F34.AC1]`'s tests pass; the demo page `docs/guide/f34-the-use-of-each-workflow.md`. Read at the review after the next merge into `main`: whether _External pull requests_ leaves the Actions tab.
