import { boxAroundBlock, isValidText } from "editor";

import { readTextMeasure, readTextMetrics } from "../text-measure";

import type { Text } from "editor";

/**
 * Reads the box a text would take from its content: the block it measures when it fits its
 * content (`readTextMeasure`), rounded up to integers (`boxAroundBlock`). The text is
 * measured as if Fit content were chosen, its box 0 × 0, so that a width or height emptied or
 * refused does not stop the measure; a text refused otherwise, or a font without metrics, gives
 * nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text read from the fields, valid or not
 * @returns the fitted box's width and height, nothing when the text cannot be measured
 */
export function readFittedBox(
  document: Document,
  text: Text,
): Pick<Text, "boxHeight" | "boxWidth"> | undefined {
  const fitted: Text = { ...text, boxHeight: 0, boxWidth: 0, isInBox: false };
  const metrics = readTextMetrics(document, fitted);

  return metrics === undefined || !isValidText(fitted)
    ? undefined
    : boxAroundBlock(readTextMeasure(metrics, fitted).block);
}
