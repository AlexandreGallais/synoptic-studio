---
id: US-046
epic: E15
feature: F17
title: Installed fonts read from collections, and faces that failed
status: ready
points: 3
---

# E15 · F17 · US-046 — Installed fonts read from collections, and faces that failed

As a symbol designer, I want every installed font I choose to be read so that I can use the fonts of my computer.

From the Product Owner's validation of E15 (2026-10-15): installed faces held in a collection were refused as "not a TrueType or OpenType font, collection among them", and some faces failed with a `TypeError`. Causes from SP-019.

## Acceptance criteria

- Given an installed family whose faces are held in a collection (`.ttc`), when it is chosen, then each face is read from the collection at its own offset (`REF-OT-OTFF`), counted as read, and the text is measured and drawn with it.
- Given an installed face whose access failed with a `TypeError`, when its family is chosen, then the face is read once SP-019's cause is handled, or refused with that cause said in one line, the other faces kept, and no error left uncaught.
- Given a collection holding faces of several families, when one family is chosen, then only that family's faces are kept.

## Product Owner test

| Step | Do                                                                           | You should see                                   |
| ---- | ---------------------------------------------------------------------------- | ------------------------------------------------ |
| 1    | List the installed fonts, choose a family held in a `.ttc` (named by SP-019) | "…: n faces read.", none refused as a collection |
| 2    | Choose the family that failed with a `TypeError`                             | its faces read, or one line saying why           |

## Tasks

One task = one commit, referenced as `US-046.Tn`.

- [ ] T1 — A face read from a collection in the library (`ttcf` header, its offsets), with tests (3 h)
- [ ] T2 — The playground reads installed faces from collections and handles the `TypeError` (2 h)
- [ ] T3 — `shapes.md`, the test card and the guided step (1 h)
