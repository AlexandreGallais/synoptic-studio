---
id: US-066
epic: E02
feature: F37
title: Delete removes the selected shape
status: ready
points: 2
---

# E02 · F37 · US-066 — Delete removes the selected shape

As a symbol designer, I want to remove the selected shape by a key, so that a mistake is cleared at once.

## Acceptance criteria

- Given three shapes and the second selected, when Delete or Backspace is pressed, then the second is removed, the first and the third unchanged and in their order, and none is selected.
- Given the focus in a field of the panel, when Delete or Backspace is pressed, then the field's text is edited and no shape is removed.
- Given no shape selected, when Delete is pressed, then nothing changes.
- Given the Points mode and vertices selected, when Delete is pressed, then the vertices are deleted as before (`interaction.md` §3), never the shape.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The removal of a shape from the drawing: interface and failing tests (1 h)
- [ ] T2 — The removal and the key in the playground, the tests of T1 passing; the Product Owner test card (1 h)

One task = one commit, referenced as `US-066.Tn` in the commit footer.
