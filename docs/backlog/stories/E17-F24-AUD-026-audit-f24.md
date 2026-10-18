---
id: AUD-026
epic: E17
feature: F24
title: Audit F24
status: done
points: 1
---

# E17 · F24 · AUD-026 — Audit F24

Audit F24 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F24's criteria, when the audit ends, then each is traced to a test titled `[F24.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, the workflows read against GitHub's documentation, findings fixed (1 h)

One task = one commit, referenced as `AUD-026.T1`.

## Findings

The `auditor` alone reviewed F24 read-only (ADR-0086); the main thread verified and fixed each finding.

| Id | Finding | Evidence | Result |
| --- | --- | --- | --- |
| 1 | Criterion 3's warm time not yet written | `docs/tooling/ci.md:55` | no action now: it can only be measured once F24 is in `develop` and #370's CI runs again on `develop`'s cache; written at the next review, a pending line of VAL-025 |
| 2 | `f24_ac3_requires_on_main_…` proves criterion 1 | `tools/repo-tools/tests/check_all.rs:489` | renamed `f24_ac1_…`, EN-124's evidence updated |
| 3 | EN-124 still describes criterion 1's first grouping | EN-124's decisions | reworded: the criterion aligned on the five jobs at EN-124's `done` commit |
| 4 | Comments say `VITEST_JUNIT` is set by the test job only; `ci.yml` sets it on every lane | `tests/vitest-reporters.test.ts:20`, `vite.config.ts:15` | comments reworded: set on every lane, read by Vitest alone |
| 5 | The `release` job's cache restores not held equal by a test | `tests/ci-caches.test.ts` | `[F24.AC3]` test added; seen failing with the `release` job's key changed, passing restored |
| 6 | Cold time given as 21 min, 26 min 07 s and 45 min | F24, EN-124, EN-125 | 26 min 07 s (cancelled) everywhere, the 45 min dropped |
