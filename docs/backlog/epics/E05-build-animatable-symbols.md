---
id: E05
title: Build animatable symbols
status: dropped
---

# E05 — Build animatable symbols

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E34](./E34-build-symbols-with-ports.md), the symbol, its ports and zones, and [E35](./E35-animate-parts-of-symbols.md), the animatable parts and their preview.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need symbols whose parts can react to a state |
| the solution | symbols in the Symbol Editor |
| is a | a group of shapes with ports and animatable parts |
| that | one symbol drawn once, animated by any configuration |
| unlike | one drawing per state |
| our solution | parts are animated, not redrawn |
| Business outcomes | a symbol with ports and animatable parts (color, blinking, opacity, visibility, partial fill, rotation) previews each animation |
| Leading indicators | reference symbols (pump, valve, tank) built |
| Non-functional requirements | animations smooth enough for a synoptic view |
| In scope | ports, animatable parts, animation preview (the shape tree comes from E13) |
| Out of scope | business meaning (E06) |
| Closure criteria | a pump symbol with two ports and animated parts works end to end |

## Candidate features

| Feature | Status |
| --- | --- |
| Ports | to refine |
| Animatable parts | to refine |
| Animation preview | to refine |
