---
id: E41
title: Open and save files from the web page
status: draft
---

# E41 — Open and save files from the web page

A list of features grouped by theme, with no closing of its own (ADR-0086). One subject: saving and reloading, the editor a web page. Written at SP-033 (2026-10-25, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)) from E10's note and E11; needs every kind of document it saves, E34 to E40.

| Field | Content |
| --- | --- |
| For | symbol and view designers |
| who | need their work kept and reloaded exactly, in any browser |
| the solution | one file per kind of document, opened from the disk and saved by the browser |
| is a | the editors' files, the editor working as a web page (Q23, settled on 2026-10-25) |
| that | keeps the integer model exact through a round trip |
| unlike | work lost when the page is closed |
| our solution | a file opened from the disk; Save as by a download, which stands in for the working folder; Ctrl + S saving in place where the browser allows it, otherwise opening Save as; a file saved only when the designer asks |
| Business outcomes | everything saved is reloaded identically, in every browser |
| Leading indicators | features accepted |
| Non-functional requirements | lossless round trip of the integer model; no server (ADR-0030) |
| In scope | point 1 alike in every shape, first, before the format is fixed; a format per kind of document; open, Save as and Ctrl + S (`configuration.md` §6, `symbols-and-views.md` §6, `editions.md` §1) |
| Out of scope | the working folder and the desktop application (E46); SVG from other tools (E42); any server, project management (pro edition, ADR-0030) |

## Features

Proposed at SP-033 (2026-10-25) as drafts, point 1 alike first (Product Owner, 2026-10-25): a feature gets its id when it starts, and the epic's refinement may cut it again (ADR-0100, decision 2).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Point 1 alike in every shape | the list of what changes (saved shapes, guided steps, test values); the rectangle from its bottom-right corner; the round shape from its right point (RET-002) (3) | to refine |
| One file per kind | a format per kind of document, versioned; the round trip of every kind; the integer model kept exact (3) | to refine |
| Open and save from the page | a file opened from the disk; Save as by a download; Ctrl + S saving in place where the browser allows it, otherwise opening Save as, never saving on its own (3) | to refine |

Size: 3 features, 9 stories, 20 + 25 × 9 + (45 to 75) × 3 = 380 to 470 active minutes, 6.3 to 7.8 hours (note 0034, §1), before the open point below.

## Taken from the old epics

- E10.4: point 1 alike in every shape (RET-002), its first feature (Product Owner, 2026-10-25).
- E11.1, E11.4, E11.5: the file formats, local only (ADR-0030), Q30 settled for the files.
- E11.2 (its saving part): files saved one by one and reloaded, now from the web page; the working folder itself went to E46 (Product Owner, 2026-10-25).

## Open points

- Saving in place: which browsers let a page write back to the file it opened (the File System Access API, to source at the refinement), and what Ctrl + S does elsewhere.
