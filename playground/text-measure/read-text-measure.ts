import { textMeasure } from "../../src";

import { readCachedValue } from "./read-cached-value";
import { TEXT_MEASURE_CACHE } from "./text-measure-cache";

import type { FontMetrics, MeasuredText, Text } from "../../src";

/** The values of a text that play no part in its lines and block: where it is, its angle, its colors and opacities. */
const PLACEMENT_AND_PAINT: ReadonlySet<string> = new Set([
  "angle",
  "backgroundColor",
  "backgroundOpacity",
  "color",
  "hasStrikeThroughColor",
  "hasUnderlineColor",
  "opacity",
  "strikeThroughColor",
  "strikeThroughOpacity",
  "underlineColor",
  "underlineOpacity",
  "x",
  "y",
]);

/**
 * Reads what a text measures, its lines shown and their block (`textMeasure`), kept while its
 * metrics and every value but its position, angle, colors and opacities stay the same: an
 * opacity changed draws again from the lines kept, a size changed measures them again.
 *
 * @kind procedure
 * @param metrics - the metrics of its face, as `readTextMetrics` keeps them
 * @param text - the text read from the fields
 * @returns its lines shown and its block
 */
export function readTextMeasure(metrics: FontMetrics, text: Text): MeasuredText {
  const measuredValues = Object.entries(text).filter(([name]) => !PLACEMENT_AND_PAINT.has(name));

  return readCachedValue(
    TEXT_MEASURE_CACHE,
    { key: JSON.stringify(measuredValues), source: metrics },
    () => textMeasure(metrics, text),
  );
}
