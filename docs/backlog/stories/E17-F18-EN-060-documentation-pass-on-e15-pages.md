---
id: EN-060
epic: E17
feature: F18
title: A documentation pass on E15's long pages
status: done
points: 2
---

# E17 · F18 · EN-060 — A documentation pass on E15's long pages

Shorten the long pages of E15 with the `doc-editor` agent, after checking what ADR-0060 already removed (row 31 of RET-003; I24, I31, I39, I45; F18.AC17).

## Acceptance criteria

- Given `shapes.md` (Box, Text), Q37, Q38, `interaction.md` (Colors), `DERIV-text-box`, `DERIV-color-spaces` and notes 0017–0018, when the pass ends, then no bullet is longer than 400 characters and rules with several fields are tables.
- Given `cssFirstBaselineOffset`, when its TSDoc is read, then it uses one convention for the descender.

## Tasks

One task = one commit, referenced as `EN-060.Tn`.

- [x] T1 — The doc-editor pass and its fixes (3 h)

## For the main thread

- `cssFirstBaselineOffset` was removed with the HTML layer (ADR-0060, US-044); the criterion is applied to the functions that hold its formula now: `firstBaselineOffset` and `halfLeading` state the descender `D` negative below the baseline, as `DERIV-text-box` (notation, step 7) and the code (`ascender - descender`) do; `lineHeight` says "descender" instead of "descent". Code and derivation agreed: no question to ask.
- Form only, no rule added, removed or changed: the Text section of `shapes.md` is cut into `####` groups (its `### Text` anchor kept), Colors of `interaction.md` §5 gets its own `###` heading, long bullets are split into sub-bullets, the weight order of a font face and the keyboard of the picker are tables, the sources of `DERIV-text-box` a table, and the longest paragraphs of both derivations are split. Q37 and Q38 are table rows already and stay as they are; notes 0017 and 0018 had no bullet over 400 characters.
- `tests/long-page-bullets.test.ts` (`[F18.AC17]`) keeps every bullet of the passed parts within 400 characters; mutation-checked (a bullet lengthened in `interaction.md` failed it).
- Footer for the `done` commit: `Rule-docs: none affected, form only, no rule changed`.
