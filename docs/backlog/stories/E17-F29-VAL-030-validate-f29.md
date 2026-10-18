---
id: VAL-030
epic: E17
feature: F29
title: Validate F29
status: done
points: 1
---

# E17 · F29 · VAL-030 — Validate F29

F29 is a tooling feature: the agent validates it on the local checks (ADR-0072). The pull that brings F29's post hooks into a checkout runs them with its older `git-hooks`, which has no `post-merge`: after it, `npm run rust:build` is run once, and criterion 3 is proven on a later pull (AUD-031, F5).

## Acceptance criteria

- Given F29's criteria, when the validation ends, then each is proven again and shown in a demo page, with the pushes of the feature's stories that failed on a timing test counted.

## Tasks

- [x] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-030.T1`.

## Result (2026-10-18)

Validated by the agent (ADR-0072): `[F29.AC1]`, `f29_ac2_*` and `f29_ac3_*` pass; the demo page `docs/guide/f29-local-checks-stable-and-fast.md`, which says to run `npm run rust:build` once after the pull that brings the hooks (AUD-031, F5). `Rust coverage` added to `main`'s required checks at F29's merge.
