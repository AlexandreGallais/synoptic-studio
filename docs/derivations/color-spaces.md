# DERIV-color-spaces — A color in hexadecimal, its opacity, and its HSL and HSB views

Used by: EN-037 (the hexadecimal forms and the opacity), EN-038 (HSL and HSB from the hexadecimal and back), US-035 (the picker's plane and the values it shows, the exact bytes of its values), EN-043 (the values shown as integers, read from exact fractions, and one value edited with the exact others). Sources: `REF-CSS-COLOR-4` (§5.1, §5.2, §7, §7.1, §7.2, §16.1.1, §16.2.1), `REF-WIKI-HSL-HSV` (From RGB, HSV to RGB alternative). The stored color is six hexadecimal digits and an integer opacity (F16.AC1); HSL and HSB are computed from it and never stored (ADR-0056). This page shows that the round trip from the hexadecimal is exact, and which precision of the shown HSL and HSB values keeps it exact when they are typed back.

## Notation

| Symbol        | Definition                                                                                                        |
| ------------- | ----------------------------------------------------------------------------------------------------------------- |
| `R`, `G`, `B` | the red, green and blue bytes, integers from 0 to 255                                                             |
| `r`, `g`, `b` | the same as fractions: `r = R / 255`, from 0 to 1                                                                 |
| `M`, `m`      | `max(r, g, b)` and `min(r, g, b)`                                                                                 |
| `C`           | the chroma, `M − m`                                                                                               |
| `H`           | the hue, in degrees, from 0 included to 360 excluded                                                              |
| `S_L`, `L`    | HSL's saturation and lightness, as fractions from 0 to 1 (shown in percent)                                       |
| `S_V`, `V`    | HSB's saturation and brightness (HSV's value), as fractions (shown in percent)                                    |
| `p`, `A`      | the opacity in percent, an integer from 0 to 100, and an alpha byte from 0 to 255                                 |
| `round(x)`    | the nearest integer, a tie rounded up: `⌊x + 0.5⌋` (`REF-CSS-COLOR-4` §16.1.1), computed by `Math.round` (step 8) |

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

CSS maps an alpha percentage `p` to the byte `p × 2.55` rounded to the closest integer, rounding up on a tie (`REF-CSS-COLOR-4`, §16.1.1). Computed as one quotient of integers, so that no inexact factor enters:

```text
A = round(255 · p / 100)
p = round(100 · A / 255)
```

Each is `round(n / d)` (`nearestInteger`, `Math.round`) of a quotient of integers, `n = 255 · p`, `d = 100` and `n = 100 · A`, `d = 255`. The quotient computed in binary64 is exact enough: a quotient that is a half-integer, `k + ½` with `|k|` small, is a binary fraction and is computed exactly, so the tie is seen and rounded up; any other quotient lies at least `1 / (2d)` from every half-integer (`2n / d` differs from an odd integer by a nonzero integer over `d`), far more than the division's error, at most half a unit in the last place, about `10⁻¹⁴` here, so rounding it gives the same integer.

`2.55` is not exact in binary64: `50 × 2.55` gives `127.49999999999999`, which rounds to 127, while `255 · 50 / 100 = 127.5` rounds up to 128 (`#80`); `90 × 2.55` gives `229.49999999999997`, which rounds to 229, while `229.5` rounds up to 230 (`#e6`). The quotients of integers give 128 and 230.

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
| 4   | `4 + 2h`        | 1 (`k − 3 ≥ 1`, `9 − k ≥ 3`)    | `L − C/2 = b`                     |

The five other sectors permute the components and shift `k` by the same offsets; a grey (`C = 0`) gives `a = 0` and `V · S_V = 0`, so every component is `L` or `V`, the grey itself. The script of the cross-check runs every sector.

## Step 8 — Back to the bytes

Each byte is `round(255 · f(n))`: a component whose precision cannot be kept "should be rounded towards +∞" (`REF-CSS-COLOR-4`, §5.1; the rounding to a byte is the project's choice, ADR-0056), and CSS rounds to the nearest integer "in the direction of +∞ when the fractional portion is exactly 0.5" (`REF-CSS-VALUES-4`, §5.2). Over the 16 777 216 colors, computed in binary64 from the unrounded HSL or HSB values, `|255 · f(n) − byte|` is at most `3.7 × 10⁻¹³` for HSL and `3.8 × 10⁻¹³` for HSB, far below the 0.5 rounding forgives: **the round trip from the hexadecimal is exact** for every color.

The library rounds with `Math.round` (`nearestInteger`, `REF-MDN-MATH-ROUND`), as the cross-check did: it is CSS's rule, while `⌊x + 0.5⌋` in binary64 rounds 0.49999999999999994 up, and the floor `⌊(2n + d) / 2d⌋` taken with a modulo, exact for integers, is not for a fraction: by 1, it gave `−5.6 × 10⁻¹⁷` for `−10⁻¹³`, not an integer, and 1 for 0.49999999999999994, `2n + 1` rounding to 2 (EN-038).

## Step 9 — The precision of the values shown

A designer reads the HSL or HSB values and may type them back. Integers cannot keep every color: there are `360 × 101 × 101 = 3 672 360` integer triples for `16 777 216` colors. One decimal can. Every color was converted, its values rounded, and converted back:

| Values shown | HSL: colors changed       | HSB: colors changed       |
| ------------ | ------------------------- | ------------------------- |
| unrounded    | 0                         | 0                         |
| integers     | 15 001 662 (about 89.4 %) | 14 732 448 (about 87.8 %) |
| one decimal  | 0                         | 0                         |
| two decimals | 0                         | 0                         |

The integer counts are those of the SP-017 script; another binary64 implementation of the same formulas gave counts up to 45 apart (AUD-017), since values lying exactly on a half are computed a last bit above or below it. The zeros do not depend on it.

A value shown is the nearest tenth of the value computed in binary64. When the exact value lies on a half-tenth, the computed one may lie a last bit below it and is shown rounded down: `#002330` has the hue 196.25° exactly, computed 196.24999999999997 and shown 196.2; `#030330` has the HSB saturation 93.75 %, shown 93.7. Either neighbour gives the hexadecimal back, as the one-decimal row shows (AUD-017, provisional, VAL-016).

Since ADR-0058 the values are shown as integers, and the integer row's changes are avoided another way: the color is never rebuilt from the three values shown, only from one value typed and the exact values of the two others (step 12).

A hue rounded to 360.0 is shown 0.0, by the remainder `n − 360 · ⌊n / 360⌋` of the rounded hue, which is never negative; the library takes it with `%` (`nonNegativeRemainder`, `REF-ECMA-262-NUMBER-REMAINDER`), exact in binary64, where `modulo`'s added divisor turns 30.1 into 30.100000000000023 (EN-038). No bound proves the one-decimal case by itself: rounding the lightness to a tenth of a percent moves a byte by up to 0.26, rounding the hue to a tenth of a degree by up to 0.21, together more than 0.5. The exhaustive count is the proof.

## Step 10 — The plane of the picker

A picker shows a slice of the model at one hue, with a slider choosing the hue (`REF-WIKI-HSL-HSV`, Use in end-user software). The plane's horizontal axis is the saturation, as in CSS's tables of HSL colors (`REF-CSS-COLOR-4`, §7.3), from 0 at the left to 100 % at the right, and its vertical axis the lightness or the brightness, 100 % at the top. A point at `(x, y)` in a plane `w` wide and `h` tall stands for:

```text
S = 100 · x / w,   L or V = 100 · (1 − y / h)
```

CSS's tables list the saturation from 100 % down to 0 % across; the plane takes the increasing order for both models, one convention (provisional, VAL-016).

The same convention reads a slider `w` long holding values from 0 to `M`, its left end at 0: the point at `x` stands for `M · x / w`, the hue slider's `M` being 360° (a hue of 360 is shown 0.0, step 9) and the opacity's 100 %. The plane is two such sliders: `S` across, and `L` or `V` from the bottom edge, at the height `h − y`, so `100 · (h − y) / h`, the line above. Solving each line for its position gives where a value sits, for the marks the picker draws:

```text
x = w · v / M,   x = w · S / 100,   y = h − h · (L or V) / 100
```

The multiplication comes first, as for the stretched shapes (`DERIV-free-shape-stretch`). A point beyond an edge, during a drag, counts at that edge, and a slider not laid out yet (`w = 0`) holds 0 (US-035, provisional, VAL-016). The middle of the right edge of a plane 200 × 100 is `S = 100 · 200 / 200 = 100 %`, `V = 100 · 50 / 100 = 50 %`; at the hue 210° in HSB it is `#004080` (step 6: red 0, green `0.5 − 0.5 · 0.5 = 0.25`, 63.75 gives 64, blue 127.5 gives 128).

## Step 11 — Exact bytes for values given to one decimal

Step 8 rounds `255 · f(n)` with `f(n)` computed in binary64. For a color read from its hexadecimal that is safe: step 9's check found no byte changed. A value picked or typed in the picker is any number to one decimal, and some land a byte exactly on a half, where binary64 may fall below it: HSB 30.1°, 90 %, 100 % has the blue `1 − 0.9 = 0.1`, `255 · 0.1 = 25.5`, which the domain rounds up to 26, but `1 − 0.9` is `0.09999999999999998` in binary64 and gives 25. So the picker's values take a second form of step 6, in integers.

Each value is counted in tenths, `h = 10 · H`, `s = 10 · S`, `v` or `l = 10 · (V or L)`, integers (`h` from 0 to 3599, `s`, `v`, `l` from 0 to 1000). Multiplying step 6's `k` by 600 for HSB and by 300 for HSL, and writing each fraction over one common denominator:

```text
HSB:  K = (600 n + h) mod 3600,  p = clamp(min(K, 2400 − K), 0, 600),
      f(n) = N / D,  N = v · (600 000 − s · p),  D = 6 · 10⁸
HSL:  K = (300 n + h) mod 3600,  q = clamp(min(K − 900, 2700 − K), −300, 300),
      f(n) = N / D,  N = 300 000 · l − s · min(l, 1000 − l) · q,  D = 3 · 10⁸
byte = round(255 · N / D)
```

Each line is step 6 with `k = K / 600` (or `K / 300`), `V = v / 1000`, `S = s / 1000`, `min(k, 4 − k) = p / 600`, `min(k − 3, 9 − k) = q / 300` and `a = S · min(L, 1 − L) = s · min(l, 1000 − l) / 10⁶`; the clamps are step 6's.

Why it is exact:

1. Every quantity is an integer: `|N| ≤ D` and `255 · N ≤ 255 · 6 · 10⁸ < 1.6 · 10¹¹`, far below 2⁵³, so `K`, `p`, `q`, `N` and `255 · N` are exact in binary64.
2. The quotient `255 · N / D` is one correctly rounded division (IEEE 754). If the byte lies on a half, `255 · N / D = m + ½` with `m` an integer below 256, which binary64 represents exactly; the division returns it exactly and `round` takes it up.
3. Otherwise `|255 · N / D − (m + ½)| = |510 · N − (2m + 1) · D| / (2D) ≥ 1 / (2D) ≥ 8.3 · 10⁻¹⁰`, the numerator being a non-zero integer, while the division's error is below 256 · 2⁻⁵³ ≈ 2.8 · 10⁻¹⁴: the rounding cannot cross the half.

The binary64 form of step 6 stays for colors read from their hexadecimal (step 9's round trip); the integer form serves the values a picker holds, given to one decimal at US-035, integers since US-040 (ten tenths each; step 12 for any denominator). HSB 30.1°, 90 %, 100 % gives `#ff8d1a`, HSB 200°, 50 %, 100 % `#80d5ff` (green 5/6, 212.5 up to 213), HSB 0°, 75 %, 40 % `#661a1a`, HSL 0°, 75 %, 40 % `#b31a1a`, HSL 0°, 75 %, 60 % `#e64d4d`; the tests check every byte against the same arithmetic in exact integers (BigInt).

## Step 12 — Exact values from the bytes, shown as integers

Steps 3 to 5 written over the bytes keep every value an exact fraction of integers, with `M`, `m` and `C` taken in bytes (`brightness`, `smallestComponent` and `chroma` read bytes as they read fractions, `max`, `min` and a difference keeping the scale):

```text
H   = 60 · H″ / C,   H″ = (G − B) mod 6C if M = R, (B − R) + 2C if M = G, (R − G) + 4C;  0 / 1 if C = 0
S_V = C / M (0 / 1 if M = 0),   V = M / 255
S_L = C / min(M + m, 510 − (M + m)) (0 / 1 if that is 0),   L = (M + m) / 510
```

`H″` is step 3's `H′ · C`, an integer from 0 to `6C`. `S_L` is step 4's `(M − L) / min(L, 1 − L)` with both terms multiplied by 510: `M − L` is `C / 2` in fractions, `C / 510` over 255, and `min(L, 1 − L)` is `min(M + m, 510 − (M + m)) / 510`. `#336699` (51, 102, 153) is HSB `21 420 / 102 = 210°`, `102 / 153`, `153 / 255` and HSL 210°, `102 / 204`, `204 / 510`. Every denominator is at most 510.

A value is shown as the nearest integer, a tie up, `round(k · N / D)` with `k` 1 for the hue and 100 for a percent, and a hue rounding to 360 shown 0 as in step 9 (ADR-0058). `k · N` is an integer below `100 · 510`, so step 11's argument holds: the quotient is correctly rounded, a value lying on a half is seen as a half, and any other lies at least `1 / (2D) ≥ 1 / 1020` from one. `#ff8000` shows HSB 30, 100, 100 and HSL 30, 100, 50; `#336699` shows HSB 210, 67, 60; `#ff0001`, at `91 740 / 255 = 359.76471°`, shows 0; `#080100`, at `60 · 1 / 8 = 7.5°`, shows 8; `#c8c7c7`, its HSB saturation `1 / 200 = 0.5 %`, shows 1.

**One value edited, the others exact** (ADR-0058, decisions 2 and 3). The color is rebuilt from the integer typed and the exact values of the two others; a value given equal to the one read (when the picker opened, a hexadecimal typed, another model chosen; ADR-0059) stands for its exact value, so a field left as it is changes nothing. Step 11's integer form holds for any fractions `H = h / d`, `S = s / e` and `V` or `L = v / w` (or `l / w`), with `t = 60 d` for HSB and `t = 30 d` for HSL in place of 600 and 300:

```text
HSB:  K = (t n + h) mod 6t,   p = clamp(min(K, 4t − K), 0, t),            f(n) = v · (e t − s p) / (w e t)
HSL:  K = (t n + h) mod 12t,  q = clamp(min(K − 3t, 9t − K), −t, t),      f(n) = (l e t − s · min(l, w − l) · q) / (w e t)
```

Tenths are `d = 10`, `e = w = 1000`, step 11's lines; an integer typed is `d = 1` or `e = w = 100`; an exact value has a denominator up to 255 (`d = C`, `e = M` or `min(M + m, 510 − (M + m))`) or 510 (`w` for `L`). The denominator `w e t` is at most `255 · 255 · 60 · 255` or `510 · 255 · 30 · 255`, about `9.95 · 10⁸`, and `|N| ≤ w e t`, so `255 · N < 2.6 · 10¹¹ < 2⁵³` and a value off a half lies at least `1 / (2 w e t) ≥ 5 · 10⁻¹⁰` from it: step 11's three reasons hold. With the three values exact, `f(n)` is `R / 255` exactly (step 7), so the color comes back.

`#336699`, its brightness typed 61: HSB `21 420 / 102`, `102 / 153`, `61 / 100`. Red: `t = 6120`, `K = (30 600 + 21 420) mod 36 720 = 15 300`, `p = 6120`, `f = 61 · 51 / (100 · 153) = 0.20333`, 51.85 gives 52; green: `K = 3060`, `p = 3060`, `f = 0.40667`, 103.7 gives 104; blue `0.61`, 155.55 gives 156: `#34689c`. The three integers shown, 210, 67, 61, would give `#33679c`. Black (HSL 0, 0, 0 shown) given 30, 100, 50 gives `#ff8000`, the green 127.5 rounded up.

## Cross-check

At EN-043 the tests run the sample described below through the integers shown (`toWholeColor`) given back unchanged (`wholeValuesToHex`), in HSL and in HSB: 0 colors changed; a property test moves one value of any color by 1 and checks each byte within ½ of `255 · f(n)` computed in binary64 by `hsbToRgb` or `hslToRgb` from that integer and the exact others.

At AUD-017 two reviewers ran every color through the picker's own path as it then was, its values shown to one decimal (`hexToPickedColor`) then turned back in exact integers (`pickedColorToHex`, step 11), in HSL and in HSB: 0 colors changed in each. Both functions take integers since US-040.

A script written at SP-017 (Node.js, binary64) converted each of the 16 777 216 colors: the hue, HSL and HSL back by the sample code of `REF-CSS-COLOR-4` §7.2 and §7.1 (without its `1e−5` threshold, a grey's hue 0), HSB and HSB back by the formulas of `REF-WIKI-HSL-HSV`, the values rounded by `Math.round(x · 10ᵈ) / 10ᵈ` and a hue of 360 written 0, each byte by `Math.round`; unrounded and rounded to 0, 1 and 2 decimals, and gave the counts of step 9 and the bounds of step 8; it also checked the 101 percents of step 2. At EN-038 the library's own functions (`rgbToHsl`, `rgbToHsb`, `hslToRgb`, `hsbToRgb`, `toShownHsl`, `toShownHsb`, `fractionToByte`; the two one-decimal views removed at US-040, the unrounded sample kept in the tests of `hsbToRgb` and `hslToRgb`) ran over every color once, unrounded and to one decimal, in HSL and in HSB: 0 colors changed in each of the four runs, about 13 to 17 s each. That is too slow for every test run, so the tests run a sample with the same functions: one color in 31 (31 is prime to 256, so every byte of every component is drawn) and every color with two equal components, where the hue changes sector, about 731 000 colors, each round trip a test of its own taking about 1.3 to 1.8 s, about 3 s for each model under coverage; a property test draws any color besides.

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
| 50 % to a byte                 | `round(255 · 50 / 100) = round(127.5)`                                                  | 128 (`80`), not 127             |
| byte 236 to a percent          | `round(100 · 236 / 255) = round(92.549)`                                                | 93 %                            |
| every percent 0 to 100         | percent → byte → percent                                                                | the same percent                |
| every color, one decimal shown | step 9                                                                                  | the same hexadecimal            |
| `#336699`, brightness typed 61 | step 12: `210°`, `102 / 153` exact, `61 / 100`                                          | `#34689c`, not `#33679c`        |
| `#336699`, saturation typed 67 | step 12: 67 is the value shown, so `102 / 153`                                          | `#336699`                       |
| plane 200 × 100 at (200, 50)   | step 10: `S = 100 · 200 / 200`, `V = 100 · (100 − 50) / 100`; HSB at 210°               | 100 %, 50 %, `#004080`          |
| hue slider 200 long at 100     | step 10: `360 · 100 / 200`                                                              | 180°                            |
