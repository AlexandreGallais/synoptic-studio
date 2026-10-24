---
id: E24
title: Compute in number fields
status: draft
---

# E24 — Compute in number fields

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: arithmetic typed in a field. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E18 and E20; needs E02.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to type a computation rather than its result |
| the solution | number fields that compute |
| is a | one reader of every number field, in the library |
| that | reads `+`, `−`, `×`, `÷` and parentheses, as Figma's fields (`REF-FIGMA-ADJUST`) |
| unlike | a number or a sign alone, an expression refused (`interaction.md` §5) |
| our solution | every field of the page reading its text by one function; a letter refused as it is typed |
| Business outcomes | `+10` moves several points or shapes, `+100` works in Width, `1a` cannot be typed |
| Leading indicators | features accepted |
| Non-functional requirements | the value stored stays an integer (ADR-0003, Q45) |
| In scope | arithmetic in every number field, a letter refused as typed |
| Out of scope | units, variables |

## Features

Proposed at SP-033 (2026-10-25) as a draft: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2). A single feature: the subject holds no more.

| Feature | Expected stories | Status |
| --- | --- | --- |
| Arithmetic in every number field | `+`, `−`, `×`, `÷` and parentheses read by one function of the library; every field of the page using it, `+10` moving several points or shapes, `+100` in Width reproduced; a letter refused as it is typed (3) | to refine |

Size: 1 feature, 3 stories, 20 + 25 × 3 + (45 to 75) × 1 = 140 to 170 active minutes, 2.3 to 2.8 hours (note 0034, §1).

## Taken from the old epics

- E18.5, E18.15: number fields that compute, `+100` in Width seen not to work.
- E20.10: a letter refused as it is typed (the agent's placement, 2026-10-25).

## Open points

None known beyond the refinement's own questions.
