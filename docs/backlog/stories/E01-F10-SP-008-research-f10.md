---
id: SP-008
epic: E01
feature: F10
title: Research for F10
status: ready
points: 1
---

# E01 · F10 · SP-008 — Research for F10

Find, read and record the sources of every story of F10 before its implementation (ADR-0020).

## Acceptance criteria

- Given each story of F10, when the spike ends, then every formula and behavior it needs has a verified `REF-*` or a `DERIV-*`.
- Given the sources found, when the spike ends, then the stories cite them, and a research note records what was found and what remains open.

## Questions

- The capsule as a rectangle whose corners are rounded by half its smaller side: does `DERIV-local-radius-clamp` give it exactly, the arcs meeting at the middle of the short sides and leaving the long sides straight?
- A name and a source for the capsule (stadium) shape, and how design tools draw it (Figma's rounded rectangle at maximal radius, SVG's `rect` with `rx`).
- The model: a round shape of its own, how the code tells it from a free shape and a regular polygon; an ADR if the model of shapes changes.

## Tasks

One task = one commit, referenced as `SP-008.Tn`.

- [ ] T1 — Sources, derivation if needed, ADR (1 h)
- [ ] T2 — Research note, stories updated (0.5 h)
