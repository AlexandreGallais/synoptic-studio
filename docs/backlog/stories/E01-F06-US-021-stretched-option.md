---
id: US-021
epic: E01
feature: F06
title: Stretch a polygon to its box in the playground
status: ready
points: 2
---

# E01 · F06 · US-021 — Stretch a polygon to its box in the playground

As a symbol designer, I want to tick « Stretched » on a polygon, so that it fills its whole box, every side touched, while staying a polygon whose sides stay as even as possible.

## Acceptance criteria

- Given a polygon, when « Stretched » is ticked, then it fills its box, touching its four sides, and the anchor picker is disabled; unticked, it is the polygon of F02 placed by its anchor again (F06.AC1, F06.AC5).
- Given a stretched polygon, when its number of corners, width, height, radii or angle change, then it stays stretched, and the effective radius of each corner is shown (F06.AC3, F06.AC5).
- Given a stretched polygon, when it is converted to a free shape, then it does not move (F06.AC4).

## Product Owner test

Written with the story, once SP-009 has settled the rule and its values.

## Tasks

One task = one commit, referenced as `US-021.Tn`.

- [ ] T1 — « Stretched » check box, anchor picker disabled, effective radii; tests (1.5 h)
- [ ] T2 — Product Owner test card (0.5 h)
