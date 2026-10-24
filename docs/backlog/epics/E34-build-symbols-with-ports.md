---
id: E34
title: Build symbols with ports
status: draft
---

# E34 — Build symbols with ports

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the symbol as a document. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E05 and E11; needs E27.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need a drawing turned into a symbol that pipes and views can use |
| the solution | a symbol, its ports and its zones |
| is a | the symbol as a document of the Symbol Editor |
| that | holds a group, an integer origin, ports and zones |
| unlike | a drawing of shapes with no entry nor exit |
| our solution | ports with an id, an integer position and an exit direction; clickable and status zones laid over the symbol |
| Business outcomes | a symbol with ports and zones built in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | integer positions (ADR-0003) |
| In scope | the symbol, its saved position (Q30), ports, snapping to ports, clickable and status zones (`symbols-and-views.md` §1, §2) |
| Out of scope | animatable parts (E35); business meaning (E36) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| A symbol | a symbol as a document, a group with its integer origin; its saved position (Q30) (2) | to refine |
| Ports | a port's id, integer position and exit direction; shown, added and moved in the playground; snapping to ports (3) | to refine |
| Clickable and status zones | a clickable zone laid over the symbol; a shared status zone (2) | to refine |

Size: 3 features, 7 stories, 20 + 25 × 7 + (45 to 75) × 3 = 330 to 420 active minutes, 5.5 to 7 hours (note 0034, §1).

## Taken from the old epics

- E05.1, E05.4 to E05.7: ports, the clickable and status zones, the saved position, snapping to ports (from E19).
- E11.5: Q30 asked here, settled for the files (E41).

## Open points

- Q30, the position a symbol is saved at: the top-left of its box at (0, 0), its center, or left where it is.
