import { alignedOffset } from "../../geometry";

import type { Alignment } from "../../geometry";
import type { TextAlignment } from "../text";

/** Each alignment of a line as the place it takes on its axis (`REF-CSS-TEXT-3`, §6.1). */
const ON_AXIS: Readonly<Record<TextAlignment, Alignment>> = {
  center: "mid",
  left: "min",
  right: "max",
};

/**
 * How far a line starts from its block's left edge: aligned to the left edge, none of the room
 * the line leaves in the block goes before it; centered, half; aligned to the right edge, all of
 * it (`alignedOffset`, `DERIV-text-width`, step 5). The alignment is physical, whatever the
 * direction.
 *
 * Formula: (B − w) × a, a = 0, ½ or 1 for left, center or right.
 *
 * @kind domain
 * @param blockWidth - the block's width, its widest line
 * @param lineWidth - the width measured for the line
 * @param alignment - left, center or right
 * @returns the distance from the block's left edge to the line's, in user units
 * @see DERIV-text-width
 */
export function lineOffsetInBlock(
  blockWidth: number,
  lineWidth: number,
  alignment: TextAlignment,
): number {
  return alignedOffset(blockWidth - lineWidth, ON_AXIS[alignment]);
}
