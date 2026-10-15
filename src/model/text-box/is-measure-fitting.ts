import { EPSILON } from "../../math";
import { lineMeasureWidth } from "../text-block";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { LineMeasure } from "../text-block";

/**
 * Whether a measured line fits the width of its text's box: its width is at most the box's
 * (`DERIV-text-box`, step 2), within a rounding error (`EPSILON`). The measure is that of the
 * line without its trailing spaces.
 *
 * Formula: w(trim(t)) ≤ W + ε.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param measure - the measure of the candidate line without its trailing spaces
 * @param text - its size, spacing and box width
 * @returns `true` when the line fits
 * @see DERIV-text-box
 */
export function isMeasureFitting(metrics: FontMetrics, measure: LineMeasure, text: Text): boolean {
  return lineMeasureWidth(metrics, measure, text) <= text.boxWidth + EPSILON;
}
