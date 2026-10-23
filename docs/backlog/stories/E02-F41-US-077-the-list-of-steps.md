---
id: US-077
epic: E02
feature: F41
title: The list of the history's steps
status: ready
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

`npm run dev`, `http://localhost:5173/`. A **step** is one finished change of the drawing, which Undo takes back.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page and click **Steps** in the toolbar, after Undo and Redo | a list under it holding `Start` alone, marked as the present |
| 2 | Choose the Rectangle tool and click the empty drawing; choose the Selector; click the first rectangle and press Right once; click the new rectangle and press Delete; click the first rectangle, type 150 in W and press Enter | the list reads `Start`, `Add`, `Move`, `Delete`, `Width`, `Width` marked |
| 3 | Click `Move` in the list | the rectangle 120 wide again, the new one back; `Move` marked, `Delete` and `Width` greyed and in italics; Redo enabled |
| 4 | Click `Width` in the list | the second rectangle gone again and the first 150 wide: `Width` marked |
| 5 | Click `Move` again, then press Right once | `Delete` and `Width` gone from the list, which ends with `Move`, `Move`, the last marked; Redo greyed out |
| 6 | Press Right 20 more times, releasing it each time | the list holds 15 steps, the last marked, under `9 earlier steps`: 24 states, Start and 23 steps, the 15 last shown |
| 7 | Click Reset | the list holds `Start` alone |

## Tasks

- [ ] T1 — Each step named by its action: interface and failing tests (1 h)
- [ ] T2 — The list in the playground, a click going back to a step, the tests of T1 passing; the Product Owner test card (2 h)
- [ ] T3 — F41's guide page, left to this story by EN-147, and the list's timings with a long history (1 h)

One task = one commit, referenced as `US-077.Tn` in the commit footer.

## The measures

Measured on Node 24, the median of 7 runs, a history built as the page records it from an empty one, a rectangle moved n times, each step a new rectangle, bounded by the default 100 MB, 5 steps kept:

| n | The list (`historySteps`, 15 rows) | Going back 14 steps (`sizedHistoryAt`) | The list after going back |
| --- | --- | --- | --- |
| 20 000 | 0.08 ms, the first 0.46 ms | 0.46 ms | 0.06 ms |
| 80 000 | 0.06 ms, the first 0.13 ms | 1.07 ms | 0.06 ms |

A drawing of 1 000 rectangles, 200 steps each moving one: the list 0.22 ms, the first 0.39 ms, each row comparing 1 000 shapes by reference and the shape moved by its fields. The list costs its 15 rows whatever the history holds; going back costs its undos, each copying the past's list. The page writes the list again only when the history changes.
