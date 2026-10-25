import { writeInputValue } from "../page";

import { writeStraightBoxLine } from "./write-straight-box-line";

import type { BoundingBox } from "editor";

/**
 * Writes a value in a field, unless the field is being typed in.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the field
 * @param value - value to show
 */
function writeUntypedValue(document: Document, id: string, value: number): void {
  if (document.activeElement?.id !== id) {
    writeInputValue(document, id, value);
  }
}

/**
 * Shows the straight box of the shape: its x and y in the position fields, as in Inkscape,
 * and its x, y, width and height in a line under them (`writeStraightBoxLine`).
 *
 * @kind procedure
 * @param document - page document
 * @param box - straight box of the drawn shape
 */
export function writeStraightBox(document: Document, box: BoundingBox): void {
  writeUntypedValue(document, "shape-x", box.minX);
  writeUntypedValue(document, "shape-y", box.minY);
  writeStraightBoxLine(document, box);
}
