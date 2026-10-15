---
id: US-045
epic: E15
feature: F17
title: Fonts embedded only when a file is exported or saved
status: ready
points: 2
---

# E15 · F17 · US-045 — Fonts embedded only when a file is exported or saved

As a symbol designer, I want fonts embedded only in the file I export or save so that the drawing stays light while I edit it.

From the Product Owner's validation of E15 (2026-10-15): embedded fonts are applied only when a file is exported or saved, never during editing (ADR-0060). Changes how F15's criterion 2 is met.

## Acceptance criteria

- Given **Embed fonts** on and texts in Arial 400 and 700, both files held, when the drawing is redrawn on screen, then its SVG holds no `<style>`.
- Given the same, when **Export SVG** is clicked, then `drawing.svg` holds one `<style>` in its `<defs>` with 2 `@font-face` rules, Arial 400 and 700, as ADR-0053 decision 2 writes them.
- Given **Embed fonts** off, when **Export SVG** is clicked, then `drawing.svg` holds no `<style>`.

## Product Owner test

| Step | Do                                                | You should see                                    |
| ---- | ------------------------------------------------- | ------------------------------------------------- |
| 1    | Import Arial, a text in Arial, **Embed fonts** on | the drawing as before, the list of faces left out |
| 2    | **Export SVG**, open the file as text             | one `<style>` with an `@font-face` for Arial      |

## Tasks

One task = one commit, referenced as `US-045.Tn`.

- [ ] T1 — The drawing written without fonts, the export with them, in the library and the playground (2 h)
- [ ] T2 — Tests of both, guided step and `shapes.md` (1 h)
