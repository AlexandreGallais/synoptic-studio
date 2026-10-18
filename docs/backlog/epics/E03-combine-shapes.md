---
id: E03
title: Combine shapes without losing their numbers
status: draft
---

# E03 — Combine shapes without losing their numbers

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need composite outlines |
| the solution | non-destructive booleans and shape builder |
| is a | boolean nodes evaluated at each render |
| that | operands stay integer and editable |
| unlike | destructive path operations |
| our solution | nothing is flattened; the result is recomputed |
| Business outcomes | the six operations and the shape builder work on rounded shapes and stay editable |
| Leading indicators | operations accepted on reference cases |
| Non-functional requirements | exact on segments and arcs; robust predicates |
| In scope | union, difference, intersection, exclusion, division, cut path, shape builder |
| Out of scope | destructive flattening, corner radius on created vertices (Q9) |
| Closure criteria | each operation demonstrated on rounded operands edited afterwards |

## Candidate features

| Feature | Status |
| --- | --- |
| Union and difference | to refine |
| Intersection and exclusion | to refine |
| Division and cut path | to refine |
| Shape builder | to refine |

## Ideas from the Product Owner (2026-10-11, refinement of F08)

- Operations on edges: cut the contour between two points, join two points, delete an edge, add a connection drawn over the shape; then rebuild shapes from the pieces, with the intersections of this epic.
