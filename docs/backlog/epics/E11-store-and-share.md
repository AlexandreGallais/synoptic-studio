---
id: E11
title: Store and share as files in a working folder
status: draft
---

# E11 — Store and share as files in a working folder

| Field | Content |
| --- | --- |
| For | every user |
| who | need their work persisted and shared |
| the solution | files in a working folder |
| is a | files read and written in a folder on the user's computer, one by one |
| that | symbols, configurations, libraries, drawings and views reloaded identically |
| unlike | no persistence |
| our solution | local only in the free V1 (ADR-0030) |
| Business outcomes | everything saved is reloaded identically from the working folder |
| Leading indicators | round trip demonstrated for each kind of file |
| Non-functional requirements | lossless round trip of the integer model |
| In scope | file formats, working folder, import and export |
| Out of scope | any server (local or online), project management: pro edition (ADR-0030) |
| Closure criteria | every kind of file survives a save / load round trip |

## Candidate features

| Feature | Status |
| --- | --- |
| File formats | to refine |
| Working folder | to refine |

## Ideas for refinement

- **Import of SVG from other sources** (Product Owner, 2026-10-09): a shape far larger than a screen (over a million units) is accepted without loss of data, with a warning; the tool offers to simplify it and scale it down to a sensible size when it can (straight lines, resizable); beyond a level of complexity it is refused — synoptic views are schematic, not realistic drawings (Q20: one unit is one screen pixel).
- **Local only** (Product Owner, 2026-10-11): no server in the free V1, not even a local one; the local server first planned here went to the pro edition (ADR-0030, Q5).
