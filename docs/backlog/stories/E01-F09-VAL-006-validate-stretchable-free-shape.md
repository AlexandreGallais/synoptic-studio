---
id: VAL-006
epic: E01
feature: F09
title: Validate F09
status: ready
points: 2
---

# E01 · F09 · VAL-006 — Validate F09

Replay every acceptance criterion of F09 in the playground.

## Acceptance criteria

- Given the playground, when each reference case of F09 is entered, then the drawing matches the expected values.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F09 is `done`.

## Tasks

One task = one commit, referenced as `VAL-006.Tn`. This story is not merged automatically (ADR-0028).

- [x] T1 — One test per criterion `[F09.AC1]`…`[F09.AC4]` (1 h)
- [ ] T2 — Guided test in the playground: the steps of the US-017 card (1 h)
- [ ] T3 — Demo page in `docs/guide/` in plain language (1 h)
- [ ] T4 — Product Owner runs the guided test and validates; feedback recorded; feature pull request into `main` (0.5 h)

## Tests per criterion (2026-10-11)

Written with the stories and the audit; counted at the validation (`[F09.ACn]` in test titles).

| Criterion                                          | Tests | Where                                                                                                 |
| -------------------------------------------------- | ----- | ----------------------------------------------------------------------------------------------------- |
| F09.AC1 — rectangle tool makes a free shape        | 2     | `is-valid-free-shape`, `free-shape-corners`; and every F01 and F03 rectangle test, now run through it |
| F09.AC2 — width and height stretch, top-left fixed | 5     | `stretch-to-size` (two properties), `free-shape-corners`                                              |
| F09.AC3 — stretching without loss                  | 3     | `stretch-to-size`, `free-shape-corners`, playground (`rectangle-free-shape`)                          |
| F09.AC4 — the playground rectangle unchanged       | 3     | playground (`rectangle-free-shape`)                                                                   |
