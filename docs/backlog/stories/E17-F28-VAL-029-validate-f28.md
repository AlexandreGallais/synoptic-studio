---
id: VAL-029
epic: E17
feature: F28
title: Validate F28
status: ready
points: 1
---

# E17 · F28 · VAL-029 — Validate F28

F28 is a tooling feature: the agent validates it on GitHub's Actions tab (ADR-0072).

## Acceptance criteria

- Given F28's criteria, when the validation ends, then each is proven again and shown in a demo page.
- Given each workflow of `.github/workflows/`, when it is read, then its file name, its `name:` and its first comment say the same purpose: `[F28.AC1]` checks only that the comment and the `name:` exist, their agreement being a reading (AUD-030).
- Given criterion 3, when the validation ends, then the times before EN-133 are in `docs/tooling/ci.md` and those after are left to the next review's CI into `main`, which measures and writes them (AUD-030).

## Tasks

- [ ] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-029.T1`.
