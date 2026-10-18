---
id: AUD-027
epic: E17
feature: F25
title: Audit F25
status: ready
points: 1
---

# E17 · F25 · AUD-027 — Audit F25

A one-line change of `ci.yml`, audited by the main thread against the failing log of run 39138216016 and #370's next run (ADR-0086: a specialist only where its independence pays).

## Acceptance criteria

- Given F25's criterion, when the audit ends, then it is traced to a test titled `[F25.AC1] …`, broken once on purpose and seen failing.

## Tasks

- [ ] T1 — Criterion traced, the condition read against `REF-GH-CONTEXTS` (main thread)

One task = one commit, referenced as `AUD-027.T1`.

## Findings
