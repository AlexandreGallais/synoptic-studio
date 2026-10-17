---
id: EN-101
epic: E17
feature: F20
title: Every page fetched through the library, with the project recorded
status: draft
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

- [ ] T1 — The project in the library's `uses` table and its stats (2 h)
- [ ] T2 — The hook sending pages to the library, its tests, `docs/tooling/sources.md` updated (2 h)
