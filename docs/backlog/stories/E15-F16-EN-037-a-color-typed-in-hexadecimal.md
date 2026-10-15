---
id: EN-037
epic: E15
feature: F16
title: A color typed in hexadecimal, kept as six digits and an opacity
status: ready
points: 2
---

# E15 · F16 · EN-037 — A color typed in hexadecimal, kept as six digits and an opacity

The library reads a color typed or pasted in any hexadecimal form CSS knows and gives its stored form, `#` and six lower-case digits, with the opacity its alpha gives, if any; the check of a stored color is shared by every color of the model (ADR-0056, `DERIV-color-spaces`, steps 1 and 2).

## Acceptance criteria

- Given `#FF8000`, `ff8000`, or `#ff8000` with spaces before and after it, when read, then the color is `#ff8000` and no opacity is given (`REF-CSS-COLOR-4`, §5.2 and §16.2.1).
- Given `#f80`, when read, then the color is `#ff8800`; given `#F80C`, then `#ff8800` and 80 %; given `#0000ffcc`, then `#0000ff` and 80 % (CSS's own example); given `#000000ec`, then `#000000` and 93 % (`⌊(200 · 236 + 255) / 510⌋ = 93`).
- Given `#ff800` (5 digits), `#ff80000` (7), `#gg0000`, an empty text, `red` or `rgb(255, 128, 0)`, when read, then it is refused (no CSS names, no RGB, F05 refinement).
- Given an opacity of 50 %, when written as an alpha byte, then it is 128 (`⌊(255 · 50 + 50) / 100⌋`), not 127 as `50 × 2.55` in binary64 would give; given each percent from 0 to 100, when written as a byte and read back, then it is the same percent (a test over the 101 values).
- Given a text whose color or background color is `#FF8000`, as files written before F16 hold, when checked, then it is still valid, by the same check as every other color of the model.

## Tasks

One task = one commit, referenced as `EN-037.Tn`.

- [ ] T1 — A color folder in the model (ADR-0043): the check of a stored color, shared by the text's color and background (`isTextColor` becomes the color's check, its callers updated), the byte of a pair of hexadecimal digits, `@see REF-CSS-COLOR-4`; tests first (2 h)
- [ ] T2 — A typed color read into its stored form and an opacity or none: `#` optional, any case, white space around ignored, 3, 4, 6 or 8 digits, refused otherwise, `@see REF-CSS-COLOR-4`, `DERIV-color-spaces` step 1; tests first, then a property test: a stored color read again is itself (2 h)
- [ ] T3 — The opacity from an alpha byte and the byte from an opacity, in integers, `@see DERIV-color-spaces` step 2; tests first, the 101 percents (1 h)
