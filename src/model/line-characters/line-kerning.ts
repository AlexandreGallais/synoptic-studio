import { pairKerning } from "../kerning";

import type { Kerning } from "../kerning";

/** The kerning summed up to a character of a line, and that character, none before the first. */
type KernedRun = {
  /** The kerning of the pairs up to the character. */
  readonly sum: number;
  /** The character, the left one of the next pair; none before the line's first. */
  readonly left: number | undefined;
};

/**
 * The kerning of a line: the sum of the kerning of each pair of adjacent characters; the first
 * character has no character before it, so it starts no pair (`DERIV-text-width`, step 2), and no
 * kerning row is read for a character that is not there.
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
  return codePoints.reduce<KernedRun>(
    (run, right) => ({
      left: right,
      sum: run.left === undefined ? run.sum : run.sum + pairKerning(kerning, run.left, right),
    }),
    { left: undefined, sum: 0 },
  ).sum;
}
