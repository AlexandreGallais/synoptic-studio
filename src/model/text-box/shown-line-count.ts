import { EPSILON } from "../../math";

import type { BlockLineHeights } from "./block-line-heights";
import type { Text } from "../text";

/**
 * The number of lines a box shows: the most whose block fits in the box's height, at least one,
 * at most the maximum number of lines when one is set, and never more than the lines the text
 * wraps into, so that an empty text shows none (`DERIV-text-box`, step 5). A block of `n` lines
 * is `h(n) = f + (n − 1) × λ` tall, `f` its first line's height; a block that fits within a
 * rounding error (`EPSILON`) fits.
 *
 * Formula: n = min(max(1, max{n ≥ 1 : h(n) ≤ H}), N or ∞, L), with max ∅ = 0, N = 0 for none;
 * max{…} = ⌊(H − f) / λ + ε⌋ + 1 when f ≤ H.
 *
 * @kind domain
 * @param text - its box's height and its maximum number of lines
 * @param heights - the height of its block's first line and its line height, more than 0
 * @param wrappedCount - the number of lines the text wraps into
 * @returns the number of lines shown
 * @see DERIV-text-box
 */
export function shownLineCount(
  text: Text,
  heights: BlockLineHeights,
  wrappedCount: number,
): number {
  const fitting =
    heights.firstLine > text.boxHeight
      ? 0
      : Math.floor((text.boxHeight - heights.firstLine) / heights.line + EPSILON) + 1;
  const maximum = text.maximumLines === 0 ? wrappedCount : text.maximumLines;

  return Math.min(Math.max(1, fitting), maximum, wrappedCount);
}
