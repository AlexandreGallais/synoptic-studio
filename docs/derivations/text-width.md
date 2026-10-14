# DERIV-text-width — The width of a line of text and the line height, from a font's metrics

Used by: F05 (EN-031, EN-032, ADR-0052), then F14 for wrapping. Sources: `REF-OT-HEAD` (`unitsPerEm`), `REF-OT-CMAP` (character to glyph, glyph 0 when missing), `REF-OT-HMTX` (the advance of a glyph, the last record for the remaining glyphs), `REF-OT-KERN` and `REF-OT-GPOS` (kerning of a pair of glyphs), `REF-OT-HHEA` (ascender, descender, line gap), `REF-CSS-TEXT-3` (letter spacing between adjacent characters, not at the start or end of a line, in addition to kerning; word spacing on each word separator).

## Notation

| Symbol       | Definition                                                                                           |
| ------------ | ---------------------------------------------------------------------------------------------------- |
| `c₁ … cₙ`    | the characters of one line, after the case transform, `n ≥ 0`                                        |
| `g(c)`       | the glyph of character `c` in the font's `cmap`; 0 (`.notdef`) when the font has none                |
| `adv(g)`     | the advance of glyph `g` in `hmtx`, in font units                                                    |
| `kern(a, b)` | the kerning of the glyph pair `(a, b)`, in font units, 0 when the font lists none; negative tightens |
| `U`          | `unitsPerEm` of the font, an integer from 16 to 16 384                                               |
| `s`          | the text's size, an integer from 1 to 100 000, in user units                                         |
| `L`, `W`     | the letter spacing and the word spacing, in user units                                               |
| `m`          | the number of word separators in the line: spaces U+0020 and no-break spaces U+00A0                  |
| `A, D, G`    | the `hhea` ascender, descender (negative below the baseline) and line gap, in font units             |

## Step 1 — One glyph's advance in user units

A character is drawn by its glyph `g(c)`, glyph 0 when the font has no glyph for it (`REF-OT-CMAP`). Its advance is read from `hmtx`, from the last record when the glyph's index is past `numberOfHMetrics` (`REF-OT-HMTX`). Font units are fractions of the em, `U` of them per em (`REF-OT-HEAD`), and the em is the text's size, so one font unit is `s / U` user units: the glyph advances `adv(g(c)) × s / U`.

## Step 2 — The line without spacing

Glyphs follow each other, each starting where the previous advance ends, so the line's width is the sum of the advances, plus the kerning of each adjacent pair, which adjusts "the advances of two glyphs in relation to one another" (`REF-OT-GPOS`; `REF-OT-KERN` adds its values to the same pairs):

```text
w₀ = (s / U) × ( Σᵢ₌₁ⁿ adv(g(cᵢ)) + Σᵢ₌₁ⁿ⁻¹ kern(g(cᵢ), g(cᵢ₊₁)) )
```

Kerning is read only between characters of one line: the line's last character has no next pair.

## Step 3 — Letter spacing and word spacing

Letter spacing is "additional spacing between adjacent typographic character units", "in addition to kerning and word-spacing", and "not applied at the beginning or end of a line" (`REF-CSS-TEXT-3`): it is added once between each pair of adjacent characters, `n − 1` times. Word spacing is "applied to each word separator", the space and the no-break space among them (`REF-CSS-TEXT-3`): it is added `m` times.

```text
w = w₀ + max(n − 1, 0) × L + m × W
```

An empty line (`n = 0`) has width 0. The order of the sums does not change the value beyond rounding: the library sums integers of font units first, then multiplies once by `s / U`.

## Step 4 — The line height and the block

The `hhea` table gives a typographic ascent `A`, descent `D` and line gap `G` (`REF-OT-HHEA`), in font units. One line takes from its top the ascent, then down to the descent, then the gap before the next line:

```text
lineHeight = (A − D + G) × s / U
baseline of line k (from 0) = top + k × lineHeight + A × s / U
```

A text of `k ≥ 1` lines has a block as wide as its widest line and `k × lineHeight` tall; an empty text draws nothing (F05).

## Cross-check

Arial 5.01 (macOS `Arial.ttf`): `U = 2048`, `A = 1854`, `D = −434`, `G = 67`, read by a script from the file's tables, and the advances below read the same way. Measured by the prototype parser of the spike and by hand, the two agree on every row of the table.

| Character | `adv` | Character | `adv` |
| --------- | ----- | --------- | ----- |
| `H`       | 1479  | `W`       | 1933  |
| `e`       | 1139  | `r`       | 682   |
| `l`       | 455   | `d`       | 1139  |
| `o`       | 1139  | space     | 569   |
| `A`       | 1366  | `V`       | 1366  |
| `T`       | 1251  | `R`       | 1479  |

Kerning pairs used: `W o` −37; `A V`, `V A`, `A T`, `T A` −152 each.

## Checks (test cases)

| Case                                | Computation                                                                                                              | Expected                                     |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------- |
| "Hello World", 16, no kerning       | `1479 + 1139 + 3 × 455 + 2 × 1139 + 569 + 1933 + 682 + 1139`                                                             | 10 584 units, `10 584 × 16 / 2048` = 82.6875 |
| "Hello World", 16, kerned           | `(10 584 − 37) × 16 / 2048`                                                                                              | 82.39844                                     |
| "Hello World", 16, `L = 2`, `W = 4` | `82.6875 + 10 × 2 + 1 × 4`                                                                                               | 106.6875                                     |
| "AVATAR", 16, no kerning            | `(3 × 1366 + 1366 + 1251 + 1479) × 16 / 2048` = `8194 × 16 / 2048`                                                       | 64.01563                                     |
| "AVATAR", 16, kerned                | `(8194 − 4 × 152) × 16 / 2048`                                                                                           | 59.26563                                     |
| "H", 16, `L = 2`                    | one character: no letter spacing                                                                                         | 11.55469                                     |
| empty line                          | `n = 0`                                                                                                                  | 0                                            |
| line height, 16                     | `(1854 + 434 + 67) × 16 / 2048` = `2355 / 128`                                                                           | 18.39844                                     |
| first baseline, 16                  | `1854 × 16 / 2048`                                                                                                       | 14.48438                                     |
| block of "Hello" / "World", 16      | widths `(1479 + 1139 + 910 + 1139) × 16 / 2048` = 36.46094 and `(1933 + 1139 + 682 + 455 + 1139) × 16 / 2048` = 41.78125 | 41.78125 × 36.79688                          |
