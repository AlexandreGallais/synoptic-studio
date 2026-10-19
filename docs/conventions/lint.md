# Lint

Two linters run one after the other in `npm run lint` (ADR-0075). **oxlint** runs first, with its native rules, the type-aware rules of `tsgolint` and TypeScript's errors (`.oxlintrc.json`, ADR-0071). **The project's linter** (`tools/lint`, Rust) runs next, with the project's own rules, the plugin rules oxlint lacks, and the Markdown rules.

## Principles

- **Every rule is decided once, explicitly.** `.oxlintrc.json` turns off oxlint's default category and names each rule it runs, with its options; no preset is imported. A preset changes with its package, while a named rule changes only by a commit that gives its reason. `tests/oxlint-config.test.ts` fails when `.oxlintrc.json` names a rule oxlint does not have.
- **Everything is an error.** oxlint runs with `--deny-warnings`, and the project's linter fails on any finding. A warning nobody must fix is noise.
- **Strictest by default, relaxed by zone.** The first override of `.oxlintrc.json` (`**/*.ts`) and the rules the project's linter turns on everywhere set the functional core's strictness. Later overrides, and the `SCOPES` of `tools/lint/src/scopes.rs`, relax or tighten by path.
- **A rule stays if it serves the one who writes and reads this code**: it catches a bug the compiler lets through, it guards a domain rule or an ADR's decision, or it gives a construct one form. A rule that only costs time on what is already respected goes. A rule that guards correctness, the domain or an ADR is never dropped without a new ADR. EN-093 applied this test to every rule ([rules](./rules.md)).

## Zones

| Zone | Files | Effect |
| --- | --- | --- |
| pure layers (`.oxlintrc.json`) | `src/math`, `src/geometry`, `src/model`, `src/routing`, `src/io` | forbids the DOM and the host |
| `dom-layers` (`tools/lint`) | `src/render/`, `src/interaction/`, `playground/` | the functional core's statement rules off (ADR-0014) |
| barrels (both) | `src/**/index.ts`, `playground/*/**/index.ts` (not the entry) | any number of imports; the barrel's exports checked |
| tooling (both) | root `*.config.ts`, `*.setup.ts`, `*.test.ts`, `tests/`, `docs/.vitepress/`, `scripts/` | Node.js, development packages, the core's statement rules off |
| tests (both) | `**/*.test.ts` | literal numbers, free length, callbacks nested up to 4, no `@kind`, `@param` and `@returns` optional |
| root configurations (`.oxlintrc.json`) | root `*.config.ts`, `docs/.vitepress/config.ts` and `theme/index.ts` | a default export |

## Changing a rule

1. Find it: `grep -n '"rule-name"' .oxlintrc.json`, or its module in `tools/lint/src/rules/`.
2. Change its value there, and only there. A value taken from a table, such as `PROCEDURE_VERBS` or `KIND_LIMITS`, is changed in `tools/lint/src/settings.rs`.
3. Give an exception for a path to an override of `.oxlintrc.json` or a scope of `tools/lint/src/scopes.rs`, never to the rule itself, so that the rule stays whole everywhere else.
4. A rule turned off leaves `.oxlintrc.json` or its scope, with its reason in the commit and, for a rule of the project's linter, in its module's comment.
5. Run `npm run fix`, then `npm run lint` on the whole tree. `check:all` runs at the push.

After an oxlint upgrade, read its release notes for new rules, and decide each in `.oxlintrc.json`.

## One-off exceptions

Turn a rule off for one line only, with its reason, and register the line ([register of exceptions](./exceptions.md), ADR-0066):

```ts
// oxlint-disable-next-line @typescript-eslint/consistent-type-assertions -- data read from outside
// lint-disable-next-line local/kind-limits -- reason, for a rule of the project's linter
```

- The reason after `--` is mandatory (`unicorn/no-abusive-eslint-disable`).
- A directive that silences nothing is an error in each linter (oxlint's `--report-unused-disable-directives-severity=error`).
- oxlint reads `@typescript-eslint/x` and `typescript/x` as one rule; the directives use the first name.
- Write no `eslint-disable` directive: oxlint reads it as its own.

## Known autofix pitfall

`unicorn/prefer-import-meta-properties` replaces `fileURLToPath(new URL(".", import.meta.url))`, which ends with a slash, by `import.meta.dirname`, which does not: a path concatenated to it breaks. Always build paths with `join()`.

## The project's linter (`tools/lint`)

A crate of the Rust workspace on Oxc's parser and semantic model (research note 0023): `cargo run --release -q -p lint -- [--fix] <targets>`, on oxlint's targets. It prints each finding as `file:line:column rule message`, then a count, and fails on any.

- **One rule, one module** of `tools/lint/src/rules/`, with its tests beside it. A rule ported from a plugin keeps the plugin's name, message and options, and its tests run the cases of the plugin's documentation; the plugin's documentation is its reference. To add a rule: write the module, add its line in `rules::ALL`, and add its name to `ON_EVERYWHERE` or to a scope of `tools/lint/src/scopes.rs`. `rules::tests` fails when a rule is in no scope, or a scoped name has no rule.
- **Each file once**: each file is read and parsed once, with its comments and semantic model, then every rule its scopes turn on runs over that tree. Markdown pages are parsed into an mdast tree (`REF-MARKDOWN-RS`). What the rules read beside a file, such as folder listings, the bibliography or the ADRs, is read once per run. The whole tree takes about 0.2 s.
- **`--fix`** applies fixes as ESLint did: from the first range, a fix that touches one already applied waits for the next pass, and the file is linted again, ten passes at most.
- **Project values** (kinds, layers, verbs, reference sources, the folder signal) are in `tools/lint/src/settings.rs`.
- Every rule, by theme, with what it guards, is listed in [rules](./rules.md#every-rule-run-by-theme). Each rule dropped when ESLint left is listed with its reason in ADR-0075 and research note 0023.

### Rules read without types

Oxc has no type checker, so a rule that once asked the compiler for a type reads the written code instead (EN-076). Where the code does not tell a type, the rule never guesses: each row says what it does then. Write the type out when a rule must see it.

| Rule | How it reads the code |
| --- | --- |
| `@typescript-eslint/naming-convention` | a name's type from its annotation, else its initializer; a call's result is not read, so the next option applies ([naming](./naming.md#casing-typescript-eslint-naming-convention)) |
| `unicorn/no-non-function-verb-prefix` | a `getX`, `setX`, `addX`, `createX`… holds a function, read from its written type or initializer; an unknown value is not reported |
| `functional/no-return-void` | a core function returns a value: no `void`, `undefined` nor `null` return type; without a written type, the body is read |
| `functional/no-mixed-types` | a type holds only data or only functions; a member is a function when it is a method, a call or construct signature, or a property typed as a function |
| `functional/no-conditional-statements` | in the core, an `if` or `switch` stays only when each branch returns, throws, breaks or continues; a call that never returns does not end a branch |
| `require-atomic-updates` | in an `async` function or a generator, no assignment to a variable, or to a property through a variable, read before an `await` or `yield` and not since |
| Unicorn's array and collection rules | a receiver is an array or a collection when its literal, its `const` binding, its annotation or a type alias of the file says so; an iterator is no array |

### `functional/immutable-data`

In the functional core, no existing object or array is changed. That rules out an assignment, a `delete`, `++` or `--` on a member; a call by name of `copyWithin`, `fill`, `pop`, `push`, `reverse`, `shift`, `sort`, `splice`, `unshift`, `add`, `clear`, `delete` or `set`, whatever the object; and `Object.assign`, `defineProperty`, `defineProperties` or `setPrototypeOf` on an existing object.

### `functional/prefer-immutable-types` and `functional/type-declaration-immutability`

In the functional core, every parameter with a written type, and every declared type, is deeply read-only, written in place: each property, index signature, mapped type, array and tuple is marked `readonly`, or sits inside `Readonly<…>`. The rule cannot look a type up, so a named type passes only when its own declaration is checked the same way:

- a type of the same file, or imported by a relative path from the core;
- a type parameter, whose argument is checked where the caller writes it;
- `Pick`, `Omit`, `Partial`, `Required`, `NonNullable`, `Exclude` or `Extract` of such a type;
- `ReadonlyArray`, `ReadonlyMap` and `ReadonlySet`.

A type of the standard library or of a package is never trusted by its name: write `Readonly<DataView>`, `Readonly<Temporal.PlainDate>`. The autofix writes `readonly` and `Readonly<…>` where they are missing.

### `no-restricted-syntax`

No `enum`, no `for…in`, no getter or setter, no `export * as name`, no dynamic import; in `src/`, no default nor optional parameter (ADR-0016).

### The project's own rules

| Rule | Checks | Why |
| --- | --- | --- |
| `local/require-kind` | every top-level function carries exactly one known `@kind` | ADR-0011 |
| `local/kind-in-layer` | the `@kind` is allowed in the file's layer | [architecture](./architecture.md#layers) |
| `local/kind-limits` | lines, statements, complexity and depth by `@kind` | [functions](./functions.md#limits-per-kind-local-kind-limits) |
| `local/kind-naming` | a verb for `procedure`, a conversion for `format`, no vague verb for `math`, `geometry` and `domain`, a predicate prefix exactly when it returns a boolean | [naming](./naming.md#names-by-kind-local-kind-naming) |
| `local/see-references` | every function has a `@see`, each pointing to an existing, verified reference | [documentation](./documentation.md#see-references-local-see-references) |
| `local/one-export-per-file` | one exported value per module, the file named after it, a type beside it only when part of its signature | ADR-0019 |
| `local/folder-has-index` | every code folder holds an `index.ts` | ADR-0012 |
| `local/folder-layout` | a folder holds modules or domain folders, never both; a folder past 15 modules is listed with its reason | ADR-0043, [files](./files.md#folders-past-15-modules) |
| `local/barrel-exports` | an `index.ts` holds only sorted `export * from "./x";` lines, `export type * from "./x";` for a module holding only types; autofixed, a line written `export type *` kept so (EN-138) | ADR-0012, ADR-0092 |
| `local/canonical-import-path` | the shortest legal import path, never `index` nor `.ts`; autofixed | [imports](./imports.md#canonical-path-local-canonical-import-path-autofixed) |
| `local/image-alt-text` | an image's text alternative under 155 characters, never opening with "Image of" | [writing](./writing.md#building-blocks) |

### Markdown rules

Every Markdown page is checked by `@eslint/markdown`'s 20 rules, with that plugin's names, options and messages (`REF-ESLINT-MARKDOWN-RULES`), and by `local/image-alt-text`. They check heading levels, one title, link targets and their anchors, link definitions, table column counts, code fence languages, text alternatives, bare URLs and raw HTML (only `<span>` is allowed, for `<span v-pre>`). The list is in [rules](./rules.md#every-rule-run-by-theme). The formatter fixes none of these.
