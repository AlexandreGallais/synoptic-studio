---
id: EN-077
epic: E17
feature: F19
title: The Markdown rules in the Rust linter
status: ready
points: 5
---

# E17 · F19 · EN-077 — The Markdown rules in the Rust linter

Run the 21 Markdown rules of `@eslint/markdown` in the project's Rust linter, since oxlint reads no Markdown. Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "The ESLint rules on oxlint" and "The project's linter in Rust" (a Markdown parser crate chosen in this story); `REF-OXLINT`. The parser crate is read and recorded as a `REF-*` source before it is used.

## Acceptance criteria

- Given a Markdown parser crate chosen in this story, its documentation read and recorded in `docs/references.md`, its row in `docs/tooling/dependencies.md` (at least 7 days old), when the Rust linter reads a page, then it finds the headings, links, code blocks, tables and front matter the rules read (F19.AC4).
- Given the 21 Markdown rules, when they run in the Rust linter on every Markdown target, then each reports the cases of its documentation, and on the tree and on a copy with one planted case per rule the findings are those ESLint's Markdown plugin gave on the same pages (F19.AC4).
- Given ESLint, when the story ends, then `@eslint/markdown`'s rules are off in it (F19.AC4).

## Tasks

One task = one commit, referenced as `EN-077.Tn`.

- [ ] T1 — The parser crate chosen and read, pages walked and parsed (2 h)
- [ ] T2 — The first 11 Markdown rules, each with its documented cases (3 h)
- [ ] T3 — The last 10 Markdown rules, each with its documented cases (3 h)
- [ ] T4 — The findings compared with ESLint on the tree and on planted cases, the rules off in ESLint (2 h)
