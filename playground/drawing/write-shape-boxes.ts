import { writeDrawnBox } from "./write-drawn-box";
import { writeStraightBox } from "./write-straight-box";

import type { BoundingBox, TurnedBox } from "../../src";

/**
 * Shows the straight box of the shape just drawn and keeps its turned box for the next edit
 * (F04).
 *
 * @kind procedure
 * @param document - page document
 * @param straight - straight box of the drawn shape
 * @param turned - angle, position and size of the shape
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
export function writeShapeBoxes(
  document: Document,
  straight: BoundingBox,
  turned: TurnedBox,
): void {
  writeStraightBox(document, straight);
  writeDrawnBox(document, turned);
}
