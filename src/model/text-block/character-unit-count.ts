/**
 * A typographic character unit: a character other than a nonspacing mark with the marks that
 * follow it, or a run of marks opening the line.
 */
const CHARACTER_UNITS = /^\p{Mn}+|\P{Mn}\p{Mn}*/gu;

/**
 * The number of typographic character units of a line, between which letter spacing is added:
 * each character with the nonspacing marks (category Mn) that follow it, as a mark is kept with
 * the character before it rather than treated as a unit of its own (`DERIV-text-width`, step 3).
 * Grapheme clusters approximated by marks, which covers the scripts measured (ADR-0052).
 *
 * @kind domain
 * @param line - the characters measured of a line
 * @returns the number of its units; 0 for an empty line
 * @see REF-CSS-TEXT-3
 */
export function characterUnitCount(line: string): number {
  return (line.match(CHARACTER_UNITS) ?? []).length;
}
