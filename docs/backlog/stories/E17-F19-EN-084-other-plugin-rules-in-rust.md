---
id: EN-084
epic: E17
feature: F19
title: The other plugins' rules in Rust
status: ready
points: 3
---

# E17 · F19 · EN-084 — The other plugins' rules in Rust

Rewrite in the project's Rust linter the 20 kept rules of the other plugins that oxlint runs only as JS plugins: `eslint-plugin-functional` (11), `@stylistic` (4), Import (2), `unused-imports` (2) and typescript-eslint without types (1) (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`, `REF-FUNCTIONAL-PLUGIN`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 20 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-084.Tn`.

- [ ] T1 — functional (11): `no-let`, `prefer-property-signatures`, `readonly-type`, `no-classes`, `no-this-expressions`, `functional-parameters`, `no-expression-statements`, `no-loop-statements`, `no-promise-reject`, `no-throw-statements`, `no-try-statements`, each with its documented cases as tests (3 h)
- [ ] T2 — @stylistic, Import, unused-imports, typescript-eslint (9): `line-comment-position`, `multiline-comment-style`, `spaced-comment`, `padding-line-between-statements`, `import-x/no-extraneous-dependencies`, `import-x/no-restricted-paths`, `unused-imports/no-unused-imports`, `unused-imports/no-unused-vars`, `@typescript-eslint/member-ordering`, each with its documented cases as tests (3 h)
- [ ] T3 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)
