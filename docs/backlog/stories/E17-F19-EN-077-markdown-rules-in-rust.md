---
id: EN-077
epic: E17
feature: F19
title: The Markdown rules in the Rust linter
status: done
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

## For the main thread

- Parser: the crate `markdown` 1.0.0 (`markdown-rs`), the Rust sibling of the parser `@eslint/markdown` reads pages with, so the same mdast tree; one dependency, `unicode-id` 0.3.7; `cargo deny` passes.
- Rules: 20 of the 21 moved with their names, options and messages; `markdown/no-reference-like-urls` dropped (pages link inline, definitions on one copied page, such a link is a dead link the site's build refuses); `local/image-alt-text` added (writing.md: under 155 characters, never "Image of"). oxfmt enforces none of the 21 (checked on planted cases), so none was dropped as the formatter's.
- Compared: on the tree, ESLint and the Rust linter both report nothing on the 518 pages; on a copy of `docs/` with one planted case per rule, the same 25 findings, plus the added rule's one. Every test case of the 20 rules gives ESLint's lines, columns and messages.
- ESLint's Markdown plugin crashes on some pages (`Custom getLoc() method must be implemented`, `@eslint/plugin-kit` 0.7.3), for example on `(ESLint)[https://eslint.org/]`, and `--fix-dry-run` crashes on a page with several bare URLs: the Rust linter reports them.
- Rule-docs: none affected, the story changes tooling only, no rule of `docs/domain/`.
- `docs/backlog/features/E17-F21-agent-tools-in-rust.md` (line 50, a shared file) still lists `@eslint/markdown` among the packages to remove: it is gone since EN-077.

### Journal

- `#run` EN-077: the Markdown rules in the project's linter, parsed once by `markdown-rs` (mdast, as `@eslint/markdown`); 20 rules moved, `no-reference-like-urls` dropped, `local/image-alt-text` added; `@eslint/markdown` removed from ESLint and the dependencies; the 518 pages in 0.1 s against 6.7 s in ESLint, `npm run lint` from 25.2–28.2 s to 22.7–25.3 s.

### Improvements

| 2026-10-17 | EN-077 | ESLint's Markdown plugin crashed on some pages (`getLoc` of `@eslint/plugin-kit` 0.7.3), a reversed link around a URL or a `--fix` of several bare URLs; the Rust linter has no such crash, and a page that crashed ESLint never reached the tree | tool |
