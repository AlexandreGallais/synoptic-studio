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

The 8 rules `@oxlint/migrate` skipped as unstable ("nursery") run in oxlint when named, so they are triaged too rather than left in ESLint:

| Rule | Verdict |
| --- | --- |
| `no-unreachable-loop`, `no-restricted-exports` (its five `restrictDefaultExports` options; off in the root configurations, as in ESLint), `typescript/no-unnecessary-condition` (types), `typescript/prefer-optional-chain` (types), `unicorn/no-useless-iterator-to-array` | carried, named in `.oxlintrc.json` with ESLint's options; `no-restricted-exports` also reports `export { default } from`, which `import/no-default-export` lets through |
| `import/export` | dropped: TypeScript refuses a name exported twice (ts(2323), ts(2528)) |
| `import/named` | dropped: TypeScript refuses an import of a name the module does not export (ts(2614)) |
| `no-useless-assignment` | dropped: only `const` can be declared and no parameter is reassigned, so no value can be assigned and overwritten unread |

## The scopes as overrides (T2)

`@oxlint/migrate` wrote one override per scope of `eslint/scopes/` that sets a native rule: the functional core (`no-restricted-globals`), the barrels and the aggregators (`import/max-dependencies`, `unicorn/no-empty-file`), the tooling (`new-cap`, `import/no-nodejs-modules`), the tests (`max-*`, `no-magic-numbers`) and the root configurations (`import/no-default-export`, `no-restricted-exports`); the other scopes set only rules oxlint lacks. One difference found on a probe file: a pattern without a slash (`*.test.ts`, `*.config.ts`, `*.setup.ts`) matches the root only in ESLint but any depth in oxlint, so a nested test got the tooling scope; written `./*.test.ts`, it matches the root only, as in ESLint.

Checked on 2026-10-17 with probe files, one per scope, then removed: `document` reported in `src/math/` and not in `playground/`; `node:path` reported in `src/` and `playground/`, not in `eslint/`; a magic number reported outside a test, not in one; an empty `index.ts` accepted and an empty module reported; a default export accepted in a root `*.config.ts` and reported in `src/`. `tests/oxlint-config.test.ts` keeps the overrides equal to the scopes' native rules, files and settings (`[F19.AC1]`), and fails when a glob of the tooling scope is widened (tried).

## The new findings (T3)

With the rules triaged, oxlint gave 122 findings ESLint did not, in 11 rules, on 2026-10-17 (the spike counted 110 in 8 with `tsgolint` 7.0.2003; `no-deprecated`, `no-array-callback-reference` and the unstable `no-unnecessary-condition` came on top). Each rule got its code fixed, or an option that brings back ESLint's reading where oxlint reads more or where two kept rules contradict; none is turned off, and the tree now reports none.

| Rule | Findings | Decision |
| --- | --- | --- |
| `unicorn/prefer-number-properties` | 73, the global `NaN` | option `checkNaN: false`, the ESLint plugin's default: the kept `unicorn/prefer-global-number-constants` asks for the global `NaN`, so the two rules contradict on it; the rest of the rule (`Number.isNaN`, `Number.parseInt`…) still runs |
| `id-length` | 16, type parameters `S` and `T` | option `checkGeneric: false`: ESLint's rule never read type parameters, a single letter is TypeScript's usual name for one, and longer names (tried: `State`, `Item`) push two `math` functions of `scan.ts` over their 10 lines (`local/kind-limits`) |
| `typescript/no-deprecated` | 10, `defaultOptions` of the project's ESLint rules | code fixed: moved to `meta.defaultOptions`, which typescript-eslint's `RuleCreator` reads first |
| `preserve-caught-error` | 8, `catch` without a binding that throws nothing | option `requireCatchParameter: false`: oxlint asks for a binding on every `catch`, where ESLint asks for one only when the block throws, and the kept `unicorn/prefer-optional-catch-binding` forbids an unused one; with the option, a `throw` that loses the caught error is still reported |
| `unicorn/no-useless-undefined` | 6, an `undefined` argument | option `checkArguments: false`: each is meaningful — the seed of `reduce` (without it the first item seeds) or a required parameter typed `… \| undefined` (TypeScript refuses the call without it), both skipped by the ESLint plugin |
| `jsdoc/require-param`, `jsdoc/require-returns` | 3 and 3 | code fixed: oxlint's JSDoc reader loses the tags after inline code that runs across two lines (two comments rewrapped), and reads the comment above `export default defineConfig(…)` as the arrow's (its parameter and result documented) |
| `unicorn/prefer-global-this` | 1, `typeof window` in the site's theme | code fixed: `"window" in globalThis` |
| `unicorn/no-array-callback-reference` | 1, `.filter(isNode)` | code fixed: an arrow |
| `typescript/no-unnecessary-condition` | 1, a type predicate inferred on an `any` item | code fixed: the parsed list typed `readonly unknown[]` first |

`@typescript-eslint/prefer-destructuring` runs as the native `prefer-destructuring` with ESLint's options (`array: false`, `object: true`, `enforceForRenamedProperties: false`): with `array` off the rule never reads a type (`REF-TSESLINT-PREFER-DESTRUCTURING`, `REF-OXLINT-PREFER-DESTRUCTURING`); it reports nothing on the tree and an undestructured property on a probe file. `@typescript-eslint/no-unsafe-enum-assignment` is dropped: `enum` is forbidden (`no-restricted-syntax`), so it can never report (`REF-TSESLINT-NO-UNSAFE-ENUM-ASSIGNMENT`); its reason goes to the ADR of EN-085. One option of the migration is left out: `jsdoc/require-param`'s `enableRestElementFixer`, a fixer setting oxlint does not have.

## The triage of the 265 confirmed (T4)

Each verdict of note 0023's triage of the rules oxlint lacks was checked on 2026-10-17 against `src/`, `playground/`, `tests/` and the root configuration files: no class, `this`, accessor, `enum`, `for…in`, `let`, CommonJS export nor JSDoc type is there; `fetch` and `node:sqlite` appear only in `scripts/`, which moves to Rust in F21; `dependencies` is empty. The 33 drops hold. Four kept rules guard what another kept rule, the compiler or the formatter already refuses, so they are dropped too, each tried on a probe file; the rewrite stories lose them (EN-078 keeps 44 rules, EN-079 36, EN-080 37):

| Rule | Reason |
| --- | --- |
| `unicorn/no-optional-chaining-on-undeclared-variable` | TypeScript refuses a name declared nowhere (ts(2304)), with or without `?.` |
| `unicorn/no-xor-as-exponentiation` | `no-bitwise` (native, kept) forbids `^` |
| `unicorn/no-useless-continue` | `no-continue` (native, kept) forbids every `continue` |
| `unicorn/no-leading-empty-lines` | oxfmt removes the blank lines at a file's start, and `npm run format:check` fails on them |

So 228 of the 265 are kept and 37 dropped. Kept after a second look: `jsdoc/ts-ban-ts-comment` (the native `typescript/ban-ts-comment` does not read a `@ts-` tag inside a JSDoc block, tried), the DOM and web rules (the playground), the loop, `throw` and `try` rules (allowed in the playground and the tests).

The four ESLint core rules oxlint lacks, triaged the same way:

| Rule | Verdict |
| --- | --- |
| `no-restricted-syntax`, `require-atomic-updates` | kept, rewritten in the Rust linter (EN-075, which loses `consistent-this`) |
| `consistent-this` | dropped: `this` cannot appear (`functional/no-this-expressions`), so no alias of it can |
| `strict` | dropped: every file is a module, so strict code, and `unicorn/prefer-module` (native, kept) reports any `"use strict"` directive |
