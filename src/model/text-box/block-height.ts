import type { BlockLineHeights } from "./block-line-heights";

/**
 * The height of a block of lines: its first line's height, then one line height for each line
 * after it; no line, no height (`DERIV-text-box`, step 5). Written as `n` line heights less what
 * the first line gives up, so that a block whose first line is a whole line height is exactly
 * `n × λ`, as in F05.
 *
 * Formula: h(n) = f + (n − 1) × λ = n × λ − (λ − f) for n ≥ 1; h(0) = 0.
 *
 * @kind domain
 * @param heights - the height of its first line and its line height
 * @param count - its number of lines
 * @returns its height, in user units
 * @see DERIV-text-box
 */
export function blockHeight(heights: BlockLineHeights, count: number): number {
  return count === 0 ? 0 : count * heights.line - (heights.line - heights.firstLine);
}
