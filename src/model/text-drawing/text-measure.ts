import { shownLines } from "../text-box";
import { textBlock } from "../text-layout";

import type { MeasuredText } from "./measured-text";
import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * What a text measures: its lines shown (`shownLines`), wrapped in its box and cut at its maximum
 * lines, and the block they take (`textBlock`). Its position, angle, alignments, colors and
 * opacities play no part in it, so that it is kept while only those change (US-048).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is measured with
 * @param text - its content, size, spacing, case, box and options
 * @returns its lines shown and its block
 * @see DERIV-text-box
 */
export function textMeasure(metrics: FontMetrics, text: Text): MeasuredText {
  const lines = shownLines(metrics, text);

  return { block: textBlock(metrics, text, lines), lines };
}
