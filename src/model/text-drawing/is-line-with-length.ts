import type { PositionedLine } from "../text-layout";

/**
 * Whether a line has a length to draw a rectangle along, an underline, a strike-through or a
 * background: a line empty, or tightened past nothing by a negative spacing, has none and gets
 * none (`DERIV-text-width`, step 6; `DERIV-text-box`, step 10).
 *
 * Formula: w > 0.
 *
 * @kind domain
 * @param line - the line, placed
 * @returns whether its width is positive
 * @see DERIV-text-box
 */
export function isLineWithLength(line: PositionedLine): boolean {
  return line.width > 0;
}
