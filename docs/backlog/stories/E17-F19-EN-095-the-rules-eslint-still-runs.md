---
id: EN-095
epic: E17
feature: F19
title: The rules ESLint still runs, carried or dropped
status: done
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

- [x] T1 — The 22 Unicorn rules (2 h)
- [x] T2 — The 8 core and Import rules; the test that ESLint runs nothing (1 h)

## Each rule checked again (Product Owner, 2026-10-17)

Read against the code and, for the drops on what the compiler refuses, tried on a probe file (each documented case, `tsc` of TypeScript 7, the project's `tsconfig.json`).

| Rule | Still guards | Verdict |
| --- | --- | --- |
| `unicorn/no-double-comparison` | two comparisons where one says it; `oxc/double-comparisons` misses `x <= y && x !== y` and `x >= y && x !== y` | Rust, reported only |
| `unicorn/prefer-math-constants` | a hand-written `3.14`; `oxc/approx-constant` misses it, asking for more decimals | Rust, reported only |
| `unicorn/no-invalid-character-comparison` | a character compared with two, `text[0] === '/n'`; the compiler lets it through | Rust, reported only |
| `unicorn/no-constant-zero-expression` | `x * 0`, `x & 0`, `0 / x` | oxlint's `oxc/erasing-op`, the same five findings |
| `unicorn/no-misrefactored-assignment` | `a += a + b` | oxlint's `oxc/misrefactored-assign-op`, the same three findings |
| `unicorn/no-useless-concat` | `'a' + 'b'` on one line | oxlint's `no-useless-concat`: the same findings, at the left literal, no fix, and `'$' + '{x}'` reported too |
| `unicorn/consistent-class-member-order`, `no-undeclared-class-members` | nothing: no class can exist | dropped |
| `unicorn/no-accidental-bitwise-operator` | nothing more than `no-bitwise` (oxlint), on every file | dropped |
| `unicorn/no-boolean-sort-comparator` | nothing: TS2345 on its three cases | dropped |
| `unicorn/no-chained-comparison` | nothing: TS2365 and TS2367 (`a === b === flag`, a boolean `flag`, passes both) | dropped |
| `unicorn/no-collection-bracket-access` | nothing: TS7052 and TS7053, `map['size']` allowed by both | dropped |
| `unicorn/no-for-loop` | nothing: its loop needs a `let` index (`functional/no-let`), and `typescript/prefer-for-of` reports it | dropped |
| `unicorn/no-invalid-argument-count` | nothing: TS2554 on its five cases | dropped |
| `unicorn/no-missing-local-resource` | nothing here: in TypeScript it reads `new URL('…', import.meta.url)` only, which the code never writes | dropped |
| `unicorn/no-nonstandard-builtin-properties` | nothing: TS2551, TS2339, TS2349 and TS2351 on its six cases | dropped |
| `unicorn/no-shorthand-property-overrides`, `no-transition-all` | nothing: CSS and JSX, none linted; `no-transition-all` reads DOM strings only with types | dropped |
| `unicorn/no-unsafe-promise-all-settled-values` | nothing: TS2339 on its two cases (`playground/fonts/load-font-files.ts` reads `.status` first) | dropped |
| `unicorn/no-useless-coercion` | nothing more than `typescript/no-unnecessary-type-conversion` (type-aware), on its ten cases | dropped |
| `unicorn/no-useless-template-literals` | nothing more than `typescript/no-unnecessary-template-expression` and `no-implicit-coercion` | dropped |
| `unicorn/require-proxy-trap-boolean-return` | nothing: TS2322 on its two cases | dropped |
| `sort-imports` | the members of an import in order (`ignoreDeclarationSort`) | oxlint, native, the same options and the same three findings on a planted file |
| `import-x/order` | the order of the import statements | oxfmt's `sortImports` (`.oxfmtrc.json`): the same groups; among the `import type` statements, siblings, then parents, then modules, where `import-x/order` accepted parents and siblings in any order (17 files reordered) |
| `@typescript-eslint/no-invalid-this` | nothing: `this` is forbidden (`functional/no-this-expressions`), TS2683 | dropped |
| `import-x/no-unresolved` | nothing: TS2307, with the same resolution | dropped |
| `no-dupe-args`, `no-octal`, `no-octal-escape` | nothing: TS2300, TS1121, TS1487 | dropped |
| `no-undef-init` | nothing: it reports `let` and `var` only, both forbidden; `unicorn/no-useless-undefined` (oxlint) reports `= undefined` | dropped |

## Compared with ESLint

- Tree: no finding from either side for the 30 rules; oxlint, the project's linter and ESLint all clean after the 17 files oxfmt reordered.
- Planted cases: the Rust rules' findings at the same places as ESLint's on a file of the documented cases (3 + 3 + 3); oxlint's `oxc/erasing-op` (5), `oxc/misrefactored-assign-op` (3) and `no-useless-concat` (4, one more than Unicorn's: `' + '{PLUGINS}'`) on the same lines as Unicorn's; `sort-imports` the same three findings; on a test file of eleven imports out of order, `import-x/order` gave 16 findings, oxfmt's check refused the file, and once formatted `import-x/order` gave none.
- What the compiler refuses: one probe file of each documented case of the 13 rules dropped for it, each line refused by `tsc` (TypeScript 7), then removed.

## Timings (ADR-0069)

Measured on this Mac, 2026-10-17, on `npm run lint`'s targets (load average 1.4 to 3.2):

| Tool | Before (EN-084's tip) | After |
| --- | --- | --- |
| ESLint | 5.2–5.6 s; the 30 rules 691 ms (`TIMING=all`), `unicorn/no-invalid-argument-count` 292 ms, `import-x/no-unresolved` 126 ms, `import-x/order` 75 ms | 4.0–4.2 s, no rule: parsing only |
| Rust linter (`tools/lint`), 2,051 files | 0.38–0.47 s (EN-084) | 0.35–0.38 s |
| oxlint, with `--type-check` | 1.01 s | 1.08–1.09 s |

## Tests and mutations

Nine tests in the crate, the three rules on their documented cases and the cases their code distinguishes; in `tests/oxlint-config.test.ts`, ESLint runs none of the 22 Unicorn rules, the three oxlint carries are on, and ESLint runs no rule on any of the files `npm run lint` gives it. Mutations by hand: 6 tried, 4 caught at once; a regular expression on the right of a comparison led to a test; the optional read and call checks of `no-invalid-character-comparison` could never be reached (an optional read is a chain), and were removed. The configuration tests made to fail once: `prefer-math-constants` back on, `no-octal` out of `DROPPED_RULES`.

## For the main thread

- Rule-docs: none affected, the story moves lint rules between linters and the formatter and changes no rule of `docs/domain/`.
- ESLint now runs no rule: EN-085 only removes the package, `eslint/` and the ESLint lane of `npm run lint`; `tests/oxlint-config.test.ts` and `eslint/config.test.ts` go with it.
- Shared lists edited: `DROPPED_RULES` (22 names), `.oxlintrc.json` (four rules in its first override), `.oxfmtrc.json` (`sortImports`), Unicorn's `RULES` and `NAMES`, note 0023's counts (Unicorn 219, 154 kept and 65 dropped; Import 6, 3 and 3; typescript-eslint without types 3, 0 and 3; all 292, 203 kept and 89 dropped), its dropped table (18 rows), its kept table and its table of the core rules (five rows).
