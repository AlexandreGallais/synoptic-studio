---
id: EN-081
epic: E17
feature: F19
title: Unicorn's modules, platform and promises rules in Rust
status: ready
points: 5
---

# E17 · F19 · EN-081 — Unicorn's modules, platform and promises rules in Rust

Rewrite in the project's Rust linter the 29 kept rules of `eslint-plugin-unicorn` on modules, the platform and promises, which oxlint runs only as JS plugins (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 29 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-081.Tn`.

- [ ] T1 — Modules (8): `default-export-style`, `no-global-object-property-assignment`, `no-top-level-side-effects`, `no-unnecessary-global-this`, `no-useless-re-export`, `prefer-global-number-constants`, `prefer-identifier-import-export-specifiers`, `prefer-json-import`, each with its documented cases as tests (3 h)
- [ ] T2 — Platform (11): `consistent-json-file-read`, `no-invalid-intl-options`, `no-invalid-temporal-arithmetic`, `no-unsafe-buffer-conversion`, `no-unsafe-json-serialization`, `no-using-resource-escape`, `prefer-dispose`, `prefer-temporal`, `prefer-temporal-conversion`, `prefer-uint8array-base64`, `prefer-uint8array-hex`, each with its documented cases as tests (3 h)
- [ ] T3 — Promises (10): `no-async-iterator-callback`, `no-async-promise-finally`, `no-multiple-promise-resolver-calls`, `prefer-array-from-async`, `prefer-await`, `prefer-promise-static-methods`, `prefer-promise-try`, `prefer-promise-with-resolvers`, `prefer-queue-microtask`, `prefer-then-catch`, each with its documented cases as tests (3 h)
- [ ] T4 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)
