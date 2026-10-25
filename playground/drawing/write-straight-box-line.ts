import { boxSize, formatSvgNumber } from "../../src";
import { writeText } from "../page";

import type { BoundingBox } from "../../src";

/**
 * Writes the x, y, width and height of a straight box in the line under the position fields,
 * with at most 5 decimals (Q10): a shape's and a turned text's.
 *
 * @kind procedure
 * @param document - page document
 * @param box - straight box of the drawn shape or text
 */
export function writeStraightBoxLine(document: Document, box: BoundingBox): void {
  const size = boxSize(box);

  writeText(
    document,
    "straight-box",
    `Straight box: x ${formatSvgNumber(box.minX)}, y ${formatSvgNumber(box.minY)}, width ${formatSvgNumber(size.width)}, height ${formatSvgNumber(size.height)}`,
  );
}
