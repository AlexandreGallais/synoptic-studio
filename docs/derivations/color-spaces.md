# DERIV-color-spaces — A color in hexadecimal, its opacity, and its HSL and HSB views

Used by: EN-037 (the hexadecimal forms and the opacity), EN-038 (HSL and HSB from the hexadecimal and back), US-035 (the picker's plane and the values it shows). Sources: `REF-CSS-COLOR-4` (§5.1, §5.2, §7, §7.1, §7.2, §16.1.1, §16.2.1), `REF-WIKI-HSL-HSV` (From RGB, HSV to RGB alternative). The stored color is six hexadecimal digits and an integer opacity (F16.AC1); HSL and HSB are computed from it and never stored (ADR-0056). This page shows that the round trip from the hexadecimal is exact, and which precision of the shown HSL and HSB values keeps it exact when they are typed back.

## Notation

| Symbol        | Definition                                                                        |
| ------------- | --------------------------------------------------------------------------------- |
| `R`, `G`, `B` | the red, green and blue bytes, integers from 0 to 255                             |
| `r`, `g`, `b` | the same as fractions: `r = R / 255`, from 0 to 1                                 |
| `M`, `m`      | `max(r, g, b)` and `min(r, g, b)`                                                 |
| `C`           | the chroma, `M − m`                                                               |
| `H`           | the hue, in degrees, from 0 included to 360 excluded                              |
| `S_L`, `L`    | HSL's saturation and lightness, as fractions from 0 to 1 (shown in percent)       |
| `S_V`, `V`    | HSB's saturation and brightness (HSV's value), as fractions (shown in percent)    |
| `p`, `A`      | the opacity in percent, an integer from 0 to 100, and an alpha byte from 0 to 255 |
| `round(x)`    | the nearest integer, a tie rounded up: `⌊x + 0.5⌋` (`REF-CSS-COLOR-4` §16.1.1)    |

## Step 1 — The hexadecimal forms

A hex color is `#` and 3, 4, 6 or 8 hexadecimal digits, the case of the letters not mattering (`REF-CSS-COLOR-4`, §5.2):

| Digits | Meaning                                                                 | Example                         |
| ------ | ----------------------------------------------------------------------- | ------------------------------- |
| 6      | `RRGGBB`, each pair a byte from `00` to `ff`, opaque                    | `#FF8000` is 255, 128, 0        |
| 8      | `RRGGBBAA`, the last pair the alpha byte, `00` transparent, `ff` opaque | `#0000ffcc`: blue, alpha 204    |
| 3      | `RGB`, each digit doubled                                               | `#f80` is `#ff8800`             |
| 4      | `RGBA`, each digit doubled                                              | `#f80c` is `#ff8800`, alpha 204 |

The stored form is `#` and six lower-case digits, the HTML-compatible serialization of an 8-bit sRGB color (`REF-CSS-COLOR-4`, §16.2.1: `#ff00ff`, "not `#FF00FF`").

## Step 2 — The opacity and the alpha byte

CSS maps an alpha percentage `p` to the byte `p × 2.55` rounded to the closest integer, rounding up on a tie (`REF-CSS-COLOR-4`, §16.1.1). Computed with integers, so that no binary fraction enters:

```text
A = round(255 · p / 100) = ⌊(255 · p + 50) / 100⌋
p = round(100 · A / 255) = ⌊(200 · A + 255) / 510⌋
```

Both are one rounded quotient of integers (`roundedQuotient`): for `d > 0`, `round(n / d) = ⌊n / d + ½⌋ = ⌊(2n + d) / 2d⌋`, with the first line `n = 255 · p`, `d = 100` and the second `n = 100 · A`, `d = 255`. The floor of `t / u` is taken as `(t − t mod u) / u`, `t mod u` from 0 included to `u` excluded, so that the division is exact; for a negative `n` a tie still rounds toward +∞ (`−7 / 2 = −3.5` gives −3).

`2.55` is not exact in binary64: `50 × 2.55` gives `127.49999999999999`, which rounds to 127, while `255 · 50 / 100 = 127.5` rounds up to 128 (`#80`); `90 × 2.55` gives `229.49999999999997`, which rounds to 229, while `229.5` rounds up to 230 (`#e6`). The integer forms give 128 and 230.

**Every percent comes back.** `|A − 2.55 p| ≤ 0.5`, so `|100 A / 255 − p| ≤ 0.5 / 2.55 < 0.2 < 0.5`: rounding gives `p` again. The byte from a percent never ties back: `100 A / 255 = k + ½` would need `200 A = 255 (2k + 1)`, an even number equal to an odd one.

**A byte that no percent gives is rounded.** 256 bytes and 101 percents: an 8-digit hexadecimal pasted keeps its alpha to the nearest percent. `cc` (204) gives 80 % exactly (CSS's own example); `ec` (236) gives `92.549…`, so 93 %, written back as 237 (`ed`).

## Step 3 — Hue and chroma

From the fractions, the largest and smallest component give the chroma `C = M − m`, and the hue is read from the largest (`REF-WIKI-HSL-HSV`, From RGB; `REF-CSS-COLOR-4`, §7.2):

```text
H = 60° · ((g − b) / C mod 6)   when M = r
H = 60° · ((b − r) / C + 2)     when M = g
H = 60° · ((r − g) / C + 4)     when M = b
```

CSS writes the first line `(g − b)/C + 6` when `g < b`, the same value. A grey has `C = 0`: its hue is undefined, "powerless" in CSS (§7), which writes it `NaN`; Wikipedia sets it to 0. The library returns **0**, so that a grey is a plain number; the picker keeps the hue its slider shows (US-035).

## Step 4 — HSL from the fractions

`L = (M + m) / 2`; `S_L = (M − L) / min(L, 1 − L)`, and 0 when `L` is 0 or 1 (`REF-CSS-COLOR-4`, §7.2; `REF-WIKI-HSL-HSV`, `S_L = (V − L) / min(L, 1 − L)`). For an sRGB color `M − L = C / 2 ≥ 0`, so `S_L` is never negative and CSS's turn of the hue by 180° for a negative saturation never applies.

## Step 5 — HSB from the fractions

`V = M`; `S_V = C / V`, and 0 when `V = 0` (`REF-WIKI-HSL-HSV`, From RGB). HSB is HSV under another name (`REF-WIKI-HSL-HSV`, lead).

## Step 6 — Back to the fractions

HSL (`REF-CSS-COLOR-4`, §7.1):

```text
k_n = (n + H / 30°) mod 12,   a = S_L · min(L, 1 − L)
f(n) = L − a · max(−1, min(k_n − 3, 9 − k_n, 1))
(r, g, b) = (f(0), f(8), f(4))
```

HSB (`REF-WIKI-HSL-HSV`, HSV to RGB alternative):

```text
k_n = (n + H / 60°) mod 6
f(n) = V − V · S_V · max(0, min(k_n, 4 − k_n, 1))
(r, g, b) = (f(5), f(3), f(1))
```

## Step 7 — Steps 6 undo steps 3 to 5

Take the sector where `r ≥ g ≥ b` and `C > 0`, and `h = (g − b) / C`, from 0 to 1 included (`h = 1` when `g = r`, still read by the first line of step 3, as CSS's `switch` tests red first), so `H = 60° · h`. At `h = 1`, `k_5 = 6` wraps to 0, which gives 0 as well, and the rows below hold with `g = r`.

HSB: `V = r`, `V · S_V = C`, `V − C = b`.

| n   | `k_n`             | `max(0, min(k, 4 − k, 1))` | `f(n)`                                    |
| --- | ----------------- | -------------------------- | ----------------------------------------- |
| 5   | `5 + h`, ≥ 5 or 0 | 0 (`4 − k < 0`)            | `V = r`                                   |
| 3   | `3 + h`           | `4 − k = 1 − h`            | `V − C (1 − h) = b + C h = b + g − b = g` |
| 1   | `1 + h`           | 1                          | `V − C = b`                               |

HSL: `L = (r + b) / 2`, `a = S_L · min(L, 1 − L) = M − L = C / 2`, `L − C/2 = b`, `L + C/2 = r`.

| n   | `k_n`           | `max(−1, min(k − 3, 9 − k, 1))` | `f(n)`                            |
| --- | --------------- | ------------------------------- | --------------------------------- |
| 0   | `2h`, at most 2 | −1 (`k − 3 ≤ −1`)               | `L + C/2 = r`                     |
| 8   | `8 + 2h`        | `9 − k = 1 − 2h`                | `L − (C/2)(1 − 2h) = b + C h = g` |
| 4   | `4 + 2h`        | 1 (`k − 3 ≥ 1`, `9 − k > 3`)    | `L − C/2 = b`                     |

The five other sectors permute the components and shift `k` by the same offsets; a grey (`C = 0`) gives `a = 0` and `V · S_V = 0`, so every component is `L` or `V`, the grey itself. The script of the cross-check runs every sector.

## Step 8 — Back to the bytes

Each byte is `round(255 · f(n))` (`REF-CSS-COLOR-4`, §16.1.1: "rounded to the closest integer (rounding up if two values are equally close)"). Over the 16 777 216 colors, computed in binary64 from the unrounded HSL or HSB values, `|255 · f(n) − byte|` is at most `3.7 × 10⁻¹³` for HSL and `3.8 × 10⁻¹³` for HSB, far below the 0.5 rounding forgives: **the round trip from the hexadecimal is exact** for every color.

The library rounds with `Math.round` (`nearestInteger`, `REF-MDN-MATH-ROUND`), as the cross-check did: it is CSS's rule, while `⌊x + 0.5⌋` in binary64 rounds 0.49999999999999994 up, and `roundedQuotient`, exact for integers, is not for a fraction: by 1, it gives `5.6 × 10⁻¹⁷` for `−10⁻¹³`, not an integer, and 1 for 0.49999999999999994, `2n + 1` rounding to 2 (EN-038).

## Step 9 — The precision of the values shown

A designer reads the HSL or HSB values and may type them back. Integers cannot keep every color: there are `360 × 101 × 101 = 3 672 360` integer triples for `16 777 216` colors. One decimal can. Every color was converted, its values rounded, and converted back:

| Values shown | HSL: colors changed       | HSB: colors changed       |
| ------------ | ------------------------- | ------------------------- |
| unrounded    | 0                         | 0                         |
| integers     | 15 001 662 (about 89.4 %) | 14 732 448 (about 87.8 %) |
| one decimal  | 0                         | 0                         |
| two decimals | 0                         | 0                         |

A hue rounded to 360.0 is shown 0.0. No bound proves the one-decimal case by itself: rounding the lightness to a tenth of a percent moves a byte by up to 0.26, rounding the hue to a tenth of a degree by up to 0.21, together more than 0.5. The exhaustive count is the proof.

## Step 10 — The plane of the picker

A picker shows a slice of the model at one hue, with a slider choosing the hue (`REF-WIKI-HSL-HSV`, Use in end-user software). The plane's horizontal axis is the saturation, as in CSS's tables of HSL colors (`REF-CSS-COLOR-4`, §7.3), from 0 at the left to 100 % at the right, and its vertical axis the lightness or the brightness, 100 % at the top. A point at `(x, y)` in a plane `w` wide and `h` tall stands for:

```text
S = 100 · x / w,   L or V = 100 · (1 − y / h)
```

CSS's tables list the saturation from 100 % down to 0 % across; the plane takes the increasing order for both models, one convention (provisional, VAL-016).

## Cross-check

A script written at SP-017 (Node.js, binary64) converted each of the 16 777 216 colors: the hue, HSL and HSL back by the sample code of `REF-CSS-COLOR-4` §7.2 and §7.1 (without its `1e−5` threshold, a grey's hue 0), HSB and HSB back by the formulas of `REF-WIKI-HSL-HSV`, the values rounded by `Math.round(x · 10ᵈ) / 10ᵈ` and a hue of 360 written 0, each byte by `Math.round`; unrounded and rounded to 0, 1 and 2 decimals, and gave the counts of step 9 and the bounds of step 8; it also checked the 101 percents of step 2. EN-038 turns the one-decimal count and the unrounded one into a test over every color, with the library's functions, so that the proof lives with the code.

## Checks (test cases)

| Case                           | Computation                                                                             | Expected                        |
| ------------------------------ | --------------------------------------------------------------------------------------- | ------------------------------- |
| `#FF8000` to HSL               | `g = 128/255 = 0.50196`, `C = 1`, `H = 60 · 0.50196`; `L = 0.5`, `S_L = 1`              | 30.11765°, 100 %, 50 %          |
| `#FF8000` to HSB               | `V = 1`, `S_V = 1 / 1`                                                                  | 30.11765°, 100 %, 100 %         |
| HSL(30.1°, 100 %, 50 %)        | `f(8) = 0.5 + 0.5 · (30.1/30 − 1) = 0.50167`, `255 · 0.50167 = 127.925`                 | `#ff8000`                       |
| HSL(30°, 100 %, 50 %)          | `f(8) = 0.5`, `127.5` rounds up                                                         | `#ff8000`                       |
| `#336699` to HSL               | `r, g, b = 0.2, 0.4, 0.6`, `C = 0.4`, `H = 60 · (−0.5 + 4)`; `L = 0.4`, `S_L = 0.2/0.4` | 210°, 50 %, 40 %                |
| `#336699` to HSB               | `V = 0.6`, `S_V = 0.4/0.6`                                                              | 210°, 66.66667 %, 60 %          |
| HSB(210°, 67 %, 60 %)          | `f(5) = 0.6 − 0.6 · 0.67 = 0.198`, `255 · 0.198 = 50.49`                                | `#326699`, not `#336699`        |
| HSB(210°, 66.7 %, 60 %)        | `f(5) = 0.6 − 0.6 · 0.667 = 0.1998`, `50.949`                                           | `#336699`                       |
| `#808080`, a grey              | `C = 0`; `L = V = 128/255`                                                              | hue 0, saturation 0, 50.19608 % |
| `#000001` at integer HSL       | HSL(240°, 100 %, 0.19608 %) shown 0 % lightness                                         | `#000000`; at 0.2 %: `#000001`  |
| `#000000`, `#ffffff`           | `C = 0`; `L` is 0 or 1                                                                  | saturation 0, hue 0             |
| `#f80`, `#F80C`                | each digit doubled; `cc` = 204                                                          | `#ff8800`; with alpha, 80 %     |
| 50 % to a byte                 | `⌊(12 750 + 50) / 100⌋`                                                                 | 128 (`80`), not 127             |
| byte 236 to a percent          | `⌊(47 200 + 255) / 510⌋ = ⌊93.05⌋`                                                      | 93 %                            |
| every percent 0 to 100         | percent → byte → percent                                                                | the same percent                |
| every color, one decimal shown | step 9                                                                                  | the same hexadecimal            |
