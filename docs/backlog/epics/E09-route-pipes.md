---
id: E09
title: Connect symbols with orthogonal pipes
status: dropped
---

# E09 — Connect symbols with orthogonal pipes

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E39](./E39-connect-symbols-with-orthogonal-pipes.md), whole.

| Field | Content |
| --- | --- |
| For | business users |
| who | need readable connections |
| the solution | orthogonal pipes |
| is a | connectors routed between ports |
| that | pipes follow symbols and avoid them |
| unlike | manual polylines |
| our solution | literature routing with P&ID conventions |
| Business outcomes | pipes created port to port, routed around symbols, following moves, adjustable |
| Leading indicators | pipes demonstrated on a reference view |
| Non-functional requirements | orthogonal only (ADR-0004) |
| In scope | routing, following moves, segment dragging, rounded bends, crossings |
| Out of scope | diagonals |
| Closure criteria | a reference circuit stays orthogonal through moves |

## Candidate features

| Feature | Status |
| --- | --- |
| Port-to-port pipe | to refine |
| Obstacle-avoiding routing | to refine |
| Manual adjustment | to refine |
| Bends and crossings | to refine |
