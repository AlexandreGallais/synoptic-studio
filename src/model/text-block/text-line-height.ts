import { isAutomaticLineHeight } from "./is-automatic-line-height";
import { lineHeight } from "./line-height";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The distance between two baselines of a text: the font's line height when automatic
 * (`lineHeight`, as in F05), otherwise the integer chosen, "used as the preferred line height"
 * (`REF-CSS-INLINE-3`, §5.1; `DERIV-text-box`, step 7).
 *
 * Formula: λ = (A − D + G) × s / U when automatic, the integer chosen otherwise.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its size and line height
 * @returns the line height λ, in user units
 * @see DERIV-text-box
 */
export function textLineHeight(metrics: FontMetrics, text: Text): number {
  return isAutomaticLineHeight(text) ? lineHeight(metrics, text.size) : text.lineHeight;
}
