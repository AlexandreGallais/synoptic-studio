---
id: E29
title: Combine shapes by boolean operations
status: draft
---

# E29 — Combine shapes by boolean operations

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the six operations, non-destructive. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E03; needs E27.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need shapes combined without losing their numbers |
| the solution | non-destructive boolean operations (Q2, ADR-0006) |
| is a | a node of the shape tree holding its operands |
| that | recomputes the result from the operands, nothing flattened |
| unlike | the destructive booleans of drawing tools |
| our solution | the operands stay editable, the result drawn again |
| Business outcomes | the six operations work on rounded shapes and stay editable |
| Leading indicators | features accepted |
| Non-functional requirements | exact on segments and arcs; robust predicates (`shapes.md` §5) |
| In scope | union, difference, intersection, exclusion, division, cut path (`REF-MARTINEZ-2009`, `REF-GREINER-HORMANN`) |
| Out of scope | destructive flattening; corner radius on created vertices (Q9); the shape builder (E30) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Union and difference | the boolean node in the model, its operands kept; the intersections of segments and arcs, robust; union and difference drawn (3) | to refine |
| Intersection and exclusion | intersection; exclusion (2) | to refine |
| Division and cut path | division into pieces; the outlines cut at their intersections; the operands edited afterwards (3) | to refine |

Size: 3 features, 8 stories, 20 + 25 × 8 + (45 to 75) × 3 = 355 to 445 active minutes, 5.9 to 7.4 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E03.1 to E03.3: union, difference, intersection, exclusion, division and cut path.

## Open points

- Booleans on curved sides (cubics, ADR-0046, ADR-0048): the domain names an extension of the sweep to arcs (`shapes.md` §5) and nothing for cubics, whose intersection has no exact closed form; subdivision, an approximation or curves refused as operands is a source question, perhaps an ADR (note 0034, §5).
- Whether the stories of robust geometry fit the 25 minutes measured on E02's interaction stories.
- Q9, the corner radius on vertices created by a boolean.
