# Dependencies

## Policy

- **Zero runtime dependency**: `dependencies` stays empty; the published library has no `import` of any package (`import-x/no-extraneous-dependencies` in `src/`).
- Every development package is pinned to an **exact version** (`npm install -D --save-exact`).
- Every package has **one row below** stating why it is needed and why this one rather than an alternative. The test `tests/package.test.ts` fails when a package of `package.json` has no row here, or when a row names a package that is not installed.
- Adding a package: search alternatives (official docs, maintenance date, weekly use, license), check `npm audit`, add the row **in the same change**, mention it in the commit (`build(deps): …`). A package bringing a structural choice also gets an ADR.
- Removing a package: remove its row.
- A crate of crates.io, for the Rust tools of `tools/`, follows the same rules: an exact version in the root `Cargo.toml` (`[workspace.dependencies]`, `name = "=x.y.z"`), a row under [Crates](#crates), its release notes read, a release at least 7 days old; `tests/package.test.ts` fails when a crate of `Cargo.toml` has no row there, or a row names no crate of it.

## Packages

### Language and build

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `@typescript/native` | TypeScript 7, the native compiler, under the alias of `REF-TS7-ANNOUNCEMENT` (`npm:typescript@7.0.2`): its `tsc` runs the type check and writes the declarations of `build`, about 5 times faster than 6.0 (EN-071); becomes `typescript` again at EN-085 | TypeScript 6's `tsc`: about 4.3 s against 0.9 s on this tree |
| `typescript` | TypeScript 6's API, as `npm:@typescript/typescript6@6.0.2` (binary `tsc6`), for the tool that imports `typescript` through a peer dependency until it leaves: typescript-eslint (EN-085); TypeDoc left with EN-072; patch upgrades only (`.ncurc.cjs`) | `typescript` 7 alone: no API until 7.1, so typescript-eslint cannot load it |
| `@types/node` | Node.js types for the tooling (ESLint plugin, configuration files) | — |
| `vite` | development server of the playground and library build (Product Owner requirement, ADR-0009) | — |
| `vitest` | tests, sharing Vite's configuration and transforms | Jest: separate transform pipeline |
| `@vitest/coverage-v8` | test coverage of `src/`, with 100 % thresholds (lines, branches, functions, statements): every behavior of the library is tested | `@vitest/coverage-istanbul`: slower, instruments the code |
| `happy-dom` | DOM implementation for the tests of `src/render/` (per-file `// @vitest-environment happy-dom`), so the shell keeps 100 % coverage | `jsdom`: heavier and slower |
| `fast-check` | property-based tests of `math` and `geometry` (ADR-0022): random inputs, shrunk counterexamples, seeded replay; MIT, one dependency (`pure-rand`) | hand-written random loops: no shrinking, no replay; `jsverify`: unmaintained |
| `@fast-check/vitest` | `test.prop` for Vitest: properties reported as tests, with the failing seed in the test name (ADR-0022) | calling `fc.assert` inside `it`: more boilerplate per property |

### Lint

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `eslint` | the linter; every rule decided explicitly (ADR-0010) | — |
| `@eslint/js` | lists the live core rules for the completeness audit (`eslint/config.test.ts`) | deprecated `builtinRules` API |
| `typescript-eslint` | TypeScript parser and type-aware rules | — |
| `@typescript-eslint/utils` | typed helpers to write the custom `local/*` rules | untyped `Rule.RuleModule` |
| `@typescript-eslint/rule-tester` | tests of the custom rules with Vitest | ESLint's untyped RuleTester |
| `jiti` | loads `eslint.config.ts`, the method documented by ESLint (`REF-ESLINT-TS-CONFIG`) | experimental native flag |
| `eslint-plugin-jsdoc` | mandatory, well-formed TSDoc | — |
| `eslint-plugin-import-x` | cycles, layer boundaries, import order and resolution | `eslint-plugin-import`: slower, less maintained; a home-made cycle checker: re-inventing a maintained tool |
| `eslint-plugin-unused-imports` | **removes** unused imports on save (core and typescript-eslint only report) | — |
| `eslint-plugin-functional` | immutability and no classes: "a function never modifies its arguments" (ADR-0014) | — |
| `eslint-plugin-unicorn` | ~370 modern best-practice rules | `eslint-plugin-sonarjs`: overlaps unicorn and complexity rules |
| `@stylistic/eslint-plugin` | blank lines between statements and comment style, which Prettier does not decide | — |
| `eslint-config-prettier` | turns off the ESLint rules that conflict with Prettier (ADR-0013) | `eslint-plugin-prettier`: discouraged by Prettier (`REF-PRETTIER-LINTERS`) |
| `@eslint/markdown` | lints the structure of every Markdown page (headings, links, code languages, tables) inside ESLint, with the same "every rule decided" audit | `markdownlint-cli2`: depends on `braces`, vulnerable without fix (GHSA-vfj7-8cjw-p6xm) |
| `oxfmt` | the formatter of every file kind the repository formats, Markdown included, natively in Rust (`REF-OXFMT`, `REF-OXFMT-LANGUAGES`), run by `tools/format`, which then writes Markdown tables without padding (ADR-0068, EN-073); 0.72.0, released 2026-10-05, its notes read (Markdown formatted by `oxc_formatter_markdown`), 0.73.0 being under 7 days old | Prettier 3.9: about 30 times slower (12.4 s against 0.4 s here), pads tables too, a JavaScript tool against the Rust rule (note 0023) |
| `oxlint` | the one lint command (`npm run lint`): ESLint's native rules, typescript-eslint's, Unicorn's, JSDoc's and Import's built in Rust, with their options (`REF-OXLINT`, `REF-OXLINT-PLUGINS`), its configuration `.oxlintrc.json` written by `@oxlint/migrate --type-aware` from `eslint/` (`REF-OXLINT-MIGRATE`, EN-074); 1.87.0, released 2026-10-05, its notes read (suggestions and fixes only, no rule removed), 1.88.0 being under 7 days old | ESLint 10 with typescript-eslint: about 62 s on this tree against 2 s; kept only for the rules oxlint lacks until F19 rewrites them and EN-085 removes it |
| `oxlint-tsgolint` | the type-aware rules of `oxlint` (`options.typeAware`), typescript-eslint's rules run on typescript-go: `no-floating-promises`, `strict-boolean-expressions`, the `no-unsafe-*` family… (`REF-OXLINT-TYPE-AWARE`, `REF-TSGOLINT`), loaded by `oxlint`, imported by no file (`knip.json`); 7.0.2004, released 2026-10-10, its notes read (fixes to `no-unsafe-return`, `no-misused-promises`, `no-floating-promises`, `no-deprecated` and line disable comments) | typescript-eslint's type-aware rules in ESLint: TypeScript 6's API and most of ESLint's 62 s |

### Documentation site

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `vitepress` | Markdown documentation site built on Vite (`REF-VITEPRESS`); 2.0 alpha because 1.6 bundles a vulnerable Vite 5 (ADR-0009) | Docusaurus: React and webpack/rspack toolchain, no native PDF either |

| `mermaid` | diagrams written as text in `mermaid` code blocks (flowcharts, Git graphs, sequences), drawn by the site's theme in the browser and natively by GitHub (`REF-MERMAID`, `REF-GH-DIAGRAMS`) | `vitepress-plugin-mermaid`: last release 2024, VitePress 1 only; hand-drawn SVG: hard to keep up to date |

### Git workflow

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `husky` | installs the Git hooks (`.husky/`) on `npm install` | `lefthook`: equally good, less widespread |
| `lint-staged` | runs the formatter, ESLint and secretlint on staged files only, before each commit (`.lintstagedrc.json`) | — |
| `@commitlint/cli` | checks commit messages | — |
| `@commitlint/config-conventional` | Conventional Commits rule set (`REF-CONVENTIONAL-COMMITS`) | — |
| `@commitlint/types` | types of `commitlint.config.ts` | — |
| `validate-branch-name` | checks the branch name before pushing | a home-made script |

### Security

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `secretlint` | detects keys, tokens and passwords in any file: before each commit, before each push (`check:all`) and in the CI into `main` | `gitleaks`: a Go binary outside npm; GitHub secret scanning also runs, server side |
| `@secretlint/secretlint-rule-preset-recommend` | secretlint's recommended rule set (cloud keys, GitHub, Slack, npm tokens, private keys…) | — |

### Maintenance

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `npm-check-updates` | `npm run deps:outdated`: fails when a dependency is not on its latest version | `npm outdated`: no per-package target (TypeScript limited to patches) |
| `knip` | `npm run deps:unused`: unused files, exports and dependencies (`knip.json`), which lint cannot see across modules | `ts-prune`: unused exports only; ESLint `no-unused-vars`: one module at a time |

## Rust tools

Tools outside npm, for the checks of the Rust workspace (EN-069). Same rules: a row here, the release notes read, a version at least 7 days old. Each version is pinned where it is installed — `.github/workflows/ci.yml`, the install line of [agent tools](./agent-tools.md#rust), the messages of `rust:coverage` and `rust:advisories` in `package.json` — and `deps:outdated` does not see them: a newer release is looked for by hand (`cargo search <tool>`) at each retrospective.

| Tool | Version, released | Why | Alternatives rejected |
| --- | --- | --- | --- |
| cargo-llvm-cov | 0.9.1, 2026-09-06 (41 days on 2026-10-17) | `rust:coverage`: the workspace's tests with LLVM's source-based coverage, failing under the floor of lines and functions (`REF-CARGO-LLVM-COV`); notes of 0.9.0 and 0.9.1 read: fixes (doctests in 2024-edition workspaces, build scripts left out of reports, Windows paths) and a `--show-missing-lines` format change, unused here | — |
| cargo-deny | 0.20.2, 2026-07-09 (100 days) | `rust:advisories`: every crate of `Cargo.lock` against the RustSec database, offline from its local copy (`REF-CARGO-DENY-ADVISORIES`); notes of 0.20.2 read: the command line refactored, `--offline` now an option of `cargo deny` itself, written before `check`; a new `bans.std-replacements` lint, unused as `bans` does not run | `cargo audit` 0.22.2 (`REF-CARGO-AUDIT`): the same database, advisories only, where cargo-deny also checks licences, bans and sources |
| llvm-tools, rustup's component (`llvm-tools-preview`) | of Rust 1.99.1, 2026-08-27 (51 days) | the `llvm-profdata` and `llvm-cov` that cargo-llvm-cov calls; listed in `rust-toolchain.toml`, so rustup installs it with the toolchain | a system LLVM: its version must match the compiler's |

## Assets

Data the library ships that is not an npm package. Same rules: a row here, its licence and its release read, a release at least 7 days old.

| Asset | Why | Alternatives rejected |
| --- | --- | --- |
| Inter 4.1, metrics only (`REF-INTER-RELEASE`, 2024-11-16) | the default font: its advances, kerning pairs and vertical metrics ship as data, so that a text is measured with no font file (ADR-0052, EN-031); weights 100 to 900, upright and italic, the 18 static files of the release's `extras/ttf/`; `src/model/font/inter-metrics.json`, written by `npm run font:metrics`, 1.60 MB (306 kB compressed with gzip), for the 2080 code points of the measured blocks, of which Inter maps 1372 (no Hebrew) | Arial: `fsType` 8, editable embedding only (`REF-OT-OS2`), and no open licence read; the fonts of the reader's system: not known in advance |
| Inter 4.1 Regular, the font file, tests only (`REF-INTER-REGULAR-FILE`) | a real font that the font reader's tests read on every system, compared with `inter-metrics.json` (EN-062, F18.AC19); `tests/inter-regular.ttf`, the release's `extras/ttf/Inter-Regular.ttf` unchanged, 412 kB, with `tests/inter-license.txt` beside it; never shipped (`files` of `package.json` names neither) | Arial, Helvetica: macOS fonts without an open licence, absent elsewhere, so their tests are skipped there |

Licence: SIL Open Font License 1.1 (`REF-INTER-LICENSE`). The fonts may be "bundled, embedded, redistributed and/or sold with any software"; each copy carries the copyright notice and the licence. The metrics file is a format change of the font, a Modified Version under the licence: it is distributed under the OFL with Inter's copyright notice and the licence text beside it, and keeps no reserved font name (Inter's notice names none). The licence does not reach the library's own code nor the documents made with the font. The notice and the licence sit beside the data, in `src/model/font/inter-license.txt`, the release's `LICENSE.txt` unchanged; that file ships with the package (`files` of `package.json`) and `NOTICE` names it.

## Overrides

Checked at each retrospective: an override goes once the package that pulls the vulnerable version asks for a fixed one. Still needed on 2026-10-13: `eslint-plugin-functional` 10.0.1 asks for `deepmerge-ts` `^7.1.5`, and `micromark-extension-math` 3.1.0 (through `@eslint/markdown`) for `katex` `^0.16.0`.

| Override | Why |
| --- | --- |
| `deepmerge-ts` → `^8.0.0` | `eslint-plugin-functional` pulls a version affected by GHSA-ggr8-5vv4-36mx; 8.x fixes it |
| `katex` → `^0.19.0` | `@eslint/markdown` pulls a version affected by GHSA-238p-pmpm-9mq7 (math rendering, unused here); 0.19 fixes it |

## Crates

The crates.io dependencies of the Rust tools, all from Oxc (`REF-OXC-CRATE`, MIT licence), at 0.153.0, published 2026-10-05, its release notes read (parser fixes for TypeScript diagnostics; nothing on the AST the tools read); 0.154.0 of 2026-10-13 is under 7 days old. The same version for every Oxc crate: they are released together and depend on each other at that exact version.

| Crate | Why | Alternatives rejected |
| --- | --- | --- |
| `oxc_parser` | reads the library's TypeScript into an AST with its comments, so that `tools/api-pages` writes the API pages from the exports and their TSDoc (EN-072), as the project's linter will on the same crates (note 0023) | `deno_doc` (`REF-DENO-DOC`): a second parser stack (`deno_ast`, `deno_graph`) writing HTML or JSON, not the site's Markdown pages; TypeDoc: needs TypeScript 6's API (`REF-TYPEDOC-TS7`) |
| `oxc_ast` | the AST's types, read by the tools | — (the parser's own types) |
| `oxc_allocator` | the arena the parser builds the AST in | — (the parser's own arena) |
| `oxc_semantic` | resolves each name of a type to the module's symbols, so that a type parameter, an import and a global of the language are told apart: the check that no internal name is reachable from the API | resolving names by hand: misses shadowing and imports |
| `oxc_span` | the source type (TypeScript) and the spans that give each export its line | — (the parser's own spans) |

## Considered for later

Not installed yet; to reconsider when the library grows (Product Owner, 2026-10-08).

| Package | Would bring | When |
| --- | --- | --- |
| `size-limit` | size budget of the published bundle | before the first npm publication |
| `@microsoft/api-extractor` | report of the public API, to detect breaking changes in pull requests | once the public API stabilizes |
| `@stryker-mutator/core` | mutation testing: proves the tests catch real changes, not only cover lines | when stryker-js issue 6210 (Vitest 5 runs no test against mutants) is fixed and released — tried 2026-10-08, ADR-0022 |
