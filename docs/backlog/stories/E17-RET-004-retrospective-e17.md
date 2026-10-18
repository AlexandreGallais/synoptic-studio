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
- [x] T4 — The Product Owner's decisions of 2026-10-18 written: the record, ADR-0083 to ADR-0085, F22 and F23 with their stories, the method in the rules and templates, the logs pruned, the cards moved (2 h)

## For the main thread

### Journal

- `#retro` RET-004: E17's retrospective held with the Product Owner on 2026-10-18 (`docs/process/retrospectives/E17-retrospective.md`); every proposal adopted as recommended. ADR-0083 (the merge into `main` makes the tag and the release, no release branch nor version commit, superseding ADR-0079), ADR-0084 (from E02: features of 2 to 4 stories one at a time, `T1` the interface and its failing tests then `T2` the code, a linear history with a feature rebased with lease when `develop` moved, two agents per story as a trial, researchers on separate topics, a lighter main thread), ADR-0085 (a tool that fails is a card in `docs/process/tool-incidents.md`, tools changed only at an epic's end, amending ADR-0074). New: F22, E17's last feature (EN-116 the CI green, EN-117 the release of ADR-0083, AUD-024, VAL-023), then 0.3.0 and E17's close; F23, E02's first feature (EN-118 to EN-121, AUD-025, VAL-024); E02's NAM-003, REV-005, RET-005 drafted. The improvement log from 113 rows to 32, the feedback log emptied into the habits sheet, the three subagents kept.
- `#decision` Parked until RET-005: the story auto-merge into `develop` for the agent's stories, the permissions of the `story-finish` and `mutate` binaries, ADR-0082's option B, a configuration file for local paths.
