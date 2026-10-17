---
id: EN-049
epic: E17
feature: F18
title: The stories index generated from the story files
status: done
points: 2
---

# E17 · F18 · EN-049 — The stories index generated from the story files

Generate `docs/backlog/stories/README.md` from each story's front matter, so that no branch writes it by hand (ADR-0065, decision 3; F18.AC5).

## Acceptance criteria

- Given the story files, when `npm run backlog:status` (or a `backlog:index` step) runs, then the index is written from their front matter in the order of their features.
- Given an index edited by hand, when `backlog.test.ts` runs, then it fails and says to run the generator.

## Tasks

One task = one commit, referenced as `EN-049.Tn`.

- [x] T1 — The generator and its test; `backlog:status` calls it (2 h)

## For the main thread

- The stories index is now generated: `npm run backlog:index` (or `npm run backlog:status -- <ID> <status>`) writes it; after a merge that conflicts on it, regenerate it instead of merging the rows.
- Its order: epics by id; in each epic the ids its tables link, a feature bringing its stories in its own table's order, then what no table lists, by file name. The first run moved three groups of rows (F09 before F04 in E01, F11's and F12's validations beside their features in E14); no row's content changed.
- `check:rust` joins `check` and the fourth lane of `check:all`; EN-047's crate in `tools/heavy-lock/` can be added to it at merge. The CI runner needs `cargo` with clippy and rustfmt (GitHub's Ubuntu images have them).
