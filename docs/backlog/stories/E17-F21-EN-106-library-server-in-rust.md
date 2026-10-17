---
id: EN-106
epic: E17
feature: F21
title: The library server and its command line in Rust on rmcp
status: ready
points: 8
---

# E17 · F21 · EN-106 — The library server and its command line in Rust on rmcp

The `library` MCP server (`scripts/library-mcp.ts`) and its command line (`library-cli.ts`), with their store, pages, HTML, chunks and usage modules, rewritten as one Rust crate on `rmcp`, the official Rust SDK of the Model Context Protocol, over `stdio` (`REF-RMCP`; 3.5.1 or later, at least 7 days old), and a SQLite crate with FTS5 full-text search (`REF-SQLITE-FTS5`), checked as every crate is: version at least 7 days old, licence, advisories (F21's decisions of 2026-10-17; F21.AC3). Today the server starts Node.js at each session and reads `node:sqlite` (`REF-NODE-SQLITE`); its start was not measured ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), Everything around the library). Before the metrics tools (EN-107), whose Sources section reads the same store.

The crates, chosen and checked by SP-023 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), The crates): `rmcp` 3.5.1 (Apache-2.0, 2026-10-05) without its default features, with `server`, `macros` and `transport-io`; `tokio` 1.53.2 (MIT) with `rt`, `macros` and `io-std`; `rusqlite` 0.40.2 (MIT, `REF-RUSQLITE`) with `bundled` under `[target.'cfg(not(windows))'.dependencies]`, since the bundled SQLite's C fails `check:rust`'s Windows check; `serde` 1.0.229 and `serde_json` 1.0.152 or later; `schemars` held at 1.2.2 and every crate Cargo resolves under 7 days pinned back with `cargo update --precise`. No advisory on 2026-10-18; about 73 crates new to the workspace's lock. A page is fetched by the `curl` command (an HTTP crate's TLS compiles C and fails the same check), and the HTML is converted by a port of `library-html.ts`, without an HTML crate. A trial server answered `initialize` in 3.5 ms and read and wrote a copy of the real store with `node:sqlite` reading its row back.

The store is shared by every project on the machine (`~/.local/share/agent-library/library.sqlite`): EN-101 recorded each use with its project in `project_uses`, kept `uses` as a view of it for an older copy of the scripts, and made the `library-pages` hook read the store by `sqlite3 -readonly` ([sources](../../tooling/sources.md)). The port keeps all of it.

## Acceptance criteria

- Given the tools `library_search`, `library_get` and `library_read`, when the Rust server answers the same calls as the TypeScript one on a copy of the store, then its answers are the same, text for text, and `.mcp.json` starts the binary.
- Given the store as the TypeScript server leaves it — `pages`, the FTS5 table `chunks`, `project_uses`, the view `uses` and its trigger — when the Rust server opens it, then it reads and writes it without a migration, and a file it writes is read again by the TypeScript server of an older checkout.
- Given each search and each page read, when the Rust server serves it, then one `project_uses` row is written with the project as `projectOf` names it (EN-101), and `npm run library -- stats` gives the same report as before.
- Given the `library-pages` hook, when a page is refused or stored with fewer than 300 characters, then it behaves as before on its tests (`tools/hooks/tests/pages.rs`).
- Given `npm run library -- search | get | read | list | stats`, when each runs, then its output is the same as the TypeScript command line's.
- Given a page fetched online, when the server reads it, then the HTML is cut into the same sections and chunks as `library-html.ts` and `library-chunks.ts` cut it, on their tests' cases.
- Given the variable that names the store, when a process inherits the C toolchain's `LIBRARY_PATH` (Apple's `python3` sets it to `/usr/local/lib`, SP-023), then the server and the command line never open nor create a store at that value: the store's variable gets a name of its own, read first, and the docs and the `library-pages` hook use it.
- Given `check:rust`, when the crate is added, then `cargo check --workspace --target x86_64-pc-windows-msvc` still passes.
- Given each crate the story adds (`rmcp` and the crates it pulls, the SQLite crate), when the story ends, then each has its row in `docs/tooling/dependencies.md` (Crates) with its version, age, licence and release notes read, `cargo deny` finds no advisory, and `NOTICE` lists any package whose code is ported (ADR-0076).
- Given the server and the command line, when they are timed, then the server's start to its first answer, and one search, one get of a stored page and one read, are stated before and after.

## Files

`tools/library/` (new crate: the server and the command line), `Cargo.toml` (`[workspace.dependencies]`), `Cargo.lock`, `deny.toml` if a licence needs allowing, `.mcp.json`, `package.json` (`library`), `scripts/library-*.ts` and their tests (removed), `tools/hooks/src/pages.rs` if it shares the store's reading, `docs/tooling/sources.md`, `docs/tooling/dependencies.md`, `docs/tooling/agent-tools.md`, `NOTICE`, `THIRD-PARTY-LICENSES.md` if code is ported.

## Measures (ADR-0069)

Before and after, 20 calls each on this Mac: the server's start to its answer to `initialize`; `library_search` of three words; `library_get` of a stored page; `library_read` of three sections; the binary's size and its first build time.

Before, measured by SP-023 on a copy of the store (0025, Each tool's time before its port), medians: the server's start to `initialize` 76.8 ms; inside it, `library_search` 2.3 ms, `library_get` of a stored page 14.3 ms, `library_read` of three sections 6.7 ms; the command line's `search`, `get`, `read`, `list`, `stats` 83 to 95 ms each. The trial server: 3.5 ms to `initialize`, 5.2 MB, 20 s for its first build.

Before and after, measured on 2026-10-18 between 01:15 and 01:33 local time, load average 2.2 to 6.8: the TypeScript server and command line of the commit before T4, against the release binaries of T4, each on its own copy of the real store (`VACUUM INTO`), each call a process started and waited for, or a request answered by a running server, timed by a script of the scratch folder (Python's `perf_counter`), as note 0025 timed them. Medians of 20 calls.

| Call | TypeScript | Rust, the binary by its path | Rust, through `cargo run` |
| --- | --- | --- | --- |
| the server's start to its answer to `initialize` | 86.4 ms | 4.0 ms | 86.0 ms (`.mcp.json`'s call) |
| inside the server, `library_search` of three words | 0.7 ms | 0.7 ms | — |
| inside the server, `library_get` of a stored page | 7.2 ms | 6.6 ms | — |
| inside the server, `library_read` of three sections | 7.2 ms | 6.6 ms | — |
| the command line's `search`, `get`, `read`, `list`, `stats` | 104.0 to 104.5 ms | 5.7, 11.3, 11.3, 5.7, 7.4 ms | 84.2 to 92.7 ms |

- **The binaries**: `library-mcp` 5.4 MB, `library` 2.6 MB; a cold release build of the crate and its tree 24.6 s (95 s of CPU on 10 cores).
- **The launcher costs the gain**: `.mcp.json` and `npm run library` start the binaries through `cargo run`, as ADR-0073 asks, which adds about 80 ms, Cargo's own start and freshness check; the binary alone starts in 4 ms. This is EN-104's question to the Product Owner (note 0025, For the Product Owner). The first `cargo run` after a change of the crate rebuilds it before the server answers.
- **Inside the server the times are SQLite's**: a page's sections are read by `url`, an `UNINDEXED` column of the FTS5 table, so each `get` and `read` scans the 11,369 sections of the store, about 6 ms in either language; the schema is kept as it is (criterion 2).

## Tasks

One task = one commit, referenced as `EN-106.Tn`.

- [x] T1 — The crates recorded as SP-023 chose them: `rmcp` and its tree, `rusqlite` bundled except for Windows; versions checked again, the crates under 7 days pinned back, licences, advisories; the Windows check (2 h)
- [x] T2 — The store in Rust: the same schema, the view and its trigger, `project_uses`; tested on a copy of the real store and on a fresh one (4 h)
- [x] T3 — Pages, HTML, chunks and usage in Rust, on their TypeScript tests' cases (6 h)
- [x] T4 — The server on `rmcp` and the command line; `.mcp.json`, `package.json`, the docs; the TypeScript files removed; times measured (5 h)

## For the main thread

The evidence, on 2026-10-18, each on copies of the real store (`VACUUM INTO`), `LIBRARY_PATH` set to `/usr/local/lib` for the Rust binaries and `HOME` to an empty scratch folder:

- **The same answers**: 85 tool calls sent to both servers (the 40 latest searches of the store, 25 gets and 15 reads of stored pages, a limit, an empty query, an unknown tool name): 85 answers the same, text and `isError`; `tools/list` the same names, descriptions and schemas. Seven gets online with `refresh` (SQLite, Claude Code, GitHub, the MCP specification, a raw Markdown page, a name that does not resolve, a 404): the same tables of contents and stored pages, the same uses recorded (`online`, `failed`); only the error text of the name that does not resolve differs, `curl`'s message for `fetch`'s.
- **The same command line**: `search`, `get`, `read`, `list`, `stats`, `stats <since>` and an unknown command print the same output and exit with the same code; the TypeScript `stats` reads the file the Rust server wrote and prints the same report as on its own copy.
- **The same cut**: the 431 pages of the store cut again by `page_chunks` give their 11,369 stored sections; 20,000 generated pages give the same Markdown and the same facts from both converters.
- **Decisions taken**: `rmcp` without `macros` (the tools declared by hand keep their schemas); `serde` not taken (no derive needed); `serde_json` held at 1.0.152, the crates under 7 days pinned back by their publication time; the variable named `AGENT_LIBRARY_FILE`; a sixth command, `library path`, so that `records:save` and `metrics` read the library through the binary and every TypeScript twin goes; each call of the server recorded as a use of `library-mcp` (EN-102); ADR-0077 written, ADR-0039 marked partly superseded: its status says accepted from F21's decisions, to be changed to proposed if the Product Owner wants to confirm the variable's name.
- **Rule docs**: none affected, the story changes tools only.

### Improvements

| 2026-10-18 | EN-106 | The library reads a page's sections by `url`, an `UNINDEXED` column of the FTS5 table `chunks`, so each `library_get` and `library_read` scans the store's 11,369 sections, about 6 ms of the 6.6 ms a call takes; a table of sections keyed by page, beside the index, would make it an index lookup, at the cost of a change of schema an older checkout must still read | tool |
| 2026-10-18 | EN-106 | A measure that sets `HOME` to a scratch folder for a command run through `cargo run` makes rustup install the whole toolchain there (1.6 GB, 41 s for the first call): a timer script keeps `HOME` and names the library's file by its variable | tool |
