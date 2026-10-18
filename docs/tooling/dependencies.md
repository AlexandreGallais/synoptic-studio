# Dependencies

## Policy

Every dependency is code the project did not write and must trust, so each one is chosen, pinned and justified.

- **Zero runtime dependency**: `dependencies` stays empty; the published library imports no package (`import-x/no-extraneous-dependencies` in `src/`).
- **Pinned exactly**: development packages by `npm install -D --save-exact`, crates as `name = "=x.y.z"` in the root `Cargo.toml`'s `[workspace.dependencies]`, so that a build never changes under a release nobody read.
- **One row each** below, saying why it is needed and why this one rather than an alternative; `tests/package.test.ts` fails on a package or a crate without its row, and on a row naming nothing installed.
- **Adding** one: compare the alternatives (docs, maintenance, use, licence), read its release notes, take a release at least 7 days old ([versions](./versions-and-security.md)), check the audit, and add its row in the same commit (`build(deps): …`); a structural choice also gets an ADR. **Removing** one removes its row.
- The same rules hold for everything else the project takes from outside: the crates, the [Rust tools](#rust-tools) of the checks and the [assets](#assets).

## Packages

### Language and build

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `typescript` | TypeScript 7, the native compiler (`REF-TS7-ANNOUNCEMENT`): its checker runs under oxlint's `--type-check` and as `npm run typecheck`; no tool of the repository imports its API, which 7.0 lacks (ADR-0075) | TypeScript 6's `tsc`: about 4.3 s against 0.9 s on this tree |
| `@types/node` | Node.js types for the tooling | — |
| `vite` | development server of the playground and library build (ADR-0009, ADR-0075) | — |
| `vitest` | tests, sharing Vite's configuration and transforms | Jest: a separate transform pipeline |
| `@vitest/coverage-v8` | coverage of `src/` with 100 % thresholds | `@vitest/coverage-istanbul`: slower, instruments the code |
| `happy-dom` | a DOM for the tests of `src/render/` (`// @vitest-environment happy-dom`), so the shell keeps 100 % coverage | `jsdom`: heavier and slower |
| `fast-check` | property-based tests of `math` and `geometry` (ADR-0022): random inputs, shrunk counterexamples, seeded replay | hand-written random loops: no shrinking, no replay; `jsverify`: unmaintained |
| `@fast-check/vitest` | `test.prop`: properties reported as tests, the failing seed in the name | `fc.assert` inside `it`: more boilerplate |

### Lint

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `oxfmt` | the formatter of every file kind, Markdown included, in Rust (`REF-OXFMT`, `REF-OXFMT-LANGUAGES`), run by `tools/format` (ADR-0068) | Prettier: about 30 times slower, pads tables, a JavaScript tool |
| `oxlint` | the lint: ESLint's, typescript-eslint's, Unicorn's, JSDoc's and Import's rules built in Rust (`REF-OXLINT`, `REF-OXLINT-PLUGINS`), configured by `.oxlintrc.json` | ESLint with typescript-eslint: about 62 s on this tree against 2 s (ADR-0075) |
| `oxlint-tsgolint` | oxlint's type-aware rules on typescript-go (`REF-OXLINT-TYPE-AWARE`, `REF-TSGOLINT`), loaded by oxlint, imported by no file (`knip.json`) | typescript-eslint's type-aware rules: TypeScript 6's API |

### Documentation site

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `vitepress` | Markdown documentation site on Vite (`REF-VITEPRESS`); 2.0 alpha because 1.6 bundles a vulnerable Vite 5 (ADR-0075) | Docusaurus: a React toolchain, no native PDF either |
| `mermaid` | diagrams written as text, drawn by the site's theme and by GitHub (`REF-MERMAID`, `REF-GH-DIAGRAMS`) | `vitepress-plugin-mermaid`: VitePress 1 only; hand-drawn SVG: hard to keep up to date |

### Security

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `secretlint` | finds keys, tokens and passwords in any file, at commit, at push and in the CI | `gitleaks`: a Go binary outside npm |
| `@secretlint/secretlint-rule-preset-recommend` | secretlint's recommended rules | — |

### Maintenance

| Package | Why | Alternatives rejected |
| --- | --- | --- |
| `npm-check-updates` | `npm run deps:outdated` | `npm outdated`: no per-package target |
| `knip` | `npm run deps:unused`: unused files, exports and dependencies across modules | `ts-prune`: exports only; `no-unused-vars`: one module at a time |

## Rust tools

The tools outside npm that the Rust checks run. `deps:outdated` does not see them: each version is pinned where it is installed — `.github/workflows/ci.yml`, the install line of [agent tools](./agent-tools.md#rust), the messages of `rust:coverage` and `rust:advisories` in `package.json` — and a newer release is looked for by hand (`cargo search <tool>`) at each retrospective.

| Tool | Version, released | Why | Alternatives rejected |
| --- | --- | --- | --- |
| cargo-llvm-cov | 0.9.1, 2026-09-06 | `rust:coverage`: the workspace's tests under LLVM's source-based coverage, failing under a floor (`REF-CARGO-LLVM-COV`) | — |
| cargo-deny | 0.20.2, 2026-07-09 | `rust:advisories`: every crate of `Cargo.lock` against the RustSec database, offline (`REF-CARGO-DENY-ADVISORIES`); `--offline` comes before `check` | `cargo audit` (`REF-CARGO-AUDIT`): the same database, advisories only |
| llvm-tools, rustup's component | of the pinned Rust | the `llvm-profdata` and `llvm-cov` cargo-llvm-cov calls; listed in `rust-toolchain.toml`, installed with the toolchain | a system LLVM: its version must match the compiler's |

## Assets

Data the library ships that is not a package.

| Asset | Why | Alternatives rejected |
| --- | --- | --- |
| Inter 4.1, metrics only (`REF-INTER-RELEASE`) | the default font: advances, kerning and vertical metrics as data, so that a text is measured without a font file (ADR-0052); `src/model/font/inter-metrics.json`, written by `npm run font:metrics` from the release's 18 static files | Arial: editable embedding only (`REF-OT-OS2`), no open licence; the reader's system fonts: unknown in advance |
| Inter 4.1 Regular, the font file, tests only (`REF-INTER-REGULAR-FILE`) | a real font the font reader's tests read on every system: `tests/inter-regular.ttf`, unchanged, with `tests/inter-license.txt`; never shipped | Arial, Helvetica: absent outside macOS, without an open licence |

Inter is under the SIL Open Font License 1.1 (`REF-INTER-LICENSE`): it may be bundled and redistributed with software if each copy carries the copyright notice and the licence. The metrics file is a Modified Version under the licence, so it is distributed under the OFL with Inter's notice and keeps no reserved font name (Inter's notice names none). The licence reaches neither the library's own code nor the documents made with the font. The notice and the licence sit beside the data, `src/model/font/inter-license.txt`, shipped (`files` of `package.json`) and named by `NOTICE`.

## Overrides

An override forces a fixed version of a transitive package until the package that pulls it asks for a fixed one; checked at each retrospective, removed as soon as it is no longer needed.

| Override | Why |
| --- | --- |
| `katex` → `^0.19.0` | `mermaid` pulls a version affected by GHSA-238p-pmpm-9mq7 (`>= 0.11.0, < 0.18.2`; math rendering, unused here) |

## Crates

The crates.io dependencies of the Rust tools: Oxc's (`REF-OXC-CRATE`, MIT), all at one version since they are released together and depend on each other exactly, and `markdown` (`REF-MARKDOWN-RS`, MIT) with its one dependency `unicode-id`. The library's server (`tools/library`, EN-106) adds `rmcp`, `tokio`, `rusqlite` and `serde_json`, chosen and tried by SP-023 ([research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md), The crates), and the Git hooks (`tools/git-hooks`, EN-108) `regex`; each row gives the version, its date, its licence and what its release notes say.

| Crate | Why | Alternatives rejected |
| --- | --- | --- |
| `oxc_parser` | reads TypeScript into an AST with its comments, for `tools/api-pages` and the project's linter `tools/lint` | `deno_doc` (`REF-DENO-DOC`): a second parser stack writing HTML or JSON; TypeDoc: needs TypeScript 6's API (`REF-TYPEDOC-TS7`) |
| `oxc_ast` | the AST's types | — (the parser's own) |
| `oxc_ast_visit` | walks the AST in source order, entering and leaving each node, for the linter's measures and its rules that follow reads and `await`s | — (Oxc's own walker) |
| `oxc_allocator` | the arena the parser builds the AST in | — (the parser's own) |
| `oxc_semantic` | resolves each name to its symbol, so that a type parameter, an import and a global are told apart | resolving names by hand: misses shadowing and imports |
| `oxc_span` | the source type and the spans that give each node its line | — (the parser's own) |
| `oxc_isolated_declarations` | the library's declarations from each module alone, without a type checker, in `tools/declarations` (`REF-OXC-ISOLATED-DECLARATIONS`) | `tsc -p tsconfig.build.json`: a second compiler run; `oxc-transform` from npm: the same code behind a Node.js binding |
| `oxc_codegen` | prints the declarations as `.d.ts` text, comments kept | — (Oxc's own printer) |
| `markdown` | parses Markdown into an mdast tree with offsets, GFM and front matter, for the linter's Markdown rules: the Rust sibling of the parser `@eslint/markdown` used, so the same tree | `pulldown-cmark`: events, not a tree; `comrak`: seven required dependencies; a parser of our own: a project in itself |
| `rmcp` | the MCP server of the library over `stdio` (`REF-RMCP`), the official Rust SDK; 3.5.1 of 2026-10-05, Apache-2.0; its notes for 3.5.1 fix the server's discovery and keep invalid-parameter errors in the answer. Without its default features, with `server` and `transport-io` only: `macros` is left out since the three tools are declared by hand with the schemas they had, and its other features pull `reqwest`, `hyper` and `oauth2` | the protocol written by hand, as `library-mcp.ts` did: the SDK was the Product Owner's choice (F21, 2026-10-17) |
| `tokio` | the runtime `rmcp` runs on, with `rt`, `macros` and `io-std`; 1.53.2 of 2026-10-03, MIT; its notes for 1.53.2 list fixes of locks and timers only. 1.53.3 (2026-10-13) is under 7 days | — (`rmcp` requires it) |
| `rusqlite` | reads and writes the library's SQLite file, FTS5 included (`REF-RUSQLITE`, `REF-SQLITE-FTS5`); 0.40.2 of 2026-08-08, MIT, its notes for 0.40.2 only lower its minimum Rust. `bundled` compiles SQLite 3.53.2 (public domain) into the binary on macOS and Linux; on Windows the system's SQLite is linked, since the bundled C fails the Windows check of `check:rust` | the `sqlite3` command: one process per query and its text output parsed; `sqlx`: async and a larger tree |
| `regex` | the branch pattern and the commit message's patterns of `tools/git-hooks` (EN-108), Unicode classes included, and the secret scan's (EN-110); 1.13.1 of 2026-07-15, MIT OR Apache-2.0, the Rust project's own (`REF-REGEX-CHANGELOG`): its notes for 1.13.1 fix match offsets that could point past the leftmost-first match, never whether a match occurs; 1.13.0 adds the `regex!` macro. It brings `regex-automata` 0.4.18 (2026-08-04), `regex-syntax` 0.8.11 (2026-06-09) and `aho-corasick` 1.1.5 (2026-08-03, Unlicense OR MIT: MIT), `memchr` 2.8.3 already in the lock | `fancy-regex`: a backtracking engine for look-arounds the patterns do without, the look-aheads of `es-toolkit`'s words being matched by hand (`case.rs`) |
| `serde_json` | the tools' arguments and the JSON of the protocol; the transcripts and logs the metrics read (`tools/metrics`, EN-107), a fifth faster than the hooks' own reader on the 1.1 GB of one session (4.5 s of CPU against 5.6 s); 1.0.152 of 2026-10-10, MIT OR Apache-2.0, its notes fix two number cases of `arbitrary_precision`, a feature not used. 1.0.153 and 1.0.154 (2026-10-11) were under 7 days when it was taken: the lock went back to 1.0.152 | — (`rmcp`'s own) |

The four bring 67 crates into `Cargo.lock` (2026-10-18), every target's included, MIT or Apache-2.0 but `foldhash` (Zlib) and `r-efi` (MIT OR Apache-2.0 OR LGPL-2.1-or-later, taken as MIT, UEFI targets only); `cargo deny` finds no advisory. The crates Cargo resolved under 7 days were pinned back with `cargo update -p <crate> --precise <version>`: `cc` 1.7.0, `equivalent` 1.0.2, `libc` 0.2.190, `schemars` and `schemars_derive` 1.2.2.

## Considered for later

Not installed; reconsidered when the library grows (Product Owner, 2026-10-08).

| Package | Would bring | When |
| --- | --- | --- |
| `size-limit` | a size budget of the published bundle | before the first npm publication |
| `@microsoft/api-extractor` | a report of the public API, to catch breaking changes | once the public API stabilizes |
| `@stryker-mutator/core` | mutation testing of the whole suite | once stryker-js issue 6210 (Vitest 5 runs no test against mutants) is fixed (ADR-0022); `npm run mutate` covers it meanwhile |
