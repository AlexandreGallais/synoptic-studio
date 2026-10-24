---
id: E32
title: Refine the text's lines and options
status: draft
---

# E32 — Refine the text's lines and options

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the text's lines, options and turn, a second pass over E15. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E20; needs E22.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | place texts in symbols and need them simple and predictable |
| the solution | a second pass over E15's text |
| is a | the text's lines, options and turn |
| that | drops the options of no use in synoptic views and treats a turned text as any shape |
| unlike | trailing empty lines taking space, a direction option, a text's box that can no longer be picked |
| our solution | lines that take only the space they need, fewer options, a turned text resized and turned by E21's and E22's tools |
| Business outcomes | a text edited, turned and resized as a shape in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | the text's measure without the DOM (ADR-0025) |
| In scope | the lines of a text, fewer options, a turned text as a shape (`shapes.md` Text) |
| Out of scope | fonts (E33); text styles (E37); a text scrolled in its box (E38) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The lines of a text | trailing empty lines taking no space; one line in a box when it is wide enough, a model field; a text with a negative word spacing picked and its box computed again (3) | to refine |
| Fewer text options | the direction removed; the underline's and the strike-through's colors reset by a button, or removed (2) | to refine |
| A turned text as a shape | a turned text resized and turned by the tools of E21 and E22; its box's handles (2) | to refine |

Size: 3 features, 7 stories, 20 + 25 × 7 + (45 to 75) × 3 = 330 to 420 active minutes, 5.5 to 7 hours (note 0034, §1).

## Taken from the old epics

- E20.2, E20.3, E20.4, E20.11: fewer text options, the lines of a text, a turned text as a shape, the box of a text with a negative word spacing (the agent's placement, 2026-10-25).

## Open points

None known beyond the refinement's own questions.
