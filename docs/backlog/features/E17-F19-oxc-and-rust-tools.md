---
id: F19
epic: E17
title: Checks on Oxc, TypeScript 7 and Rust
status: in-progress
---

# E17 · F19 — Checks on Oxc, TypeScript 7 and Rust

**Benefit hypothesis**: with oxlint, oxfmt, TypeScript 7 and a Rust linter of the project in place of ESLint, Prettier and TypeScript 6, `check:all` runs in 70 to 90 s most of the time, with every current rule kept or its loss stated.

Opened by its spike ([SP-021](../stories/E17-F19-SP-021-research-f19.md), [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md)); refined with the Product Owner in four rounds on 2026-10-17, its run started the same day ([E17](../epics/E17-fast-tooling-with-oxc-and-rust.md)). The agent tools, the library server and the clean-up of E17 moved to [F21](./E17-F21-agent-tools-in-rust.md) on 2026-10-17.

## Decisions of the Product Owner (2026-10-17)

Recorded as understood, with the research behind each in note 0023, section "The Product Owner's answers".

- **`check:all`**: option B — Oxc instead of ESLint and Prettier, the docs build only when the docs change; about 70 to 90 s most of the time.
- **Scope**: the library stays TypeScript, run as JavaScript. Every tool around it is written in Rust or Go — the project's own in Rust, third-party ones rewritten natively (TypeScript 7, `tsgolint`, Oxc); no JavaScript tool of the project's own. Vite, Vitest and VitePress stay.
- **Split**: this feature holds the checks; the agent tools in Rust, the library server and the clean-up of E17 form F21.
- **Rules that need types**: `prefer-destructuring` native, `no-unsafe-enum-assignment` dropped because `enum` is forbidden; `naming-convention` and the five type-aware functional rules rewritten without types in their strict form — `readonly` or `Readonly<…>` written in place, nothing trusted from a type defined elsewhere; nothing kept in ESLint.
- **The project's rules**: in Rust, in a linter of the project on Oxc's crates (`oxc_parser`, `oxc_ast`, `oxc_semantic`), not as JS plugins (this supersedes the JS-plugin decision taken earlier the same day).
- **One linter**: oxlint, in Rust, for JavaScript and TypeScript. `tsgolint` (Go) only for rules that need types: 55 of the typescript-eslint rules the project enables do (`no-floating-promises`, `strict-boolean-expressions`, the `no-unsafe-*` family, `switch-exhaustiveness-check`…), so it stays for them, oxlint remaining the one lint command; the type check itself is TypeScript 7's compiler, not a linter. Every rule oxlint lacks is triaged, then rewritten in the project's Rust linter, all within F19, however long.
- **TypeScript**: TypeScript 7 only; TypeScript 6 removed. TypeDoc, which needs TypeScript 6's API, is replaced.
- **Rust**: a coverage floor and an advisory check of the crates.
- **Platforms**: the library's tests never depend on macOS. The tools stay on the Product Owner's Mac: written with Rust's portable standard calls where one exists, a goal and not a blocker, with one cheap check and no three-system CI.

## Acceptance criteria

1. `npm run lint` runs oxlint 1.87.0 or later (each version at least 7 days old) with its native rules and `tsgolint`'s type-aware rules, then the project's Rust linter, and reports nothing on the tree; every rule of `eslint/` is carried, rewritten, or dropped with its reason in the ADR.
2. `@typescript-eslint/prefer-destructuring` is carried by the native `eslint/prefer-destructuring` with the same options; `no-unsafe-enum-assignment` is dropped because `enum` is forbidden; `naming-convention` and the five type-aware functional rules are rewritten without types in their strict form, each with tests of what it catches and of a type defined elsewhere that it refuses. `unicorn/no-non-function-verb-prefix`, the last rule of ESLint needing types, runs in the Rust linter too (EN-076).
3. The 10 project rules, `functional/no-return-void`, `no-restricted-syntax` (its 7 selectors, the fold-spread one carried by oxlint's `oxc/no-accumulating-spread`), and `require-atomic-updates` run in the project's Rust linter on the `oxc` crates (0.153.0 or later, at least 7 days old, each crate in `docs/tooling/dependencies.md`), each with its tests carried over from `eslint/`.
4. The 21 Markdown rules run in the same Rust linter, with the findings ESLint's Markdown plugin gave on the same pages.
5. `npm run format:check` runs oxfmt on the same targets as Prettier, Markdown included.
6. TypeScript 7.0.2 or later is the only TypeScript: the type check reports what TypeScript 6 reported, through oxlint's type check (EN-091), and `build` writes declarations meaning the same, through Oxc's isolated declarations (EN-091, ADR-0071); no package of TypeScript 6 is left.
7. The API pages of the documentation site (one per exported function, type and constant, the index and the sidebar) come from a Rust generator on Oxc instead of TypeDoc, and its checks hold: every export documented, every link resolved, nothing internal published.
8. `check:all` runs the docs build only when `docs/`, `src/` or the playground changed, and ends in 70 to 90 s on a code story on the Product Owner's machine, lane times printed as now.
9. A `rust` lane runs `cargo fmt --check`, Clippy with warnings denied, the tests, a coverage floor (`cargo-llvm-cov`) and `cargo deny check advisories` (offline in the pre-push).
10. The Rust tools use Rust's portable standard calls where one exists (`File::lock` instead of `flock`), and the `rust` lane checks them for Windows without linking.
11. Every rule that oxlint runs today only as a JS plugin of a third-party package (265) is triaged — dropped with its reason when a project rule already forbids what it checks or when it guards nothing this code can contain, kept otherwise (note 0023: 228 kept, 37 dropped, after EN-074 confirmed the triage against the code) — and each kept rule runs in the project's Rust linter, with tests of the cases its plugin's documentation gives; ESLint leaves at F19's end.
12. The formatter prints Markdown tables without padding and keeps paragraphs as written (moved from F18, ADR-0068).

## Not moved, and why

- **Vitest, Vite and VitePress**: kept by the Product Owner's decision. They run with TypeScript 7 alone (tried on 2026-10-17: they transform with Oxc, not `tsc`). Vite 8.3.4 already bundles with Rolldown 1.2.
- **`default-font-metrics.ts`**: it runs the library to write `inter-metrics.json`; a Rust copy would duplicate the font reader (F21 holds it).
- **secretlint, knip, npm-check-updates, commitlint, lint-staged, husky, validate-branch-name**: reviewed in [F21](./E17-F21-agent-tools-in-rust.md), which rewrites them in Rust.

## Decisions of the run (Product Owner, 2026-10-17)

- Nothing is added to ESLint from now on: every rule that moves goes to oxlint or to the project's Rust linter, and ESLint only loses rules until EN-085 removes it.
- Each rewrite story checks each of its rules again before writing it: a rule that guards nothing useful here is dropped with its reason, not rewritten.
- A rule found useful on the way — for the project's code or for the agent's work — is added to the Rust linter in F19, with its source and its tests.

## Stories

In delivery order, written on 2026-10-17 when the Product Owner started F19's run. TypeScript 6 cannot leave at EN-071 as first drafted: ESLint's parser and TypeDoc read its API, which TypeScript 7 lacks until 7.1, so it stays under its alias until EN-072 replaces TypeDoc and EN-085 takes ESLint out. Each rewrite story turns its rules off in ESLint, so that a rule runs in one linter only until ESLint leaves.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-021](../stories/E17-F19-SP-021-research-f19.md) | Spike | Research for F19, Oxc and Rust tools | done |
| [EN-068](../stories/E17-F19-EN-068-cargo-workspace-and-rust-lane.md) | Enabler | The Cargo workspace at the root and the rust lane | done |
| [EN-069](../stories/E17-F19-EN-069-rust-coverage-and-advisories.md) | Enabler | A coverage floor and an advisory check for the Rust tools | done |
| [EN-070](../stories/E17-F19-EN-070-portable-rust-tools.md) | Enabler | Rust tools on portable standard calls | done |
| [EN-071](../stories/E17-F19-EN-071-typescript-7-for-the-type-check.md) | Enabler | TypeScript 7 for the type check and the declarations | done |
| [EN-072](../stories/E17-F19-EN-072-api-pages-from-a-rust-generator.md) | Enabler | The API pages from a Rust generator on Oxc | done |
| [EN-073](../stories/E17-F19-EN-073-oxfmt-instead-of-prettier.md) | Enabler | oxfmt instead of Prettier | done |
| [EN-074](../stories/E17-F19-EN-074-oxlint-with-native-and-type-aware-rules.md) | Enabler | oxlint with its native and type-aware rules | done |
| [EN-075](../stories/E17-F19-EN-075-project-linter-in-rust.md) | Enabler | The project's Rust linter and its own rules | done |
| [EN-076](../stories/E17-F19-EN-076-naming-and-functional-rules-without-types.md) | Enabler | naming-convention and the functional rules without types | done |
| [EN-089](../stories/E17-F19-EN-089-font-files-read-once-per-test-run.md) | Enabler | Font files read once per test run | done |
| [EN-090](../stories/E17-F19-EN-090-tests-without-over-testing.md) | Enabler | Tests without over-testing, and the rules to keep them so | done |
| [EN-091](../stories/E17-F19-EN-091-configuration-files-reviewed-for-speed.md) | Enabler | Every configuration file reviewed for speed, type errors reported with the lint | done |
| [CHK-020](../stories/E17-F19-CHK-020-checkpoint-f19.md) | Checkpoint | Checkpoint in the middle of F19 | done |
| [EN-077](../stories/E17-F19-EN-077-markdown-rules-in-rust.md) | Enabler | The Markdown rules in the Rust linter | done |
| [EN-078](../stories/E17-F19-EN-078-unicorn-arrays-collections-iteration.md) | Enabler | Unicorn's arrays, collections and iteration rules in Rust | done |
| [EN-079](../stories/E17-F19-EN-079-unicorn-control-flow-functions-errors-numbers.md) | Enabler | Unicorn's control-flow, functions, errors and numbers rules in Rust | done |
| [EN-080](../stories/E17-F19-EN-080-unicorn-objects-strings-naming-misc.md) | Enabler | Unicorn's objects, strings, naming and miscellaneous rules in Rust | done |
| [EN-081](../stories/E17-F19-EN-081-unicorn-modules-platform-promises.md) | Enabler | Unicorn's modules, platform and promises rules in Rust | done |
| [EN-082](../stories/E17-F19-EN-082-unicorn-dom-and-web.md) | Enabler | Unicorn's DOM and web rules in Rust | done |
| [EN-083](../stories/E17-F19-EN-083-jsdoc-rules-in-rust.md) | Enabler | JSDoc's rules in Rust | done |
| [EN-084](../stories/E17-F19-EN-084-other-plugin-rules-in-rust.md) | Enabler | The other plugins' rules in Rust | done |
| [EN-095](../stories/E17-F19-EN-095-the-rules-eslint-still-runs.md) | Enabler | The rules ESLint still runs, carried or dropped | done |
| [EN-094](../stories/E17-F19-EN-094-adrs-reviewed-with-the-product-owner.md) | Enabler | The ADRs reviewed with the Product Owner, the review applied | done |
| [EN-093](../stories/E17-F19-EN-093-rules-and-format-chosen-by-the-agent.md) | Enabler | Every rule and tool reviewed by the agent for its own work | ready |
| [EN-085](../stories/E17-F19-EN-085-docs-build-on-change-eslint-out-oxc-adr.md) | Enabler | The docs build on change, ESLint out, the ADR on Oxc | done |
| [AUD-021](../stories/E17-F19-AUD-021-audit-f19.md) | Audit | Audit F19 | ready |
| [VAL-020](../stories/E17-F19-VAL-020-validate-f19.md) | Validation | Validate F19 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| F19.AC1 | EN-074, EN-075, EN-085 | AUD-021, VAL-020, `[F19.AC1]` |
| F19.AC2 | EN-074, EN-076 | AUD-021, VAL-020, `[F19.AC2]` |
| F19.AC3 | EN-075 | AUD-021, VAL-020, `[F19.AC3]` |
| F19.AC4 | EN-077 | AUD-021, VAL-020, `[F19.AC4]` |
| F19.AC5 | EN-073 | AUD-021, VAL-020, `[F19.AC5]` |
| F19.AC6 | EN-071, EN-072, EN-085 | AUD-021, VAL-020, `[F19.AC6]` |
| F19.AC7 | EN-072 | AUD-021, VAL-020, `[F19.AC7]` |
| F19.AC8 | EN-085 | AUD-021, VAL-020, `[F19.AC8]` |
| F19.AC9 | EN-068, EN-069 | AUD-021, VAL-020, `[F19.AC9]` |
| F19.AC10 | EN-070 | AUD-021, VAL-020, `[F19.AC10]` |
| F19.AC11 | EN-074, EN-078, EN-079, EN-080, EN-081, EN-082, EN-083, EN-084, EN-085 | AUD-021, VAL-020, `[F19.AC11]` |
| F19.AC12 | EN-073 | AUD-021, VAL-020, `[F19.AC12]` |
