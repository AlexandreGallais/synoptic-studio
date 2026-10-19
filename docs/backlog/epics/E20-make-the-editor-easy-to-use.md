---
id: E20
title: Make the editor easy to use
status: draft
---

# E20 — Make the editor easy to use

The later epic on ease of use the Product Owner named at the validations of E15 and F08, its ideas gathered here from E02 on 2026-10-19 (SP-025). Not in the run order: the Product Owner places it later. Its aim is the fewest options doing the most; its edition, free V1 or pro, is decided then (ADR-0030).

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | are not power users and should not have to learn options of no use to a synoptic view |
| the solution | fewer options, fonts chosen from a list that says what each can do, messages that explain |
| is a | a pass over the editor's options and messages |
| that | keeps only what a synoptic view needs |
| unlike | every option a general drawing tool offers |
| our solution | options of no use dropped, fonts suited to synoptic views shipped, each action that changes a shape's nature explained once |
| Business outcomes | to refine |
| Leading indicators | to refine |
| Non-functional requirements | to refine |
| In scope | the ideas below |
| Out of scope | to refine |

## Features

Candidates grouped by the agent at SP-025 (2026-10-19), to be cut again at the epic's refinement.

| Feature | Expected stories | Status |
| --- | --- | --- |
| Fonts found and shipped | a search over the installed fonts; free fonts shipped and installed with the software; a font chosen from the computer, the list saying which can be exported; a font that cannot be embedded, exported after a confirmation or refused (2 to 4) | to refine |
| Fewer text options | the text direction removed; the underline's and the strike-through's colors reset by a button, or removed (2) | to refine |
| Lines of a text | trailing empty lines taking no space; one line in a box, the line breaks ignored when the box is wide enough (2) | to refine |
| A turned text as a shape | a turned text handled as rectangles, polygons and round shapes are, with E18's tools (2 to 3) | to refine |
| Messages that explain | a polygon converted to a free shape says what happens, with a **do not show again** box (1 to 2) | to refine |

## Ideas from the Product Owner (2026-10-11, refinement of F08)

- When a polygon is converted to a free shape, a message saying what happens, with a **do not show again** box.

## Ideas from the Product Owner (2026-10-15, validation of E15)

- **A search field over the installed fonts**: typing filters the font list.
- **A reset button on the stroke colors**: the underline's and the strike-through's colors get a button that makes them follow the text's color again, instead of the box "follows the text's color".
- **Fewer options**: simplify the Text tool's options, the stroke colors perhaps removed (kept for now, VAL-016).

## Ideas from the Product Owner (2026-10-16, first and second passes of VAL-018)

- **Trim trailing empty lines at run time**: line breaks with nothing after them take no space.
- **One line in a box**: in box mode, an option to show everything on one line, ignoring line breaks, when the box is wide enough; it adds a model field, so it was not done then.
- **A curated set of fonts**: the software ships fonts suited to synoptic views; installing a local font is a choice, through an "Install" button listing the local fonts to pick from, beside importing a font file. Not every font is accepted (Comic Sans, Braille and emoji fonts are of no use in a synoptic view).
- **Turned text**: handle a turned text like rectangles, polygons and round shapes.

## Ideas from the Product Owner (2026-10-16, decisions after US-056)

- **Fonts shipped**: free fonts that can be exported, installed with the software.
- **A font chosen from the computer**: the designer selects an installed font to import, and the software says whether it can be exported.
- **A font that cannot be embedded**: exported once the designer forces it after a confirmation, or refused outright — to decide then (ADR-0062 refuses it for now).
- **A font list that says more**: which fonts are usable, and which can be exported.
- **Fonts installed automatically**.

## Ideas from the Product Owner (2026-10-16, review of E15)

- **Remove the text direction**: right to left is of no use, the Latin script being the strict target, as for fonts. It stays until then; this epic drops the options that make no sense, the aim being the fewest options doing the most.
