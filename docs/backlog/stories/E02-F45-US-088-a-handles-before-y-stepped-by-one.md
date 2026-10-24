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

## Cause seen

Reproduced on 2026-10-24, in happy-dom with the keys a browser sends (focus, `keydown`, `keyup`; the handles typed by `typeInField`) and in Chromium's headless shell with Playwright's real keys on `npm run dev`: not Before y itself, but every field of its own showing a negative number. Point 1 of the rectangle given handles, after (30, 15), before mirrored at (−30, −15): Up in Before y showed −29, −57, −113, then Shift + Up −216, the value doubling at each key; Up in a point's Y at −5 gave −9, −17, −33. Before x, After x and After y stepped right because they showed positive numbers; Before y was the one negative, a handle before going up and left of its point, and 10 keys doubling −15 reach about 15 000.

The arrow's step is applied twice (`applySteppedValue`, `playground/number-fields/listen-to-number-fields.ts`): the page shows the value stepped within its range (`writeOwnValue`), −15 + 1 = −14, then sends the field a `change`, so that its own listeners read it as Enter does; but that `change` is first heard by the listener of a value typed (`applyTypedValue`), which reads the text again as typed (`applyOwnText`, `parsePanelValue`), and `-14`, with its sign, is a relative value: added to the −15 shown before the key, it gives −29. A positive value is read as a fixed one, the same value again, so that the fields AUD-046 tested, the radius, the Angle and the opacities, never showed it. With Shift, −15 + 10 = −5 read as relative gives −20.

The fix: the `change` an arrow sends a field of its own carries its value already applied, which the listener of a value typed leaves alone; a `-14` typed and applied by Enter stays a relative value, as the rule says (`docs/domain/interaction.md` §2, Every number field, the same rule).

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [x] T1 — The bug reproduced (each method and mode, Up and Shift + Up in each handle field, the point's X and Y, a point on a curve), its cause found and written; tests of the criteria first, failing (1 h)
- [ ] T2 — The fix, the tests passing; the Product Owner test card (1 h)

One task = one commit, referenced as `US-088.Tn` in the commit footer.
