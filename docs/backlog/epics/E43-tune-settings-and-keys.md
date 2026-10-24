---
id: E43
title: Tune settings and keys
status: draft
---

# E43 — Tune settings and keys

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: what the designer tunes. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E20 and E02's remarks; needs E02.

| Field | Content |
| --- | --- |
| For | symbol and view designers |
| who | need the editor to answer their hands and their habits |
| the solution | settings of the view, keys of one's own and an expert menu |
| is a | the editors' settings |
| that | keeps Figma's keys by default (Q50), each one changeable |
| unlike | fixed keys and a fixed pan and zoom |
| our solution | the pan's speed, the zoom's step and direction tuned by hand; every shortcut and modifier changed in a standard place; every option of the free editor in one menu |
| Business outcomes | the Product Owner finds good values, which become the fixed defaults |
| Leading indicators | features accepted |
| Non-functional requirements | settings kept from one session to the next |
| In scope | the view's settings, keys editable (`interaction.md` §1), the expert menu |
| Out of scope | the review of every button and option (E45) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Settings of the view | the pan's speed by the trackpad and the wheel, the zoom's step and the wheel's factor tuned by hand; the zoom's direction (2) | to refine |
| Keys of one's own | a standard place for settings; every shortcut and modifier changed there, Figma's by default (2) | to refine |
| The expert menu | every option of the free editor in one menu; kept from one session to the next (2) | to refine |

Size: 3 features, 6 stories, 20 + 25 × 6 + (45 to 75) × 3 = 305 to 395 active minutes, 5.1 to 6.6 hours (note 0034, §1).

## Taken from the old epics

- E20.6, E20.9, E20.15: keys of one's own, the view's settings and the expert menu, Ctrl + wheel's zoom direction.

## Open points

None known beyond the refinement's own questions.
