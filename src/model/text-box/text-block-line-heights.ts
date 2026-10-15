import { lineHeight } from "../text-block";

import type { BlockLineHeights } from "./block-line-heights";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** A line height of 0 asks for the font's own. */
const AUTOMATIC = 0;

/**
 * The heights a text's block of lines is counted with: its line height, the font's when automatic
 * (`lineHeight`) or the integer chosen, for every line, its first one included
 * (`DERIV-text-box`, steps 5 and 7). The trim, which starts the first line at the cap height,
 * needs the font's cap height, read with EN-035.
 *
 * Formula: λ = (A − D + G) × s / U when automatic, the integer chosen otherwise.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its size and line height
 * @returns the first line's height and the line height, in user units
 * @see DERIV-text-box
 */
export function textBlockLineHeights(metrics: FontMetrics, text: Text): BlockLineHeights {
  const line = text.lineHeight === AUTOMATIC ? lineHeight(metrics, text.size) : text.lineHeight;

  return { firstLine: line, line };
}
