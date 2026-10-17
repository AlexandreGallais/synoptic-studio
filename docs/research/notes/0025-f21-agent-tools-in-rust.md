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
