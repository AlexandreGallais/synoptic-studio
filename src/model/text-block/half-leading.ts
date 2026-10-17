import { contentAreaHeight } from "./content-area-height";
import { textLineHeight } from "./text-line-height";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * Half the leading of a text's lines: CSS computes "the leading L as L = line-height - (A + D)",
 * and "Half the leading … is added above A … and the other half below D"; "L may be negative"
 * (`REF-CSS-INLINE-3`, §5.3; `DERIV-text-box`, step 7).
 *
 * Formula: (λ − (A − D) × s / U) / 2, the descender D negative below the baseline, so that
 * CSS's depth below it is −D.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its size and line height
 * @returns the room added above the ascent of each line, in user units, negative when the line
 * height is smaller than the content area
 * @see REF-CSS-INLINE-3
 */
export function halfLeading(metrics: FontMetrics, text: Text): number {
  return (textLineHeight(metrics, text) - contentAreaHeight(metrics, text.size)) / 2;
}
