---
id: EN-073
epic: E17
feature: F19
title: oxfmt instead of Prettier
status: ready
points: 2
---

# E17 · F19 · EN-073 — oxfmt instead of Prettier

Format the repository with oxfmt instead of Prettier, Markdown included, and print Markdown tables without padding while keeping paragraphs as written (ADR-0068, moved from F18). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "oxfmt against Prettier" (3 union types differ, no Markdown file differs, 0.24 s against 8.8 s); `REF-OXFMT`, `REF-OXFMT-MIGRATE`, `REF-OXFMT-LANGUAGES`, `REF-OXFMT-MD-DIVERGENCES`.

## Acceptance criteria

- Given `.oxfmtrc.json` written by `oxfmt --migrate=prettier` from `.prettierrc.json` and `.prettierignore`, `printWidth` set to 100 by hand, when `npm run format:check` runs, then oxfmt checks the same targets as Prettier, Markdown included, and the tree is formatted (F19.AC5).
- Given the 3 union types oxfmt lays out one member per line (`TextCase`, `LeftOutReason`, `MessageIds`), when the story ends, then they are formatted by oxfmt, and no other file changes (F19.AC5).
- Given a Markdown table whose widest cell grows, when oxfmt formats the page, then no other row of the table changes, and a paragraph or a list item stays on the lines it was written on (F19.AC12); if oxfmt pads tables, the story stops with the measure and a proposal.
- Given `npm run fix`, `lint-staged` and `check:all`, when they run, then they call oxfmt and not Prettier; oxfmt (0.72.0 or later, at least 7 days old) has its row in `docs/tooling/dependencies.md` (F19.AC5).

## Tasks

One task = one commit, referenced as `EN-073.Tn`.

- [ ] T1 — `.oxfmtrc.json` from the migration, the scripts and `lint-staged` on oxfmt, the tree formatted (2 h)
- [ ] T2 — Tables without padding and paragraphs kept, measured on the tree, with a test on a table whose cell grows (2 h)
- [ ] T3 — Prettier out of `check:all` and `lint-staged`, its row and ADR-0068's decision 2 marked as carried (1 h)
