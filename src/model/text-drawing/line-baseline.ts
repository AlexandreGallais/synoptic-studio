import { fontUnitsAtSize } from "../font";
import { lineHeight } from "../text-block";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The baseline of a text's line: the top of its block, the heights of the lines above, then the
 * ascent of the line (`DERIV-text-width`, step 4); written as the line's `y`, without
 * `dominant-baseline` (ADR-0052).
 *
 * Formula: top + k × lineHeight + A × s / U, for line k from 0.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param text - the top of its block and its size
 * @param index - the line's place, from 0
 * @returns the ordinate of the line's baseline, in user units
 * @see DERIV-text-width
 */
export function lineBaseline(metrics: FontMetrics, text: Text, index: number): number {
  return (
    text.y +
    index * lineHeight(metrics, text.size) +
    fontUnitsAtSize(metrics.ascender, text.size, metrics.unitsPerEm)
  );
}
