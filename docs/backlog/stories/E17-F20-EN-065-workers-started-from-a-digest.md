---
id: EN-065
epic: E17
feature: F20
title: Workers started from a digest instead of all of CLAUDE.md
status: draft
points: 2
---

# E17 · F20 · EN-065 — Workers started from a digest instead of all of CLAUDE.md

Start each worker from a short digest of the rules it needs rather than the whole of `CLAUDE.md`, as the research of 2026-10-16 recommends; the change is shown to the Product Owner before it is applied. It builds on the worker brief and the ADR digest of F20.

## Acceptance criteria

- Given a worker started from the digest, when it works a story, then it follows the same rules as with the full files, checked on one story.
- Given the tokens of a worker's input, when measured before and after, then the figures are recorded.

## Tasks

One task = one commit, referenced as `EN-065.Tn`.

- [ ] T1 — Write the digest and show it to the Product Owner (2 h)
- [ ] T2 — Start workers from it and measure (2 h)
