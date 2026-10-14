import { createPathElement, rectangleToPathData } from "../../src";

import type { Size, Text } from "../../src";

/** Width of the line drawing the block's frame, in user units. */
const LINE = 1;

/**
 * Draws the measured block of a text as a thin frame: a fill between the block and the block
 * inset by one unit, never a native stroke. A block too small to hold a frame gets none, as the
 * frame would cover the text.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text, for the top-left corner of its block
 * @param block - its measured block
 * @returns the frame, or nothing for a block 2 units wide or high at most
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function createBlockFrameElements(
  document: Document,
  text: Text,
  block: Size,
): readonly SVGPathElement[] {
  if (block.width <= 2 * LINE || block.height <= 2 * LINE) {
    return [];
  }

  const outer = rectangleToPathData({ ...block, x: text.x, y: text.y });
  const inner = rectangleToPathData({
    height: block.height - 2 * LINE,
    width: block.width - 2 * LINE,
    x: text.x + LINE,
    y: text.y + LINE,
  });
  const frame = createPathElement(document, `${outer} ${inner}`);

  frame.classList.add("text-block");

  return [frame];
}
