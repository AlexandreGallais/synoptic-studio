---
id: EN-140
epic: E17
feature: F33
title: The anchors, the rows and the agent pages checked
status: ready
points: 2
---

# E17 · F33 · EN-140 — The anchors, the rows and the agent pages checked

From RET-004's "Checks to add" (P8), split from EN-121 on 2026-10-18 (SP-024): one anchor slug for GitHub and the site, `move-rows` naming the notes it leaves, `backlog:index --check` reaching the index tool and not the formatter, a test refusing `npm run <binary>` in the agent's pages (I99, a fitness function still only written at RET-004, §3).

## Acceptance criteria

- Given a link to a heading's anchor in a Markdown page, when the test computes the heading's slug as GitHub does and as the site does, then both are the anchor linked; a heading whose two slugs differ fails, naming the page and the heading.
- Given a story file whose `## For the main thread` holds lines `move-rows` does not move, when it runs, then it prints each such line with its story, and the `done` commit's hook shows them.
- Given `backlog:index --check`, when it runs, then it runs the index tool's own check and not the formatter, and reports the first row that differs.
- Given the agent's pages (`AGENTS.md`, `CLAUDE.md`, `.claude/`, `docs/tooling/agent-tools.md`), when a test reads them, then none names a binary of `target/release/` through `npm run` (ADR-0082).

## Tasks

- [x] T1 — Each criterion's test, failing (1 h)
- [ ] T2 — The slug, `move-rows`'s notes, the index check, the pages fixed (2 h)

One task = one commit, referenced as `EN-140.T1` in the commit footer.
