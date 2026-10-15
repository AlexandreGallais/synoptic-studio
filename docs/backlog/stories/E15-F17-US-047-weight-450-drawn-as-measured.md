---
id: US-047
epic: E15
feature: F17
title: A weight such as 450 drawn with the face it is measured with
status: ready
points: 2
---

# E15 · F17 · US-047 — A weight such as 450 drawn with the face it is measured with

As a symbol designer, I want a text drawn with the face it is measured with so that its frame hugs its letters at every weight.

From the Product Owner's validation of E15 (2026-10-15): weight 450 of a family such as IBM Plex was measured but not visibly drawn, the browser seeming to draw another face. Cause from SP-019.

## Acceptance criteria

- Given a family holding a 450 face (IBM Plex Sans Text, named by SP-019), when a text is set to 450, then the browser draws that face: the drawn line's width, read in a test page, equals the measured width within 0.5 of a unit.
- Given the same family at 400 and 500, when drawn, then each is drawn with its own face, as measured.
- Given a family without a 450 face, when 450 is asked, then the face the Font face rule matches is measured and drawn (`shapes.md`, Font face).

## Product Owner test

| Step | Do                                             | You should see                                            |
| ---- | ---------------------------------------------- | --------------------------------------------------------- |
| 1    | Choose IBM Plex Sans, weight 450, "Pump P-101" | letters a little heavier than 400, the frame hugging them |
| 2    | Switch between 400, 450 and 500                | three different weights, the frame following              |

## Tasks

One task = one commit, referenced as `US-047.Tn`.

- [ ] T1 — The fix from SP-019's cause, with a test (2 h)
- [ ] T2 — Test card, guided step and `shapes.md` when a rule changes (1 h)
