# Research request 0004 — Regular polygons in process and synoptic symbols

**Status**: done (→ `notes/0004-f02-regular-polygon.md`) — run by the Product Owner in claude.ai on 2026-10-09

## Precise question

Which regular polygons (number of sides, orientation: flat base or standing on a vertex) are used as outlines of graphical symbols in ANSI/ISA-5.1-2009 (R2022) Table 5.1.1, ISO 10628-2:2012, ISO 14617 and IEC 60617, and in the display guidance of ANSI/ISA-101.01-2015 and ANSI/ISA-18.2-2016 (for example alarm priority indicators)? For each one: the polygon, its orientation, its meaning, and the clause or table that defines it. Is the "diamond" a square standing on a vertex? Is any regular polygon with more than 8 sides used?

## Why it blocks

Q19 (`docs/domain/README.md`): the largest number of corners a regular polygon may have (F02, US-005 validity check). The Product Owner wants the limit to follow what synoptic and process symbols really use.

## Already consulted

| Source                                          | What it says                                                                                    | Why insufficient                                                  |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `REF-ISA-INTECH-2019`                           | ISA-5.1 has a "diamond" symbol since 2009, in a box, for safety systems                         | one symbol only, from a quiz page; its geometry is not stated     |
| `REF-INKSCAPE-POLYGON`                          | a "Corners" field; no range stated                                                              | a drawing tool, not a symbol standard                             |
| Search summaries (not read: pages refused, 403) | circle, square, circle in a square, hexagon (computer function) in ISA-5.1; triangles in valves | not readings; ISA-5.1, ISO 10628-2, ISO 14617, IEC 60617 are paid |

## Expected deliverable

- [ ] table: polygon, sides, orientation, meaning, standard and clause
- [ ] complete reference (title, edition, year, URL of the publisher's page)
- [ ] whether a source was read directly or through a summary
- [ ] the largest number of sides found
