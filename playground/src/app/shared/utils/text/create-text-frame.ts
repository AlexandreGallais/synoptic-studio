import {
  SVG_NAMESPACE,
  add,
  boundingBox,
  formatRotation,
  hasArea,
  textFrame,
  textStraightBox,
} from "editor";

import { createFrameElements, createStraightBoxElement } from "../drawing";

import type { DrawnText, Text } from "editor";

/**
 * Draws the frame of the selected text, its measured block when it fits its content, its box
 * when it sits in one (`textFrame`), in a `<g>` of class `text-frame` turned as the text
 * is, about its middle, and as a shape's boxes are drawn: upright, dashed by thin filled
 * rectangles (`createStraightBoxElement`); turned, as a turned shape's, a thin blue frame
 * (`createFrameElements`, `turned-box`), with its straight box behind it, upright and dashed
 * (`textStraightBox`). A frame without area, an empty block or a box 0 wide or 0 tall, is
 * not drawn, nor its straight box.
 *
 * @kind procedure
 * @param document - page document
 * @param text - its position, its box and its angle
 * @param drawn - its block and its middle, from `textDrawing`
 * @returns the dashed frame of an upright text; the straight box and the turned frame of a turned
 * one; nothing for a frame without area
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

  if (text.angle === 0) {
    group.append(createStraightBoxElement(document, boundingBox([position, corner])));

    return [group];
  }

  group.append(...createFrameElements(document, { ...frame, angle: 0 }, "turned-box"));

  return [createStraightBoxElement(document, textStraightBox(text, drawn.block)), group];
}
