import { fontUnitsAtSize } from "../font";
import { textLineHeight } from "../text-block";

import type { BlockLineHeights } from "./block-line-heights";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The heights a text's block of lines is measured with: its line height (`textLineHeight`), and
 * its first line's height, the line height, or the cap height when the block is trimmed, its
 * first line's top then at the cap height (`DERIV-text-box`, steps 5 and 7).
 *
 * Formula: f = C × s / U trimmed, λ otherwise.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its size, line height and trim
 * @returns the first line's height and the line height, in user units
 * @see DERIV-text-box
 */
export function textBlockLineHeights(metrics: FontMetrics, text: Text): BlockLineHeights {
  const line = textLineHeight(metrics, text);

  return {
    firstLine: text.isTrimmed
      ? fontUnitsAtSize(metrics.capHeight, text.size, metrics.unitsPerEm)
      : line,
    line,
  };
}
