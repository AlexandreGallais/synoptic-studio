---
id: EN-084
epic: E17
feature: F19
title: The other plugins' rules in Rust
status: done
points: 3
---

# E17 · F19 · EN-084 — The other plugins' rules in Rust

Rewrite in the project's Rust linter the 20 kept rules of the other plugins that oxlint runs only as JS plugins: `eslint-plugin-functional` (11), `@stylistic` (4), Import (2), `unused-imports` (2) and typescript-eslint without types (1) (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`, `REF-FUNCTIONAL-PLUGIN`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 20 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-084.Tn`.

- [x] T1 — functional (11): `no-let`, `prefer-property-signatures`, `readonly-type`, `no-classes`, `no-this-expressions`, `functional-parameters`, `no-expression-statements`, `no-loop-statements`, `no-promise-reject`, `no-throw-statements`, `no-try-statements`, each with its documented cases as tests (3 h)
- [x] T2 — @stylistic, Import, unused-imports, typescript-eslint (9): `line-comment-position`, `multiline-comment-style`, `spaced-comment`, `padding-line-between-statements`, `import-x/no-extraneous-dependencies`, `import-x/no-restricted-paths`, `unused-imports/no-unused-imports`, `unused-imports/no-unused-vars`, `@typescript-eslint/member-ordering`, each with its documented cases as tests (3 h)
- [x] T3 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)

## Each rule checked again (Product Owner, 2026-10-17)

| Rule | Still guards | Verdict |
| --- | --- | --- |
| `functional/no-let` | a variable reassigned: `prefer-const` was dropped for it (EN-074) | Rust |
| `functional/readonly-type` | one way to write a read-only type literal, `readonly` on its members | Rust, autofixed |
| `functional/no-classes`, `no-this-expressions` | the bans themselves, on which many drops rest | Rust |
| `functional/functional-parameters` | in the core, a function without parameter (a constant); its `arguments` part is oxlint's `prefer-rest-params`, on every file | Rust, the count only |
| `functional/no-expression-statements`, `no-loop-statements`, `no-throw-statements`, `no-try-statements` | the core's statements without side effect, off in the shell, the tools and the tests as in ESLint | Rust |
| `functional/no-promise-reject` | a rejected promise, also through a `throw` in an `async` function, on every file | Rust |
| `functional/prefer-property-signatures` | nothing more than oxlint's `typescript/method-signature-style` (`property`) | dropped |
| `@stylistic/spaced-comment`, `multiline-comment-style` | the comments' layout, which oxfmt leaves as written | Rust, autofixed |
| `@stylistic/padding-line-between-statements` | the blank lines between statements, which oxfmt keeps but never adds | Rust, autofixed |
| `@stylistic/line-comment-position` | nothing more than oxlint's `no-inline-comments`, which reports every comment beside code | dropped |
| `import-x/no-extraneous-dependencies` | `dependencies` empty: no package imported by the library or the playground; in the tools, no package `package.json` does not list | Rust |
| `import-x/no-restricted-paths` | the layers' imports, the playground through the public API only | Rust |
| `unused-imports/no-unused-imports`, `no-unused-vars` | unused imports and variables; oxlint runs the same check natively | oxlint's `no-unused-vars` |
| `@typescript-eslint/member-ordering` | nothing: no class, interface, method signature, nor call signature beside data can be written, and no type of the tree mixes an index signature with properties | dropped |

Found on the way (main thread's decision): `unicorn/no-useless-recursion` (EN-079) suggests a loop, which `functional/no-loop-statements` forbids in the core: it is left where that rule runs (`scopes::linted_by`).

## Compared with ESLint

- Tree: no finding from either linter for the 20 rules, once the byte buffer of `facePartsToFile` had its block of `eslint-disable` written as two `lint-disable-next-line` directives (its rows of the register follow).
- Planted cases (a file per layer, the tools and the tests): the same findings at the same places, 33 for the functional rules, 37 for the comment and blank-line rules, 9 for the Import rules; the same fixed text for `readonly-type`, the comment rules and the blank lines.
- `no-unused-vars` in oxlint against `unused-imports`: the same 13 findings per file, but a variable assigned and never read, reported at its declaration where ESLint reports its last assignment; its fix leaves an unused `import * as`, which `import/no-namespace` forbids.

## Timings (ADR-0069)

Measured on this Mac, 2026-10-17, on `npm run lint`'s targets, other worktrees running beside (load average 8 to 21):

| Tool | Before (EN-082's tip) | After |
| --- | --- | --- |
| ESLint | 10.7–11.8 s; the 20 rules 938 ms (`TIMING=all`), `no-extraneous-dependencies` 278 ms, `no-unused-imports` 276 ms, `no-restricted-paths` 140 ms | 9.8–10.3 s |
| Rust linter (`tools/lint`), 2,053 files | 0.39–0.45 s | 0.38–0.47 s |
| oxlint, without `--type-check` | 0.79–0.82 s | 0.79–0.80 s, `no-unused-vars` on |

## Tests and mutations

48 new tests in the crate (638 in all, EN-083's included), each rule on its documented cases; `[F19.AC11]` in `tests/oxlint-config.test.ts`: none of the 20 rules runs in ESLint on the sample files, and `no-unused-vars` carries `unused-imports`' options (both made to fail once: `no-let` back on, `caughtErrors` changed). 50 mutations tried by hand (one change, the tests run, the file restored): 46 caught at once; of the four left, two were equivalent (a body check that the declared functions already skip, a `<reference` skip that the marker already covers, both removed) and two led to a test (a margin offset of a bare block, a label read through).

## What ESLint still runs

On eight sample files (the core, the shell, the playground's entry, a script, a test, a root configuration, the site's configuration, `eslint/`), after this story rebased on EN-083, 30 rules:

- Unicorn, in no story nor in note 0023's triage (22): `consistent-class-member-order`, `no-accidental-bitwise-operator`, `no-boolean-sort-comparator`, `no-chained-comparison`, `no-collection-bracket-access`, `no-constant-zero-expression`, `no-double-comparison`, `no-for-loop`, `no-invalid-argument-count`, `no-invalid-character-comparison`, `no-misrefactored-assignment`, `no-missing-local-resource`, `no-nonstandard-builtin-properties`, `no-shorthand-property-overrides`, `no-transition-all`, `no-undeclared-class-members`, `no-unsafe-promise-all-settled-values`, `no-useless-coercion`, `no-useless-concat`, `no-useless-template-literals`, `prefer-math-constants`, `require-proxy-trap-boolean-return`.
- Core and the others, which EN-085 removes with ESLint (8): `@typescript-eslint/no-invalid-this`, `import-x/no-unresolved`, `import-x/order`, `no-dupe-args`, `no-octal`, `no-octal-escape`, `no-undef-init`, `sort-imports` (note 0023 counts `import-x/order` as oxfmt's import sorting and `no-invalid-this` as `noImplicitThis`).

## For the main thread

- Rule-docs: none affected, the story moves lint rules between linters and changes no rule of `docs/domain/`.
- The 22 Unicorn rules above run in ESLint but appear in no story and not in note 0023's triage (neither kept nor dropped): EN-085 cannot take ESLint out before they are carried, sent to oxlint or dropped.
- ADR-0014 still names `eslint/scopes/dom-layers.ts`, removed here (its five relaxed rules are scoped in `tools/lint/src/scopes.rs`): an accepted ADR is not rewritten; EN-085's ADR on Oxc may say where the relaxation lives now.
- Shared lists edited: `rules::named` (three `chain` lines), `scopes::rules_for` (three `chain` lines), `scopes::linted_by` (the `no-useless-recursion` lines), `settings::LAYERS` (the `imports` column), `DROPPED_RULES` (three names), note 0023's counts (functional 10 and 2, `@stylistic` 3 and 2, typescript-eslint 0 and 2; all 267, 196 kept and 71 dropped with EN-083's), its dropped table (three rows) and its kept table.

### Journal

- `#run` EN-084: the other plugins' rules carried — 10 functional, 3 `@stylistic` and 2 Import rules to the Rust linter, the 2 `unused-imports` rules to oxlint's `no-unused-vars`, 3 dropped (`prefer-property-signatures`, `line-comment-position`, `member-ordering`); the same findings and fixes as ESLint on the documented and planted cases, none on the tree; `no-useless-recursion` left in the core; 22 Unicorn rules found in no story.
