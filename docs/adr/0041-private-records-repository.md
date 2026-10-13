# ADR-0041 — Private records in a sibling repository, saved by `npm run records:save`

**Status**: Accepted (Product Owner, 2026-10-13). Decision 1, the place beside the project, superseded by ADR-0044.

## Context

Some files must never be public and never be lost: the agent's metrics (`.git/metrics/tool-calls.jsonl`, `turns.jsonl`), Claude Code's transcripts of the project (`~/.claude/projects/<project-path>/`), the source library (`library.sqlite`, ADR-0039). They hold the Product Owner's raw messages and local paths, so they stay out of this public repository; today they live on one disk only. The Product Owner asked for a private Git repository attached to the project, more such files to join it later.

| Option                              | What the public repository shows                                                                          | CI                                                                               | Simplicity                                                                                                                                   |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Git submodule                       | a `.gitmodules` file with the private URL and a commit id that moves at every save (`REF-GIT-SUBMODULES`) | none if `actions/checkout` keeps its default (`REF-GH-CHECKOUT`), else a failure | a commit in the public repository after each save; a clone shows an empty folder; the files are still copied in, their sources are elsewhere |
| A folder inside, ignored by Git     | its name in `.gitignore`                                                                                  | none                                                                             | every tool that walks the tree (secretlint, Prettier, ESLint, knip, VitePress) must skip 140 MB of transcripts                               |
| **A sibling repository, outside**   | nothing but the script and its default folder name                                                        | none                                                                             | one script copies and commits; nothing to ignore                                                                                             |
| An orphan branch of this repository | the branch itself: it would be public                                                                     | —                                                                                | rejected                                                                                                                                     |

## Decision

1. The records live in a **separate private repository**, cloned **beside** this one (`../synoptic-studio-records`, or `RECORDS_PATH`), with a private GitHub remote so that losing the disk loses nothing. Nothing in this repository points to it but the script.
2. **`npm run records:save`** (`scripts/records-save.ts`) copies each record, commits when something changed and pushes. Files and folders are copied over, never mirrored: a transcript deleted by Claude Code stays saved. The library is copied with `VACUUM INTO`, a consistent snapshot even while the MCP server has it open (`REF-SQLITE-VACUUM`).
3. A new private file joins by one line in `recordCopies`, its test and the records repository's README.
4. It runs at the end of every session (with the journal entry) and at every retrospective.

## Consequences

- The public repository and its CI do not change; a clone of it does not see the records.
- A new machine clones the records repository beside the project to restore them (its README says where each folder goes).
- The library is a binary file: each save that changed it adds a version (16 MB before compression). If the records repository passes 1 GB, save a text dump instead.
- Transcripts of sessions run from a worktree outside the project folder are under another project folder: not saved yet, as for the metrics (`docs/tooling/metrics.md`).

## References

`REF-GIT-SUBMODULES`, `REF-GH-CHECKOUT`, `REF-SQLITE-VACUUM`, ADR-0024, ADR-0039
