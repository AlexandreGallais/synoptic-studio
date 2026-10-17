---
id: EN-080
epic: E17
feature: F19
title: Unicorn's objects, strings, naming and miscellaneous rules in Rust
status: ready
points: 8
---

# E17 · F19 · EN-080 — Unicorn's objects, strings, naming and miscellaneous rules in Rust

Rewrite in the project's Rust linter the 37 kept rules of `eslint-plugin-unicorn` on objects, strings, naming and the miscellaneous group, which oxlint runs only as JS plugins (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 37 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-080.Tn`.

- [ ] T1 — Objects (12): `consistent-destructuring`, `no-computed-property-existence-check`, `no-invalid-property-descriptor`, `no-invalid-well-known-symbol-methods`, `no-unreadable-new-expression`, `no-unreadable-object-destructuring`, `no-unsafe-property-key`, `no-unused-builtin-method-return`, `no-unused-properties`, `prefer-object-define-properties`, `prefer-object-destructuring-defaults`, `prefer-single-object-destructuring`, each with its documented cases as tests (3 h)
- [ ] T2 — Strings (14): `no-incorrect-template-string-interpolation`, `no-unnecessary-string-trim`, `no-unsafe-string-replacement`, `prefer-escaped-irregular-whitespace`, `prefer-literal-ascii`, `prefer-regexp-escape`, `prefer-short-escape-sequences`, `prefer-single-replace`, `prefer-split-limit`, `prefer-string-match-all`, `prefer-string-pad-start-end`, `prefer-string-repeat`, `prefer-unicode-code-point-escapes`, `string-content`, each with its documented cases as tests (3 h)
- [ ] T3 — Naming and miscellaneous (11): `comment-content`, `consistent-compound-words`, `consistent-tuple-labels`, `name-replacements`, `no-keyword-prefix`, `prefer-type-literal-last`, `single-line-block-comment-style`, `consistent-optional-chaining`, `no-useless-compound-assignment`, `no-useless-delete-check`, `prefer-https`, each with its documented cases as tests (3 h)
- [ ] T4 — The options of `string-content`, `name-replacements` and `comment-content` carried from `eslint/rules/unicorn/`, each with a test (2 h)
- [ ] T5 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)
