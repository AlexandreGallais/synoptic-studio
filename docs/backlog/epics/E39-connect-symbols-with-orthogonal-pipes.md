---
id: E39
title: Connect symbols with orthogonal pipes
status: draft
---

# E39 — Connect symbols with orthogonal pipes

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: pipes between ports. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E09; needs E34's ports and E38.

| Field | Content |
| --- | --- |
| For | view designers |
| who | need pipes drawn between symbols as process diagrams draw them |
| the solution | orthogonal pipes from port to port (ADR-0004) |
| is a | the views' pipes |
| that | routes around symbols, as the literature does, with P&ID conventions |
| unlike | lines drawn and moved by hand |
| our solution | a pipe created by two clicks, routed, following the symbols' moves, adjusted by hand |
| Business outcomes | pipes created port to port, routed around symbols, following moves, adjustable |
| Leading indicators | features accepted |
| Non-functional requirements | orthogonal only (ADR-0004) |
| In scope | routing, following moves, segment dragging, rounded bends, crossings (`symbols-and-views.md` §5; `REF-WYBROW-2009`, `REF-MARRIOTT-2014`, `REF-LIBAVOID`) |
| Out of scope | diagonals |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| A pipe from port to port | a pipe created by two clicks on ports; its first orthogonal path; drawn and stored (3) | to refine |
| Routing around symbols | the routing that avoids the symbols; its cost measured on a reference view; re-routed when a symbol moves (3) | to refine |
| Manual adjustment | a segment dragged perpendicular to itself; its neighbors adapting; kept through later moves (3) | to refine |
| Bends and crossings | rounded bends by the corner radius; a crossing broken on the vertical line (2) | to refine |

Size: 4 features, 11 stories, 20 + 25 × 11 + (45 to 75) × 4 = 475 to 595 active minutes, 7.9 to 9.9 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E09.1 to E09.4: the port-to-port pipe, the routing, the manual adjustment, bends and crossings.

## Open points

- The routing's size: its sources are read only at the refinement, and an incremental router may outgrow 3 stories (note 0034, §5).
