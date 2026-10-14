/**
 * The characters of a line, as Unicode code points: a string iterates by code point, so a
 * character beyond the basic plane is one character, not two halves (`DERIV-text-width`,
 * notation `c₁ … cₙ`).
 *
 * @kind domain
 * @param line - one line of a text
 * @returns its code points, in order
 * @see DERIV-text-width
 */
export function lineCodePoints(line: string): readonly number[] {
  // A character the iteration gives always has a code point: the filter only narrows the type.
  return Array.from(line, (character) => character.codePointAt(0)).filter(
    (codePoint) => codePoint !== undefined,
  );
}
