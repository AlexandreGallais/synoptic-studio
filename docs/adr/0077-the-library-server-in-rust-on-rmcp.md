# ADR-0077 — The library's server in Rust on rmcp, its file named by its own variable

**Status**: Accepted (Product Owner, 2026-10-17, F21's decisions: the server in Rust on `rmcp` and a SQLite crate with full-text search; the variable's rename is EN-106's criterion, from SP-023). Partly supersedes ADR-0039, decisions 1 (the variable naming the file) and 4 (the server, written without the SDK).

## Context

- ADR-0039 made the library a SQLite file behind an MCP server, `scripts/library-mcp.ts`, written without the MCP SDK, Node.js starting at each session and reading the file through `node:sqlite` (`REF-NODE-SQLITE`); the file was named by `LIBRARY_PATH`, else `~/.local/share/agent-library/library.sqlite`.
- F21 moves every tool of the project to Rust (ADR-0073), the library's server included, on `rmcp`, the official Rust SDK of the protocol (`REF-RMCP`), and on a SQLite crate with FTS5 (`REF-RUSQLITE`, `REF-SQLITE-FTS5`).
- SP-023 found that `LIBRARY_PATH` is the C toolchain's library search path: Apple's `python3` sets it to `/usr/local/lib` for the processes it starts, and a library script started from it opened or created a store at that value ([research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md), The trial).
- The store is shared by every project of the machine, an older checkout's scripts included: its schema cannot change (EN-101's `project_uses`, the view `uses` and its trigger).

## Decision

1. **The server and the command line are the Rust crate `tools/library`**: the binary `library-mcp` on `rmcp` over `stdio`, declared in `.mcp.json` and run by `cargo run --release -q --manifest-path … -p library --bin library-mcp`; the binary `library` behind `npm run library`. The three tools keep their names, descriptions, schemas and answers, text for text; the tools are declared by hand, without `rmcp`'s macros, so that their schemas stay those `tools/list` gave.
2. **The store is read and written by `rusqlite`**, SQLite compiled into the binary except for the Windows check of `check:rust`; the schema is the one the TypeScript scripts wrote, with no migration.
3. **A page is fetched by the `curl` command**, HTTP and HTTPS only: an HTTP crate's TLS compiles C and fails the Windows check (0025).
4. **The file is named by `AGENT_LIBRARY_FILE`**, else `~/.local/share/agent-library/library.sqlite`; `LIBRARY_PATH` is never read, by the server, the command line nor the `library-pages` hook.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| `rmcp` and `rusqlite` (chosen) | the server answers `initialize` in milliseconds instead of Node.js's start; the SDK follows the protocol's versions | about 67 crates in the lock, each checked (`docs/tooling/dependencies.md`) |
| The protocol written by hand in Rust, as the TypeScript did | no SDK | the Product Owner chose the SDK (F21, 2026-10-17) |
| Keep `LIBRARY_PATH` | no rename | a process started by Apple's `python3` opens a store at `/usr/local/lib` |

## Consequences

- An older checkout's scripts still read and write the same file, and still read `LIBRARY_PATH`: on this machine neither variable is set, so both find the default file.
- A fetch that gives no answer returns `curl`'s message instead of `fetch`'s; it is recorded as `failed` as before.
- The metrics report and the records' save read the library through the command line (`stats`, `path`) until EN-107 and EN-103 port them.

## References

`REF-RMCP`, `REF-RUSQLITE`, `REF-SQLITE-FTS5`, `REF-NODE-SQLITE`, `REF-MCP-TRANSPORTS`, `REF-MCP-TOOLS`, ADR-0039, ADR-0073, EN-106, [research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md)
