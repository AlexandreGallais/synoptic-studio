---
id: EN-086
epic: E17
feature: F20
title: A run that parks a question and pushes it, and stops only when it must
status: draft
points: 2
---

# E17 · F20 · EN-086 — A run that parks a question and pushes it, and stops only when it must

Decided by the Product Owner at F20's refinement (2026-10-17):

- **Scope**: the agent sets no bound of its own; the Product Owner sets a run's scope at its start, as a time ("until I am back tonight") or an end ("stop at epic E02").
- **A question never stops a run**: it parks its story, and the run goes on with work that does not depend on it. A run stops only on a problem it cannot get around, at the end of its scope, or when the Product Owner says so.
- **Pushes**, on the desktop and the phone, only for a question or a blocker that prevents finishing the scope asked; never at a plain stop, at a merge, nor to say a run ended early.

`PushNotification` sends a desktop notification, and a phone push when Remote Control is connected (`REF-CC-TOOLS-REFERENCE`); Claude decides when to push, with no setting per event, and the repository cannot turn Remote Control on (`REF-CC-REMOTE-CONTROL`). So the run skill states when to call it; the Product Owner turns on Remote Control for every session and **Push when Claude decides** in `/config`, once, on their side ([research note 0024](../../research/notes/0024-f20-context-diet.md)).

## Acceptance criteria

- Given `/run`, when it starts, then the scope the Product Owner set — a time or an end — is recorded in the journal and the run skill works to it, without a bound of its own.
- Given a question for the Product Owner during a run, when it arises, then its story is parked with the question written in it, a push says it in one line, and the run goes on with the next story that does not depend on it.
- Given a failure the run cannot get around, when it blocks the scope, then a push says what blocks, and the run goes on with independent work if any, else stops.
- Given a story merged, a plain stop or a scope ended, when it happens, then no push is sent.
- Given the setting on the Product Owner's side, when it is done, then one push is tried on the desktop and on the phone.

## Tasks

One task = one commit, referenced as `EN-086.Tn`.

- [ ] T1 — The run skill: the scope set at the start, a question parking its story, the stops (2 h)
- [ ] T2 — When to push, in the run skill; the Product Owner's setting in `docs/tooling/agent.md`; one push tried on each device (1 h)
