---
id: EN-074
epic: E17
feature: F19
title: oxlint with its native and type-aware rules
status: ready
points: 5
---

# E17 · F19 · EN-074 — oxlint with its native and type-aware rules

Run oxlint as the one lint command, with its native rules and `tsgolint`'s 55 type-aware rules, and confirm against the code the triage of the 265 rules it runs only as JS plugins. Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "The ESLint rules on oxlint", "The 8 rules that need types", "The Product Owner's third round" and "Triage of the rules oxlint lacks"; `REF-OXLINT`, `REF-OXLINT-PLUGINS`, `REF-OXLINT-MIGRATE`, `REF-OXLINT-TYPE-AWARE`, `REF-TSGOLINT`, `REF-OXLINT-PREFER-DESTRUCTURING`, `REF-TSESLINT-PREFER-DESTRUCTURING`, `REF-TSESLINT-NO-UNSAFE-ENUM-ASSIGNMENT`, `REF-OXLINT-ACCUMULATING-SPREAD`.

## Acceptance criteria

- Given `.oxlintrc.json` written by `@oxlint/migrate --type-aware` from `eslint/`, with oxlint 1.87.0 or later and `oxlint-tsgolint` 7.0.2004 or later (each at least 7 days old), when `npm run lint` runs on the tree, then oxlint runs the 432 native rules with their options and the 55 type-aware rules, and reports nothing (F19.AC1).
- Given the scopes of `eslint/scopes/`, when they are carried, then each is an `overrides` entry of `.oxlintrc.json`, and a file of each scope gets the rules it got from ESLint (F19.AC1).
- Given the 110 findings oxlint gives that ESLint did not (8 rules, from other defaults), when the story ends, then each rule has an option or a decision written in the story, and the tree reports none (F19.AC1).
- Given `@typescript-eslint/prefer-destructuring`, when it is carried, then the native `eslint/prefer-destructuring` runs with the same options; `no-unsafe-enum-assignment` is dropped, its reason (`enum` is forbidden) written for the ADR (F19.AC2).
- Given the triage of note 0023 (232 kept, 33 dropped), when each rule is checked against `src/`, `playground/`, `tests/` and the root configuration files, then each verdict is confirmed or changed with its reason, and the rewrite stories EN-078 to EN-084 hold the kept rules (F19.AC11).
- Given ESLint, when the story ends, then every rule now run by oxlint is off in ESLint's configuration, so that each rule runs in one linter only, and the ESLint configuration's tests (`eslint/config.test.ts`) that check those rules move to `.oxlintrc.json` (F19.AC1).

## Tasks

One task = one commit, referenced as `EN-074.Tn`.

- [ ] T1 — oxlint and `tsgolint` installed, `.oxlintrc.json` from the migration, their rows in `docs/tooling/dependencies.md` (2 h)
- [ ] T2 — The scopes as `overrides`, each checked on a file of its scope (3 h)
- [ ] T3 — The 110 findings decided rule by rule; `prefer-destructuring` native; `no-unsafe-enum-assignment` dropped (3 h)
- [ ] T4 — The triage of the 265 rules confirmed against the code, note 0023 updated (3 h)
- [ ] T5 — The carried rules off in ESLint, the configuration's tests moved, `npm run lint` running oxlint first (2 h)

## Decisions of the Product Owner (2026-10-17)

Recorded as understood when the story started.

- **One lint command**, oxlint; `tsgolint` only for the rules that truly need types. No rule stays in ESLint by choice: ESLint leaves at EN-085, and each later story turns its rules off in ESLint as they move.
- **A triage before anything is rewritten or enabled**, over every rule enabled today — oxlint's native ones, `tsgolint`'s type-aware ones and the 265 the spike triaged: a rule is dropped when what it guards is already forbidden by a rule kept, by the compiler or by the project's settings (the `enum` rules, the class and `this` rules, `for…in`, CommonJS…), or when this code cannot contain what it checks, each with its reason, in a table here and in note 0023. Fewer rules make a faster lint; a rule that guards nothing here is never rewritten.
- **The new findings**: each rule either gets its code fixed or is dropped with its reason; a rule is never turned off to make code pass.

## Triage of the native and type-aware rules (T1)

From `@oxlint/migrate 1.87.0 --type-aware` run on `eslint/` on 2026-10-17, each claim of overlap tried on a probe file with oxlint or `tsc` and the project's `tsconfig.json`; the method is in note 0023. A dropped rule is left out of `.oxlintrc.json`.

| Group | Enabled | Kept | Dropped |
| --- | --- | --- | --- |
| ESLint core | 163 | 127 | 36 |
| typescript-eslint, type-aware (`tsgolint`) | 53 | 46 | 7 |
| typescript-eslint, without types | 47 | 30 | 17 |
| Unicorn | 127 | 118 | 9 |
| Import | 24 | 17 | 7 |
| JSDoc | 18 | 12 | 6 |
| **All** | **432** | **350** | **82** |

The 82 dropped, by reason ("types": a type-aware rule of `tsgolint`):

| Rules dropped | Reason |
| --- | --- |
| `constructor-super` (ts(2335)), `getter-return` (ts(2378)), `no-class-assign` (ts(2629)), `no-const-assign` (ts(2588)), `no-dupe-class-members` (ts(2393)), `no-dupe-keys` (ts(1117)), `no-func-assign` (ts(2630)), `no-import-assign` (ts(2632)), `no-new-native-nonconstructor` (ts(7009)), `no-obj-calls` (ts(2349)), `no-setter-return` (ts(2408)), `no-this-before-super` (ts(2376)), `no-unsafe-negation` (ts(2322), ts(2365)), `no-with` (ts(1101)), `no-delete-var` (ts(1102)), `no-nonoctal-decimal-escape` (ts(1488)), `import/default` (ts(1192)), `import/namespace` (ts(2339)), `import/no-amd` (ts(2304)), `import/no-webpack-loader-syntax` (ts(2307)) | TypeScript's compiler refuses the code the rule reports, with the error in parentheses (`REF-TSESLINT-ESLINT-RECOMMENDED` for the core rules; the import rules tried) |
| `no-implicit-globals` | it checks scripts only; every file is a module (`moduleDetection: force`) |
| `class-methods-use-this`, `max-classes-per-file`, `no-constructor-return`, `no-empty-static-block`, `no-useless-constructor`, `typescript/class-literal-property-style`, `typescript/explicit-member-accessibility`, `typescript/no-extraneous-class`, `typescript/no-unnecessary-parameter-property-assignment`, `typescript/parameter-properties`, `typescript/prefer-readonly` (types), `typescript/prefer-return-this-type` (types), `unicorn/no-static-only-class`, `unicorn/prefer-class-fields`, `unicorn/custom-error-definition`, `unicorn/no-useless-error-capture-stack-trace` | no class can exist, error subclasses included: `functional/no-classes` forbids them |
| `typescript/no-misused-new`, `typescript/no-unsafe-declaration-merging` | no class can exist (`functional/no-classes`), nor an interface (`typescript/consistent-type-definitions: type`) |
| `jsdoc/implements-on-classes` | no class can exist, and `@implements` is refused by `jsdoc/check-tag-names` (`typed: true`) |
| `typescript/no-this-alias`, `unicorn/no-this-assignment` | `this` cannot appear: `functional/no-this-expressions` forbids it |
| `accessor-pairs`, `grouped-accessor-pairs`, `unicorn/no-accessor-recursion`, `typescript/related-getter-setter-pairs` (types) | getters and setters are forbidden (`no-restricted-syntax`) |
| `typescript/no-duplicate-enum-values`, `typescript/prefer-enum-initializers`, `typescript/prefer-literal-enum-member`, `typescript/no-mixed-enums` (types), `typescript/no-unsafe-enum-comparison` (types) | `enum` is forbidden (`no-restricted-syntax`) |
| `guard-for-in`, `typescript/no-for-in-array` (types) | `for…in` is forbidden (`no-restricted-syntax`) |
| `typescript/prefer-namespace-keyword`, `typescript/no-unnecessary-qualifier` (types) | namespaces are forbidden, declarations included (`typescript/no-namespace`), and enums too: no qualifier to check |
| `prefer-const`, `init-declarations`, `no-unassigned-vars`, `vars-on-top`, `block-scoped-var`, `import/no-mutable-exports` | only `const` can be declared, and a `const` always has its value: `no-var` and `functional/no-let` forbid `var` and `let` |
| `no-loop-func` | it reports a closure over a variable that a loop can change; only `const` can be declared and no parameter is reassigned (`no-param-reassign`) |
| `sort-vars` | `one-var: never` leaves one declarator per declaration: nothing to sort |
| `no-extra-label`, `no-label-var`, `no-unused-labels` | labels are forbidden by `no-labels` |
| `no-eq-null` | `eqeqeq: always` already reports `== null` |
| `typescript/no-confusing-non-null-assertion`, `typescript/no-extra-non-null-assertion`, `typescript/no-non-null-asserted-nullish-coalescing`, `typescript/no-non-null-asserted-optional-chain` | `!` is forbidden by `typescript/no-non-null-assertion` |
| `import/no-commonjs`, `import/no-dynamic-require`, `typescript/no-require-imports` | `unicorn/prefer-module` reports every `require` call, dynamic ones included, and TypeScript refuses `import x = require()` in a module (ts(1202)) |
| `unicorn/no-new-buffer` | `new Buffer()` is deprecated in Node.js's types: `typescript/no-deprecated` reports it |
| `unicorn/no-invalid-fetch-options`, `unicorn/prefer-response-static-json` | the library and the playground are local only (Q5) and call no `fetch`; only two tools of `scripts/` do, and they move to Rust in F21 |
| `jsdoc/check-access`, `jsdoc/check-property-names`, `jsdoc/require-property`, `jsdoc/require-property-description`, `jsdoc/require-property-name` | `@access`, `@typedef` and `@property` are refused by `jsdoc/check-tag-names` (`typed: true`) |

Kept on purpose although close to a drop: `no-unreachable` (the compiler only suggests it, `allowUnreachableCode` left unset), `no-caller` and `prefer-rest-params` (`arguments` is allowed in the tests and the tools, where `functional/functional-parameters` is off), `unbound-method` (a DOM method passed without its object loses it), `no-plusplus`, `operator-assignment` and `logical-assignment-operators` (members can still change in the shell), and the type-aware `no-unnecessary-type-assertion` and `no-unsafe-type-assertion` (they check the assertions the register of exceptions allows).
