---
id: US-077
epic: E02
feature: F41
title: The list of the history's steps
status: draft
points: 3
---

# E02 · F41 · US-077 — The list of the history's steps

As a symbol designer, I want to see the last steps of the history and go back to one by a click, so that I return to an earlier state without pressing Ctrl + Z many times.

Asked by the Product Owner on 2026-10-23 (SP-029's answer 8), to see its use; cut from F40 so that F40 keeps four stories. Inkscape's Undo History, Penpot's history panel and GIMP's dialog do the same (note 0030).

## Acceptance criteria

- Given A nudged, B deleted and W typed 150, when the list is shown, then it names the steps in order, each by its action (Move, Delete, Width), the present one marked.
- Given the list, when Move is clicked, then the drawing goes back to the state after Move, the steps after it kept as steps to redo.
- Given a step taken after going back, then the steps that were to redo are dropped, as an undo followed by a change drops them (ADR-0095).

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — Each step named by its action: interface and failing tests (1 h)
- [ ] T2 — The list in the playground, a click going back to a step, the tests of T1 passing; the Product Owner test card (2 h)

One task = one commit, referenced as `US-077.Tn` in the commit footer.
