---
id: E06
title: Configure business types in cascade
status: dropped
---

# E06 — Configure business types in cascade

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E36](./E36-configure-business-types.md), whole.

| Field | Content |
| --- | --- |
| For | business configurators |
| who | need to give symbols a business meaning without redrawing them |
| the solution | the Configurator |
| is a | configuration trees of interfaces, property groups, defaults and rules |
| that | few configurations reused everywhere through the cascade |
| unlike | per-symbol hand settings |
| our solution | a deep node narrows the options; trees can be combined |
| Business outcomes | a configuration tree (Pump → Positive displacement → Gear) with interfaces, grouped properties, defaults and rules drives a symbol's animations |
| Leading indicators | Q14 settled; reference trees built |
| Non-functional requirements | rules evaluated deterministically |
| In scope | configuration trees, interfaces, properties, property groups, rules, defaults, hidden-by-default lines |
| Out of scope | geometry editing |
| Closure criteria | a gear pump configuration animates a pump symbol as specified |

## Candidate features

| Feature | Status |
| --- | --- |
| Configuration tree | to refine |
| Interfaces and rules | to refine |
| Property groups and defaults | to refine |
