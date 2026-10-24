---
id: E07
title: Publish business symbol libraries
status: dropped
---

# E07 — Publish business symbol libraries

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E37](./E37-publish-business-symbols-and-libraries.md), whole.

| Field | Content |
| --- | --- |
| For | business configurators |
| who | need per-project business objects |
| the solution | business symbols and libraries |
| is a | symbol + selected configuration + business name |
| that | same symbols, different business meanings per project |
| unlike | copying symbols per project |
| our solution | a library gives the symbol its meaning for a project |
| Business outcomes | a project library offers business symbols such as "gear pump" |
| Leading indicators | libraries built for two projects sharing symbols |
| Non-functional requirements | — |
| In scope | business symbols, business libraries |
| Out of scope | storage format details (E11); online publication (pro edition, ADR-0030) |
| Closure criteria | two projects use the same symbol through different libraries |

## Candidate features

| Feature | Status |
| --- | --- |
| Business symbol | to refine |
| Business library | to refine |
