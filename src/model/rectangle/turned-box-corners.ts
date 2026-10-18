import { curveCornerOf } from "../handles";
import { turnedCorners } from "../placement";

import { rectangleCorners } from "./rectangle-corners";

import type { CurveCorner } from "../../geometry";
import type { TurnedBox } from "../placement";

/** A sharp corner at each of the four vertices. */
const SHARP = [0, 0, 0, 0];

/**
 * The four sharp corners of a box, turned as a shape is turned (`turnedCorners`,
 * `DERIV-turned-shape` step 4): the rectangle tool's corners (`rectangleCorners`), so that every
 * box drawn or measured turned (a text's frame, the playground's frames) takes one route. The box
 * turns by the angle of `turn`, about the middle of `turn`; at 0° its corners are returned as
 * they are, bit for bit.
 *
 * @kind domain
 * @param box - position, width and height of the box before it turns
 * @param turn - the angle, and the box whose middle it turns about
 * @returns four corners, clockwise from the box's top-left before it turns (Q11), handles off
 * @see DERIV-turned-shape
 */
export function turnedBoxCorners(
  box: Omit<TurnedBox, "angle">,
  turn: TurnedBox,
): readonly CurveCorner[] {
  const corners = rectangleCorners({ ...box, angle: 0, radii: SHARP });

  return turnedCorners(
    corners.map((corner) => curveCornerOf(corner)),
    turn,
  );
}
