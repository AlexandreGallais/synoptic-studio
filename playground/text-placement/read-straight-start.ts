import { textStraightBox } from "../../src";

import type { Point, SizedText } from "../../src";

/**
 * Reads where a text's straight box starts: the top-left of the smallest upright box around its
 * frame as drawn, turned by its angle (`textStraightBox`), what X and Y show, as a shape's
 * (Product Owner, 2026-10-23). At 0° it is the text's position, bit for bit.
 *
 * @kind procedure
 * @param sized - a text and its measured block
 * @returns the top-left of its straight box, in the SVG
 * @see RULE-shapes-scripts-and-turning
 */
export function readStraightStart(sized: SizedText): Point {
  const box = textStraightBox(sized.text, sized.block);

  return { x: box.minX, y: box.minY };
}
