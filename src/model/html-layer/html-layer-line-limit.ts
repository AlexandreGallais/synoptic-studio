import { shownLineCount, textBlockLineHeights } from "../text-box";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** No limit to the lines a box shows. */
const NO_LIMIT = 0;

/**
 * The most lines a text's box shows, which its HTML layer shows at most whatever the browser's
 * wrapping: the lines its box's height and its maximum show (`shownLineCount`), for a text that
 * would wrap into any number (`DERIV-text-box`, step 5); 0, no limit, for a text that fits its
 * content, and for a box whose lines take no height and that sets no maximum.
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - whether it sits in a box, the box's height, its maximum, line height and trim
 * @returns the most lines shown, or 0 for no limit
 * @see DERIV-text-box
 */
export function htmlLayerLineLimit(metrics: FontMetrics, text: Text): number {
  const limit = text.isInBox
    ? shownLineCount(text, textBlockLineHeights(metrics, text), Infinity)
    : NO_LIMIT;

  return Number.isFinite(limit) ? limit : NO_LIMIT;
}
