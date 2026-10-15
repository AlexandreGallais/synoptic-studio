import { textLineHeight } from "../text-block";

import { firstBaselineOffset } from "./first-baseline-offset";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** A line's place in its block: its index from 0, and the ordinate of the block's top. */
type LineInBlock = {
  /** The line's place, from 0. */
  readonly index: number;
  /** The ordinate of its block's top, in the SVG. */
  readonly top: number;
};

/**
 * The baseline of a text's line: the top of its block, a line height for each line above, then
 * the first baseline's distance below the top (`firstBaselineOffset`): the ascent with an
 * automatic line height, as in F05, half the leading then the ascent with one chosen, the cap
 * height when trimmed (`DERIV-text-box`, step 7); written as the line's `y`, without
 * `dominant-baseline` (ADR-0052).
 *
 * Formula: top + k × λ + first baseline, for line k from 0.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param text - its size, line height and trim
 * @param line - the line's index and the top of its block
 * @returns the ordinate of the line's baseline, in user units
 * @see DERIV-text-box
 */
export function lineBaseline(metrics: FontMetrics, text: Text, line: LineInBlock): number {
  return line.top + line.index * textLineHeight(metrics, text) + firstBaselineOffset(metrics, text);
}
