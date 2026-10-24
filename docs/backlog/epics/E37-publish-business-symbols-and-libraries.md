---
id: E37
title: Publish business symbols and libraries
status: draft
---

# E37 — Publish business symbols and libraries

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: a symbol's meaning per project. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E07; needs E36.

| Field | Content |
| --- | --- |
| For | business configurators |
| who | need one symbol to mean a given equipment in a given project |
| the solution | business symbols, business libraries and text styles |
| is a | a project's library |
| that | gives a symbol its meaning for a project |
| unlike | a symbol copied per project |
| our solution | a symbol, its selected configuration and its business name; libraries sharing symbols between projects; named text styles as local templates |
| Business outcomes | a project library offers business symbols such as "gear pump" |
| Leading indicators | features accepted |
| Non-functional requirements | none beyond the files' |
| In scope | business symbols, business libraries (`configuration.md` §3, §4), text styles as local templates (`shapes.md` Text, Not built) |
| Out of scope | the files' format (E41); online publication and enforced text styles (pro edition, ADR-0030) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Business symbols | a symbol, its selected configuration and its business name; one symbol in two business symbols (2) | to refine |
| Business libraries | a project's library; two projects sharing symbols through two libraries (2) | to refine |
| Text styles | named text styles as local templates that forbid nothing; applied to a text (2) | to refine |

Size: 3 features, 6 stories, 20 + 25 × 6 + (45 to 75) × 3 = 305 to 395 active minutes, 5.1 to 6.6 hours (note 0034, §1).

## Taken from the old epics

- E07.1 to E07.3: business symbols, business libraries, text styles.

## Open points

None known beyond the refinement's own questions.
