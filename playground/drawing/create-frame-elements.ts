import { createPathElement, screenToUser } from "../../src";
import { readView } from "../view";

import { readBoxPathData } from "./read-box-path-data";

import type { TurnedBox } from "../../src";

/** Width of the line drawing a frame, in screen pixels. */
const LINE = 1;

/**
 * Draws a box as a thin frame: a fill between the box and the box inset by its line, 1 screen
 * pixel divided by the view's scale (`screenToUser`, ADR-0098, decision 5), never a native stroke.
 * A box too small to hold a frame gets none, as the frame would cover what it holds.
 *
 * @kind procedure
 * @param document - page document
 * @param box - the box, turned with its angle
 * @param look - the class that colors the frame
 * @returns the frame, or nothing for a box 2 lines wide or high at most
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function createFrameElements(
  document: Document,
  box: TurnedBox,
  look: string,
): readonly SVGPathElement[] {
  const line = screenToUser(LINE, readView(document).scale);

  if (box.width <= 2 * line || box.height <= 2 * line) {
    return [];
  }

  const frame = createPathElement(
    document,
    `${readBoxPathData(box, 0)} ${readBoxPathData(box, line)}`,
  );

  frame.classList.add(look);

  return [frame];
}
