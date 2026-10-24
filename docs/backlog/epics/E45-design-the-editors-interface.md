---
id: E45
title: Design the editor's interface
status: draft
---

# E45 — Design the editor's interface

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: every button and option questioned. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E20; needs E21 to E44.

| Field | Content |
| --- | --- |
| For | symbol and view designers |
| who | need an editor that is quick to learn and never slows them |
| the solution | the interface designed once the library is built |
| is a | a review of every button, option and message of the editors |
| that | asks of each whether it matters, whether it is said twice, whether it is clear, whether it helps or slows the designer (the order rule of the epics, 2026-10-19) |
| unlike | the playground's fields, which may offer more than a designer needs |
| our solution | a page for any window, messages that explain what an action does, the history's list fast at any length |
| Business outcomes | to refine at the epic's refinement |
| Leading indicators | features accepted |
| Non-functional requirements | the history's list fast with 1 000 000 steps |
| In scope | the review of every button and option, a page for any window, messages that explain, the history's list scrolled whole, the selection rectangle that touches |
| Out of scope | the settings and keys (E43); the layers panel (E44) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| A page for any window | the page fitting any window; a toolbar at the bottom as Figma's, the selector first, then each shape, then the multiple selection (3) | to refine |
| Messages that explain | a polygon converted to a free shape saying what happens; a **do not show again** box (2) | to refine |
| The history's list scrolled whole | every row of a history of 1 000 000 steps reached by the scroll, only the rows in sight drawn, 15 of them (US-083's draft) (1) | to refine |

The list scrolled whole is a single story: the subject holds no more (decided on 2026-10-23, SP-029; kept here by the agent on 2026-10-25). The review of every button and option, the epic's subject, makes the rest of its features at its refinement.

Size: 3 features, 6 stories, 20 + 25 × 6 + (45 to 75) × 3 = 305 to 395 active minutes, 5.1 to 6.6 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E20.5, E20.8: messages that explain, a page for any window.
- E20.7: the history's list scrolled whole, US-083's draft.
- E20.12, E20.16: the review of every button, option and message; the selection rectangle taking a shape it touches by one pixel, to discuss.

## Open points

- Its features beyond the three known come from the review of every button and option, made at its refinement (note 0034, §5).
