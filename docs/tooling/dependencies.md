# Dependencies

## Policy

- **Zero runtime dependency**: `dependencies` stays empty; the published library has no `import` of any package (`import-x/no-extraneous-dependencies` in `src/`).
- Every development package is pinned to an **exact version** (`npm install -D --save-exact`).
- Every package has **one row below** stating why it is needed and why this one rather than an alternative. The test `tests/package.test.ts` fails when a package of `package.json` has no row here, or when a row names a package that is not installed.
- Adding a package: search alternatives (official docs, maintenance date, weekly use, license), check `npm audit`, add the row **in the same change**, mention it in the commit (`build(deps): …`). A package bringing a structural choice also gets an ADR.
- Removing a package: remove its row.

## Packages

### Language and build

| Package               | Why                                                                                                                                               | Alternatives rejected                                                        |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `typescript`          | compiler and type checker; pinned to 6.0 because `typescript-eslint` requires `<6.1` (ADR-0009)                                                   | TypeScript 7: no typed lint yet                                              |
| `@types/node`         | Node.js types for the tooling (ESLint plugin, configuration files)                                                                                | —                                                                            |
| `vite`                | development server of the playground and library build (Product Owner requirement, ADR-0009)                                                      | —                                                                            |
| `vitest`              | tests, sharing Vite's configuration and transforms                                                                                                | Jest: separate transform pipeline                                            |
| `@vitest/coverage-v8` | test coverage of `src/`, with 100 % thresholds (lines, branches, functions, statements): every behavior of the library is tested                  | `@vitest/coverage-istanbul`: slower, instruments the code                    |
| `happy-dom`           | DOM implementation for the tests of `src/render/` (per-file `// @vitest-environment happy-dom`), so the shell keeps 100 % coverage                | `jsdom`: heavier and slower                                                  |
| `fast-check`          | property-based tests of `math` and `geometry` (ADR-0022): random inputs, shrunk counterexamples, seeded replay; MIT, one dependency (`pure-rand`) | hand-written random loops: no shrinking, no replay; `jsverify`: unmaintained |
| `@fast-check/vitest`  | `test.prop` for Vitest: properties reported as tests, with the failing seed in the test name (ADR-0022)                                           | calling `fc.assert` inside `it`: more boilerplate per property               |

### Lint

| Package                          | Why                                                                                                                                          | Alternatives rejected                                                                                      |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `eslint`                         | the linter; every rule decided explicitly (ADR-0010)                                                                                         | —                                                                                                          |
| `@eslint/js`                     | lists the live core rules for the completeness audit (`eslint/config.test.ts`)                                                               | deprecated `builtinRules` API                                                                              |
| `typescript-eslint`              | TypeScript parser and type-aware rules                                                                                                       | —                                                                                                          |
| `@typescript-eslint/utils`       | typed helpers to write the custom `local/*` rules                                                                                            | untyped `Rule.RuleModule`                                                                                  |
| `@typescript-eslint/rule-tester` | tests of the custom rules with Vitest                                                                                                        | ESLint's untyped RuleTester                                                                                |
| `jiti`                           | loads `eslint.config.ts`, the method documented by ESLint (`REF-ESLINT-TS-CONFIG`)                                                           | experimental native flag                                                                                   |
| `eslint-plugin-jsdoc`            | mandatory, well-formed TSDoc                                                                                                                 | —                                                                                                          |
| `eslint-plugin-import-x`         | cycles, layer boundaries, import order and resolution                                                                                        | `eslint-plugin-import`: slower, less maintained; a home-made cycle checker: re-inventing a maintained tool |
| `eslint-plugin-unused-imports`   | **removes** unused imports on save (core and typescript-eslint only report)                                                                  | —                                                                                                          |
| `eslint-plugin-functional`       | immutability and no classes: "a function never modifies its arguments" (ADR-0014)                                                            | —                                                                                                          |
| `eslint-plugin-unicorn`          | ~370 modern best-practice rules                                                                                                              | `eslint-plugin-sonarjs`: overlaps unicorn and complexity rules                                             |
| `@stylistic/eslint-plugin`       | blank lines between statements and comment style, which Prettier does not decide                                                             | —                                                                                                          |
| `eslint-config-prettier`         | turns off the ESLint rules that conflict with Prettier (ADR-0013)                                                                            | `eslint-plugin-prettier`: discouraged by Prettier (`REF-PRETTIER-LINTERS`)                                 |
| `@eslint/markdown`               | lints the structure of every Markdown page (headings, links, code languages, tables) inside ESLint, with the same "every rule decided" audit | `markdownlint-cli2`: depends on `braces`, vulnerable without fix (GHSA-vfj7-8cjw-p6xm)                     |
| `prettier`                       | code formatting                                                                                                                              | —                                                                                                          |

### Documentation site

| Package                   | Why                                                                                                                                                                                       | Alternatives rejected                                                                                    |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `vitepress`               | Markdown documentation site built on Vite (`REF-VITEPRESS`); 2.0 alpha because 1.6 bundles a vulnerable Vite 5 (ADR-0009)                                                                 | Docusaurus: React and webpack/rspack toolchain, no native PDF either                                     |
| `typedoc`                 | generates the API reference from the TSDoc; supports TypeScript 6.0                                                                                                                       | —                                                                                                        |
| `typedoc-plugin-markdown` | TypeDoc output as Markdown pages (`REF-TYPEDOC-MARKDOWN`)                                                                                                                                 | TypeDoc's HTML site: separate from the docs                                                              |
| `typedoc-vitepress-theme` | VitePress-flavored Markdown and sidebar for the API pages                                                                                                                                 | —                                                                                                        |
| `mermaid`                 | diagrams written as text in `mermaid` code blocks (flowcharts, Git graphs, sequences), drawn by the site's theme in the browser and natively by GitHub (`REF-MERMAID`, `REF-GH-DIAGRAMS`) | `vitepress-plugin-mermaid`: last release 2024, VitePress 1 only; hand-drawn SVG: hard to keep up to date |

### Git workflow

| Package                           | Why                                                                                                  | Alternatives rejected                     |
| --------------------------------- | ---------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| `husky`                           | installs the Git hooks (`.husky/`) on `npm install`                                                  | `lefthook`: equally good, less widespread |
| `lint-staged`                     | runs Prettier, ESLint and secretlint on staged files only, before each commit (`.lintstagedrc.json`) | —                                         |
| `@commitlint/cli`                 | checks commit messages                                                                               | —                                         |
| `@commitlint/config-conventional` | Conventional Commits rule set (`REF-CONVENTIONAL-COMMITS`)                                           | —                                         |
| `@commitlint/types`               | types of `commitlint.config.ts`                                                                      | —                                         |
| `validate-branch-name`            | checks the branch name before pushing                                                                | a home-made script                        |

### Security

| Package                                        | Why                                                                                                                          | Alternatives rejected                                                              |
| ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `secretlint`                                   | detects keys, tokens and passwords in any file: before each commit, before each push (`check:all`) and in the CI into `main` | `gitleaks`: a Go binary outside npm; GitHub secret scanning also runs, server side |
| `@secretlint/secretlint-rule-preset-recommend` | secretlint's recommended rule set (cloud keys, GitHub, Slack, npm tokens, private keys…)                                     | —                                                                                  |

### Maintenance

| Package             | Why                                                                                                               | Alternatives rejected                                                          |
| ------------------- | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `npm-check-updates` | `npm run deps:outdated`: fails when a dependency is not on its latest version                                     | `npm outdated`: no per-package target (TypeScript limited to patches)          |
| `knip`              | `npm run deps:unused`: unused files, exports and dependencies (`knip.json`), which lint cannot see across modules | `ts-prune`: unused exports only; ESLint `no-unused-vars`: one module at a time |

## Assets

Data the library ships that is not an npm package. Same rules: a row here, its licence and its release read, a release at least 7 days old.

| Asset                                                     | Why                                                                                                                                                                                                                                                                                                                                                                                                                                                | Alternatives rejected                                                                                                                       |
| --------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Inter 4.1, metrics only (`REF-INTER-RELEASE`, 2024-11-16) | the default font: its advances, kerning pairs and vertical metrics ship as data, so that a text is measured with no font file (ADR-0052, EN-031); weights 100 to 900, upright and italic, the 18 static files of the release's `extras/ttf/`; `src/model/font/inter-metrics.json`, written by `npm run font:metrics`, 1.58 MB (303 kB compressed with gzip), for the 1968 code points of the measured blocks, of which Inter maps 1306 (no Hebrew) | Arial: `fsType` 8, editable embedding only (`REF-OT-OS2`), and no open licence read; the fonts of the reader's system: not known in advance |

Licence: SIL Open Font License 1.1 (`REF-INTER-LICENSE`). The fonts may be "bundled, embedded, redistributed and/or sold with any software"; each copy carries the copyright notice and the licence. The metrics file is a format change of the font, a Modified Version under the licence: it is distributed under the OFL with Inter's copyright notice and the licence text beside it, and keeps no reserved font name (Inter's notice names none). The licence does not reach the library's own code nor the documents made with the font. The notice and the licence sit beside the data, in `src/model/font/inter-license.txt`, the release's `LICENSE.txt` unchanged.

## Overrides

Checked at each retrospective: an override goes once the package that pulls the vulnerable version asks for a fixed one. Still needed on 2026-10-13: `eslint-plugin-functional` 10.0.1 asks for `deepmerge-ts` `^7.1.5`, and `micromark-extension-math` 3.1.0 (through `@eslint/markdown`) for `katex` `^0.16.0`.

| Override                  | Why                                                                                                             |
| ------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `deepmerge-ts` → `^8.0.0` | `eslint-plugin-functional` pulls a version affected by GHSA-ggr8-5vv4-36mx; 8.x fixes it                        |
| `katex` → `^0.19.0`       | `@eslint/markdown` pulls a version affected by GHSA-238p-pmpm-9mq7 (math rendering, unused here); 0.19 fixes it |

## Considered for later

Not installed yet; to reconsider when the library grows (Product Owner, 2026-10-08).

| Package                    | Would bring                                                                 | When                                                                                                                  |
| -------------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `size-limit`               | size budget of the published bundle                                         | before the first npm publication                                                                                      |
| `@microsoft/api-extractor` | report of the public API, to detect breaking changes in pull requests       | once the public API stabilizes                                                                                        |
| `@stryker-mutator/core`    | mutation testing: proves the tests catch real changes, not only cover lines | when stryker-js issue 6210 (Vitest 5 runs no test against mutants) is fixed and released — tried 2026-10-08, ADR-0022 |
