---
id: VAL-019
epic: E17
feature: F18
title: Validate F18
status: ready
points: 1
---

# E17 · F18 · VAL-019 — Validate F18

Show the Product Owner F18 on one demo page: the figures before and after (subagents refused, heavy commands queued, conflicts on shared files, `check:all` under load), and US-061 on the checklist.

## Acceptance criteria

- Given the demo page, when the Product Owner reads it, then each criterion of F18 is shown with its evidence.
- Given the checklist, when the Product Owner tries US-061's lines, then the font list behaves as its test card says.

## Decisions to confirm

- US-061: a face refused when its family is read again is dropped, and a text asking for it is measured with the nearest face held, as for a weight the family lacks; the alternative, only saying the refusal, would have measured a face then left it out of the export (rule of US-060). Chosen by the agent on 2026-10-17, the choice left to it by the criterion.

## Tasks

One task = one commit, referenced as `VAL-019.Tn`.

- [ ] T1 — Demo page and checklist lines (1 h)
- [ ] T2 — The Product Owner validates; the result recorded (1 h)
