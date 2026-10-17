# 0025 — Sources of F21: agent tools and the library server in Rust

Spike SP-023, 2026-10-18, for F21 (E17). Every page cited was read through the local library; the registry data came from the crates.io API, the packages' behaviour from their documentation and from their code at the installed version (`node_modules`, read only). Nothing was added to the repository's `Cargo.toml` nor `package.json`: the crates were tried in a scratch workspace, the tools timed in scratch clones. Machine: Apple M1 Pro, 10 cores, 16 GB; Node.js 24.18.0, npm 12.1.0, Rust 1.99.1; other agents' work kept the load average between 2 and 13, noted with each measure. The TypeScript tools timed are F20's newest (`origin/feature/f20-a-context-diet` at `f645b69a`), since F21 ports them after F20 merges.

## The crates

### What each story needs

Each need was first matched against the standard library and the commands already installed (`git` 2.x, `gh`, `npm`, `curl` 8.7.1, `sqlite3` 3.54.0); a crate only where neither does it.

| Story | Need | Choice | Why |
| --- | --- | --- | --- |
| EN-102 | append one line per use | the standard library (`OpenOptions::append`) | no crate needed |
| EN-103 | Git, files, the backlog's Markdown | the standard library and the `git` command | the scripts already run `git` as a command |
| EN-104 | the hooks' JSON input, the transcripts | the `hooks` crate's own reader, or `serde_json` once EN-106 brings it | the hooks read a few fields today without a crate |
| EN-105 | one row or one section of a page | the standard library | a line scan |
| EN-106 | the MCP server | `rmcp` 3.5.1 with `tokio` 1.53.2 (`REF-RMCP`) | the official SDK, decided by the Product Owner |
| EN-106 | the store, FTS5 | `rusqlite` 0.40.2 with `libsqlite3-sys` 0.38.2 (`REF-RUSQLITE`, `REF-SQLITE-FTS5`) | reads and writes the store `node:sqlite` writes, tried below |
| EN-106 | a page fetched online, `If-None-Match` and `If-Modified-Since` | the `curl` command | a crate's TLS breaks the Windows check (below); `curl` ships with macOS, Windows 10 and Ubuntu |
| EN-106 | HTML to Markdown, sections, chunks | no crate: `library-html.ts` and `library-chunks.ts` ported line by line | the criterion asks for the same sections and chunks; an HTML parser would cut them differently |
| EN-107 | transcripts as JSON lines | `serde_json` 1.0.152, already pulled by `rmcp` | hundreds of megabytes read incrementally; a parser of our own for every field of a transcript is a project |
| EN-108 | the branch pattern, the message rules | `regex` 1.13.1, shared with EN-110, or a matcher by hand | one expression; the commit message is parsed by hand as commitlint's parser does (below) |
| EN-109 | the registry, GitHub, Node.js's index | `curl` and `gh api`, `serde_json` | no HTTP crate (same reason as EN-106) |
| EN-109 | imports and exports of the TypeScript | the workspace's `oxc_parser` and `oxc_semantic` 0.153.0, no `oxc_resolver` | the imports here are relative (3,933) or bare package names (`vitest`, `node:*`, `vitepress`, `mermaid`, `vite`); `oxc_resolver` 11.24.3 adds 59 crates for a resolution a path join does (`REF-OXC-RESOLVER`) |
| EN-110 | the preset's patterns | `regex` 1.13.1, the lookarounds checked in code | `fancy-regex` 0.19.2 backtracks; each lookaround of the preset tests one character before or after the match (below) |
| EN-110 | the GCP rule's PKCS #12 MAC | HMAC-SHA1 and HMAC-SHA256: the RustCrypto crates or a port of their algorithms, decided in EN-110 | the only rule that reads a binary file |
| EN-111 | the build cache | `actions/cache` (`REF-GH-CACHE`) | no crate |
| EN-112 | tags, releases, pull requests | the `git` and `gh` commands, `serde_json` for `gh --json` | no GitHub crate |

### The crates, checked

Read from the crates.io API on 2026-10-18 (`REF-CRATES-IO-API`): each version's date, licence and yanked flag. "7 days" means published on 2026-10-11 or before.

| Crate | Version | Published | Licence | Maintenance |
| --- | --- | --- | --- | --- |
| `rmcp` | 3.5.1 | 2026-10-05 | Apache-2.0 | 13 releases since 3.0.0 (2026-07-28), a minor version about every two weeks; 37 million downloads |
| `rmcp-macros` | 3.5.1 | 2026-10-05 | Apache-2.0 | released with `rmcp` |
| `tokio` | 1.53.2 | 2026-10-03 | MIT | 1.53.3 of 2026-10-13 is under 7 days |
| `rusqlite` | 0.40.2 | 2026-08-08 | MIT | bundles SQLite 3.53.2 (public domain) through `libsqlite3-sys` |
| `libsqlite3-sys` | 0.38.2 | 2026-08-08 | MIT | released with `rusqlite` |
| `serde` | 1.0.229 | 2026-07-18 | MIT OR Apache-2.0 | — |
| `serde_json` | 1.0.152 | 2026-10-10 | MIT OR Apache-2.0 | 1.0.154 (2026-10-11), already in F20's `Cargo.lock`, is 7 days old on 2026-10-18 |
| `schemars` | 1.2.2 | 2026-07-27 | MIT | pulled by `rmcp`'s `server` feature; 1.2.3 (2026-10-12) is what Cargo picks, under 7 days |
| `regex` | 1.13.1 | 2026-07-15 | MIT OR Apache-2.0 | the Rust project's own |

`rmcp` is taken without its default features, with `server`, `macros` and `transport-io` only: its default adds `base64`, and its other features pull `reqwest`, `hyper`, `oauth2` and `jsonwebtoken`, which a `stdio` server does not need.

**Advisories**: `cargo deny check advisories` (0.20.2, the repository's `deny.toml`: unmaintained and yanked crates fail too, `REF-CARGO-DENY-ADVISORIES`), database fetched on 2026-10-18, finds nothing on any of the three trial trees below.

**Crates under 7 days that Cargo resolves by itself**: on 2026-10-18 a fresh resolution picks `schemars` and `schemars_derive` 1.2.3, `syn` 3.0.9, `unicode-ident` 1.0.27, `equivalent` 1.0.3, `libc` 0.2.191 (2026-10-16) and `cc` 1.8.0 (2026-10-11, a build dependency of `libsqlite3-sys`). The story pins each to its latest version 7 days old with `cargo update -p <crate> --precise <version>` before its commit; F20's `Cargo.lock` already holds `syn` 3.0.9 and `unicode-ident` 1.0.27, so the rule is checked today on `[workspace.dependencies]` only, not on the lock (EN-109's check covers both).

**Licences in the trees** beyond MIT and Apache-2.0, each with the option the workspace takes: `foldhash` 0.2.0 (Zlib), `memchr` and `aho-corasick` (Unlicense OR MIT: MIT), `unicode-ident` ((MIT OR Apache-2.0) AND Unicode-3.0), `r-efi` (MIT OR Apache-2.0 OR LGPL-2.1-or-later: MIT; UEFI targets only, never built here), `ryu` (Apache-2.0 OR BSL-1.0: Apache-2.0) and `self_cell` (Apache-2.0 OR GPL-2.0-only: Apache-2.0), the last two only under `oxc_resolver`, which is not taken. Nothing copyleft is required by any choice (ADR-0076, decision 4). `deny.toml` runs the advisories check only, so no licence needs allowing there.

### The trial

A scratch workspace with the repository's `rust-toolchain.toml` built three trial crates: the library server (`rmcp`, `tokio`, `rusqlite`, `serde`, `serde_json`), an HTTP client (`ureq` 3.4.2) and the dependency check (`oxc_resolver`, `regex`, `serde_json`).

| Finding | Measure |
| --- | --- |
| A tools-only server on `rmcp`, `#[tool_router(server_handler)]` over `stdio()` as the SDK's README shows, builds and answers | start to its answer to `initialize`: 3.5 ms median (20 starts), against 76.8 ms for `library-mcp.ts` |
| Its binary and its first build | 5.2 MB; 20 s cold (76 s of CPU on 10 cores) |
| `rusqlite` with `bundled` compiles SQLite with `SQLITE_ENABLE_FTS5` (seen in the `cc` command line), and `porter unicode61`, `bm25()` and `snippet()` work | 92 crates in the tree, all targets, 73 not yet in the workspace's lock |
| On a copy of the real store (47 MB, written by `node:sqlite`, SQLite 3.53.1), the Rust trial searched `chunks` and found the same first page as `node:sqlite`, then inserted a row through the view `uses` and its trigger; `node:sqlite` then read that row from `project_uses` | 3.4 ms for open, search and insert; no migration |
| **`cargo check --target x86_64-pc-windows-msvc` fails** with `bundled`: `cc` compiles `sqlite3.c` for the Windows target and finds no `stdlib.h` | `check:rust` would fail |
| `rusqlite` without `bundled` links macOS's SQLite 3.54.0 (FTS5 compiled in) and passes the Windows check; `bundled` under `[target.'cfg(not(windows))'.dependencies]` passes it too and keeps one SQLite on macOS and Linux | chosen: `bundled` except for Windows |
| **`ureq` fails the same check**: its default TLS, `rustls` on `ring`, compiles C (`ring` 0.17.14 finds no `assert.h`) | an HTTP crate with C inside breaks `check:rust`; `curl` 8.7.1 starts in 9 ms |
| `oxc_resolver` 11.24.3 builds and resolves; the Windows check passes | 59 crates more than the workspace holds |

**`LIBRARY_PATH` is taken.** The library's store is found through the variable `LIBRARY_PATH` (`libraryPath()` in `scripts/library-store.ts`), the name the C toolchain uses for its library search path. Apple's `python3` sets it to `/usr/local/lib` for the processes it starts: a library script started from it opened, or created, a store at `<path>:/usr/local/lib` or failed with "unable to open database file", as two measures of this spike did before the cause was found. EN-106 is the story that can rename it (a criterion added there).

## The rewritten packages

What each package does **in this repository**, read in its documentation and in its code at the installed version, so that its Rust twin loses nothing. Each row is a behaviour the port keeps, or drops with its reason in its story.

### husky 9.1.7 (EN-108)

Read in the installed `index.js` and `husky` script (`REF-HUSKY-INSTALL`, `REF-HUSKY-PATH`).

| Behaviour | Used here |
| --- | --- |
| `husky` (from `prepare`) sets `core.hooksPath` to `.husky/_`, writes there `.gitignore` (`*`), the runner `h` and one stub per hook name (14 names, from `pre-commit` to `pre-auto-gc`), each sourcing `h` | yes: the three hooks `.husky/pre-commit`, `commit-msg`, `pre-push`; `worktree:new` runs it in each worktree (EN-052) |
| it refuses a folder holding `..`, needs `.git` (a worktree's `.git` file counts), skips with `HUSKY=0` | `HUSKY=0` is refused by `guard-bash` |
| `h` runs `.husky/<name>` with `sh -e` and the hook's arguments, after `PATH="node_modules/.bin:$PATH"`; exits 0 when that file is missing | yes: the hooks call `oxlint`, `commitlint`, `lint-staged`, `validate-branch-name` by name |
| `h` sources `~/.config/husky/init.sh` when present; `HUSKY=2` traces | no |
| a failure prints `husky - <name> script failed (code N)`, and for 127 the `PATH` searched | the only message a failing hook adds |

### lint-staged 17.6.0 (EN-108)

Read in its README (`REF-LINT-STAGED`) and its installed `matchFiles.js` and `getDiffCommand.js`. The repository's `.lintstagedrc.json`: `*.ts` → format, `oxlint --type-check --fix …`, the project's linter with `--fix`; `*.md` → format, the project's linter; `*.{cjs,css,html,json,scss,toml,yaml,yml}` → format; `*` → `secretlint`.

| Behaviour | Kept by the port |
| --- | --- |
| the staged files are `git diff --staged --diff-filter=ACMR` (added, copied, modified, renamed), submodules and symlinks left out | yes |
| a glob without `/` matches the file's base name anywhere (`matchBase`), dot files included (`dot: true`) | yes: `*` also matches `.github/…` files |
| each glob's commands run in order, the next one only when the previous one exits 0; the globs run at the same time | yes, or one after the other if the time allows (the formatter and the secret scan read the same files) |
| each command receives the matched files as absolute paths, in chunks under the platform's maximum argument length (`--max-arg-length`) | yes |
| a backup stash of the whole state (`git stash create` and `store`), dropped only after success | yes: an interrupted commit can be recovered |
| a partly staged file: its unstaged lines saved as a patch and removed while the commands run, applied back after | yes (EN-108's criterion) |
| the files the commands changed are added to the index; on a failure everything is reset from the backup | yes |

### commitlint 21.2.3 with `config-conventional` (EN-108)

Read in the installed packages (`REF-COMMITLINT`): `@commitlint/config-conventional`, `@commitlint/is-ignored`, `@commitlint/ensure`, `@commitlint/rules`, and the parser preset `conventional-changelog-conventionalcommits` 10.4.1 with `conventional-commits-parser` 7.1.3.

| Rule or step | Level | Setting |
| --- | --- | --- |
| header parsed by `^(\w*)(?:\((.*)\))?!?: (.*)$` into type, scope, subject; `!` before `:` marks a breaking change; `BREAKING CHANGE` and `BREAKING-CHANGE` start a note; a footer is a line starting with a token followed by `: ` or ` #`, the Git trailer convention (`Refs: SP-023.T1`) | — | the reason why a body line starting with `word:` is read as a footer (`AGENTS.md`) |
| `type-enum` | error | `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, `test` |
| `type-case` lower case, `type-empty` never | error | — |
| `scope-enum` | error | the project's 14 scopes (`commitlint.config.ts`) |
| `subject-empty` never, `subject-full-stop` never `.` | error | — |
| `subject-case` never `sentence-case`, `start-case`, `pascal-case`, `upper-case` | error | checked only when the subject starts with a cased letter; quoted and back-quoted parts removed first; each case computed by `es-toolkit` 1.52.0 (`upperFirst`, `startCase`, `camelCase`): an id first (`SP-023 …`) is sentence case |
| `header-max-length` 100, `header-trim` | error | — |
| `body-max-line-length` 100, `footer-max-line-length` 100 | error | a line holding `http://` or `https://` is exempt |
| `body-leading-blank`, `footer-leading-blank` | warning | printed, the commit passes |
| ignored messages: merges (`Merge pull request`, `Merge branch …`, `Merge … into …`, `Merge tag`, `Merge remote-tracking branch`, `Merged …`, `Automatic merge`, `Auto-merged … into …`), `Revert …`, `Reapply …`, `amend!`, `fixup!`, `squash!`, a header that is a semantic version after `chore(…):` | — | commitlint's defaults |
| ignored by the project: release-please's `chore(develop): release …` and `chore(main): release …`, and Dependabot's commits (`Signed-off-by: dependabot[bot]`) | — | the first goes with release-please (EN-112) |

### validate-branch-name 1.3.2 (EN-108)

Read in the installed `cli.js` and `lib/` (`REF-VALIDATE-BRANCH-NAME`). It reads the current branch from `.git` (`git-local-info`), or `--test <name>` (the CI's `history` job), tests it against the configuration's `pattern` (`.validate-branch-namerc.json`, found by `cosmiconfig`) as a JavaScript expression with the `g` flag, prints `Result: "passed"` or `"failed"` with the configured message, the name and the pattern, and exits 1 on a failure, or when no branch is found (a detached HEAD prints "not a git repository"). The pattern is the project's own: nothing of the package is ported.

### secretlint 13.0.7 and its recommended preset (EN-110)

Read in secretlint's README (`REF-SECRETLINT`) and the preset's installed bundle (`REF-SECRETLINT-PRESET`).

| Behaviour | Used here |
| --- | --- |
| exit 0 without a finding, 1 with one, 2 on a fatal error | yes |
| secrets masked in the messages by default | yes (EN-110: never the secret) |
| files skipped: the built-in `.git`, `node_modules` and `.secretlintrc*`; `.secretlintignore`; each folder's `.gitignore` (since 13); symlinks not followed | `lint:secrets` passes `git ls-files` with `--no-glob`; `.secretlintignore` lists 12 paths |
| `secretlint-disable`, `-enable`, `-disable-line`, `-disable-next-line` comments (`secretlint-rule-filter-comments`) | none in the tree today; kept |

The preset at 13.0.7 runs **27 detecting rules and the comment filter**: AWS, GCP, private keys, npm, basic authentication, Slack, SendGrid, Shopify, Stripe, GitHub, GitLab, Grafana, OpenAI, Anthropic, Groq, Hugging Face, Linear, Notion, 1Password, database connection strings (MongoDB, MySQL, PostgreSQL), HashiCorp Vault, Vercel, Databricks, Docker, Figma, Cloudflare, Tailscale. They hold **41 regular expressions**; what a port into Rust's `regex` must handle:

- **25 use a look-behind `(?<!\p{L})` and a look-ahead** such as `(?![A-Za-z0-9_])`, which the `regex` crate does not support: each tests one character before or after the match, checked in code.
- **2 backreferences** (`^(.)\1{3,}$`, a password of one repeated character treated as a dummy), checked in code.
- **`\b` in JavaScript is ASCII-only**, Unicode in Rust's `regex`: written `(?-u:\b)` (tried: the AWS pattern matches with it).
- **Named groups and per-rule options** (`allows`, the dummy values of the basic-authentication and database rules) carried with their defaults.
- **The GCP rule also reads binary `.p12` files** and checks their PKCS #12 MAC (RFC 7292) against the well-known password `notasecret`, with HMAC over SHA-1 or SHA-256 (the rule's own comments).

### npm-check-updates 23.1.1 (EN-109)

Read in its README (`REF-NCU-COOLDOWN`) and its installed `ncu --help`. The repository's `.ncurc.cjs`: `target: "latest"`, `cooldown: 7`, `errorLevel: 2`.

| Behaviour | Used here |
| --- | --- |
| the sections checked by default: `prod`, `dev`, `optional`, `packageManager` | `devDependencies` (23 packages; `dependencies` stays empty) |
| `target latest`: the version the `latest` dist-tag names; when it is inside the cooldown, the greatest version outside it, not nothing (`@latest` would skip the package) | yes |
| prereleases left out, unless the current version is one: then included | `vitepress` 2.0.0-alpha.20 is a prerelease |
| `errorLevel 2`: exit 1 when a package can be upgraded | yes, `check:all` fails then |
| a cooldown also read from `.npmrc` (`min-release-age`) when none is given | not used |

`deps:tools` (`scripts/tool-versions.ts`) adds each workflow action's latest major (`api.github.com/repos/<action>/git/matching-refs/tags/v<n+1>`, a token from `gh auth token`) and Node.js's latest LTS major (`nodejs.org/dist/index.json`).

### knip 6.41.0 (EN-109)

Read in its documentation (`REF-KNIP-ISSUE-TYPES`) and its own `--debug` output on this tree.

| Behaviour | Used here |
| --- | --- |
| issue types on by default: unused files, dependencies and devDependencies, referenced optional peer dependencies, unlisted dependencies and binaries, unresolved imports, unused exports and exported types, duplicate exports, unused catalog entries; `nsExports`, `nsTypes` and `cycles` only when asked | the defaults |
| plugins enabled from the dependencies found: commitlint, GitHub Actions, husky, lint-staged, Node.js, Oxfmt, Oxlint, TypeScript, Vite, VitePress, Vitest | each reads its tool's configuration for entry files and binaries |
| `knip.json`: 5 entry globs (hooks, scripts, the site's config and theme, `.ncurc.cjs`), 7 project globs, 3 dev packages ignored | the port's own list of entries |

### release-please, through `release-please-action` v5 (EN-112)

Read in its design and command-line documentation (`REF-RELEASE-PLEASE-DESIGN`, `REF-RELEASE-PLEASE-CLI`) and the repository's configuration. Nothing of it is ported: EN-112 writes its tool from the sources on good release notes.

| Behaviour | Used here |
| --- | --- |
| opens and updates a release pull request from a branch it names (`release-please--…`), labelled `autorelease: pending`, from the commits since the last release | on `develop`; the CI skips its jobs on that branch |
| the version from the commits: a breaking change raises the major, `feat` the minor, `fix` the patch; `bump-minor-pre-major` makes a breaking change raise the minor before 1.0.0 | yes, `initial-version` 0.1.0 |
| the `node` strategy writes `version` in `package.json` and `package-lock.json`, the changelog by `conventional-changelog-writer`, and `.release-please-manifest.json` | sections `feat`, `fix`, `perf`, `refactor`, `docs`, `build` shown, `test`, `ci`, `chore`, `style` hidden |
| on `main`: finds the merged release pull request by its label, tags its merge commit (`v0.2.0`, no component), publishes the GitHub release with the entry as notes, labels the pull request `autorelease: tagged` | yes; then the workflow attaches the test report and publishes the site |
| a personal access token (`RELEASE_PLEASE_TOKEN`), so that its pull request starts the CI | yes |

### Licences, for `NOTICE` and `THIRD-PARTY-LICENSES.md` (ADR-0076)

Read in each installed package's `package.json` and licence file. A story that ports code or copies test cases of a package adds its row and its text; a package whose behaviour is only reimplemented from its documentation needs none.

| Package | Version | Licence | Copyright line | Story |
| --- | --- | --- | --- | --- |
| `husky` | 9.1.7 | MIT | Copyright (c) 2021 typicode | EN-108 |
| `lint-staged` | 17.6.0 | MIT | Copyright (c) 2016 Andrey Okonetchnikov | EN-108 |
| `@commitlint/cli`, `config-conventional`, `types`, `is-ignored`, `ensure`, `rules`, `parse` | 21.2.3 | MIT | Copyright (c) 2016 - present Mario Nebl | EN-108 |
| `conventional-commits-parser` | 7.1.3 | MIT | Copyright © conventional-changelog team | EN-108, if its parsing is ported |
| `conventional-changelog-conventionalcommits` | 10.4.1 | ISC | Copyright © conventional-changelog team | EN-108, for the header pattern |
| `es-toolkit` | 1.52.0 | MIT | Copyright (c) 2024 Viva Republica, Inc. | EN-108, if `startCase` is ported |
| `validate-branch-name` | 1.3.2 | MIT | its licence names no holder; its author is jsonma | none: the pattern is the project's |
| `secretlint`, `@secretlint/secretlint-rule-preset-recommend` | 13.0.7 | MIT | Copyright (c) 2020 azu | EN-110 (each pattern and case ported) |
| `npm-check-updates` | 23.1.1 | Apache-2.0 | no `NOTICE` file in the package; its author is Tomas Junnonen | EN-109, if code is ported |
| `knip` | 6.41.0 | ISC | Copyright 2022-2026 Lars Kappert | EN-109, if code or cases are ported |
| `release-please-action` | v5 | Apache-2.0 (`REF-RELEASE-PLEASE-ACTION`) | — | none: nothing ported |
