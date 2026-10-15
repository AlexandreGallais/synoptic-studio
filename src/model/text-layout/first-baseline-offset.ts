import { fontUnitsAtSize } from "../font";
import { capHeightAtSize, halfLeading, isAutomaticLineHeight } from "../text-block";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * How far a text's first baseline lies below the top of its block (`DERIV-text-box`, step 7):
 * trimmed, the block starts at the cap height, whatever the line height (`text-box-trim`,
 * `REF-CSS-INLINE-3`, §6.2); with an automatic line height, the ascent, as in F05
 * (`DERIV-text-width`, step 4); with a line height chosen, half the leading then the ascent
 * (`halfLeading`).
 *
 * Formula: C × s / U trimmed; A × s / U automatic; (λ − (A − D) × s / U) / 2 + A × s / U chosen.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its size, line height and trim
 * @returns the distance from the block's top down to the first baseline, in user units
 * @see DERIV-text-box
 */
export function firstBaselineOffset(metrics: FontMetrics, text: Text): number {
  if (text.isTrimmed) {
    return capHeightAtSize(metrics, text.size);
  }

  const ascent = fontUnitsAtSize(metrics.ascender, text.size, metrics.unitsPerEm);

  return isAutomaticLineHeight(text) ? ascent : halfLeading(metrics, text) + ascent;
}
