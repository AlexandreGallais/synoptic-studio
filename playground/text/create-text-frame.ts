import { SVG_NAMESPACE, add, boundingBox, formatRotation, hasArea, textFrame } from "../../src";
import { createStraightBoxElement } from "../drawing";

import type { DrawnText, Text } from "../../src";

/**
 * Draws the frame of the selected text as a shape's box is drawn, dashed by thin filled
 * rectangles (`createStraightBoxElement`), in a `<g>` turned as the text is, about its middle:
 * its measured block when it fits its content, its box when it sits in one (`textFrame`, US-033).
 * A frame without area, an empty block or a box 0 wide or 0 tall, is not drawn.
 *
 * @kind procedure
 * @param document - page document
 * @param text - its position, its box and its angle
 * @param drawn - its block and its middle, from `textDrawing`
 * @returns the turned group holding the dashes, or nothing for a frame without area
 * @see docs/backlog/stories/E15-F14-US-033-put-a-text-in-a-box.md
 */
export function createTextFrame(
  document: Document,
  text: Text,
  drawn: DrawnText,
): readonly SVGGElement[] {
  const frame = textFrame(text, drawn.block);

  if (!hasArea(frame)) {
    return [];
  }

  const position = { x: frame.x, y: frame.y };
  const corner = add(position, { x: frame.width, y: frame.height });
  const group = document.createElementNS(SVG_NAMESPACE, "g");

  group.classList.add("text-frame");
  group.setAttribute("transform", formatRotation(text.angle, drawn.center));
  group.append(createStraightBoxElement(document, boundingBox([position, corner])));

  return [group];
}
