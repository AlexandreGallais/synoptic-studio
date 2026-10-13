---
id: EN-022
epic: E14
feature: F11
title: Folders of the library and the playground
status: ready
points: 3
---

# E14 · F11 · EN-022 — Folders of the library and the playground

Apply the layout chosen at SP-010: files moved with `git mv`, barrels and lint settings updated, no behavior changed.

## Acceptance criteria

- Given the chosen layout, when the story ends, then every module sits in its folder, every folder has the barrel the rules ask, and no folder holds more files than the limit chosen.
- Given the move, when `npm run check:all` runs, then it is green with the same tests and values, and `git log --follow` still finds each file's history.

## Tasks

One task = one commit, referenced as `EN-022.Tn`.

- [ ] T1 — Move `src/model/` into subfolders (2 h)
- [ ] T2 — Move `playground/` into subfolders (2 h)
- [ ] T3 — Conventions page and lint settings (1 h)
