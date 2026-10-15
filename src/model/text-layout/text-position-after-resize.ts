import { positionAfterResize } from "../placement";

import { textFrame } from "./text-frame";

import type { SizedText } from "./sized-text";
import type { Point } from "../../math";

/**
 * Position of a turned text after its frame changes size — its block when it fits its content,
 * its box otherwise (`textFrame`) — so that its turned top-left corner keeps its place on the
 * drawing, the text still turning about its new frame's middle (ADR-0057): the rule of a width
 * typed on a turned shape (`positionAfterResize`, `DERIV-turned-shape` step 5). A frame that keeps
 * its size, a text in a box whose spacing changes, and a text at 0° keep their position as it is.
 *
 * Formula: b' = P − k' + R(k'), P = b + k − R(k) the turned top-left corner.
 *
 * @kind domain
 * @param before - the text and its block before the change, its position and its angle
 * @param after - the text and its block after the change, at the same angle
 * @returns the top-left of its new frame before it turns
 * @see DERIV-turned-shape
 */
export function textPositionAfterResize(before: SizedText, after: SizedText): Point {
  const frame = textFrame(before.text, before.block);
  const resized = textFrame(after.text, after.block);

  return frame.width === resized.width && frame.height === resized.height
    ? { x: frame.x, y: frame.y }
    : positionAfterResize(
        { ...frame, angle: before.text.angle },
        { height: resized.height, width: resized.width },
      );
}
