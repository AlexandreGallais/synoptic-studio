---
id: SP-010
epic: E14
feature: F11
title: "Research for F11: folder layouts and test placement"
status: ready
points: 2
---

# E14 · F11 · SP-010 — Research for F11: folder layouts and test placement

Find, read and record what is common practice for laying out a TypeScript library and its tests, before EN-022 moves any file (ADR-0020, research protocol §8).

## Acceptance criteria

- Given the question of test placement, when the spike ends, then the practices found (tests next to their module, or in a separate tree) are compared with their sources, against this project's rules: one export per file (ADR-0019), barrels (ADR-0012), the 100 % coverage of `src/`.
- Given `src/model/` (50 modules) and `playground/` (80 modules), when the spike ends, then a layout by subfolder is proposed, with the limit of files per folder and the ESLint rules it touches (layers, barrels, canonical paths).
- Given the proposal, when the spike ends, then the Product Owner chooses, and the choice is an ADR.

## Questions

- Where do the tools this project uses expect tests (Vitest's default `include`, TypeDoc's exclusions), and what do their docs advise?
- What do recognized guides advise for a library's folders: by layer, by feature, by shape kind?
- Which layout keeps `import` paths short and the layer rules (`import-x/no-restricted-paths`) simple?

## Tasks

One task = one commit, referenced as `SP-010.Tn`.

- [ ] T1 — Read the sources (library first), record them (2 h)
- [ ] T2 — Layout proposal with a table of moves, research note, questions for the Product Owner (2 h)
- [ ] T3 — The Product Owner's choice as ADR-0042; F11 and EN-022 updated (1 h)
