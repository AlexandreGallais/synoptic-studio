# ADR-0039 — A local library of the sources read: SQLite full-text search behind an MCP server

**Status**: Accepted (Product Owner, RET-001, 2026-10-13). Partly supersedes ADR-0024, point 3 ("no database"), for the pages read online only: the session journal and the transcripts stay as they are.

## Context

Every spike and audit read the same documentation pages again (Claude Code, GitHub, SQLite, MDN, the SVG specification): each reading costs a network request, a summary by a small model, and tokens. The Product Owner asked for a local store the agent looks in first: a page missing or too old is fetched and stored; a page recently changed is checked again, as it may still be corrected. They asked for what retrieves fastest for the agent, not for files to read whole.

Options weighed:

| Option | Search | Cost |
| --- | --- | --- |
| Markdown files in a folder, searched with `rg` | exact words, no ranking | the agent reads whole files: many tokens |
| SQLite with FTS5 (full text, BM25 ranking) | words, stemmed, ranked, with snippets | built into Node.js (`node:sqlite`): no dependency |
| A vector database (embeddings, the "matrices") | by meaning | needs an embedding model: a runtime service or a large dependency, offline lost |

Sources read: FTS5 ranks by BM25, "the better the match, the numerically smaller the value", and returns snippets; the porter tokenizer matches word forms (`REF-SQLITE-FTS5`). Node.js 24 ships SQLite as `node:sqlite` with `DatabaseSync`, without a flag (`REF-NODE-SQLITE`). An MCP server over stdio is a process exchanging newline-delimited JSON-RPC messages; tools are listed by `tools/list` and run by `tools/call` (`REF-MCP-TRANSPORTS`, `REF-MCP-TOOLS`, `REF-MCP-LIFECYCLE`); a project declares its servers in `.mcp.json` (`REF-CC-MCP`).

## Decision

1. **One SQLite file per machine**, `~/.local/share/agent-library/library.sqlite` (`LIBRARY_PATH` to change it), shared by every project: never in a repository, since pages are other people's text.
2. **Pages are stored as Markdown and split at their headings**; each section is indexed with FTS5 (`porter unicode61`). A search returns the best sections with a snippet; a reading returns a table of contents, then only the sections asked.
3. **Freshness**: a page is read online again after 90 days; a page that says it changed less than 14 days before it was read is read again after one day; a request for a stored page is conditional (`If-None-Match`, `If-Modified-Since`).
4. **Access**: an MCP server, `scripts/library-mcp.ts`, declared in `.mcp.json`, with three tools — `library_search`, `library_get`, `library_read` — and the same functions on the command line (`npm run library`). Written without the MCP SDK: three methods and a loop.
5. **Rule for the agent**: search the library, then get a page through it; WebFetch only for a page the library cannot read (built in the browser), and its useful text is then summarized in the research note. The bibliography (`docs/references.md`) stays the list of sources cited; the library is a cache of their text.

## Consequences

- A page is fetched once per 90 days at most, and read by section: fewer tokens and fewer requests at each spike and audit.
- The library is local: a new machine starts empty and fills itself.
- The converter is simple: pages built by JavaScript in the browser come back nearly empty; WebFetch stays for them.
- Embeddings are left out: the trigger to reconsider is a search that words cannot answer, recorded in the improvement log.

## References

`REF-SQLITE-FTS5`, `REF-NODE-SQLITE`, `REF-MCP-TRANSPORTS`, `REF-MCP-TOOLS`, `REF-MCP-LIFECYCLE`, `REF-CC-MCP`, ADR-0024
