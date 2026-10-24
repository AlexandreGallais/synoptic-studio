---
id: E33
title: Choose fonts for synoptic views
status: draft
---

# E33 — Choose fonts for synoptic views

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the fonts found, shipped and exported. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E20; needs E15.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need fonts that read well on factory screens and can be exported |
| the solution | a font list, and fonts shipped with the software |
| is a | the editors' fonts |
| that | says which fonts can be used and embedded (ADR-0062) |
| unlike | a font file picked by hand for each session (F15) |
| our solution | a search over the installed fonts, a curated set of free fonts shipped, a font that cannot be embedded forced after a confirmation or refused |
| Business outcomes | a font of the computer or of the shipped set chosen and exported in a symbol |
| Leading indicators | features accepted |
| Non-functional requirements | embedded fonts as ADR-0053 and ADR-0060 write them |
| In scope | the font list, the fonts shipped (`shapes.md` Text, Fonts) |
| Out of scope | a font embedded once per view (E38) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The font list | a search over the installed fonts; the list saying which fonts can be used and exported; a font of the computer chosen and imported (3) | to refine |
| Fonts shipped | free fonts shipped and installed with the software, a curated set; a font that cannot be embedded forced after a confirmation or refused (ADR-0062); the fonts of no use refused (3) | to refine |

Size: 2 features, 6 stories, 20 + 25 × 6 + (45 to 75) × 2 = 260 to 320 active minutes, 4.3 to 5.3 hours (note 0034, §1).

## Taken from the old epics

- E20.1, E20.13, E20.14: fonts found and shipped, a curated set (Comic Sans, Braille and emoji fonts refused), a font that cannot be embedded (the agent's placement, 2026-10-25).
- D.4: Q42, font subsetting at export, unsized.

## Open points

- Q42, font subsetting at export: which texts are fixed at export is a business question still open (note 0034, §5).
