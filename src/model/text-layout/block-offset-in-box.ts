import { alignedOffset } from "../../geometry";

import type { Alignment } from "../../geometry";
import type { TextVerticalAlignment } from "../text";

/** Each vertical alignment of a block as the place it takes on its box's height. */
const ON_AXIS: Readonly<Record<TextVerticalAlignment, Alignment>> = {
  bottom: "max",
  middle: "mid",
  top: "min",
};

/**
 * How far a block of lines starts below its box's top: the block is placed down the box by the
 * room it leaves, as a line is placed across its block (`alignedOffset`): at the top, none of
 * it above; in the middle, half; at the bottom, all of it (`DERIV-text-box`, step 8). A block
 * taller than its box leaves a negative room: in the middle, it overflows both edges equally.
 *
 * Formula: (H − h) × v, v = 0, ½ or 1 for top, middle or bottom.
 *
 * @kind domain
 * @param boxHeight - how tall the box is, in user units
 * @param blockHeight - how tall its block of lines is (`blockHeight`)
 * @param verticalAlignment - top, middle or bottom
 * @returns the distance from the box's top down to the block's, in user units
 * @see DERIV-text-box
 */
export function blockOffsetInBox(
  boxHeight: number,
  blockHeight: number,
  verticalAlignment: TextVerticalAlignment,
): number {
  return alignedOffset(boxHeight - blockHeight, ON_AXIS[verticalAlignment]);
}
