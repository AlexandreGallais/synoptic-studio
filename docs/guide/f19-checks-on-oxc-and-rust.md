# F19 — Checks on Oxc, TypeScript 7 and Rust

## What changed for you

Every change the agent makes passes `check:all` before it is pushed: the format, the lint, the type check, the tests, the Rust tools and, before F19, the documentation site each time. At F19's start this took about 2 minutes, most of it ESLint, Prettier, TypeScript 6 and a slow test setup, all JavaScript. F19 moved each check to a native tool — oxlint and oxfmt from the Oxc project, TypeScript 7, and a linter of the project's own in Rust — made the tests leaner, and builds the documentation site only when the change touches it. A code change is now checked in about 40 s instead of 124 s. Nothing changes in the editor.

## The words

| Word | In plain words |
| --- | --- |
| lint | a program that reads the code and reports what breaks a rule: a forbidden construct, a missing comment, a badly named function |
| formatter | a program that rewrites the code and the pages in one layout, so that nobody argues about spaces |
| type check | the TypeScript compiler's check that each value is used as its declared type allows |
| Oxc | a set of JavaScript and TypeScript tools written in Rust: a parser, oxlint (lint) and oxfmt (formatter) |
| the project's Rust linter | `tools/lint`, the rules oxlint lacks, rewritten in Rust on Oxc's parser |
| declarations | the `.d.ts` files that tell a program using the library the types of what it exports |
| API pages | the documentation site's pages listing each exported function, type and constant |
| lane | one step of `check:all`; the lanes run side by side and `check:all` ends with the slowest |

## The time gained

Measured on this Mac on 2026-10-17, under the heavy lock. Before: the figures recorded by F19's stories, each named; after: measured now by VAL-020.

| Check | Before F19 | After F19 |
| --- | --- | --- |
| `check:all`, a change outside the site's inputs | 124.0 s (EN-090), 123.4 s (SP-021) | 41.3 s and 39.6 s, the site's build skipped |
| `check:all`, a change in `docs/`, `src/` or the playground | the same: the site was always built | 57.0 s and 50.6 s, the site built in 56.3 s and 49.9 s |
| lint | ESLint 45.6 s alone (SP-021), 62.0 s (EN-074) | `npm run lint` 1.75 s: oxlint with the type check 1.35 s, the Rust linter 0.40 s |
| format check | Prettier 8.8 s alone (SP-021), 12.4 s (EN-073) | oxfmt and the Markdown tables 0.67 s |
| type check | TypeScript 6, 5.1 s alone (SP-021) | inside oxlint's lint pass, about 0.3 s more; `tsc` 7 alone 0.85 s |
| API pages | TypeDoc 2.8 s (note 0023) | 0.27 s, the Rust generator |
| declarations | TypeScript 6's `tsc`, inside `build` | 0.16 s, Oxc's isolated declarations; the whole `build` 1.8 s |
| test lane | 113.2 s alone (SP-021), 104.5 s (EN-090) | 29.4 s alone, 3 465 tests |
| Rust lane | 8 crates checked one by one, no coverage | 65.9 s alone (33.9 to 46.4 s in `check:all`): format, Clippy, 884 tests, coverage 94.05 % of lines, advisories, the Windows check |

F19 aimed at 70 to 90 s most of the time: a code change ends at about 40 s, a change to the site at about 55 s.

## Tools replaced and removed

| Before | After |
| --- | --- |
| ESLint 10 and 12 packages of plugins, configurations and test helpers (typescript-eslint, Unicorn, JSDoc, functional, @stylistic, import-x, unused-imports, @eslint/markdown, `eslint-config-prettier`…), and `jiti`, which loaded its configuration | oxlint 1.87.0 with `oxlint-tsgolint` for the rules that need types, then the project's Rust linter |
| Prettier 3.9 | oxfmt 0.72.0, then `tools/format` for the Markdown tables without padding |
| TypeScript 6 | TypeScript 7.0.2 only |
| TypeDoc and its two plugins | `tools/api-pages`, on Oxc's crates |
| `tsc -p tsconfig.build.json` for the declarations | `tools/declarations`, Oxc's isolated declarations |
| 8 Rust crates, each checked alone | one Cargo workspace of 12 crates, one `rust` lane |

18 dev packages left `package.json` and 3 came in (`oxlint`, `oxlint-tsgolint`, `oxfmt`); `typescript` went from 6.0.3 to 7.0.2. `dependencies` is still empty.

## The rules, triaged

- **The plugins' rules** that oxlint could run only as JavaScript plugins: 265, triaged at the spike — 228 kept and 37 dropped (research note 0023); each story checked its rules again before writing them.
- **The final count** (ADR-0075, checked by a test against the rules recorded from Git): of the 854 rules `eslint/rules/` decided, 374 run in oxlint, 205 in the project's Rust linter, 181 were dropped, each with its reason, and 94 were already off.
- **The rules that needed types**: `prefer-destructuring` runs natively; `no-unsafe-enum-assignment` was dropped, `enum` being forbidden; `naming-convention` and the five functional rules were rewritten without types, in their strict form: `readonly` written in place, nothing trusted from a type defined elsewhere.
- **The project's own**: its 10 rules, the 20 Markdown rules of `@eslint/markdown` kept and the project's image check, in the Rust linter, with their tests carried over.
- **A directive that silences nothing** is an error again, as it was with ESLint.

## Tests made leaner

| Story | What changed | Effect |
| --- | --- | --- |
| EN-089 | each font file read once per test run, one test proving the reading | no test over Vitest's 5 s under load |
| EN-090 | one claim proven in one place; modules shared within a worker; slow preparation done once | the test lane from 104.5 s to 37.4 s, 5 duplicate tests removed |
| EN-091 | every configuration file reviewed for speed; the type errors come with the lint | the setup's share of the lane from 14 % to 1 % |

The rules behind them are ADR-0070: one claim, one place; the level of each claim; an example is a hand-computed value; slow preparation done once; a time limit guards a hang, never speed.

## The decisions written down

| ADR | Decision |
| --- | --- |
| [0069](../adr/0069-tooling-fast-for-the-agent.md) | the tooling as fast as it can be, for the agent |
| [0070](../adr/0070-tests-without-over-testing.md) | tests without over-testing |
| [0071](../adr/0071-type-errors-with-the-lint-declarations-by-oxc.md) | type errors reported with the lint, declarations written by Oxc |
| [0072](../adr/0072-how-a-run-works.md) | how a run works: the go-ahead, branches and merges, three agents, one heavy command; a tooling feature validated by the agent |
| [0073](../adr/0073-every-new-tool-in-rust.md) | every new script, hook or agent tool written in Rust |
| [0074](../adr/0074-the-agent-shapes-its-own-method-and-tools.md) | the agent shapes its own method and tools, the Product Owner decides |
| [0075](../adr/0075-the-toolchain-on-oxc.md) | the toolchain on Oxc: oxlint, the project's Rust linter, oxfmt and TypeScript 7; supersedes 0009, 0010 and 0013 |
| [0076](../adr/0076-ported-tooling-code-under-its-licence.md) | code ported into the tools under its licence, never into the library |

## What the audit found

AUD-021 had three independent reviews (auditor, verifier, documentation editor); each finding was reproduced before it was fixed.

| Finding | Outcome |
| --- | --- |
| the code ported from 14 packages and the test cases copied named no licence | `NOTICE` and `THIRD-PARTY-LICENSES.md`, ADR-0076, a test: the one finding the Product Owner decided |
| a file moved out of `docs/` did not rebuild the site; the Rust toolchain files were not its inputs | fixed, a rename tested in a temporary repository |
| a directive silencing nothing passed | reported as an error again |
| three criteria traced on lists only; the rule accounting lost with `eslint/` | the 854 rules accounted against ADR-0075; a consumer type-checked against the declarations |
| a rule removed from the linter's lists, or a floating promise allowed: the tests still passed | each now fails a test |
| pages cut by a replacement string | rejoined; every tool passes a function to `replace` |

Deferred: duplicate helpers across four crates and three tests reading every source on their own, to F21; one fact in one place across `AGENTS.md`, `CLAUDE.md` and the skills, to F20.

## Each criterion and its evidence

Proven again on 2026-10-17 by VAL-020: its story holds the command and what was seen for each.

| Criterion | What you get | Evidence |
| --- | --- | --- |
| AC1 one lint command | `npm run lint`: oxlint, then the Rust linter, nothing reported | `tests/oxlint-config.test.ts`, `tests/eslint-rules-accounted.test.ts` |
| AC2 the rules that needed types | rewritten without types, strict | `tools/lint/src/rules/naming_convention.rs` and the five functional rules |
| AC3 the project's rules in Rust | on `oxc` 0.153.0 | `tools/lint/src/rules/` |
| AC4 the Markdown rules | 21 rules on the pages | `tools/lint/src/markdown/rules/` |
| AC5 oxfmt | `npm run format:check`, Markdown included | `tools/format/tests/format.rs` |
| AC6 TypeScript 7 alone | the type check in the lint, declarations by Oxc | `tests/type-check-with-lint.test.ts`, `tests/declarations.test.ts` |
| AC7 the API pages from Rust | 537 pages, every export documented, every link resolved | `tools/api-pages/tests/` |
| AC8 the site built on change | about 40 s for a code change | `tools/repo-tools/tests/check_all.rs` (`scripts/check-all.test.ts` until EN-103) |
| AC9 the `rust` lane | format, Clippy, tests, coverage floor 93 %, advisories | `tests/check-targets.test.ts`, `tests/rust-advisories.test.ts` |
| AC10 portable Rust | `File::lock`, checked for Windows | `tests/check-targets.test.ts` |
| AC11 every plugin rule triaged, ESLint out | kept rules in Rust with their documented cases | `tools/lint/src/rules/unicorn/`, `tools/lint/src/rules/jsdoc/` |
| AC12 tables without padding | the formatter writes them so | `tools/format/tests/format.rs` |

## How to try it

Run `npm run dev` and open `http://localhost:5173/?checklist=E17`: E17's checklist, F19's 12 lines after F18's, each a command to run in a terminal at the repository's root. A probe file planted to see a finding is deleted afterwards.

| Try | You should see |
| --- | --- |
| `npm run lint` | `lint: 0 finding(s) in <n> file(s)`, in about 2 s |
| a type error planted in `src/probe.ts`, then `npm run lint` | `error typescript(TS2322)` at its place |
| an `oxlint-disable` directive over a line without a finding | `Unused oxlint-disable directive` |
| a table row of three cells under two in `docs/probe.md` | `markdown/table-column-count` at that row |
| `npm run format:check`; a padded table, then `npm run fix` | `Markdown: 0 of <n> files not formatted.`; the table rewritten without padding |
| `npx tsc --version`, then `grep -c typescript6 package-lock.json` | `Version 7.0.2`, then `0` |
| `npm run build`, then count `dist/types` | 589 declarations |
| `npm run docs:api` | `api-pages: 537 pages written to docs/api` |
| `npm run check:all`, nothing of the site changed | `docs:build, playground:build skipped`, about 40 s |
| `npm run check:all`, a page of `docs/` changed | the site built too, about 55 s |
| `npm run check:rust` | `advisories ok`, the coverage above 93 %, the Windows check |
| `npx vitest run tests/eslint-rules-accounted.test.ts`, `cargo test --release -q -p lint` | every test passed |

## Limits

- Vitest, Vite and VitePress stay JavaScript, by decision; `oxlint-tsgolint` is written in Go.
- The project's TypeScript scripts and hooks, the `library` server and the dev packages still in JavaScript (secretlint, knip, npm-check-updates, commitlint, lint-staged, husky) are F21's: rewritten in Rust or kept with a reason; `default-font-metrics.ts` stays TypeScript, as it runs the library.
- F20 trims what each agent loads at its start, one fact in one place.
- A change to the site still waits for its build, about 50 s of the 55.

## Validation

Validated by the agent on 2026-10-17 (`VAL-020`), F19 being a tooling feature (ADR-0072): each of the 12 criteria proven again by its tests and its command, and F19's 12 checklist lines tried. The Product Owner may try the lines that concern them.

| Remark | What was done or where it went |
| --- | --- |
