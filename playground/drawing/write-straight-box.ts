import { boxSize, formatSvgNumber } from "../../src";
import { writeInputValue, writeText } from "../page";

import type { BoundingBox } from "../../src";

/**
 * Writes a value in a field, unless the field is being typed in.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the field
 * @param value - value to show
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function writeUntypedValue(document: Document, id: string, value: number): void {
  if (document.activeElement?.id !== id) {
    writeInputValue(document, id, value);
  }
}

/**
 * Shows the straight box of the shape (F04): its x and y in the position fields, as in Inkscape,
 * and its x, y, width and height in a line under them, written with at most 5 decimals (Q10).
 *
 * @kind procedure
 * @param document - page document
 * @param box - straight box of the drawn shape
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function writeStraightBox(document: Document, box: BoundingBox): void {
  writeUntypedValue(document, "shape-x", box.minX);
  writeUntypedValue(document, "shape-y", box.minY);

  const x = formatSvgNumber(box.minX);
  const y = formatSvgNumber(box.minY);
  const size = boxSize(box);
  const width = formatSvgNumber(size.width);
  const height = formatSvgNumber(size.height);

  writeText(
    document,
    "straight-box",
    `Straight box: x ${x}, y ${y}, width ${width}, height ${height}`,
  );
}
