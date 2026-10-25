import { textFrameCorners } from "../../src";

import type { Point, SizedText } from "../../src";

/**
 * Reads where a text's top-left is drawn: the top-left corner of its frame, turned by its angle
 * about the frame's middle (`textFrameCorners`, `DERIV-turned-shape` step 4), the point a turned
 * text that grows keeps in place (ADR-0057). At 0° it is the text's position, bit for bit.
 *
 * @kind procedure
 * @param sized - a text and its measured block
 * @returns its turned top-left corner, in the SVG
 * @see US-054
 */
export function readTurnedCorner(sized: SizedText): Point {
  const [corner = sized.text] = textFrameCorners(sized.text, sized.block);

  return { x: corner.x, y: corner.y };
}
