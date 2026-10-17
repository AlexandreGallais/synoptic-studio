---
id: EN-101
epic: E17
feature: F20
title: Every page fetched through the library, with the project recorded
status: done
points: 2
---

# E17 · F20 · EN-101 — Every page fetched through the library, with the project recorded

`WebFetch` takes about 6 s and returns a summary written by a small model, which cannot be cited; the library serves a stored page in 12 to 265 ms and keeps its text. From 2026-10-14 to 2026-10-17, 58 `WebFetch` and 32 `curl` calls went around it ([research note 0024](../../research/notes/0024-f20-context-diet.md), `REF-CC-TOOLS-REFERENCE`). Asked by the Product Owner on 2026-10-17: every page fetched goes through the library, which records which project visited each page; chosen for F20 at its refinement.

## Acceptance criteria

- Given a `WebFetch` or a `curl` of a web page, when the agent calls it, then a `PreToolUse` hook refuses it with the `library_get` call to make instead, unless the library returned that page empty, which the hook reads from the library.
- Given each search, get and read of the library, when it is recorded, then its `uses` row holds the project that asked, and `npm run library -- stats` groups by project.
- Given the hook, when it is built, then it is a crate of the workspace with its tests; the library's change is made in the TypeScript server or in its Rust version, whichever F21 has left.

## Tasks

One task = one commit, referenced as `EN-101.Tn`.

- [x] T1 — The project in the library's `uses` table and its stats (2 h)
- [x] T2 — The hook sending pages to the library, its tests, `docs/tooling/sources.md` updated (2 h)

## Notes

- **Settings**: one entry added to `.claude/settings.json`, the `PreToolUse` hook `library-pages` on `WebFetch|Bash`, nothing else; the Product Owner authorised the settings changes the stories need on 2026-10-17.
- **The library's file stays shared** by every project of the machine (ADR-0039): the uses move into a table `project_uses` with the project; `uses` stays as a view of it with an insert trigger, so that an older copy of the scripts — the running MCP server of another checkout — keeps writing to the migrated file, its uses without a project. Tried on a copy of the machine's library: 84 uses of 2026-10-17 moved, the scripts of `develop` still searching and recording.
- **The project** is the name of the project's folder, the same for each of its worktrees (`/…/SVG editor/.claude/worktrees/en-101` → `SVG editor`): from `CLAUDE_PROJECT_DIR`, else the folder the server starts in.
- **Empty page**: fewer than 300 characters of Markdown (8 of the 417 pages stored on 2026-10-17, from 0 to 235 characters), or a page its site refused to the library and never stored.

## Measures

| Reading a stored page | Before (`WebFetch`) | After (refused, then `library_get`) |
| --- | --- | --- |
| time of the tool | 6.0 s (research note 0024) | hook 0.04–0.05 s, `library_get` 0.10–0.18 s (`fts5.html`, stored) |
| what comes back | a summary of 1,484 characters, not citable | a table of contents of 9,747 characters, then the sections asked (§37: 3,044 characters), the page's own text |
| refusal | — | one model round trip (about 3 s, note 0024), once per page the agent tries to fetch |

The hook's own time is the same on a shell command it lets through (0.04–0.05 s), run beside `guard-bash` (0.05 s).
