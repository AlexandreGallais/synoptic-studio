---
id: SP-033
epic: E02
title: Regroup the epics not done into small epics of one subject
status: in-progress
points: 2
---

# E02 · SP-033 — Regroup the epics not done into small epics of one subject

Decided at REV-008 (2026-10-24, ADR-0100, decision 1): before the next epic starts, every epic not done (E18, E19, E13, E03, E04, E05, E06, E07, E08, E09, E10, E11, E20, E12, and E16 given) is read again and regrouped into small epics holding one subject each; the Product Owner allows any of them to be dropped and written again. The spike proposes the new list; the Product Owner orders it; the epic files are rewritten only after that.

## Acceptance criteria

- Given the epics not done, when the spike ends, then a note lists the proposed epics, each with one subject, its features and each feature's count of stories (2 to 4, each of at most 3 tasks), and a duration from the measured medians by kind (a spike about 20 minutes, a story about 25, an audit's fixes 45 to 75).
- Given every feature, idea and card of the epics not done, when the spike ends, then the note maps each to the new epic that takes it, or says why it is dropped: nothing is lost silently.
- Given the questions the regrouping raises, when the spike ends, then they are listed for the Product Owner, each with options and a numeric example, the order of the epics among them.

## Questions

- _filled at T1_

## Tasks

One task = one commit, referenced as `SP-033.Tn`.

- [ ] T1 — Inventory: each epic not done, its features, ideas and cards, its dependencies, the domain files it rests on
- [ ] T2 — The new list: subjects, epics, features and their sizes, the map from the old epics
- [ ] T3 — The research note and the questions for the Product Owner
