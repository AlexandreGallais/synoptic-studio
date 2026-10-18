---
id: F25
epic: E17
title: The test lane's Rust tools
status: in-progress
---

# E17 · F25 — The test lane's Rust tools

**Benefit hypothesis**: the CI of `develop` → `main` (#370) passes its `TypeScript tests` check, whose `[F19.AC9]` runs `cargo deny` with the advisory database, which only the `Rust` job installed and fetched since F24.

Found at #370's first run with F24's jobs (2026-10-18): `tests/rust-advisories.test.ts:173` expected `cargo deny`'s exit 1 and got 101, the test lane lacking `cargo-deny` and the RustSec database. A tooling fix, validated by the agent (ADR-0072), done before 0.3.0 while the Product Owner is away, as they asked for a green CI into `main`.

## Acceptance criteria

1. The `TypeScript tests` job installs `cargo-deny` on a missed tools key and fetches the advisory database, as the `Rust` job does; a test of `ci.yml` checks both steps run in every lane whose scripts call them (EN-126).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-126](../stories/E17-F25-EN-126-the-test-lanes-rust-tools.md) | EN | The test lane's Rust tools | done |
| [AUD-027](../stories/E17-F25-AUD-027-audit-f25.md) | AUD | Audit F25 | done |
| [VAL-026](../stories/E17-F25-VAL-026-validate-f25.md) | VAL | Validate F25 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-126 | AUD-027, VAL-026 |
