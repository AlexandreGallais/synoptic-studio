---
id: E23
title: Drag points and handles
status: draft
---

# E23 — Drag points and handles

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the points moved by the mouse, and what keeps a shape sound while they move. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E18 and the cards met on the way; needs E21.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to reshape a free shape by the mouse, exactly |
| the solution | points and handles dragged on the drawing |
| is a | the editors' point tool |
| that | every point dragged lands on integers |
| unlike | x and y typed in the list of points one by one |
| our solution | points and handles dragged alone or together; two points at the same place and crossing edges flagged; a polygon's removed corners given their radii back while the session lasts |
| Business outcomes | a free shape reshaped by the mouse in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | integer inputs after every drag (ADR-0003) |
| In scope | dragged points and handles (`interaction.md` §3), the flags of Q29, the radii of removed corners (Q26, `shapes.md` §2) |
| Out of scope | snapping (E26); the pen (E40); Figma's Bend tool (`shapes.md`, out of scope) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Points dragged | a point dragged on integers; several selected points moved together; a curve's handles dragged (3) | to refine |
| Corners kept and degenerate shapes flagged | two points at the same place flagged with an offer to delete one; crossing edges flagged (Q29); a polygon's removed corners given their radii back while the session lasts, 12 corners made 3 then 12 again (Q26, settled on 2026-10-25) (3) | to refine |

Size: 2 features, 6 stories, 20 + 25 × 6 + (45 to 75) × 2 = 260 to 320 active minutes, 4.3 to 5.3 hours (note 0034, §1).

## Taken from the old epics

- E18.4, E18.12: drag points and handles, points dragged together.
- D.2: the flags of degenerate free shapes (Q29).
- D.3: Q26, the radii of removed corners, kept while the session lasts, not in the saved file (Product Owner, 2026-10-25, SP-033).

## Open points

None known beyond the refinement's own questions.
