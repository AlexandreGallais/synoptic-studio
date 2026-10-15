import { fontUnitsAtSize } from "../font";
import { letterSpacingCount } from "../line-characters";

import type { LineMeasure } from "./line-measure";
import type { LineSpacing } from "./line-spacing";
import type { FontMetrics } from "../font";

/**
 * The width of a line from its measure: its advances and kerning brought to the text's size once,
 * plus the letter spacing between adjacent typographic character units and the word spacing at
 * each separator (`DERIV-text-width`, steps 1 to 3).
 *
 * Formula: (Σ adv + Σ kern) × s / U + max(u − 1, 0) × L + m × W, for u units.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param measure - the line's measure (`measuredLine`)
 * @param spacing - the text's size, letter spacing and word spacing
 * @returns the width in user units; 0 for an empty line
 * @see DERIV-text-width
 */
export function lineMeasureWidth(
  metrics: FontMetrics,
  measure: LineMeasure,
  spacing: LineSpacing,
): number {
  return (
    fontUnitsAtSize(measure.advance, spacing.size, metrics.unitsPerEm) +
    letterSpacingCount(measure.unitCount) * spacing.letterSpacing +
    measure.separatorCount * spacing.wordSpacing
  );
}
