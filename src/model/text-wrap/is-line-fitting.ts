import { measuredLine } from "../text-block";

import { isMeasureFitting } from "./is-measure-fitting";
import { withoutTrailingSpaces } from "./without-trailing-spaces";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * Whether a line fits the width of its text's box: its width without the spaces at its end, which
 * are not measured for fit, is at most the box's (`DERIV-text-box`, step 2).
 *
 * Formula: w(trim(t)) ≤ W.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param line - a candidate line, in the text's case
 * @param text - its size, spacing and box width
 * @returns `true` when the line fits
 * @see DERIV-text-box
 */
export function isLineFitting(metrics: FontMetrics, line: string, text: Text): boolean {
  return isMeasureFitting(metrics, measuredLine(metrics, withoutTrailingSpaces(line)), text);
}
