---
id: EN-095
epic: E17
feature: F19
title: The rules ESLint still runs, carried or dropped
status: ready
points: 3
---

# E17 · F19 · EN-095 — The rules ESLint still runs, carried or dropped

EN-084 listed the 30 rules ESLint still runs that no story of F19 covered: the triage of note 0023 counted neither kept nor dropped 22 Unicorn rules, and 8 core and Import rules were left out. ESLint cannot leave (EN-085) while they run in it.

- **Unicorn (22):** `consistent-class-member-order`, `no-accidental-bitwise-operator`, `no-boolean-sort-comparator`, `no-chained-comparison`, `no-collection-bracket-access`, `no-constant-zero-expression`, `no-double-comparison`, `no-for-loop`, `no-invalid-argument-count`, `no-invalid-character-comparison`, `no-misrefactored-assignment`, `no-missing-local-resource`, `no-nonstandard-builtin-properties`, `no-shorthand-property-overrides`, `no-transition-all`, `no-undeclared-class-members`, `no-unsafe-promise-all-settled-values`, `no-useless-coercion`, `no-useless-concat`, `no-useless-template-literals`, `prefer-math-constants`, `require-proxy-trap-boolean-return`.
- **Core and Import (8):** `@typescript-eslint/no-invalid-this`, `import-x/no-unresolved`, `import-x/order`, `no-dupe-args`, `no-octal`, `no-octal-escape`, `no-undef-init`, `sort-imports`.

The decisions of F19's run apply (Product Owner, 2026-10-17): nothing added to ESLint; each rule checked again first — dropped with its reason when it guards nothing useful here or a kept rule, `tsc` or oxlint already does it; moved to oxlint when it runs it natively; rewritten in the project's Rust linter otherwise.

## Acceptance criteria

- Given each of the 30 rules, when the story ends, then it runs in oxlint or the Rust linter with its documented cases as tests and the same findings as ESLint on the tree and on planted cases, or it is in `DROPPED_RULES` and note 0023 with its reason; note 0023's counts include them.
- Given ESLint's configuration, when the story ends, then it runs no rule at all (a test lists what it runs and expects none), so that EN-085 only removes the package.

## Tasks

One task = one commit, referenced as `EN-095.Tn`.

- [ ] T1 — The 22 Unicorn rules (2 h)
- [ ] T2 — The 8 core and Import rules; the test that ESLint runs nothing (1 h)
