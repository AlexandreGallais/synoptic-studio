import { measuredTextDrawing } from "./measured-text-drawing";
import { textMeasure } from "./text-measure";

import type { DrawnText } from "./drawn-text";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * What the SVG of a text draws (ADR-0052): what it measures (`textMeasure`), its lines shown and
 * their block, computed once, then placed and drawn (`measuredTextDrawing`).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, font, color, spacing, box and options
 * @returns its block, its middle, its positioned lines, their backgrounds and their decorations
 * @see DERIV-text-box
 */
export function textDrawing(metrics: FontMetrics, text: Text): DrawnText {
  return measuredTextDrawing(metrics, text, textMeasure(metrics, text));
}
