---
id: US-088
epic: E02
feature: F45
title: A handle's Before y stepped by one
status: ready
points: 2
---

# E02 · F45 · US-088 — A handle's Before y stepped by one

As a symbol designer, I want the arrows to step a point's handle fields by 1, or 10 with Shift, so that I place a handle as exactly as any other value.

Seen by the Product Owner on 2026-10-24 on F44's branch: a point given a handle, Before x, After x and After y step as they should; Up in Before y jumped by about 20 000 at a key, several times, the steps that lead to it not written down. The fields are number fields of `playground/number-fields/` since US-087 and AUD-046 (`playground/number-fields/own-field-ranges.ts`, `handle-before-y` unbounded); the handles are E14's (US-028 to US-031, `playground/points/`), their method (x, y on the page, or angle and length from the side) and their mode (mirror angle and length, mirror angle, no mirroring) changing what each field holds.

## Acceptance criteria

- Given a free shape's point with a handle, Before y showing v, when Up is pressed in Before y, then it shows v + 1, the handle 1 unit further; Shift + Up, v + 10; Down and Shift + Down the other way.
- Given each of the four handle fields, each method and each mode (mirroring included, where After follows Before), then an arrow steps the field it is pressed in by 1 or 10 and the other handle as the mode says, never by thousands.
- Given the cause found, then the test that reproduces it fails before the fix and passes after; the cause is written in the story.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The bug reproduced (each method and mode, Up and Shift + Up in each handle field, the point's X and Y, a point on a curve), its cause found and written; tests of the criteria first, failing (1 h)
- [ ] T2 — The fix, the tests passing; the Product Owner test card (1 h)

One task = one commit, referenced as `US-088.Tn` in the commit footer.
