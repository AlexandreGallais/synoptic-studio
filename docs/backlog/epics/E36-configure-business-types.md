---
id: E36
title: Configure business types
status: draft
---

# E36 — Configure business types

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: trees, interfaces and rules. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E06; needs E35.

| Field | Content |
| --- | --- |
| For | business configurators |
| who | need a symbol's properties and rules set once per kind of equipment |
| the solution | configuration trees in cascade, interfaces and rules |
| is a | the Configurator |
| that | a deep node narrows the options, trees can be combined (Q14) |
| unlike | each symbol configured on its own |
| our solution | properties and interfaces, trees whose nodes bring their ancestors' options, rules driving a symbol's parts |
| Business outcomes | a configuration tree (Pump → Positive displacement → Gear) with interfaces, grouped properties, defaults and rules drives a symbol's animations |
| Leading indicators | features accepted |
| Non-functional requirements | rules evaluated deterministically |
| In scope | configuration trees, interfaces, properties, property groups, rules, defaults, hidden-by-default lines (`configuration.md` §1, §2, §5) |
| Out of scope | geometry editing |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Properties and interfaces | a property's name, type and default; an interface of properties; the Configurator's first screen (3) | to refine |
| Trees and their cascade | a configuration tree; a node bringing its ancestors' options (Q14); trees combined (3) | to refine |
| Rules driving animations | a rule's form; its evaluation, deterministic; a rule driving a part of a symbol (3) | to refine |
| Property groups and defaults | groups of properties; lines hidden by default (2) | to refine |

Size: 4 features, 11 stories, 20 + 25 × 11 + (45 to 75) × 4 = 475 to 595 active minutes, 7.9 to 9.9 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E06.1 to E06.5: the configuration trees, interfaces and rules, property groups and defaults, trees combined, lines hidden by default.

## Open points

- The form of a rule (a form of conditions or an expression language): settled at this epic's refinement from its sources (2026-10-25, SP-033's answer 11; note 0034, §5).
