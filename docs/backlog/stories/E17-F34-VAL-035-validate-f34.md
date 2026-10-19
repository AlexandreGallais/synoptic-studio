---
id: VAL-035
epic: E17
feature: F34
title: Validate F34
status: ready
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

## For the main thread

### Journal

- `#run` The run started on 2026-10-18 after 0.3.0's merge into `main` delivered E17's tooling features F28, F29, F30, F23, F31, F32, F33 and F34 (PR #393 to #427), one agent at a time; their audits closed four ways past `guard-bash` (fake here-documents and text fed to a shell, a push judged in the wrong folder, `bench` and `cargo r` with global options), the main checkout's guard rebuilt from the fixed branch at once. It ended on 2026-10-19 at 90 % of the week's usage, under the 92.5 % the Product Owner allowed. New ADRs: 0088 to 0094.
