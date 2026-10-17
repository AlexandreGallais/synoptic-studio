---
id: EN-076
epic: E17
feature: F19
title: naming-convention and the functional rules without types
status: done
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

## The rules (T1 to T4)

Seven modules of `tools/lint/src/rules/`, each with its tests; what they read of a type is in `tools/lint/src/written.rs` (a name's `function` or `boolean` type, a value's callability) and `tools/lint/src/deep_readonly.rs` (whether a written type is deeply read-only, and the fix that writes what is missing). Each is off in ESLint, in the task that carries it.

## Each rule checked again (Product Owner, 2026-10-17)

| Rule | Still guards | Verdict |
| --- | --- | --- |
| `@typescript-eslint/naming-convention` | the casing of every name by its role; no other rule asks `UPPER_CASE` of a module constant or a prefix of a boolean | kept |
| `functional/immutable-data` | a local array or object changed in the core: `prefer-immutable-types` checks parameters only, and a local `new Map()` or `[]` is mutable to the compiler | kept |
| `functional/prefer-immutable-types` | the core's parameters deeply read-only; the compiler refuses a write only where the type says `readonly` | kept |
| `functional/type-declaration-immutability` | the core's declared types deeply read-only, which the parameter rule relies on | kept |
| `functional/no-mixed-types` | a type holding data and functions, an object with methods | kept |
| `functional/no-conditional-statements` | a `switch` case ending on its own `break` and an `if` whose branch does not leave: `no-expression-statements` and `no-loop-statements` refuse most of what such a branch could hold, not a declaration nor a `break` | kept |

Found on the way: ESLint's last type-aware rule, `unicorn/no-non-function-verb-prefix` (`getX` holding a value), reports nothing once ESLint reads no type, and the triage of research note 0023 had not listed it. Carried the same way (`REF-UNICORN-NON-FUNCTION-VERB-PREFIX`, new row of `docs/references.md`). No rule is dropped.

## The strict form, and what it became (to confirm by the Product Owner)

The decision of 2026-10-17: `readonly` or `Readonly<…>` written in place, nothing trusted from a type defined elsewhere. Taken to the letter, every named type is wrapped in `Readonly<…>` wherever it is written (`p: Readonly<Point>`, `readonly start: Readonly<Point>`): measured with the rule's fix on `src/`, **597 findings in 331 files** (656 wraps), and then **60 type errors**: `Readonly<…>` of a function type has no call signature (TS2349), of a branded number is no longer a number (TS2345, TS2365), of a buffer breaks a generic constraint (TS2344). That form cannot be written in place where the type is a function, a branded number or a constrained buffer.

What the rules do instead, still nothing trusted by its name alone: a named type passes when its own declaration is checked the same way — a type of the same file, a type imported by a relative path (the core imports only from the core), a type parameter (its argument is the caller's type, checked where the caller writes it), `Pick`, `Omit`, `Partial`, `Required`, `NonNullable`, `Exclude` and `Extract` of such a type, and `ReadonlyArray`, `ReadonlyMap`, `ReadonlySet`. A type of the standard library or a package is never trusted: `view: Readonly<DataView>`. With this form the tree had **84 findings**, all fixed: 81 `DataView` parameters and three types read from a constant (`Readonly<(typeof ALIGNMENTS)[number]>`).

## Differences from ESLint

- `naming-convention` reads `types` from the written annotation, or else the initializer (an arrow, a function, a comparison, a negation, `Boolean(…)`); a call's result is not read. Five module constants of the tests holding a function returned by a call get their function type written. On a probe of 18 cases, both linters gave the same 18 findings at the same places.
- `no-non-function-verb-prefix`: 9 of ESLint's 10 findings on a probe; the tenth, a value that is an identifier, only the compiler knows.
- `immutable-data` reads the mutating methods by name on any object: the typed array's `set` of `facePartsToFile` is reported, with its directive and its row in the register (ADR-0066). An immediate mutation of a new array (`[3, 1].sort()`) is reported, as the project's options asked.
- `no-mixed-types` reads a property typed by a name as data.
- `no-conditional-statements` does not follow a call returning `never`.
- Class members, enums and accessors are left out of `naming-convention`: `functional/no-classes` and `no-restricted-syntax` refuse them.

## ESLint without types

ESLint's parser gets no project any more (`eslint/scopes/language.ts`); every rule left in it was tried without type information on the tree: none fails, none reported differently. `tests/oxlint-config.test.ts` checks that no project is given and that the seven rules are off (`[F19.AC2]`, made to fail twice: with `projectService` back, with `no-mixed-types` on).

## Measures (ADR-0069)

On the Product Owner's machine:

| Run | Time | CPU | Load average |
| --- | --- | --- | --- |
| before: `npm run lint` (oxlint, the Rust linter, ESLint with types) | 26.8 s, 27.1 s | 38.7 s, 37.4 s | 2 |
| after: `npm run lint` (ESLint without types) | 18.7 s, 18.0 s | 26.9 s, 26.2 s | 2.4 |
| the Rust linter alone, every rule, 1,538 files | 0.05 s (3 runs) | — | 2.4 |

## Tests and mutations

120 tests in the crate (28 new), the rules' coverage 92 to 100 % of lines. 20 mutations tried by hand (one change, the tests run, the file restored), each caught: one equivalent on two-member types (`any` against `all`) led to a three-member case.

## For the main thread

- `Rule-docs: none affected, a tooling change: no rule of docs/domain/ changes`.
- The strict form above needs the Product Owner's confirmation: the letter of the decision (597 findings, 60 type errors) against the form applied (a named type proven by its own declaration).
- Research note 0023's triage misses `unicorn/no-non-function-verb-prefix`: carried here; F19's feature file may name it under F19.AC2.

## Decision on the strict form

The main thread accepted on 2026-10-17 the strict form as applied here, to be confirmed by the Product Owner: wrapping every named type in `Readonly<…>` does not compile (a function type loses its call signature, a branded number stops being a number: 597 findings, then 60 type errors), so `readonly` / `Readonly<…>` is written in place, deeply, and a named type passes only when its own declaration is checked the same way; a type from the standard library or a package is never trusted by name. Nothing escapes, as decided on 2026-10-17.
