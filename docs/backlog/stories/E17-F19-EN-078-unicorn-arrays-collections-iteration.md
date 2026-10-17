---
id: EN-078
epic: E17
feature: F19
title: Unicorn's arrays, collections and iteration rules in Rust
status: ready
points: 8
---

# E17 · F19 · EN-078 — Unicorn's arrays, collections and iteration rules in Rust

Rewrite in the project's Rust linter the 45 kept rules of `eslint-plugin-unicorn` on arrays, collections and iteration, which oxlint runs only as JS plugins (Product Owner, 2026-10-17, fourth round: every rule oxlint lacks rewritten within F19). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks" (the groups of `eslint/rules/unicorn/`); `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 45 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-078.Tn`.

- [ ] T1 — Arrays, first half (7): `no-array-from-fill`, `no-array-front-mutation`, `no-array-splice`, `no-confusing-array-splice`, `no-impossible-length-comparison`, `no-negated-array-predicate`, `no-return-array-push`, each with its documented cases as tests (3 h)
- [ ] T2 — Arrays, second half (7): `no-unnecessary-splice`, `prefer-array-from-range`, `prefer-array-last-methods`, `prefer-array-slice`, `prefer-flat-math-min-max`, `prefer-simple-sort-comparator`, `prefer-single-array-predicate`, each with its documented cases as tests (3 h)
- [ ] T3 — Collections, first half (7): `consistent-conditional-object-spread`, `no-duplicate-set-values`, `no-mismatched-map-key`, `no-object-methods-with-collections`, `no-unnecessary-array-flat-map`, `no-useless-set-construction`, `prefer-array-from-map`, each with its documented cases as tests (3 h)
- [ ] T4 — Collections, second half (6): `prefer-get-or-insert-computed`, `prefer-group-by`, `prefer-has-check`, `prefer-includes-over-repeated-comparisons`, `prefer-map-from-entries`, `prefer-set-methods`, each with its documented cases as tests (3 h)
- [ ] T5 — Iteration, first half (9): `iteration-fallback-style`, `no-array-concat-in-loop`, `no-array-sort-for-min-max`, `no-break-in-nested-loop`, `no-duplicate-loops`, `no-for-each`, `no-loop-iterable-mutation`, `no-unreadable-for-of-expression`, `no-unused-iterator-helper`, each with its documented cases as tests (3 h)
- [ ] T6 — Iteration, second half (9): `no-useless-continue`, `prefer-array-iterable-methods`, `prefer-direct-iteration`, `prefer-iterable-in-constructor`, `prefer-iterator-helpers`, `prefer-iterator-to-array`, `prefer-iterator-to-array-at-end`, `prefer-object-iterable-methods`, `prefer-while-loop-condition`, each with its documented cases as tests (3 h)
- [ ] T7 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)
