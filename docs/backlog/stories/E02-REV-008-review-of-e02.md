---
id: REV-008
epic: E02
title: Review of 2026-10-24, F35 to F47
status: in-progress
points: 3
---

# E02 · REV-008 — Review of 2026-10-24, F35 to F47

Held on 2026-10-24 with the Product Owner, after E02's merge into `develop` (#490): the features F35 to F47, validated by the Product Owner over three tries of the playground; the retrospective of the epic; the method's changes; the telemetry read; then `develop` → `main`. Time box set by the Product Owner: two hours.

## Acceptance criteria

- Given E02's features, when the review ends, then the Product Owner's validation and remarks are recorded (E02's file, Remarks from the Product Owner's first try; the journal), each remark a backlog item or a domain rule.
- Given the epic's mistakes and the Product Owner's decisions, when the review ends, then they are written: `docs/process/retrospectives/E02-retrospective.md`, ADR-0100, `AGENTS.md` (Work), the `run` skill and the worker brief, the feedback log.
- Given the telemetry since 2026-10-22, when the review ends, then its findings are acted on or placed: the worker brief's polling rule; the pre-push running no checks for a deletion and only the tests a story's change reaches (REV-008.T3).

## Tasks

One task = one commit, referenced as `REV-008.Tn`.

- [x] T1 — Naming pass: left to the next review, the time box spent on the method; each feature's audit held its own (AUD-037 to AUD-049)
- [x] T2 — Session with the Product Owner: the validation, the retrospective, the decisions recorded (1 h)
- [ ] T3 — The decisions written, the tools changed, the docs brought up to date (1 h)

## The Product Owner's decisions

Written as ADR-0100: small epics of one subject, each researched in full at its start with every question; each feature into `develop` at its audit's end, the main checkout on `develop`; an audit per feature, no `VAL` per feature; `develop` → `main` at each return; bubbles only; this project's rigour kept; estimates from the measures; advisories settled by the agent. Also: the epics regrouped before the next (`docs/backlog/epics/README.md`, Next); the permission rules for `git switch` and `git push origin --delete`; a real browser for tests when needed.
