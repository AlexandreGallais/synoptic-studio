---
id: EN-075
epic: E17
feature: F19
title: The project's Rust linter and its own rules
status: done
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

## The linter (T1)

`tools/lint`, a crate of the workspace on Oxc's crates at 0.153.0 (`oxc_ast_visit` added beside the four the story names, already built as a dependency of `oxc_semantic`): `cargo run --release -q -p lint -- [--fix] <targets>`. Each file is read and parsed once with its comments (its tokens kept only when a rule counts lines), its semantic model built once, then every rule its scopes turn on runs over that one AST: a rule reads the whole file (`check_file`) or each node of the linter's single walk (`check_node`). Files are spread over the machine's threads; folder listings, the bibliography and the ADRs are read once per run and shared. A rule is one module of `tools/lint/src/rules/` with its tests, registered in `rules::ALL`; adding one costs the module, a line in `ALL` and its name in a scope. The scopes of `eslint/scopes/` that set its rules are carried in order in `tools/lint/src/scopes.rs`, the project's values in `tools/lint/src/settings.rs`. One-off exceptions: `// lint-disable-next-line rule -- reason`; a directive that silences nothing or gives no reason is reported. The two autofixed rules keep their fix (`--fix`, in `fix`, `lint:fix` and `lint-staged`).

## Each rule checked again (Product Owner, 2026-10-17)

Before carrying a rule, what it guards was checked against the code and the other rules: none is dropped.

| Rule | Still guards |
| --- | --- |
| the 10 `local/*` rules | the project's structure (kinds, layers, sources, one export, barrels, import paths, folders), which no other tool checks |
| `no-restricted-syntax` | `enum`, `for…in`, getters and setters, `export * as`, `import()` and the library's default and optional parameters: TypeScript accepts each, and no rule kept refuses them |
| `require-atomic-updates` | a property written after an `await` in the shell and the tests, where `functional/immutable-data` is off; variables cannot be reassigned (`functional/no-let`), so its value is in the properties |
| `functional/no-return-void` | a function of the functional core returning nothing; `explicit-function-return-type` asks for the written type, not that it is a value |

No new rule was added on the way.

## Differences from ESLint

- `local/one-export-per-file` asks for the kebab-case file name (`rename the file \`wrong-name.ts\``), where ESLint's message repeated the camelCase name; `local/kind-naming`names`tools/lint/src/settings.rs` for the verbs.
- `functional/no-return-void` asked the compiler for a function's inferred type; Oxc has no type checker, so the body is read: a block returning no value, an arrow of `undefined`, `void …` or `null`. A call returning `void` as an arrow's expression is not followed (the functional core writes its return types, `typescript/explicit-function-return-type`).
- `require-atomic-updates` walks each function in order instead of ESLint's code paths: a branch's reads are joined after it, the code after `return` or `throw` is left out, a loop's body is read once (ESLint's loop edges are not followed either).
- The spread in a fold (EN-061) is oxlint's `oxc/no-accumulating-spread`, which follows the accumulator's name: the case ESLint's selector missed (a spread held in a variable, then returned) is now reported; `"plugins": ["oxc"]` in `.oxlintrc.json` turns its plugin on.

## The findings compared (T5)

On the tree, before the rules were turned off in ESLint: the Rust linter reported nothing on the 1,535 files but the directive of `tests/barrels.test.ts`, written again as `lint-disable-next-line`. On probe files breaking every rule (a barrel out of sync, a non-canonical import, a folder without barrel, a file beside a folder, each kind rule, an unknown `@see`, every selector, the three `no-return-void` forms, both `require-atomic-updates` messages), ESLint and the Rust linter gave the same 27 findings at the same lines and columns, the two messages above aside; the probes were then removed.

## Measures (ADR-0069)

On the Product Owner's machine, other agents running beside it:

| Run | Time | CPU | Load average |
| --- | --- | --- | --- |
| before: `npm run lint` (oxlint, then ESLint with these rules) | 31.8 s, then 27.5 s | 42.3 s, then 38.6 s | 10 to 20, then 4 |
| after: `npm run lint` (oxlint, the Rust linter, then ESLint without them) | 25.9 s, then 24.5 s | 35.6 s, then 34.6 s | 10 to 17, then 4 |
| the Rust linter alone, every rule, 1,535 files | 0.05 to 0.06 s | 0.11 s | 4 |

## Tests and mutations

92 tests in the crate: each case of the 10 `RuleTester` suites (with the project's limits; the measures of `kind-limits`' own cases checked one by one), the documented cases of `require-atomic-updates` and `no-return-void`, the selectors' cases. 60 mutations tried by hand (one change, the crate's tests run, the file restored): all caught but two equivalent ones, a shortcut that skips ignored folders before reading them, and an arrow's `async` flag, which no code without `await` can observe. The TypeScript tests changed were each made to fail once: the linter's targets in the three scripts (`tests/check-targets.test.ts`), the `lint-disable` directive read by the register (`tests/exceptions.test.ts`), the fold's spread on oxlint and its plugin (`tests/oxlint-config.test.ts`), the signal read from the page (`tests/folders.test.ts`).

## For the main thread

- F19's feature file: criterion 3 names `oxc/no-accumulating-spread` for the fold's spread (EN-061) and the crates with `oxc_ast_visit`.
- ADR-0011 and ADR-0043 name `eslint/settings/kinds.ts` and `eslint/settings/folders.ts` for the kinds' limits and the folders past the signal: the values now live in `tools/lint/src/settings.rs`; left untouched (an accepted ADR is not rewritten), for EN-085's ADR to state.
- Kept twice until ESLint leaves (EN-085): the layers' paths (`eslint/settings/layers.ts` for `import-x/no-restricted-paths`, `tools/lint/src/settings.rs` for the kinds) and the barrels' globs (`eslint/settings/files.ts` for ESLint's scope, `tools/lint/src/scopes.rs`).
- `@typescript-eslint/rule-tester` is removed from `package.json` and the lock (`npm install --package-lock-only`); the shared `node_modules` keeps it until the next `npm install`.
- Rust coverage after EN-075: 95.9 % of the workspace's lines and 95.8 % of its functions (`tools/lint`: every module at 94 % or more, `settings.rs` aside, its constants); the floor of 93 % (`rust:coverage`, `docs/tooling/agent-tools.md`) may be raised once EN-090 is merged.
- `Rule-docs: none affected, a tooling change: no rule of docs/domain/ changes`.

### Journal

- `#run` EN-075: the project's linter in Rust (`tools/lint`, Oxc 0.153.0) runs after oxlint, each file parsed once and every rule over one AST, 0.05 s for the tree; the 10 project rules, `no-restricted-syntax`, `require-atomic-updates` and `functional/no-return-void` moved out of ESLint, the fold's spread to `oxc/no-accumulating-spread`; `npm run lint` from 27.5 s to 24.5 s.
