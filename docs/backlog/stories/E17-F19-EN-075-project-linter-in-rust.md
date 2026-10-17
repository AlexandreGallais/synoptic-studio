---
id: EN-075
epic: E17
feature: F19
title: The project's Rust linter and its own rules
status: ready
points: 5
---

# E17 · F19 · EN-075 — The project's Rust linter and its own rules

Build the project's linter in Rust on Oxc's crates, and move into it the 10 project rules, `functional/no-return-void`, the 7 selectors of `no-restricted-syntax` and `require-atomic-updates`, which oxlint cannot run natively (Product Owner, 2026-10-17: the project's rules in Rust, not as JS plugins). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "A rule in Rust on Oxc", "The project's own rules" and "The project's linter in Rust"; `REF-OXC-CRATE`, `REF-OXLINT-JS-PLUGINS-ALPHA` (no way to add a Rust rule to oxlint from outside), `REF-FUNCTIONAL-PLUGIN`.

## Acceptance criteria

- Given a crate of the workspace on `oxc_parser`, `oxc_ast`, `oxc_semantic` and `oxc_allocator` (0.153.0 or later, at least 7 days old, each with its row in `docs/tooling/dependencies.md`), when `npm run lint` runs, then it runs this linter after oxlint, on the same targets, with the same scopes, and prints findings as file, line, column, rule and message (F19.AC1, F19.AC3).
- Given the 10 rules of `eslint/plugin/rules/`, when they run in the Rust linter, then each test of their `RuleTester` suites is carried as a test of the crate and passes, every exception of the rule kept (a type and a value sharing a name, for `one-export-per-file`) (F19.AC3).
- Given `functional/no-return-void`, the 7 selectors of `no-restricted-syntax` (no `enum`, no `for…in`, no getters or setters, no default or optional parameter, the spread in a fold of EN-061 replaced by `oxc/no-accumulating-spread`), and `require-atomic-updates`, when they run in the Rust linter, then each reports the cases of its tests and of its documentation, and nothing on the tree (F19.AC3).
- Given ESLint, when the story ends, then these rules are off in it, and the 10 rules' JavaScript sources stay until F21's clean-up only if something still imports them (F19.AC3).

## Tasks

One task = one commit, referenced as `EN-075.Tn`.

- [ ] T1 — The linter's crate: files walked, parsed, semantic model built, findings printed, the scopes read (3 h)
- [ ] T2 — The 10 project rules, their tests carried (first five) (3 h)
- [ ] T3 — The 10 project rules, their tests carried (last five) (3 h)
- [ ] T4 — `no-return-void`, the selectors of `no-restricted-syntax`, `require-atomic-updates` (3 h)
- [ ] T5 — `npm run lint` calling the linter, the rules off in ESLint, the findings compared on the tree (1 h)
