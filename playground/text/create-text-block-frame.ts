import { SVG_NAMESPACE, add, boundingBox, formatRotation, hasArea } from "../../src";
import { createStraightBoxElement } from "../drawing";

import type { DrawnText, Text } from "../../src";

/**
 * Draws the measured block of the selected text as a shape's box is drawn, dashed by thin filled
 * rectangles (`createStraightBoxElement`), in a `<g>` turned as the text is, about the middle of
 * its block; an empty block, with no area, gets no frame.
 *
 * @kind procedure
 * @param document - page document
 * @param text - its position and angle
 * @param drawn - its block and the middle of its block, from `textDrawing`
 * @returns the turned group holding the dashes, or nothing for a block without area
 * @see docs/backlog/stories/E15-F05-US-032-turn-a-text.md
 */
export function createTextBlockFrame(
  document: Document,
  text: Text,
  drawn: DrawnText,
): readonly SVGGElement[] {
  if (!hasArea(drawn.block)) {
    return [];
  }

  const position = { x: text.x, y: text.y };
  const corner = add(position, { x: drawn.block.width, y: drawn.block.height });
  const group = document.createElementNS(SVG_NAMESPACE, "g");

  group.classList.add("text-block");
  group.setAttribute("transform", formatRotation(text.angle, drawn.center));
  group.append(createStraightBoxElement(document, boundingBox([position, corner])));

  return [group];
}
