import { hasArea, textFrame, textStraightBox } from "../../src";
import { writeStraightBoxLine } from "../drawing";
import { selectElement } from "../page";

import type { Size, Text } from "../../src";

/**
 * Shows the x, y, width and height of a turned text's straight box under the position fields, as
 * for a shape (`writeStraightBoxLine`); the position fields show its turned top-left corner
 * (US-054). A text at 0°, or a frame without area, shows no line, as before (US-039).
 *
 * @kind procedure
 * @param document - page document
 * @param text - its position, its box and its angle
 * @param block - its measured block, from `textDrawing`
 * @see docs/backlog/stories/E15-F14-US-039-straight-box-of-a-turned-text.md
 */
export function writeTextStraightBox(document: Document, text: Text, block: Size): void {
  const isShown = text.angle !== 0 && hasArea(textFrame(text, block));

  selectElement(document, "straight-box").hidden = !isShown;

  if (isShown) {
    writeStraightBoxLine(document, textStraightBox(text, block));
  }
}
