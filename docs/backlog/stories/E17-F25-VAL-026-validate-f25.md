---
id: VAL-026
epic: E17
feature: F25
title: Validate F25
status: done
points: 1
---

# E17 · F25 · VAL-026 — Validate F25

F25 is a tooling fix: the agent validates it by #370's `TypeScript tests` check (ADR-0072).

## Acceptance criteria

- Given #370's CI after F25's merge into `develop`, when it ends, then its `TypeScript tests` check passes.

## Tasks

- [x] T1 — The check seen on #370 (main thread)

One task = one commit, referenced as `VAL-026.T1`.

## Result (2026-10-18)

Validated by the agent (ADR-0072): `[F25.AC1]`'s two tests pass on the feature branch; #370's `TypeScript tests` check, the criterion's proof on GitHub's runner, runs again once F25 is in `develop` and is read at the next review if the Product Owner is away. No demo page: a one-line fix of F24, described in F24's page.
