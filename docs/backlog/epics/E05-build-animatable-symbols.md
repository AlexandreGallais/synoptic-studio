---
id: E05
title: Build animatable symbols
status: draft
---

# E05 — Build animatable symbols

| Field                       | Content                                                                                                                         |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| For                         | symbol designers                                                                                                                |
| who                         | need symbols whose parts can react to a state                                                                                   |
| the solution                | symbols in the Symbol Editor                                                                                                    |
| is a                        | a group of shapes with ports and animatable parts                                                                               |
| that                        | one symbol drawn once, animated by any configuration                                                                            |
| unlike                      | one drawing per state                                                                                                           |
| our solution                | parts are animated, not redrawn                                                                                                 |
| Business outcomes           | a symbol with ports and animatable parts (color, blinking, opacity, visibility, partial fill, rotation) previews each animation |
| Leading indicators          | reference symbols (pump, valve, tank) built                                                                                     |
| Non-functional requirements | animations smooth enough for a synoptic view                                                                                    |
| In scope                    | ports, animatable parts, animation preview (the shape tree comes from E13)                                                      |
| Out of scope                | business meaning (E06)                                                                                                          |
| Closure criteria            | a pump symbol with two ports and animated parts works end to end                                                                |

## Candidate features

| Feature           | Status    |
| ----------------- | --------- |
| Ports             | to refine |
| Animatable parts  | to refine |
| Animation preview | to refine |
