import { lineOffsetInBlock } from "./line-offset-in-block";

import type { TextAlignment, TextVerticalAlignment } from "../text";

/** Each vertical alignment read as the horizontal one that leaves the same share of room before. */
const ACROSS: Readonly<Record<TextVerticalAlignment, TextAlignment>> = {
  bottom: "right",
  middle: "center",
  top: "left",
};

/**
 * How far a block of lines starts below its box's top: the block is placed down the box by the
 * room it leaves, as a line is placed across its block (`lineOffsetInBlock`): at the top, none of
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
  return lineOffsetInBlock(boxHeight, blockHeight, ACROSS[verticalAlignment]);
}
