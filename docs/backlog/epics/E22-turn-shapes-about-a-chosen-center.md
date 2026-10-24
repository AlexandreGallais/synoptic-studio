---
id: E22
title: Turn shapes about a chosen center
status: draft
---

# E22 — Turn shapes about a chosen center

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the rotation mode and its handles. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E18; needs E21.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to turn shapes by the mouse about the point they choose |
| the solution | a rotation mode, its center and the turned box's handles |
| is a | the editors' rotation tool |
| that | one angle per shape, its position the only stored fraction (ADR-0035) |
| unlike | an angle typed in a field, always about the box's center |
| our solution | a mode entered by a button, the center free or on an anchor or a point, handles on the turned box as Figma's |
| Business outcomes | a shape turned about any center in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | integer inputs after every drag (ADR-0003); a new center moving nothing |
| In scope | the rotation mode and its centers, the turned box's handles, centers on points (`shapes.md` §1 Rotation, `interaction.md` §3) |
| Out of scope | the straight box's handles (E21); a turned text as a shape (E32); turning by an animation (E35) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The rotation mode and its center | the mode entered by a button, its center shown, the box's center by default; the center free at an x and y, or on one of the nine anchors; one angle only, a new center moving nothing (3) | to refine |
| The turned box's handles | handles on the turned box's corners, as Figma's; the straight box hidden or dashed (2) | to refine |
| Centers on points | the center on a point of the shape, following it; on a point or the center of another shape, placed only (2) | to refine |

Size: 3 features, 7 stories, 20 + 25 × 7 + (45 to 75) × 3 = 330 to 420 active minutes, 5.5 to 7 hours (note 0034, §1).

## Taken from the old epics

- E18.2, E18.7, E18.8: turn about a chosen center, the rotation mode, one angle only.
- E18.3, E18.9, E18.11: handles on the turned box, centers on points, the straight box hidden or dashed.

## Open points

None known beyond the refinement's own questions.
