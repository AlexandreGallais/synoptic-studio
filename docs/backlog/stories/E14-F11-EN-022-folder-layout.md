---
id: EN-022
epic: E14
feature: F11
title: Folders of the library and the playground
status: done
points: 3
---

# E14 · F11 · EN-022 — Folders of the library and the playground

Apply the layout of ADR-0042 (research note 0012): files moved with `git mv`, barrels and lint settings updated, no behavior changed.

## Acceptance criteria

- Given the layout of ADR-0042, when the story ends, then every module sits in a folder named by its business theme, a shape's type in its folder, value imports between folders one way, and every folder has the barrel the rules ask.
- Given a folder of more than 15 modules (tests not counted), when the tests run, then they fail unless `docs/conventions/files.md` lists it with the reason no theme splits it.
- Given the move, when `npm run check:all` runs, then it is green with the same tests and values, and `git log --follow` still finds each file's history.

## Tasks

One task = one commit, referenced as `EN-022.Tn`.

- [x] T1 — Move `src/model/` and `src/geometry/` into subfolders (3 h)
- [x] T2 — Move `playground/` into subfolders (2 h)
- [x] T3 — Conventions page, the folder test, lint settings (1 h)

## Decisions during the story

Where the layout departs from the proposal of [research note 0012](../../research/notes/0012-f11-folders-and-tests.md), each within ADR-0042:

| Departure                                                                              | Reason                                                                                                                                                                                                                                                 |
| -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `src/math/vector/` (the `Vector` type and its arithmetic)                              | math had 17 modules, not under 15 as rule 5 assumed: split by rule 3 along its one clear theme; the root keeps the point, angles, bisection, `EPSILON`                                                                                                 |
| geometry `polygon/` instead of `stretched-polygon/`                                    | `unitRegularPolygon` and the stretched polygon are one construction theme; a folder of the regular polygon alone would hold one module                                                                                                                 |
| `hasLength` in `rounded-corners/`                                                      | it first serves the rounded contour, dropping its empty pieces; `box/` imports it from there, one way                                                                                                                                                  |
| playground in 8 folders, not 4: `toolbar/`, `drawing/`, `page/`, `guided-steps/` added | the 4 planned folders imported each other both ways (shape ↔ points, shape ↔ position, shape ↔ guided test): the readers they share moved to `page/`, the drawing and the toolbar became their own panels, the steps' data left the guided test's code |

`playground/page/` holds what every panel uses to read and write the page (`selectElement`, `readNumber`, `writeText`, the mode, the shape kind, the selection): a theme of the page itself, or a folder by kind of code that ADR-0042 refuses; asked to the Product Owner at the stop.
