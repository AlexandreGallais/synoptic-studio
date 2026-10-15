import { boundingBox } from "../../geometry";

import { textFrameCorners } from "./text-frame-corners";

import type { BoundingBox, Size } from "../../geometry";
import type { Text } from "../text";

/**
 * The straight box of a text: the smallest upright rectangle around its frame as drawn, its block
 * or its box turned by its angle (`textFrameCorners`), as SVG's `getBBox` gives for its group
 * (`docs/domain/shapes.md`, Text, Turning; `DERIV-turned-shape` step 4). It is taken around the
 * frame, not the letters' outlines. At 0° it is the frame itself; a frame without area gives a box
 * without area.
 *
 * @kind domain
 * @param text - its position, its box, whether it sits in it, and its angle
 * @param block - its measured block (`textBlock`)
 * @returns the extreme coordinates of its turned frame
 * @see DERIV-turned-shape
 */
export function textStraightBox(text: Text, block: Size): BoundingBox {
  return boundingBox(textFrameCorners(text, block));
}
