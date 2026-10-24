---
id: E35
title: Animate parts of symbols
status: draft
---

# E35 — Animate parts of symbols

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: what moves in a symbol. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E05; needs E34 and E31.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need parts of a symbol to show a state |
| the solution | animatable parts and their preview |
| is a | the symbol's animations |
| that | animates parts, never redraws them |
| unlike | a symbol drawn again for each state |
| our solution | a part marked animatable and the kinds it accepts; color, blinking, opacity, visibility, partial fill and rotation previewed in the editor |
| Business outcomes | a symbol with animatable parts previews each animation |
| Leading indicators | features accepted |
| Non-functional requirements | animations smooth enough for a synoptic view |
| In scope | animatable parts, the six animations, the preview (`symbols-and-views.md` §3, Q13) |
| Out of scope | the rules driving them (E36) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Parts declared animatable | a part marked animatable and the kinds it accepts; shown in the panel (2) | to refine |
| The six animations | color and color blinking; opacity, visibility and their blinking; partial fill and rotation, through the remapping (Q13) (3) | to refine |
| The animation preview | values given by hand in the editor; every animation played in the preview (2) | to refine |

Size: 3 features, 7 stories, 20 + 25 × 7 + (45 to 75) × 3 = 330 to 420 active minutes, 5.5 to 7 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E05.2, E05.3: animatable parts, the animation preview.

## Open points

- How a synoptic plays its animations outside the editor (a player of the library, CSS, or the host's code): settled at this epic's refinement from its sources (2026-10-25, SP-033's answer 11; note 0034, §5).
