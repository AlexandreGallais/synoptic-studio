# ADR-0024 — Session journal and transcript search instead of a database

**Status**: Accepted, partly superseded by ADR-0039 (a database for the pages read online)

## Context

Conversations with the Product Owner are long and get compacted; decisions end up in the repository, but the path to them (what was asked, discussed, refused) is scattered. The Product Owner suggested a local database where the agent stores everything it may need. Long-running agents keep coherent through structured notes outside the context window, reloaded just in time (`REF-ANTHROPIC-CONTEXT`). Claude Code already stores every conversation locally as a JSONL transcript.

## Decision

1. A **session journal**, `docs/process/journal.md`: per session, the requests (paraphrased), decisions and where they were recorded, with `#tags` for `grep`. Written by the agent at the end of a session or before a pause.
2. The **transcripts** are the full archive: searched with `grep` / `jq` when the journal and the repository are not enough (`docs/tooling/agent.md`).
3. No database (SQLite, MCP memory server): it would duplicate the repository, not be reviewed in pull requests, and need its own tooling; Markdown and `grep` are what the agent reads fastest.

## Consequences

- One file to skim after a compaction, besides the session-start hook.
- The journal is public like the rest of the repository: no personal data, only project matters.

## References

`REF-ANTHROPIC-CONTEXT`, ADR-0021, ADR-0023
