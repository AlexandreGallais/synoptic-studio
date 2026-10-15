import {
  characterUnitCount,
  isWordSeparator,
  lineAdvance,
  lineCodePoints,
  lineKerning,
  measuredCharacters,
} from "../line-characters";

import type { LineMeasure } from "./line-measure";
import type { FontMetrics } from "../font";

/** Before a line's first character, or after its last: no character. */
const NO_CHARACTER = -1;

/**
 * What the width of one line of a text is made of: its characters composed and its formatting
 * characters left out (`measuredCharacters`), their advances and kerning summed in font units,
 * the number of its typographic character units and of its word separators, and its first and
 * last characters measured (`DERIV-text-width`, steps 1 to 3).
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param line - the line, already in the text's case
 * @returns its measure; an advance of 0 and no unit for an empty line
 * @see DERIV-text-width
 */
export function measuredLine(metrics: FontMetrics, line: string): LineMeasure {
  const characters = measuredCharacters(line);
  const codePoints = lineCodePoints(characters);

  return {
    advance: lineAdvance(metrics, codePoints) + lineKerning(metrics.kerning, codePoints),
    first: codePoints.at(0) ?? NO_CHARACTER,
    last: codePoints.at(-1) ?? NO_CHARACTER,
    separatorCount: codePoints.filter((codePoint) => isWordSeparator(codePoint)).length,
    text: line,
    unitCount: characterUnitCount(characters),
  };
}
