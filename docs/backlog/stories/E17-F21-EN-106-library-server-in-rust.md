---
id: EN-106
epic: E17
feature: F21
title: The library server and its command line in Rust on rmcp
status: draft
points: 8
---

# E17 · F21 · EN-106 — The library server and its command line in Rust on rmcp

The `library` MCP server (`scripts/library-mcp.ts`) and its command line (`library-cli.ts`), with their store, pages, HTML, chunks and usage modules, rewritten as one Rust crate on `rmcp`, the official Rust SDK of the Model Context Protocol, over `stdio` (`REF-RMCP`; 3.5.1 or later, at least 7 days old), and a SQLite crate with FTS5 full-text search (`REF-SQLITE-FTS5`), checked as every crate is: version at least 7 days old, licence, advisories (F21's decisions of 2026-10-17; F21.AC3). Today the server starts Node.js at each session and reads `node:sqlite` (`REF-NODE-SQLITE`); its start was not measured ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), Everything around the library). Before the metrics tools (EN-107), whose Sources section reads the same store.

The store is shared by every project on the machine (`~/.local/share/agent-library/library.sqlite`): EN-101 recorded each use with its project in `project_uses`, kept `uses` as a view of it for an older copy of the scripts, and made the `library-pages` hook read the store by `sqlite3 -readonly` ([sources](../../tooling/sources.md)). The port keeps all of it.

## Acceptance criteria

- Given the tools `library_search`, `library_get` and `library_read`, when the Rust server answers the same calls as the TypeScript one on a copy of the store, then its answers are the same, text for text, and `.mcp.json` starts the binary.
- Given the store as the TypeScript server leaves it — `pages`, the FTS5 table `chunks`, `project_uses`, the view `uses` and its trigger — when the Rust server opens it, then it reads and writes it without a migration, and a file it writes is read again by the TypeScript server of an older checkout.
- Given each search and each page read, when the Rust server serves it, then one `project_uses` row is written with the project as `projectOf` names it (EN-101), and `npm run library -- stats` gives the same report as before.
- Given the `library-pages` hook, when a page is refused or stored with fewer than 300 characters, then it behaves as before on its tests (`tools/hooks/tests/pages.rs`).
- Given `npm run library -- search | get | read | list | stats`, when each runs, then its output is the same as the TypeScript command line's.
- Given a page fetched online, when the server reads it, then the HTML is cut into the same sections and chunks as `library-html.ts` and `library-chunks.ts` cut it, on their tests' cases.
- Given each crate the story adds (`rmcp` and the crates it pulls, the SQLite crate), when the story ends, then each has its row in `docs/tooling/dependencies.md` (Crates) with its version, age, licence and release notes read, `cargo deny` finds no advisory, and `NOTICE` lists any package whose code is ported (ADR-0076).
- Given the server and the command line, when they are timed, then the server's start to its first answer, and one search, one get of a stored page and one read, are stated before and after.

## Files

`tools/library/` (new crate: the server and the command line), `Cargo.toml` (`[workspace.dependencies]`), `Cargo.lock`, `deny.toml` if a licence needs allowing, `.mcp.json`, `package.json` (`library`), `scripts/library-*.ts` and their tests (removed), `tools/hooks/src/pages.rs` if it shares the store's reading, `docs/tooling/sources.md`, `docs/tooling/dependencies.md`, `docs/tooling/agent-tools.md`, `NOTICE`, `THIRD-PARTY-LICENSES.md` if code is ported.

## Measures (ADR-0069)

Before and after, 20 calls each on this Mac: the server's start to its answer to `initialize`; `library_search` of three words; `library_get` of a stored page; `library_read` of three sections; the binary's size and its first build time.

## Tasks

One task = one commit, referenced as `EN-106.Tn`.

- [ ] T1 — The crates chosen and recorded: `rmcp` and its tree, the SQLite crate with FTS5; versions, licences, advisories (2 h)
- [ ] T2 — The store in Rust: the same schema, the view and its trigger, `project_uses`; tested on a copy of the real store and on a fresh one (4 h)
- [ ] T3 — Pages, HTML, chunks and usage in Rust, on their TypeScript tests' cases (6 h)
- [ ] T4 — The server on `rmcp` and the command line; `.mcp.json`, `package.json`, the docs; the TypeScript files removed; times measured (5 h)
