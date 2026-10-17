---
id: EN-082
epic: E17
feature: F19
title: Unicorn's DOM and web rules in Rust
status: ready
points: 5
---

# E17 · F19 · EN-082 — Unicorn's DOM and web rules in Rust

Rewrite in the project's Rust linter the 31 kept rules of `eslint-plugin-unicorn` on the DOM and the web, which oxlint runs only as JS plugins; kept on purpose since the playground uses the DOM, files and `URL` (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 31 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-082.Tn`.

- [ ] T1 — DOM, first half (10): `better-dom-traversing`, `dom-node-dataset`, `no-canvas-to-image`, `no-incorrect-query-selector`, `no-invalid-dom-token`, `no-invalid-style-set-property`, `no-late-current-target-access`, `no-late-event-control`, `no-prevent-default-in-passive-listener`, `no-selector-as-dom-name`, each with its documented cases as tests (3 h)
- [ ] T2 — DOM, second half (9): `no-unsafe-dom-html`, `prefer-add-event-listener-options`, `prefer-dom-node-html-methods`, `prefer-dom-node-replace-children`, `prefer-observer-apis`, `prefer-path2d`, `prefer-scoped-selector`, `prefer-toggle-attribute`, `require-passive-events`, each with its documented cases as tests (3 h)
- [ ] T3 — Web (12): `no-blob-to-file`, `no-conflicting-constraints`, `no-ineffective-csp-directives`, `no-invalid-boolean-attribute-value`, `no-invalid-file-input-accept`, `no-invalid-url-protocol-comparison`, `no-url-in-search-params`, `prefer-location-assign`, `prefer-url-can-parse`, `prefer-url-href`, `prefer-url-search-parameters`, `require-css-escape`, each with its documented cases as tests (3 h)
- [ ] T4 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)
