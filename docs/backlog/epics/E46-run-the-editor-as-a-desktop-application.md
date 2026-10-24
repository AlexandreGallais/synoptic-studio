---
id: E46
title: Run the editor as a desktop application
status: draft
---

# E46 — Run the editor as a desktop application

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: the editor hosted by a desktop program, and its working folder. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E11 and Q23; needs E41 and E45; before the release's epics (Product Owner, 2026-10-25).

| Field | Content |
| --- | --- |
| For | symbol and view designers |
| who | work on many files of one project on their computer |
| the solution | a desktop application hosting the editor's page, likely a Rust program with a web view (Q23, settled on 2026-10-25) |
| is a | the editor's shell |
| that | keeps every computation in the page, the native code only for the files (`editions.md` §1) |
| unlike | files opened and saved one at a time by the browser's dialogs (E41) |
| our solution | a working folder opened once, its files saved one by one and reloaded |
| Business outcomes | the editor installed and run as a desktop application, a working folder opened in it |
| Leading indicators | features accepted |
| Non-functional requirements | no server (ADR-0030); the editor's page unchanged inside the shell |
| In scope | the desktop shell, the working folder (`configuration.md` §6, `symbols-and-views.md` §6) |
| Out of scope | the editor as a web page (E41); project management and servers (pro edition, ADR-0030) |

## Features

Proposed at SP-033 (2026-10-25) as drafts: a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| The desktop shell | the editor's page hosted by a desktop program, its build; the files read and written through it (2) | to refine |
| The working folder | a folder opened; files saved one by one; files reloaded identically (3) | to refine |

Size: 2 features, 5 stories, 20 + 25 × 5 + (45 to 75) × 2 = 235 to 295 active minutes, 3.9 to 4.9 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E11.2 (its folder part): the working folder (Product Owner, 2026-10-25).
- E11.6: Q23, the application shell.

## Open points

- How the shell is made: a Rust program hosting a web view is likely, the choice researched at this epic's refinement, with its ADR and its row in `docs/tooling/dependencies.md` for any new tool of the build.
