---
id: RET-004
epic: E17
title: Retrospective of E17
status: in-progress
points: 3
---

# E17 · RET-004 — Retrospective of E17

Look back at how E17 was built, decide what changes, review what the growing project needs, update the playbook (ADR-0023, `/retro` skill). Measures ADR-0063's trial of three subagents.

## Acceptance criteria

- Given the facts of E17 and `npm run epic:check`, when the retrospective is held, then both sides' views are recorded in `docs/process/retrospectives/E17-retrospective.md`, and the limit of ADR-0063 is kept or changed.
- Given every row of the improvement and feedback logs and every tool incident card, when the Product Owner has decided them, then each one is an ADR, a story, a rule, a check or "not now" with its reason, and the treated rows leave the logs.
- Given the Product Owner's method wishes of 2026-10-18 (smaller features, tests first, two agents per story, incident cards, a release from a tag), when they are decided, then each is written where it applies, and 0.3.0 is published with the release flow chosen.

## Tasks

One task = one commit, referenced as `RET-004.Tn`.

- [x] T1 — The record: the facts, the epic's check, the agent's account and review, evolvability, the logged items and the proposals to decide, the sources read (3 h)
- [x] T2 — Playbook and the agent's tools page: the habits of E17's feedback, the starter kit, the tools that would have spared writing (45 min)
- [x] T3 — Agent configuration: a tool that fails during a run is a card, treated at the epic's end (run and retro skills) (20 min)
- [ ] T4 — The Product Owner's decisions written in the record, with their ADRs, stories and rules (main thread)

## For the main thread

### Journal

- `#retro` RET-004 prepared: the record `docs/process/retrospectives/E17-retrospective.md` holds E17's facts, its epic check (two lines to look at, and a CI red on `develop` → `main` that the check does not read), the agent's account, every logged row and tool incident card with a recommendation, and proposals P1 to P11 for the Product Owner; the playbook, the tools page and the run, retro and worker skills updated (a failing tool becomes a card, fixed at the epic's end).

Once the Product Owner has answered: the decisions go into §2, §6 and §7 of the record; the feedback rows F1 to F6 leave `docs/process/feedback.md` (they are habits now, §4); the improvement rows decided leave `docs/process/improvements.md`; the cards move to where P9 settles them. The Product Owner's answers to §3 (what helped, what hurt) are asked in the same questionnaire.
