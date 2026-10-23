import { HELPS_FRAMES } from "./helps-frames";

import type { HelpsFrame } from "./helps-frame";

/**
 * Starts the frame a redraw of the layer above the drawing computes its helps in (SP-030's answer
 * 7, `DERIV-band-within-box` step 1): the box of the frame before, its bands kept for reuse, while
 * the view keeps its scale and the area shown lies within that box (`isBoxWithin`, `shownBox`);
 * otherwise a new box, the area shown widened by half the canvas on each side, no band kept. Each
 * redraw keeps only the bands it draws.
 *
 * @kind procedure
 * @param document - page document
 * @returns the frame started, kept for the page (`HELPS_FRAMES`)
 * @see docs/domain/interaction.md#_2-selector
 */
export function startHelpsFrame(document: Document): HelpsFrame {
  const frame: HelpsFrame = {
    bands: new Map(),
    box: { maxX: 0, maxY: 0, minX: 0, minY: 0 },
    previous: new Map(),
    scale: 1,
  };

  HELPS_FRAMES.set(document, frame);

  return frame;
}
