---
id: E38
title: Assemble synoptic views
status: draft
---

# E38 — Assemble synoptic views

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: instances placed in a view. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E08; needs E37.

| Field | Content |
| --- | --- |
| For | view designers |
| who | need views of many symbols, each showing its own values |
| the solution | the View Editor |
| is a | a view of instances of business symbols |
| that | places instances on integers, turned by quarter turns (ADR-0008) |
| unlike | a view drawn shape by shape |
| our solution | defaults overridden with a live animation preview; pop-up lines checked or unchecked |
| Business outcomes | a view with several instances, turned, with overridden defaults and chosen pop-up lines is assembled |
| Leading indicators | features accepted |
| Non-functional requirements | integer positions; quarter turns (ADR-0008) |
| In scope | instances, duplication, quarter turns, default overrides, live preview, pop-up lines (`symbols-and-views.md` §4, `configuration.md` §5); a font per view; a text scrolled in its box |
| Out of scope | properties not configured; geometry editing |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Instances and quarter turns | an instance placed at an integer position; duplicated; turned by quarter turns about its origin; its ports turned with it (4) | to refine |
| Overrides, preview and pop-up lines | a default overridden per instance; the live preview of its animation; the pop-up's lines unchecked and checked again; a line hidden by default (4) | to refine |
| Texts in a view | a font embedded once for the view's symbols; a long text scrolled in its box on a touch screen (2) | to refine |

Size: 3 features, 10 stories, 20 + 25 × 10 + (45 to 75) × 3 = 405 to 495 active minutes, 6.8 to 8.3 hours (note 0034, §1).

## Taken from the old epics

- E08.1 to E08.6: instances, quarter turns, overrides, pop-up lines, a font embedded once per view (F15), a long text scrolled on a touch screen (F14).

## Open points

None known beyond the refinement's own questions.
