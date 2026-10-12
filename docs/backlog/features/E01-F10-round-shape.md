---
id: F10
epic: E01
title: Round shape
status: draft
---

# E01 · F10 — Round shape

**Benefit hypothesis**: the designer draws a circle, or a capsule, in one step, without typing a radius that must stay at its maximum whatever the size.

**Product Owner's idea (2026-10-12, after the F04 run)**: « on n'a pas fait la forme cercle \[…\] peu importe si on l'agrandit, les bords de radius sont tout le temps à fond »; a rectangle with radii at 100 000 draws a circle but does not say it is one. « Fais les formes circulaires, du coup, plutôt, pas forcément un cercle. »

## Decided with the Product Owner (2026-10-12)

- **Circle and capsule**: in a square box the round shape is a circle; in a wider or taller box, a capsule (two half circles joined by straight sides). Only circular arcs (ADR-0001); the ellipse stays out of scope (Q4 open).
- **Edited as the polygon**: the round shape has its width, height, angle and position, but no radius field and no points. **No anchor**: it always fills its box, touching its four sides (« il n'y a pas vraiment d'ancrage, parce qu'elle va forcément toucher tous les côtés tout le temps »). The « Convert to free shape » button gives a free shape of 4 points whose radii are half the smaller side, rounded up to an integer (120 × 80 gives 40; 81 × 81 gives 41, clamped to 40.5 when drawn, ADR-0007): it stays round until stretched, and its points and radii can then be edited (F08). Its points are numbered as the rectangle's, point 1 at the top left, clockwise: the order of the path's `d`, never changed afterwards (Product Owner, 2026-10-12).
- **Order**: in E01 right after F04, before F05 (text) and F06; F04's validation first.

## Acceptance criteria

To be written in refinement.

## Stories

To be written in refinement, after F04.
