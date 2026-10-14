---
id: EN-026
epic: E14
feature: F12
title: Keep only the ways and modes chosen at the validation
status: done
points: 2
---

# E14 · F12 · EN-026 — Keep only the ways and modes chosen at the validation

At VAL-011 (2026-10-13) the Product Owner tried the seven methods and four modes and kept two methods and three modes (Q32). The others are removed from the library, the playground and the documentation, with what was there only for them.

## Acceptance criteria

- Given a point in handles mode, when its handles are typed, then only two methods are offered: x and y on the page, and an angle and a length from its side.
- Given the handle modes, when one is chosen, then only Mirror angle and length, Mirror angle and No mirroring exist; the stored handles hold no tension.
- Given the library, the playground and the docs, when searched, then nothing is left of dx and dy, the angle from the x axis, shares of the side, pasted curves and the tension: no function, field, test, derivation or domain rule.
- Given every other test, when run, then it passes, changed only where it used a removed method or field.

## Tasks

One task = one commit, referenced as `EN-026.Tn`.

- [x] T1 — Remove the methods and the mode from the library and the playground, with their tests (1 h)
- [x] T2 — Remove their derivations and rules; Q32 settled; a pass over the code for anything left only for them (1 h)

## Review

Auditor, asked for a full pass over the code after the removal (2026-10-13). Fixed: the angle derivation and `angledHandle` framed the x axis as a method; a reference still named dx, dy; the −0 fix written three ways now goes through `withoutNegativeZero`; a test that the lists offer exactly the methods and modes kept; a kept-angle test that typed no angle. Kept: the kept angle's owner names the way, so that an x typed on the page is never read back as an angle.
