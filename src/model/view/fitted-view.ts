import { boxSize, halfSize } from "../../geometry";
import { midpoint } from "../../math";

import { boundedScale } from "./bounded-scale";
import { viewOrigin } from "./view-origin";

import type { FitFrame } from "./fit-frame";
import type { View } from "./view";
import type { BoundingBox } from "../../geometry";

/**
 * The view fitting a straight box within the canvas less a padding on each side, centred, its
 * scale within the bounds (SP-030's answer 6). A side of 0 bounds no scale; a box of 0 × 0 keeps
 * the view's scale.
 *
 * Formula: s = min((W − 2m) / w, (H − 2m) / h), within the bounds; o = c − (W / 2, H / 2) / s.
 *
 * @kind domain
 * @param view - its scale and origin before the fit, the scale kept for a box of 0 × 0
 * @param box - the straight box shown whole, in user units
 * @param frame - the canvas's size and the padding on each side, in CSS pixels
 * @returns the view showing the box centred
 * @see DERIV-view-zoom
 */
export function fittedView(view: View, box: BoundingBox, frame: FitFrame): View {
  const size = boxSize(box);
  const fits = [
    { length: size.width, room: frame.canvas.width - 2 * frame.padding },
    { length: size.height, room: frame.canvas.height - 2 * frame.padding },
  ]
    .filter(({ length }) => length > 0)
    .map(({ length, room }) => room / length);
  const fitted = fits.length > 0 ? boundedScale(Math.min(...fits)) : view.scale;
  const center = midpoint({ x: box.minX, y: box.minY }, { x: box.maxX, y: box.maxY });

  return { origin: viewOrigin(center, halfSize(frame.canvas), fitted), scale: fitted };
}
