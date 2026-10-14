import { pairKerning } from "../kerning";

import type { Kerning } from "../kerning";

/** Before a line's first character: no character, which kerns with nothing. */
const NO_CHARACTER = -1;

/**
 * The kerning of a line: the sum of the kerning of each pair of adjacent characters; the first
 * character has no character before it (`DERIV-text-width`, step 2).
 *
 * Formula: Σᵢ₌₁ⁿ⁻¹ kern(g(cᵢ), g(cᵢ₊₁)).
 *
 * @kind domain
 * @param kerning - the font's kerning
 * @param codePoints - the line's characters
 * @returns the kerning in font units, negative when it tightens; 0 for fewer than two characters
 * @see DERIV-text-width
 */
export function lineKerning(kerning: Kerning, codePoints: readonly number[]): number {
  return codePoints.reduce(
    (sum, right, index) => sum + pairKerning(kerning, codePoints[index - 1] ?? NO_CHARACTER, right),
    0,
  );
}
