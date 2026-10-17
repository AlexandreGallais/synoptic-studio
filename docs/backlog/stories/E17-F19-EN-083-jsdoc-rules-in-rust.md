---
id: EN-083
epic: E17
feature: F19
title: JSDoc's rules in Rust
status: ready
points: 5
---

# E17 · F19 · EN-083 — JSDoc's rules in Rust

Rewrite in the project's Rust linter the 31 kept rules of `eslint-plugin-jsdoc`, which oxlint runs only as JS plugins; the 12 rules on JSDoc types are dropped since `jsdoc/no-types` forbids them (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 31 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-083.Tn`.

- [ ] T1 — Content (11): `check-param-names`, `check-syntax`, `check-template-names`, `escape-inline-tags`, `informative-docs`, `match-description`, `no-restricted-syntax`, `normalize-see-links`, `no-types`, `ts-ban-ts-comment`, `require-description`, each with its documented cases as tests (3 h)
- [ ] T2 — Layout (10): `check-alignment`, `check-indentation`, `check-line-alignment`, `lines-before-block`, `multiline-blocks`, `no-bad-blocks`, `no-blank-block-descriptions`, `no-multi-asterisks`, `require-asterisk-prefix`, `require-hyphen-before-param-description`, each with its documented cases as tests (3 h)
- [ ] T3 — Tags and requirements (10): `sort-tags`, `tag-lines`, `require-jsdoc`, `require-next-description`, `require-rejects`, `require-returns-check`, `require-template`, `require-template-description`, `require-throws`, `require-yields-check`, each with its documented cases as tests (3 h)
- [ ] T4 — A TSDoc block parser shared by the rules, from the comments `oxc_ast` keeps (2 h)
- [ ] T5 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)
