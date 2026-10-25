import { progressionTermsUpTo } from "../../math";

import type { BlockLineHeights } from "./block-line-heights";
import type { Text } from "../text";

/**
 * The most lines whose block fits a box's height: none when even the first line does not, every
 * line wrapped when a line takes no height (a font whose line height is 0).
 *
 * @kind domain
 * @param boxHeight - how tall the box is, in user units
 * @param heights - its first line's height and its line height
 * @param wrappedCount - the number of lines the text wraps into
 * @returns max{n ≥ 1 : h(n) ≤ H}, 0 when there is none
 * @see DERIV-text-box
 */
function linesFitting(boxHeight: number, heights: BlockLineHeights, wrappedCount: number): number {
  if (heights.firstLine > boxHeight) {
    return 0;
  }

  return heights.line > 0
    ? progressionTermsUpTo(heights.firstLine, heights.line, boxHeight)
    : wrappedCount;
}

/**
 * The number of lines a box shows: the most whose block fits in the box's height, at least one,
 * at most the maximum number of lines when one is set, and never more than the lines the text
 * wraps into, so that an empty text shows none (`DERIV-text-box`, step 5). A block of `n` lines
 * is `h(n) = f + (n − 1) × λ` tall, `f` its first line's height, a term of an arithmetic
 * progression (`progressionTermsUpTo`); a block that fits within a rounding error (`EPSILON`) fits.
 *
 * Formula: n = min(max(1, max{n ≥ 1 : h(n) ≤ H}), N or ∞, L), with max ∅ = 0, N = 0 for none;
 * max{…} = ⌊(H − f) / λ + ε⌋ + 1 when f ≤ H and λ > 0, every line when λ = 0.
 *
 * @kind domain
 * @param text - its box's height and its maximum number of lines
 * @param heights - the height of its block's first line and its line height, 0 or more
 * @param wrappedCount - the number of lines the text wraps into
 * @returns the number of lines shown
 * @see DERIV-text-box
 */
export function shownLineCount(
  text: Text,
  heights: BlockLineHeights,
  wrappedCount: number,
): number {
  const fitting = linesFitting(text.boxHeight, heights, wrappedCount);
  const maximum = text.maximumLines === 0 ? wrappedCount : text.maximumLines;

  return Math.min(Math.max(1, fitting), maximum, wrappedCount);
}
