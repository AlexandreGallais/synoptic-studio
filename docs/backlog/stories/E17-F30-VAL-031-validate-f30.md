---
id: VAL-031
epic: E17
feature: F30
title: Validate F30
status: ready
points: 1
---

# E17 · F30 · VAL-031 — Validate F30

F30 is a tooling feature: the agent validates it on its tests and docs (ADR-0072). A dispatch runs only a workflow on the default branch (`REF-GH-WORKFLOW-DISPATCH`), so the `develop` CI's first runs come after the merge into `main` that brings it: the review that follows reads them and writes them in the demo page.

The demo page also says that the F28 guide's row of `rust-caches.yml`, "a push to `develop` or `main`", is stale since EN-135: it runs on a push to `main`, `develop`'s caches being the `develop` CI's (AUD-032, F8).

## Acceptance criteria

- Given F30's criteria, when the validation ends, then each is proven again and shown in a demo page.

## Tasks

- [ ] T1 — Each criterion proven again, the demo page written (1 h)

One task = one commit, referenced as `VAL-031.T1`.
