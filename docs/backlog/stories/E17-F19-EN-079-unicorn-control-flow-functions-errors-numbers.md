---
id: EN-079
epic: E17
feature: F19
title: Unicorn's control-flow, functions, errors and numbers rules in Rust
status: ready
points: 8
---

# E17 · F19 · EN-079 — Unicorn's control-flow, functions, errors and numbers rules in Rust

Rewrite in the project's Rust linter the 36 kept rules of `eslint-plugin-unicorn` on control flow, functions, errors and numbers, which oxlint runs only as JS plugins (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 36 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-079.Tn`.

- [ ] T1 — Control flow, first half (9): `no-declarations-before-early-exit`, `no-duplicate-if-branches`, `no-duplicate-logical-operands`, `no-unnecessary-nested-ternary`, `no-useless-boolean-cast`, `no-useless-else`, `no-useless-logical-operand`, `prefer-boolean-return`, `prefer-combined-guards`, each with its documented cases as tests (3 h)
- [ ] T2 — Control flow, second half (8): `prefer-early-return`, `prefer-else-if`, `prefer-hoisting-branch-code`, `prefer-minimal-ternary`, `prefer-simple-condition-first`, `prefer-simplified-conditions`, `prefer-smaller-scope`, `prefer-switch`, each with its documented cases as tests (3 h)
- [ ] T3 — Functions (9): `consistent-function-style`, `isolated-functions`, `no-repeated-call-wrappers`, `no-uncalled-method`, `no-unnecessary-default-arguments`, `no-unnecessary-parameters`, `no-useless-recursion`, `prefer-block-statement-over-iife`, `prefer-short-arrow-method`, each with its documented cases as tests (3 h)
- [ ] T4 — Errors and numbers (10): `no-error-property-assignment`, `prefer-aggregate-error`, `prefer-error-is-error`, `try-complexity`, `no-negated-comparison`, `no-redundant-comparison`, `no-subtraction-comparison`, `prefer-math-abs`, `prefer-number-is-safe-integer`, `prefer-unary-minus`, each with its documented cases as tests (3 h)
- [ ] T5 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)

## Timings (ADR-0069)

Measured on this Mac, 2026-10-17, on `npm run lint`'s targets (2,048 files):

| Tool | Before | After |
| --- | --- | --- |
| Rust linter (`tools/lint`) | 0.12–0.13 s | 0.12–0.13 s, 33 rules more |
| ESLint | 20.8–25.5 s | 13.9 s, 36 rules fewer |

## For the main thread

Rule-docs: none affected, the story moves lint rules between linters and changes no rule of `docs/domain/`.

### Journal

- `#run` EN-079: 33 Unicorn rules on control flow, functions, errors and numbers carried to the Rust linter, 3 dropped (`no-unnecessary-nested-ternary`, `consistent-function-style`, `isolated-functions`); same findings as ESLint on the documented and planted cases, none on the tree; ESLint about 7 s faster.
