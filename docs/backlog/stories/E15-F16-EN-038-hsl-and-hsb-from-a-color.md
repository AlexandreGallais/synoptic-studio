---
id: EN-038
epic: E15
feature: F16
title: HSL and HSB from a hexadecimal color, and back
status: done
points: 2
---

# E15 · F16 · EN-038 — HSL and HSB from a hexadecimal color, and back

The library computes a color's hue, saturation and lightness (HSL) or brightness (HSB) from its hexadecimal, and the hexadecimal from them, exactly on the round trip from the hexadecimal; HSL and HSB are views, never stored (ADR-0056, `DERIV-color-spaces`, steps 3 to 9).

## Acceptance criteria

- Given `#ff8000`, when converted, then HSL is 30.11765°, 100 %, 50 % and HSB 30.11765°, 100 %, 100 % (`g = 128 / 255`, `H = 60 · g`); given `#336699`, then HSL is 210°, 50 %, 40 % and HSB 210°, 66.66667 %, 60 % (`REF-CSS-COLOR-4`, §7.2; `REF-WIKI-HSL-HSV`, From RGB).
- Given a grey, `#808080`, when converted, then the hue is 0, the saturation 0 and the lightness and brightness 50.19608 %; given `#000000` and `#ffffff`, then the saturation is 0 and the hue 0.
- Given HSB 210°, 66.7 %, 60 %, when converted back, then the color is `#336699`; given HSB 210°, 67 %, 60 %, then `#326699` (`f(5) = 0.198`, `50.49` rounds to 50); given HSL 30°, 100 %, 50 %, then `#ff8000`, the green 127.5 rounded up (`REF-CSS-COLOR-4`, §16.1.1).
- Given `#ff8000`, when its values are rounded to one decimal, then HSL is 30.1°, 100 %, 50 % and HSB 30.1°, 100 %, 100 %; given a hue that rounds to 360.0, then it is 0.0.
- Given every one of the 16 777 216 colors, when converted to HSL and to HSB and back, unrounded and with the values rounded to one decimal by the library's own rounding, then each gives its hexadecimal again (F16.AC3; one test over every color, within Vitest's timeout; otherwise a property test and the derivation's count, said in the story).

## Notes

- **Every color, once.** The round trip over the 16 777 216 colors, with the library's functions, unrounded and to one decimal, in HSL and in HSB, ran once at EN-038 and changed no color; at about 13 to 17 s a run, it is too slow for each test run (Vitest's timeout is 5 s, and the loaded pre-push run is about 2.5 times slower). The tests run a sample instead, one color in 31 and every color with two equal components (about 735 000 colors), each round trip, unrounded or shown, a test of its own of about 1.3 to 1.8 s, about 3 s for each model under coverage, with a timeout of 20 s; a property test draws any color (`DERIV-color-spaces`, cross-check).
- **The rounding.** A byte, a value shown and the alpha's percent and byte are rounded by one function, `nearestInteger` (`Math.round`, `REF-MDN-MATH-ROUND`, CSS's tie toward +∞, `REF-CSS-VALUES-4` §5.2); the rounded quotient of EN-037, exact for integers only, is folded into it as `nearestInteger(n / d)` (`DERIV-color-spaces`, steps 2 and 8).

## Tasks

One task = one commit, referenced as `EN-038.Tn`.

- [x] T1 — The hue and chroma of three bytes, a grey's hue 0, and HSL from the bytes, `@see REF-CSS-COLOR-4`, `REF-WIKI-HSL-HSV`, `DERIV-color-spaces` steps 3 and 4; tests first, the checks of the derivation (2 h)
- [x] T2 — HSB from the bytes; HSL and HSB back to the bytes, each rounded to the nearest, a tie up, and to the stored hexadecimal, `@see DERIV-color-spaces` steps 5 to 8; tests first (2 h)
- [x] T3 — The values rounded to one decimal, the hue 360.0 written 0.0, `@see DERIV-color-spaces` step 9; the round trip over every color, unrounded and to one decimal; property tests (2 h)
