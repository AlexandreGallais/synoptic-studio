# DERIV-html-layer — Where a browser puts the HTML layer's first baseline, and how the layer is placed to meet the library's

Used by: EN-041 (the HTML layer's `foreignObject` placed over the text's lines). Sources: `REF-CSS-INLINE-3` (§5.3, the leading of a line box), `REF-GLYPHS-VERTICAL-METRICS` and `REF-GF-VERTICAL-METRICS` (which vertical metrics browsers use), `DERIV-text-width` (the library's line height and baseline), `DERIV-text-box` (a chosen line height, half-leading). No source states the shift of step 4; each step cites what it rests on.

## Notation

| Symbol        | Definition                                                                                   |
| ------------- | -------------------------------------------------------------------------------------------- |
| `s`           | the text's size, user units                                                                  |
| `u`           | the font's units per em                                                                      |
| `A`, `D`, `G` | the ascender, the descent (positive) and the line gap of `hhea`, font units                  |
| `k`           | `s / u`, the scale from font units to user units                                             |
| `L`           | the line height the library uses, user units: automatic `k · (A + D + G)`, or the one chosen |
| `A_b`, `D_b`  | the ascent and descent a browser uses, font units                                            |

## Step 1 — The library's first baseline

From the top of a line: automatic, at `k · A`, the line gap below the line (`DERIV-text-width`; F14 keeps F05's placement); a chosen `L`, at `k · A + (L − k · (A + D)) / 2`, the room left spread half above and half below (`DERIV-text-box`, CSS's half-leading).

## Step 2 — A browser's first baseline

`REF-CSS-INLINE-3`, §5.3: with a `line-height` other than `normal`, "Calculate the leading L as L = line-height - (A + D). Half the leading … is added above A". The HTML layer is written with `line-height` set to the library's `L` in user units, so a browser puts its first baseline at

```text
k · A_b + (L − k · (A_b + D_b)) / 2
```

below the top of its block.

## Step 3 — Which metrics a browser uses

On macOS, Safari and Chrome use `hhea` "regardless of the Use Typo Metrics setting" and Firefox does unless that flag is set; on Windows, browsers use the typographic metrics when `USE_TYPO_METRICS` is set, else the `win` metrics (`REF-GLYPHS-VERTICAL-METRICS`; `REF-GF-VERTICAL-METRICS`: "`Hhea` metrics are used in macOS"). When the browser uses `hhea`, `A_b = A` and `D_b = D`. Arial 5.01 has `USE_TYPO_METRICS` unset and `win` ascent and descent 1854 and 434, equal to its `hhea` ascender and descent: `A_b = A`, `D_b = D` on both systems (measured from the file, research note 0020).

## Step 4 — The shift

With `A_b = A` and `D_b = D`, step 2 minus step 1:

- a chosen `L`: `k · A + (L − k · (A + D)) / 2` in both: no shift;
- automatic: `L = k · (A + D + G)`, so the browser's baseline is at `k · A + k · G / 2`, the library's at `k · A`: the browser's is `k · G / 2` lower.

The `foreignObject` is therefore placed `k · G / 2` above the top of the library's first line when the line height is automatic, and at it otherwise. When the library trims the first line to its cap height (`DERIV-text-box`), the top of its first line lies above the block's top by what the trim removed, and the `foreignObject` starts there, so that the browser's lines start where the library's would before the trim.

## Cross-check

Arial 16 (`u` = 2048, `A` = 1854, `D` = 434, `G` = 67, `k` = 0.0078125), automatic: `L = 0.0078125 · 2355 = 18.3984375` (as `DERIV-text-width`); the library's baseline at `0.0078125 · 1854 = 14.484375`; a browser's at `14.484375 + (18.3984375 − 17.875) / 2 = 14.484375 + 0.26171875 = 14.74609375`, `k · G / 2 = 0.0078125 · 33.5 = 0.26171875` lower, as step 4 says.

## Checks (test cases)

| Case                              | Computation                                                             | Expected                           |
| --------------------------------- | ----------------------------------------------------------------------- | ---------------------------------- |
| Arial 16, automatic               | `k · G / 2 = 0.0078125 · 33.5`                                          | layer 0.26171875 above             |
| Arial 16, line height 24          | chosen `L`: no shift; both baselines at `14.484375 + (24 − 17.875) / 2` | 17.546875, layer at the line's top |
| Inter 4.1 16, automatic (`G` = 0) | `k · 0 / 2`                                                             | no shift                           |
| Arial 160, automatic              | `0.078125 · 33.5`                                                       | layer 2.6171875 above              |
