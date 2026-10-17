---
id: US-043
epic: E15
feature: F17
title: The balanced wrap removed, lines always wrapped automatically
status: done
points: 2
---

# E15 · F17 · US-043 — The balanced wrap removed, lines always wrapped automatically

As a symbol designer, I want the lines of a box to wrap one way only so that a text has one fewer option to understand.

From the Product Owner's validation of E15 (2026-10-15): the balanced wrap is removed; fewer options make a clearer product (VAL-015, decisions 13 to 15). Changes F14's criterion 3.

## Acceptance criteria

- Given "Pump P-101 is running" in Arial 16 in a box 110 wide, when it is drawn, then it shows "Pump P-101 is" over "running", the automatic wrap.
- Given the Text tool, when a box is chosen, then no wrap style is offered; a text read with a wrap style is wrapped automatically.
- Given F14's criterion 3, when the story ends, then it names no balanced wrap; `DERIV-text-box`'s balance steps are marked as a record.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Text "Pump P-101 is running", Arial 16, in a box 110 × 60 | "Pump P-101 is" over "running" |
| 2 | Look for the wrap style | none |

## Tasks

One task = one commit, referenced as `US-043.Tn`.

- [x] T1 — The balanced wrap removed from the library (EN-036's code) with its tests (2 h)
- [x] T2 — The playground's option and guided steps removed; `shapes.md`, `DERIV-text-box` and F14's criterion 3 updated (2 h)
