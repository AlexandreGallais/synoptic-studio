import type { BoundingBox } from "../../src";

/**
 * The box the helps of the layer above the drawing are computed within, and the bands computed in
 * it (SP-030's answer 7, `DERIV-band-within-box` step 1): kept while the view keeps its scale and
 * the area shown stays within the box, so that a pan changes only the `viewBox`.
 */
export type HelpsFrame = {
  /** The bands' path data drawn by this redraw, by their contour and size. */
  readonly bands: ReadonlyMap<string, string>;
  /** The area shown widened by half the canvas on each side, when the frame was started, in user units. */
  readonly box: BoundingBox;
  /** The bands' path data drawn by the redraw before, by their contour and size: none for a new box. */
  readonly previous: ReadonlyMap<string, string>;
  /** The view's scale the box was computed at. */
  readonly scale: number;
};
