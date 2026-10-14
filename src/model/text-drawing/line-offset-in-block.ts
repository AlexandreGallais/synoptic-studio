import type { TextAlignment } from "../text";

/** The share of a line's room that goes before it, for each alignment (`REF-CSS-TEXT-3`, §6.1). */
const ROOM_BEFORE: Readonly<Record<TextAlignment, number>> = { center: 0.5, left: 0, right: 1 };

/**
 * How far a line starts from its block's left edge: aligned to the left edge, none of the room
 * the line leaves in the block goes before it; centered, half; aligned to the right edge, all of
 * it (`DERIV-text-width`, step 5). The alignment is physical, whatever the direction.
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
  return (blockWidth - lineWidth) * ROOM_BEFORE[alignment];
}
