---
id: US-025
epic: E14
feature: F12
title: "Type a side's handles as shares of the side, or paste curve data"
status: ready
points: 3
---

# E14 · F12 · US-025 — Type a side's handles as shares of the side, or paste curve data

As a symbol designer, I type a side's two handles as whole percentages of the side, as CSS `cubic-bezier()` does, or paste a `C` segment or a `cubic-bezier()` value, so that a curve I know is reused on any side.

## Acceptance criteria

- Given the side from (100, 0) to (200, 0), when the user types 25 % along and 30 % across for the first handle, then its end is at (125, −30) (a derivation).
- Given the text `cubic-bezier(0.42, 0, 0.58, 1)` pasted on the side from (100, 0) to (200, 0), then CSS's diagonal from (0, 0) to (1, 1) is laid on the side: the handles end at (121, 21) and (179, −21), the S shape of the easing (a derivation); x values outside 0 to 1 are refused, as CSS does.
- Given shares typed for the second handle, then they are measured from B along the side's reverse direction, so that equal shares at both ends give a symmetric arch.
- Given the text `C 130 -15 170 -15 200 0` pasted for the side from (100, 0), then its handles are set to those points; a `C` whose end is not B, or any other text, is refused with a line saying why; decimals pasted are kept, as stored handles are exact (ADR-0046).
- Given a point in a mirrored mode, when a pasted curve sets its handle, then the other handle of that point follows its mode, bending the neighbouring side too.

## Tasks

One task = one commit, referenced as `US-025.Tn`.

- [x] T1 — Derivation of a handle from shares of its side
- [x] T2 — The share fields; the parser of `C` and `cubic-bezier()`; tests

## Decisions during the story

| Decision                                                                                          | Reason                                                                                                                               |
| ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| shares are a fifth way of `HANDLE_WAYS`, typed in the same four fields                            | one set of fields for every way                                                                                                      |
| a paste applies to the side after the point selected                                              | the side has a start and an end: the `C`'s end must be the next point, the `cubic-bezier()` diagonal runs from the point to the next |
| a pasted curve switches both points to handles mode, each point's other handle following its mode | as typing a handle; the criterion asks it                                                                                            |

## Product Owner test

Open the page, type **Width** = 100: points 1 (0, 0), 2 (100, 0).

1. Click **Points**, point 1, **Handles**, then **Typed as** = **share of the side (%)**. The fields read along % and across %.
2. Type **After along** = 25, **After across** = 30: the top side bends upwards, the handle's end at (25, −30) — a quarter along the side, 30 % of its length above it.
3. In **Paste a curve on the side after**, type `C 30 -15 70 -15 100 0` and click **Apply**: the top side is that curve; point 1's other handle mirrors it, so the left side bends too.
4. Paste `cubic-bezier(0.42, 0, 0.58, 1)` and click **Apply**: the top side takes the S of CSS's ease-in-out.
5. Paste `C 30 -15 70 -15 90 0`: a line says the C must end on the side's last point; nothing changes.
