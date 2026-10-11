---
id: SP-006
epic: E01
feature: F09
title: Research for F09
status: ready
points: 2
---

# E01 · F09 · SP-006 — Research for F09

Find, read and record the sources of every story of F09 before its implementation (ADR-0020).

## Acceptance criteria

- Given each story of F09, when the spike ends, then every formula and behavior it needs has a verified `REF-*` or a `DERIV-*`.
- Given the sources found, when the spike ends, then the stories cite them, and a research note records what was found and what remains open.

## Questions

- Stretching points from a fixed corner of their box: the scaling formula and a source (SVG transforms, a scale about a point); how to keep it without loss when a size goes to 0 (points kept in their own box).
- The rectangle becoming a free shape: an ADR (the model of every shape), what happens to `Rectangle`, `rectangleCorners` and their tests.
- Design tools: resizing by typed width and height (which corner stays), as a functional reference.

## Tasks

One task = one commit, referenced as `SP-006.Tn`.

- [ ] T1 — Sources, derivation of the stretch if no source states it, ADR (2 h)
- [ ] T2 — Research note, stories updated (1 h)
