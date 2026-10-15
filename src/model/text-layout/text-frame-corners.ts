import { curveCornerOf } from "../handles";
import { turnedCorners } from "../placement";
import { rectangleCorners } from "../rectangle";

import { textFrame } from "./text-frame";

import type { Size } from "../../geometry";
import type { Point } from "../../math";
import type { Text } from "../text";

/** A sharp corner at each of the four vertices. */
const SHARP = [0, 0, 0, 0];

/**
 * The four corners of a text's frame as drawn — its block, or its box when it sits in one
 * (`textFrame`) — turned by its angle about the frame's middle, clockwise on screen: the
 * rectangle tool's corners (`rectangleCorners`) turned as a shape's are (`turnedCorners`,
 * `DERIV-turned-shape` step 4), so that a text and a shape turn by one route. At 0° they are the
 * frame's corners, bit for bit.
 *
 * @kind domain
 * @param text - its position, its box, whether it sits in it, and its angle
 * @param block - its measured block (`textBlock`)
 * @returns four points, clockwise from the frame's top-left before it turns (Q11)
 * @see DERIV-turned-shape
 */
export function textFrameCorners(text: Text, block: Size): readonly Point[] {
  const frame = textFrame(text, block);
  const corners = rectangleCorners({ ...frame, angle: 0, radii: SHARP });

  return turnedCorners(
    corners.map((corner) => curveCornerOf(corner)),
    { ...frame, angle: text.angle },
  ).map((corner) => corner.point);
}
