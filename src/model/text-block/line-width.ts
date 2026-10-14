import { fontUnitsAtSize } from "../font";

import { isWordSeparator } from "./is-word-separator";
import { letterSpacingCount } from "./letter-spacing-count";
import { lineAdvance } from "./line-advance";
import { lineCodePoints } from "./line-code-points";
import { lineKerning } from "./line-kerning";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** What a line's width depends on besides its characters and its font. */
export type LineSpacing = Pick<Text, "letterSpacing" | "size" | "wordSpacing">;

/**
 * The width of one line of a text: its advances and kerning summed in font units, then brought
 * to the text's size once, plus the letter spacing between adjacent characters and the word
 * spacing at each space and no-break space (`DERIV-text-width`, steps 1 to 3).
 *
 * Formula: (Σ adv + Σ kern) × s / U + max(n − 1, 0) × L + m × W.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param line - the line, already in the text's case
 * @param spacing - the text's size, letter spacing and word spacing
 * @returns the width in user units; 0 for an empty line
 * @see DERIV-text-width
 */
export function lineWidth(metrics: FontMetrics, line: string, spacing: LineSpacing): number {
  const codePoints = lineCodePoints(line);
  const units = lineAdvance(metrics, codePoints) + lineKerning(metrics.kerning, codePoints);
  const separators = codePoints.filter((codePoint) => isWordSeparator(codePoint)).length;

  return (
    fontUnitsAtSize(units, spacing.size, metrics.unitsPerEm) +
    letterSpacingCount(codePoints.length) * spacing.letterSpacing +
    separators * spacing.wordSpacing
  );
}
