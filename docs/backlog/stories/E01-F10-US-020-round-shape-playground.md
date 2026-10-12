---
id: US-020
epic: E01
feature: F10
title: Draw and convert a round shape in the playground
status: ready
points: 2
---

# E01 · F10 · US-020 — Draw and convert a round shape in the playground

As a symbol designer, I want to choose a round shape and type its size, so that I get a circle or a capsule that stays round whatever its size, without a radius to keep at its maximum.

## Acceptance criteria

- Given the shape selector, when « Round » is chosen, then a circle or a capsule fills the box typed, and the radius field and the anchor grid are hidden (F10.AC4, F10.AC1).
- Given a round shape, when its angle or its position is typed, then it turns and moves as every shape (F10.AC2).
- Given a round shape in Points mode, when « Convert to free shape » is clicked, then it becomes a free shape of 4 points with their radii, drawn the same (F10.AC3, F10.AC4).

## Product Owner test

Written when the story is done (ADR-0028).

## Tasks

One task = one commit, referenced as `US-020.Tn`.

- [ ] T1 — « Round » in the selector, fields shown and hidden, conversion; the tests of `"anchor" in model` replaced by the shape's kind (note 0010); tests (1.5 h)
- [ ] T2 — Product Owner test card (0.5 h)
