/**
 * A typographic character unit: a character other than a nonspacing mark with the marks that
 * follow it, or a run of marks opening the line.
 */
const CHARACTER_UNITS = /^\p{Mn}+|\P{Mn}\p{Mn}*/gu;

/**
 * The typographic character units of a line: each character with the nonspacing marks (category
 * Mn) that follow it, as a mark is kept with the character before it rather than treated as a
 * unit of its own (`DERIV-text-width`, step 3). Letter spacing goes between them, and a word too
 * long is broken between them (`DERIV-text-box`, step 4). Grapheme clusters approximated by
 * marks, which covers the scripts measured (ADR-0052).
 *
 * @kind domain
 * @param line - a line, or a word, of a text
 * @returns its units, in order, which joined give it back; none for an empty line
 * @see REF-CSS-TEXT-3
 */
export function characterUnits(line: string): readonly string[] {
  return Array.from(line.matchAll(CHARACTER_UNITS), ([unit]) => unit);
}
