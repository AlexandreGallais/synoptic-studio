---
id: EN-022
epic: E14
feature: F11
title: Folders of the library and the playground
status: ready
points: 3
---

# E14 · F11 · EN-022 — Folders of the library and the playground

Apply the layout of ADR-0042 (research note 0012): files moved with `git mv`, barrels and lint settings updated, no behavior changed.

## Acceptance criteria

- Given the layout of ADR-0042, when the story ends, then every module sits in a folder named by its business theme, shape types at the root of their layer, and every folder has the barrel the rules ask.
- Given a folder of more than 15 modules (tests not counted), when the tests run, then they fail unless `docs/conventions/files.md` lists it with the reason no theme splits it.
- Given the move, when `npm run check:all` runs, then it is green with the same tests and values, and `git log --follow` still finds each file's history.

## Tasks

One task = one commit, referenced as `EN-022.Tn`.

- [ ] T1 — Move `src/model/` into subfolders (2 h)
- [ ] T2 — Move `playground/` into subfolders (2 h)
- [ ] T3 — Conventions page, the folder test, lint settings (1 h)
