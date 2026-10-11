---
id: SP-004
epic: E01
feature: F03
title: Research for F03
status: ready
points: 1
---

# E01 · F03 · SP-004 — Research for F03

Find, read and record the sources of every story of F03 before its implementation (ADR-0020, research protocol §8).

## Acceptance criteria

- Given each story of F03, when the spike ends, then every behavior it needs has a verified `REF-*` or a `DERIV-*`.
- Given the sources found, when the spike ends, then the stories' criteria and tasks cite them, and a research note records what was found and what remains open.

## Questions

- Clamping with a different radius at each corner: whether `DERIV-local-radius-clamp` already covers it (each edge shares its length between its two corners in proportion to their setbacks), and hand-computed cases.
- The model of a radius per vertex: one array of requested radii, its length tied to the number of vertices; whether this is a structural decision (ADR).
- A mixed value in a properties panel (Figma "Mixed"), as a functional reference.
- Selecting several items in a list (range with Shift, toggle with Ctrl / Cmd) and on a drawing: WAI-ARIA listbox pattern, design tools.
- Node handles and the start and direction of a path in design tools (Inkscape node tool), as a functional reference.

## Inventory

| Story   | Needs                                                       |
| ------- | ----------------------------------------------------------- |
| EN-010  | clamping with unequal radii; model of the radii             |
| US-011  | mixed value; radii kept by vertex number                    |
| US-012  | node handles, first and second vertex marked                |
| US-013  | multiple selection in a list and on a drawing               |
| AUD-004 | the sources above, re-verified                              |
| VAL-004 | the check table of the clamp derivation, unequal radii rows |

## Tasks

One task = one commit, referenced as `SP-004.Tn`.

- [ ] T1 — Find, read and record the sources; update the derivation if needed (2 h)
- [ ] T2 — Write the research note and update the stories (1 h)
