---
id: E08
title: Assemble synoptic views
status: dropped
---

# E08 — Assemble synoptic views

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E38](./E38-assemble-synoptic-views.md), whole.

| Field | Content |
| --- | --- |
| For | business users |
| who | need to compose views from business symbols |
| the solution | the View Editor |
| is a | instances placed, rotated by quarter turns, with overridden defaults |
| that | views built without touching geometry or configuration |
| unlike | drawing every view by hand |
| our solution | defaults overridden with a live animation preview; pop-up lines checked or unchecked |
| Business outcomes | a view with several instances, rotated, with overridden defaults and chosen pop-up lines is assembled |
| Leading indicators | a reference view assembled and reopened identically |
| Non-functional requirements | integer positions; quarter turns (ADR-0008) |
| In scope | instancing, duplication, quarter-turn rotation, default overrides, live preview, pop-up line selection |
| Out of scope | adding properties not configured, geometry editing |
| Closure criteria | a reference view assembled |

## Candidate features

| Feature | Status |
| --- | --- |
| Instances | to refine |
| Rotation | to refine |
| Default overrides with live preview | to refine |
| Pop-up lines | to refine |
