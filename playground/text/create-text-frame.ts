import {
  SVG_NAMESPACE,
  add,
  boundingBox,
  formatRotation,
  hasArea,
  textFrame,
  textStraightBox,
} from "../../src";
import { createStraightBoxElement } from "../drawing";

import type { DrawnText, Text } from "../../src";

/**
 * Draws the frame of the selected text as a shape's box is drawn, dashed by thin filled
 * rectangles (`createStraightBoxElement`), in a `<g>` turned as the text is, about its middle:
 * its measured block when it fits its content, its box when it sits in one (`textFrame`, US-033).
 * A turned text also gets its straight box, upright and dashed the same way, behind its frame, as
 * a turned shape does (`textStraightBox`, US-039). A frame without area, an empty block or a box
 * 0 wide or 0 tall, is not drawn, nor its straight box.
 *
 * @kind procedure
 * @param document - page document
 * @param text - its position, its box and its angle
 * @param drawn - its block and its middle, from `textDrawing`
 * @returns the straight box of a turned text, then the turned group holding the frame's dashes;
 * nothing for a frame without area
 * @see docs/backlog/stories/E15-F14-US-039-straight-box-of-a-turned-text.md
 */
export function createTextFrame(
  document: Document,
  text: Text,
  drawn: DrawnText,
): readonly SVGElement[] {
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

  return text.angle === 0
    ? [group]
    : [createStraightBoxElement(document, textStraightBox(text, drawn.block)), group];
}
