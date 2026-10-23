# DERIV-band-within-box — The band about a contour computed only within a box, and the box the helps are computed within

Used by: EN-149 (F42), `contourBandWithin` in `src/geometry/contour-band/`, `shownBox` in `src/model/view/`; the outlines of the layer above the drawing (`interaction.md` §2), SP-030's answer 7. Sources: `REF-SVG2-PAINT` (the stroke a band stands for), `REF-WIKI-BOUNDING-BOX` and `REF-WIKI-INTERVAL` (a box as the product of two closed intervals), `REF-MATHWORLD-VECTOR-NORM` (the length of a vector), `REF-WIKI-TRIANGLE-INEQUALITY` (a path no shorter than the straight line between its ends), `REF-POMAX-BEZIER` (the bound on a cubic's chords, as `pieceSamples` takes it); `DERIV-view-zoom` (the area shown), `DERIV-box-overlap` (a piece meeting a box, a box widened); research note 0031, §5.

No source cuts a band of chords to a box: the sources give the band's points, a box as two intervals, and the length of a path. This page draws from them which subpaths of the band about a contour (`contourBand`) can paint a point of a box, and how to find them without computing the others.

## Notation

Frame of the SVG: x to the right, y down. Lengths are user units unless a step says CSS pixels.

| Symbol | Definition |
| --- | --- |
| `B` | the box asked, `[x₀, x₁] × [y₀, y₁]`, its sides included |
| `h, s` | the band's half width and its longest chord, `h > 0`, `s > 0` |
| `B_h` | `B` widened by `h` on each side, `[x₀ − h, x₁ + h] × [y₀ − h, y₁ + h]` (`widenedBox`) |
| `n` | the number of chords of a piece (`pieceStepCount`): 1 for a segment, `max(⌈Λ / s⌉, 1)` for a curve |
| `Λ` | a piece's length bound (`pieceLengthBound`): a segment's length, an arc's `r · \|θ\|`, a cubic's `3 · max(\|P₀P₁\|, \|P₁P₂\|, \|P₂P₃\|)` |
| `λ` | `Λ / n`, at least each chord's length |
| `Pᵢ` | the piece's sample `i`, `0 ≤ i ≤ n` (`pieceSample`), `P₀` its start, `Pₙ` its end |
| `s, o, W × H` | in step 1 only: the view's scale and origin, the canvas's size in CSS pixels (`DERIV-view-zoom`) |

## Step 1 — The box the helps are computed within

The area shown is `[o.x, o.x + W / s] × [o.y, o.y + H / s]` (`DERIV-view-zoom`, step 1). The helps are computed within it widened by half the canvas on each side (SP-030's answer 7), `W / (2s)` across and `H / (2s)` down:

`B = [o.x − W / (2s), o.x + 3W / (2s)] × [o.y − H / (2s), o.y + 3H / (2s)]`,

a padding `p` in CSS pixels on each side giving `[o.x − p_W / s, o.x + (W + p_W) / s]` and likewise down (`shownBox`, the area shown its padding 0). At the opening view, `s = 1`, `o = (−40, −40)`, 1 200 × 800: `B` runs from (−640, −440) to (1 760, 1 160), four times the area shown.

A pan of `d` pixels gives `o′ = o − d / s` (`DERIV-view-zoom`, step 6), the scale kept. The area shown then lies within `B` when `o.x − d_x / s ≥ o.x − W / (2s)` and `o.x − d_x / s + W / s ≤ o.x + 3W / (2s)`, that is `−W / 2 ≤ d_x ≤ W / 2`, and `−H / 2 ≤ d_y ≤ H / 2` likewise: a pan of at most 600 pixels across and 400 down keeps it, and the helps computed within `B` still hold every help of the area shown. A zoom changes their sizes (ADR-0098, decision 5), and a change of the drawing their contours: the helps are computed again then, as once the area shown leaves `B`. The padding is more than the band's half width, 1 pixel, so that a band crossing a side of the canvas is whole there.

## Step 2 — What a subpath of the band paints

The band about a contour is, for each piece, a disc of radius `h` about its start, then a rectangle about each chord between its samples (`contourBand`): the chord's band, the points "on the line perpendicular" to it "within distance" `h` (`REF-SVG2-PAINT`, a dash; `segmentBand`). A point `Q` of a chord's band has its foot `F` on the chord with `|Q − F| ≤ h`; a point of the disc is within `h` of its centre.

If `|Q − F| ≤ h`, each coordinate differs by at most `h`: `|q_x − f_x| ≤ √((q_x − f_x)² + (q_y − f_y)²)`, the length of a vector (`REF-MATHWORLD-VECTOR-NORM`). So if `Q` is in `B`, `F` is in `B_h`, each of its coordinates in its interval widened by `h` (`REF-WIKI-INTERVAL`, `REF-WIKI-BOUNDING-BOX`; `DERIV-box-overlap`, step 10).

## Step 3 — The subpaths kept

The band within `B` keeps, in the whole band's order:

- a piece's disc when its start is in `B_h` (`isPointInBox`);
- a chord's band when the chord meets `B_h` (`isPieceMeetingBox`, `DERIV-box-overlap` steps 4 and 5), drawn whole, its band the whole band's.

**It paints `B` as the whole band does.** A subpath painting a point `Q` of `B` has its centre, or a point `F` of its chord, within `h` of `Q`, so in `B_h` (step 2): it is kept.

**None of them lies wholly outside `B_h`.** A disc kept holds its centre, in `B_h`; a chord's band holds its chord, which meets `B_h`.

Its chords are the whole band's, so each is at most `s` long (`pieceSamples`). A subpath left out paints nothing of `B`: the band within `B` differs from the whole band only outside `B`, which the view does not show.

## Step 4 — The chords of a run of samples lie near its middle sample

Each chord is at most `λ` long: an arc's chord is shorter than its arc, `r · |θ| / n`, the path along the arc being no shorter than the straight line between its ends (`REF-WIKI-TRIANGLE-INEQUALITY`, a polygon's sides refining the arc); a cubic's is at most `3 · max(|P₀P₁|, |P₁P₂|, |P₂P₃|) / n`, the curve's speed bounded by three times its control polygon's longest side (`REF-POMAX-BEZIER`, the derivative's weights; `pieceSamples`); a segment is its one chord, `λ` its length.

Take the samples from `i₀` to `i₁`, `i₁ − i₀ ≥ 2`, and `m = ⌊(i₀ + i₁) / 2⌋`. A point `Q` of the chord from `P_k` to `P_(k+1)`, `i₀ ≤ k < i₁`, reaches `P_m` by a path of chords: for `k ≥ m`, back along its own chord to `P_k`, at most `λ`, then `k − m` chords, at most `(k + 1 − m) · λ ≤ (i₁ − m) · λ`; for `k < m`, forward to `P_(k+1)`, then `m − k − 1` chords, at most `(m − k) · λ ≤ (m − i₀) · λ`. That path is no shorter than the straight line (`REF-WIKI-TRIANGLE-INEQUALITY`), so

`|Q − P_m| ≤ ρ = max(m − i₀, i₁ − m) · λ`.

If a chord of the run met `B_h` at `Q`, then `P_m` would lie within `ρ` of `Q`, so in `B_h` widened by `ρ` (step 2's argument, `ρ` for `h`). Contrapositive: **when `P_m` is not in `B_h` widened by `ρ`, no chord from `i₀` to `i₁` meets `B_h`**, and the whole run is left out without computing its samples.

## Step 5 — Cutting the runs by halves

For each piece: the run from `0` to `n`; a run of one chord is kept by step 3's test; a longer run is left out by step 4, or cut at `m` into the runs from `i₀` to `m` and from `m` to `i₁`, both shorter. Every chord meeting `B_h` lies in a run never left out, so it reaches a run of one and is kept; the chords are found in their order. A run far from `B_h` costs one sample.

The samples carry rounding errors (ADR-0003): step 4's box is widened by `EPSILON` more, so that a chord meeting `B_h` within a rounding error of `ρ` is never left out; a run kept for that `EPSILON` is then tested chord by chord, so the band is unchanged.

Research note 0031 planned an arc cut by the angles where it meets the box, and a cubic by de Casteljau halves left out by their control points' box. Halves at `t = ½` end between two samples when `n` is odd, so their chords would no longer be the whole band's, and an arc's angles need its crossings with four lines; cutting the samples' indices keeps the whole band's chords exactly, for every kind of piece, with one bound (agent's choice, EN-149).

## Degenerate cases

- **No piece**: no subpath.
- **A chord of length 0**: no band, as in the whole band (`segmentBand`); a piece's disc is still kept by its start.
- **A box thinner than the band**, or a point: `B_h` has the band's width at least; a chord meeting it is kept whole.
- **A piece tangent to `B_h`**: its chords lie on the circle's inner side; the chord through the point of contact meets `B_h` there and is kept, the others miss it and paint nothing of `B`.
- **A box holding the whole contour widened by `h`**: every subpath is kept, the band within `B` is the whole band.

## Cross-check

A box holding everything keeps every subpath, as the whole band. Step 3's two claims together: the band within `B` and the whole band paint the same points of `B`, and the band within `B` is the whole band's subpaths less some, so it paints nothing the whole band does not. Step 4 holds for a run of one chord too: `m = i₀`, `ρ = λ`, and the chord's points lie within `λ` of `P_i₀`.

## Checks (test cases)

The canvas is 1 200 × 800. R is the rounded rectangle 99 999 × 100 000 at (0, 0) of radius 100 000, its corners quarter circles of radius 49 999.5; its top-left corner runs from (0, 49 999.5) to (49 999.5, 0) about (49 999.5, 49 999.5), `Λ = 49 999.5 · π / 2 = 78 539.03`.

| Case | Computation | Expected |
| --- | --- | --- |
| the helps' box at the opening view | `(−40 − 600, −40 − 400)`, `(−40 + 1 800, −40 + 1 200)` | (−640, −440) to (1 760, 1 160) |
| at `s = 2`, `o = (180, 130)` | `(180 − 300, 130 − 200)`, `(180 + 900, 130 + 600)` | (−120, −70) to (1 080, 730) |
| a pan of `(600, 0)`, then of `(601, 0)`, from the opening view | `\|d_x\| ≤ 600` | kept; computed again |
| R's top-left corner at 100 %, `h = 1`, `s = 2` | `n = ⌈78 539.03 / 2⌉ = 39 270`, `λ = 1.99998`; `m = 19 635`, `ρ = 39 269.5`, `P_m = (14 644.5, 14 644.5)` within `B_h` widened, cut; `m = 9 817`, `ρ = 19 635.8`, `P_m = (3 805.6, 30 866.4)`, `30 866.4 > 1 161 + 19 635.8`; `m = 29 452`, `P_m = (30 864.6, 3 806.4)`, `30 864.6 > 1 761 + 19 635.8`; start (0, 49 999.5) outside `B_h` | no subpath; 3 samples of 39 271 |
| R's nearest point to the helps' box | `√(48 239.5² + 48 839.5²) − 49 999.5` from its corner (1 760, 1 160) | 18 647.0 units |
| R fitted, `s = 0.0072`, its corner | step `2 / 0.0072 = 277.78`; `⌈78 539.03 / 277.78⌉`; a chord `2 · 49 999.5 · sin(π / 1 132)` | 283 chords of 277.52 units, 1.998 pixels |
| the segment (0, 0) to (10, 0), `h = 1`, `s = 2`, `B` from (0, 0.2) to (0.5, 0.4) | `B_h` from (−1, −0.8) to (1.5, 1.4) holds (0, 0) | the disc and the rectangle, the whole band |
| the same, `B` from (20, 0) to (30, 1) | `B_h` from (19, −1) to (31, 2) | no subpath |
| the quarter arc about (0, 0) of radius 10 from (10, 0) to (0, 10), `h = 1`, `s = 4`, `B` from (11, −5) to (20, 5) | `n = ⌈15.708 / 4⌉ = 4`; `B_h` from (10, −6) to (21, 6) holds the start on its side; the chord from (10, 0) to (9.23880, 3.82683) meets it at (10, 0); the others lie left of x = 9.24 | the disc and the first chord's band |
