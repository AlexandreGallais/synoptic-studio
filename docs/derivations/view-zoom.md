# DERIV-view-zoom — The view of the drawing: its `viewBox`, the point under the pointer, a zoom about a point, a pan, a step and a fit

Used by: EN-148 (F42), `src/model/view/`; the view as ADR-0098 sets it. Sources: `REF-SVG2-COORDS` (§8.2, the transform a `viewBox` gives), `REF-WIKI-CLAMP` (a number limited to a range), `REF-PENPOT-ZOOM` (a zoom about a point keeping it, the fits), `REF-PENPOT-VIEWPORT-ACTIONS` (the wheel's zoom and pan by its deltas), `REF-INKSCAPE-ZOOM-PREFS` (the step × √2, the bounds 0.01 and 256, `zoom_relative` keeping a point); research note 0031, §4; SP-030's answers 4, 5 and 6.

No single source states the zoom about a point as a formula: §8.2 gives the map from the drawing to the canvas, and the editors keep a point while they zoom without writing it. This page draws each formula from that map.

## Notation

Frame of the SVG: x to the right, y down. Lengths on the canvas are CSS pixels; lengths in the drawing are user units.

| Symbol | Definition |
| --- | --- |
| `s` | the view's scale, CSS pixels per user unit, `s > 0` |
| `o` | the view's origin, the point of the drawing at the canvas's top-left, in user units |
| `W × H` | the canvas's size, in CSS pixels |
| `p` | an offset on the canvas from its top-left, in CSS pixels (the pointer's) |
| `P` | a point of the drawing, in user units |
| `s₋, s₊` | the bounds of the scale, 0.01 and 256 |
| `m` | a fit's padding on each side, in CSS pixels |

## Step 1 — The `viewBox` of a view

An element of size `e-width × e-height` at `(e-x, e-y)` given the `viewBox` `(vb-x, vb-y, vb-width, vb-height)` draws its content by `translate(e-x − vb-x · scale-x, e-y − vb-y · scale-y) scale(scale-x, scale-y)`, "Initialize scale-x to e-width/vb-width", scale-y likewise; with an alignment other than `none`, `meet` is to "set the larger of scale-x and scale-y to the smaller", then adds to the translation half of what is left, `(e-width − vb-width × scale-x) / 2` for `xMid` (`REF-SVG2-COORDS`, §8.2, steps 5 to 14).

The view `(s, o)` in a canvas `W × H` at `(0, 0)` is given the `viewBox`

`(o.x, o.y, W / s, H / s)`.

Then `scale-x = W / (W / s) = s` and `scale-y = H / (H / s) = s`: one scale, `meet` changes nothing, and `W − (W / s) · s = 0` leaves nothing to centre, whatever `preserveAspectRatio`'s alignment. The transform is `translate(−o.x · s, −o.y · s) scale(s)`: a point `P` of the drawing is drawn on the canvas at

`p = (P − o) · s`.

## Step 2 — The point under the pointer

Step 1's map is a scale by `s > 0` followed by a translation, so it has an inverse: from `p = (P − o) · s`, dividing by `s` and adding `o`,

`P = o + p / s`.

## Step 3 — A zoom about a point

A zoom to the scale `s′` about the offset `p` keeps under `p` the point `P` that was there: `P = o + p / s` before, and the new origin `o′` must give `P = o′ + p / s′` after (step 2). So

`o′ = P − p / s′ = o + p / s − p / s′`.

Penpot scales its `vbox` about a centre by the old zoom over the new (`REF-PENPOT-ZOOM`); the `vbox`'s corner `o` moves to `c − (c − o) · s / s′`, `c = o + p / s` the centre, which is `o + p / s − (p / s) · s / s′ = o + p / s − p / s′`, the same origin. Inkscape's `zoom_relative(c, zoom, keep_point)` keeps a point the same way (`REF-INKSCAPE-ZOOM-PREFS`).

The same formula places any point `P` at any offset `p` at a scale `s′`: `o′ = P − p / s′`, which step 6 uses.

## Step 4 — The bounds

The scale asked, `s′`, is first limited to `[s₋, s₊] = [0.01, 256]`, 1 % to 25 600 %, Inkscape's `SP_DESKTOP_ZOOM_MIN` and `SP_DESKTOP_ZOOM_MAX` (`REF-INKSCAPE-ZOOM-PREFS`; SP-030's answer 5): `min(max(s′, s₋), s₊)`, the clamp (`REF-WIKI-CLAMP`). Step 3 is then made with the scale limited, so that the point under `p` is kept at the bound too.

## Step 5 — A step, and the wheel's zoom

A key's step multiplies the scale by √2 to zoom in and divides it by √2 to zoom out, Inkscape's `zoomincrement` 1.414213562 (`REF-INKSCAPE-ZOOM-PREFS`; SP-030's answer 4), then limits it (step 4): 1, 1.41421, 2, 2.82843, 4.

The wheel and the pinch zoom by their vertical deltas, by Penpot's factor: with `δ` the sum in pixels of the vertical deltas sent before a frame, a zoom "by 1 + 0.0057 per pixel", "its inverse for a positive sum" (`REF-PENPOT-VIEWPORT-ACTIONS`; SP-030's answer 4). Penpot computes the factor of each event from its vertical and horizontal deltas added, and multiplies the factors of a frame; the vertical deltas alone, summed over a frame before one factor, are the agent's choice (US-078, `interaction.md` §7): two of −50 give `k(−100) = 1.57`, where Penpot's product gives `1.285² = 1.65122`.

`k(δ) = 1 + 0.0057 · |δ|` for `δ ≤ 0`, `k(δ) = 1 / (1 + 0.0057 · δ)` for `δ > 0`.

A wheel turned toward the screen, `δ < 0`, zooms in; `k(δ) · k(−δ) = 1`, so a turn back gives the scale back; `k(0) = 1`.

## Step 6 — A pan

A drag of `d` screen pixels moves the drawing with the pointer: the point under the press, `P = o + p / s`, stays under the pointer at `p + d`, so `o′ + (p + d) / s = o + p / s` (step 2), and

`o′ = o − d / s`.

The wheel's deltas `δ`, in pixels, move the view the other way, as a scroll moves a page, Penpot's "pan by the deltas divided by the zoom" (`REF-PENPOT-VIEWPORT-ACTIONS`):

`o′ = o + δ / s`,

the drag being the pan by `−d`. The scale does not change.

## Step 7 — A fit to a box

A straight box `[minX, maxX] × [minY, maxY]`, `w = maxX − minX`, `h = maxY − minY`, its centre `c` the midpoint of its corners, is fitted when it is drawn whole within the canvas less `m` on each side, as large as it can be, centred. Drawn at the scale `s`, it is `w · s × h · s` pixels (step 1), within `(W − 2m) × (H − 2m)` when `s ≤ (W − 2m) / w` and `s ≤ (H − 2m) / h`; the largest such scale is

`s = min((W − 2m) / w, (H − 2m) / h)`,

then limited (step 4). Its centre goes to the canvas's centre `(W / 2, H / 2)`: by step 3's placement, `o = c − (W / 2, H / 2) / s`. Penpot fits all the shapes with a padding of 160 and the selection with 40 (`REF-PENPOT-ZOOM`); this project leaves 40 for both (SP-030's answer 6).

Degenerate boxes (agent's choice, EN-148):

- **0 wide, or 0 high**: its side of 0 bounds no scale, `0 · s` fitting any width; the scale is the other side's alone.
- **0 × 0**: no side bounds the scale, which stays the view's; the point is centred.
- **A canvas narrower than `2m`**: `W − 2m ≤ 0` gives a scale at most 0, which step 4 takes to `s₋`.

## Cross-check

Step 2 inverts step 1: `(o + p / s − o) · s = p`. A zoom to the same scale changes nothing: `s′ = s` gives `o′ = o + p / s − p / s = o`. A zoom keeps the point: `o′ + p / s′ = o + p / s` (step 3). A pan by `δ` then `−δ` gives `o` back. A fit draws the box's left side at `(minX − o.x) · s = (minX − c.x) · s + W / 2 = W / 2 − w · s / 2 ≥ W / 2 − (W − 2m) / 2 = m` pixels, the box within the padding.

## Checks (test cases)

The canvas is 1 200 × 800, the opening view `s = 1`, `o = (−40, −40)` (ADR-0098).

| Case | Computation | Expected |
| --- | --- | --- |
| the opening view's `viewBox` | `(−40, −40, 1 200 / 1, 800 / 1)` | `(−40, −40, 1 200, 800)` |
| `s = 2`, `o = (180, 130)`, its `viewBox` | `(180, 130, 1 200 / 2, 800 / 2)` | `(180, 130, 600, 400)` |
| `s = 2`, `o = (180, 130)`, `p = (440, 340)` | `(180 + 440 / 2, 130 + 340 / 2)` | `(400, 300)` |
| the opening view, `p = (40, 40)` | `(−40 + 40, −40 + 40)` | `(0, 0)` |
| the opening view, `p = (440, 340)`, to `s′ = 2` | `P = (400, 300)`; `(400 − 440 / 2, 300 − 340 / 2)` | `s = 2`, `o = (180, 130)` |
| the same, `s′ = 300` | limited to 256; `(400 − 440 / 256, 300 − 340 / 256)` | `s = 256`, `o = (398.28125, 298.67188)` |
| the same, `s′ = 0.001` | limited to 0.01; `(400 − 44 000, 300 − 34 000)` | `s = 0.01`, `o = (−43 600, −33 700)` |
| a step up from 1; from 256; a step down from 1; from 0.01 | `1 · √2`; `min(256 · √2, 256)`; `1 / √2`; `max(0.01 / √2, 0.01)` | 1.41421; 256; 0.70711; 0.01 |
| the wheel, `δ = −100`; `δ = 100`; `δ = −10`; `δ = 0` | `1 + 0.57`; `1 / 1.57`; `1 + 0.057`; `1` | 1.57; 0.63694; 1.057; 1 |
| `s = 2`, `o = (180, 130)`, a drag of `(100, 50)` | `(180 − 100 / 2, 130 − 50 / 2)` | `(130, 105)` |
| `s = 2`, `o = (180, 130)`, the wheel's `(0, 100)` | `(180 + 0 / 2, 130 + 100 / 2)` | `(180, 180)` |
| the box from `(10, 20)` to `(410, 220)`, `m = 40` | `min(1 120 / 400, 720 / 200) = min(2.8, 3.6)`; `c = (210, 120)`; `(210 − 600 / 2.8, 120 − 400 / 2.8)` | `s = 2.8`, `o = (−4.28571, −22.85714)` |
| the box from `(10, 20)` to `(10, 220)`, `m = 40` | `720 / 200`; `c = (10, 120)`; `(10 − 600 / 3.6, 120 − 400 / 3.6)` | `s = 3.6`, `o = (−156.66667, 8.88889)` |
| the box of the point `(50, 60)`, `s = 2` | the view's scale; `(50 − 600 / 2, 60 − 400 / 2)` | `s = 2`, `o = (−250, −140)` |
| the box from `(0, 0)` to `(1, 1)`, `m = 40` | `min(1 120, 720) = 720`, limited to 256; `(0.5 − 600 / 256, 0.5 − 400 / 256)` | `s = 256`, `o = (−1.84375, −1.0625)` |
| a canvas 60 × 800, the box from `(10, 20)` to `(410, 220)`, `m = 40` | `min(−20 / 400, 720 / 200) = −0.05`, limited to 0.01; `(210 − 30 / 0.01, 120 − 400 / 0.01)` | `s = 0.01`, `o = (−2 790, −39 880)` |
