import { writePlacedBox } from "./write-placed-box";
import { writePlacedText } from "./write-placed-text";

import type { ShapePlacement } from "./shape-placement";

/**
 * Writes the box of a new shape placed by a click, or by a drag back on the press's integer
 * point: a text fitting its content, any other shape 100 × 100, centred on the point.
 *
 * @kind procedure
 * @param document - page document
 * @param placement - the tool armed and the point clicked
 */
export function writeClickedShape(document: Document, placement: ShapePlacement): void {
  if (placement.tool === "text") {
    writePlacedText(document, placement.point);
  } else {
    writePlacedBox(document, placement.point);
  }
}
