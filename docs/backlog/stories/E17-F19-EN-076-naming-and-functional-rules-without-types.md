---
id: EN-076
epic: E17
feature: F19
title: naming-convention and the functional rules without types
status: ready
points: 5
---

# E17 · F19 · EN-076 — naming-convention and the functional rules without types

Rewrite in the project's Rust linter, without types and in their strict form, `@typescript-eslint/naming-convention` and the five type-aware rules of `eslint-plugin-functional`: `readonly` or `Readonly<…>` written in place, nothing trusted from a type defined elsewhere (Product Owner, 2026-10-17, second round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "The 8 rules that need types" and "The Product Owner's second round"; `REF-TSESLINT-NAMING-CONVENTION`, `REF-TSGOLINT-NAMING-PR`, `REF-FUNCTIONAL-PLUGIN`.

## Acceptance criteria

- Given `naming-convention` with the project's options, when it runs in the Rust linter, then its `types` option is read from the written annotation or the initializer (a global `const` holding a function, the boolean prefixes), and it reports the cases of its tests (F19.AC2).
- Given `immutable-data`, when it runs, then any assignment to a member, any `delete` and the mutating array methods called by name are reported, outside the scopes that relax the rule (F19.AC2).
- Given `prefer-immutable-types` and `type-declaration-immutability`, when they run, then a parameter or a declared type passes only when `readonly` or `Readonly<…>` is written in place, deep in the written type, and a type defined elsewhere is refused, a test showing it (F19.AC2).
- Given `no-mixed-types` and `no-conditional-statements`, when they run, then the first reads the members' written annotations and the second accepts a branch only when it returns or throws, read from the tree (F19.AC2).
- Given the tree, when the six rules run, then each finding is fixed in the code or the rule's form is brought to the Product Owner with its count, and ESLint's six rules are off (F19.AC2).

## Tasks

One task = one commit, referenced as `EN-076.Tn`.

- [ ] T1 — `naming-convention` without types, its tests (3 h)
- [ ] T2 — `immutable-data` and `no-mixed-types` without types, their tests (3 h)
- [ ] T3 — `prefer-immutable-types` and `type-declaration-immutability` in their strict form, a type defined elsewhere refused (3 h)
- [ ] T4 — `no-conditional-statements` without types; the tree's findings fixed; the rules off in ESLint (3 h)
