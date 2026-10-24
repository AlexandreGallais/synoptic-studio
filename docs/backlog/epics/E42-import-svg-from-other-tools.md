---
id: E42
title: Import SVG from other tools
status: draft
---

# E42 — Import SVG from other tools

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: another tool's SVG read. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E11; needs E41.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | have symbols drawn in other tools |
| the solution | an SVG importer |
| is a | a reader of plain SVG into the editor's shapes |
| that | warns past a million units and refuses beyond a level of complexity (Q20) |
| unlike | a symbol drawn again by hand |
| our solution | paths, rectangles, circles and ellipses read as shapes, straight lines simplified, the drawing scaled down to a sensible size |
| Business outcomes | an SVG of another tool opened as editable shapes |
| Leading indicators | features accepted |
| Non-functional requirements | integer inputs after the import (ADR-0003) |
| In scope | a plain SVG read, simplified and scaled down |
| Out of scope | SVG features the editor does not draw (filters, text on a path) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| A plain SVG read | paths, rectangles, circles and ellipses read as shapes; a warning past a million units (Q20); refused beyond a level of complexity (3) | to refine |
| Simplified and scaled down | the straight lines found and simplified; the drawing scaled down to a sensible size (2) | to refine |

Size: 2 features, 5 stories, 20 + 25 × 5 + (45 to 75) × 2 = 235 to 295 active minutes, 3.9 to 4.9 hours (note 0034, §1).

## Taken from the old epics

- E11.3: SVG imported from other sources.

## Open points

None known beyond the refinement's own questions.
