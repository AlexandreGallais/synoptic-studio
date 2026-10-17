import { createPathElement } from "../../src";

import { readBoxPathData } from "./read-box-path-data";

import type { TurnedBox } from "../../src";

/** Width of the line drawing a frame, in user units. */
const LINE = 1;

/**
 * Draws a box as a thin frame: a fill between the box and the box inset by one unit, never a
 * native stroke. A box too small to hold a frame gets none, as the frame would cover what it
 * holds.
 *
 * @kind procedure
 * @param document - page document
 * @param box - the box, turned with its angle
 * @param look - the class that colors the frame
 * @returns the frame, or nothing for a box 2 units wide or high at most
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function createFrameElements(
  document: Document,
  box: TurnedBox,
  look: string,
): readonly SVGPathElement[] {
  if (box.width <= 2 * LINE || box.height <= 2 * LINE) {
    return [];
  }

  const frame = createPathElement(
    document,
    `${readBoxPathData(box, 0)} ${readBoxPathData(box, LINE)}`,
  );

  frame.classList.add(look);

  return [frame];
}
