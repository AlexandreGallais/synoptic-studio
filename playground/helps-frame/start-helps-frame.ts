import { isBoxWithin, shownBox } from "../../src";
import { readCanvasSize, readView } from "../view";

import { HELPS_FRAMES } from "./helps-frames";

import type { HelpsFrame } from "./helps-frame";
import type { Size, View } from "../../src";

/** The share of the canvas the helps are computed beyond the area shown on each side (SP-030's answer 7). */
const PADDING_SHARE = 0.5;

/** The frame before the first redraw: a scale no view has, so that the first redraw starts a box. */
const NO_FRAME: HelpsFrame = {
  bands: new Map(),
  box: { maxX: 0, maxY: 0, minX: 0, minY: 0 },
  previous: new Map(),
  scale: 0,
};

/**
 * A frame of a new box for a view: the area it shows widened by half the canvas on each side
 * (`shownBox`), no band kept.
 *
 * @kind procedure
 * @param view - the view drawn
 * @param canvas - the canvas's size, in CSS pixels
 * @returns its box and scale, its bands empty
 * @see docs/domain/interaction.md#_2-selector
 */
function readNewFrame(view: View, canvas: Size): HelpsFrame {
  const padding = { height: canvas.height * PADDING_SHARE, width: canvas.width * PADDING_SHARE };

  return {
    bands: new Map(),
    box: shownBox(view, canvas, padding),
    previous: new Map(),
    scale: view.scale,
  };
}

/**
 * The frame that follows another for a view: the same box, the bands before kept for reuse, when
 * drawn at the same scale with the area the view shows within that box (`isBoxWithin`, `shownBox`
 * with no padding); a new box otherwise (`readNewFrame`).
 *
 * @kind procedure
 * @param kept - the frame before
 * @param view - the view drawn
 * @param canvas - the canvas's size, in CSS pixels
 * @returns its box and scale, its bands empty
 * @see docs/domain/interaction.md#_2-selector
 */
function readNextFrame(kept: HelpsFrame, view: View, canvas: Size): HelpsFrame {
  return kept.scale === view.scale &&
    isBoxWithin(shownBox(view, canvas, { height: 0, width: 0 }), kept.box)
    ? { bands: new Map(), box: kept.box, previous: kept.bands, scale: view.scale }
    : readNewFrame(view, canvas);
}

/**
 * Starts the frame a redraw of the layer above the drawing computes its helps in (SP-030's answer
 * 7, `DERIV-band-within-box` step 1): the box of the frame before, its bands kept for reuse, while
 * the view keeps its scale and the area shown lies within that box (`readNextFrame`); otherwise a new
 * box, the area shown widened by half the canvas on each side, no band kept (`readNewFrame`). Each
 * redraw keeps only the bands it draws.
 *
 * @kind procedure
 * @param document - page document
 * @returns the frame started, kept for the page (`HELPS_FRAMES`)
 * @see docs/domain/interaction.md#_2-selector
 */
export function startHelpsFrame(document: Document): HelpsFrame {
  const kept = HELPS_FRAMES.get(document) ?? NO_FRAME;
  const frame = readNextFrame(kept, readView(document), readCanvasSize(document));

  HELPS_FRAMES.set(document, frame);

  return frame;
}
