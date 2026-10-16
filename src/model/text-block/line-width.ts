import { lineMeasureWidth } from "./line-measure-width";
import { measuredLine } from "./measured-line";

import type { LineSpacing } from "./line-spacing";
import type { FontMetrics } from "../font";

/**
 * The width of one line of a text: its measure (`measuredLine`) brought to a width
 * (`lineMeasureWidth`), its advances and kerning at the text's size plus its letter and word
 * spacing (`DERIV-text-width`, steps 1 to 3).
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param line - the line, already in the text's case
 * @param spacing - the text's size, letter spacing and word spacing
 * @returns the width in user units; 0 for an empty line
 * @see DERIV-text-width
 */
export function lineWidth(metrics: FontMetrics, line: string, spacing: LineSpacing): number {
  return lineMeasureWidth(metrics, measuredLine(metrics, line), spacing);
}
