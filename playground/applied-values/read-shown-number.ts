/**
 * A number as a field acting as a number field shows it (US-085), as `formatSvgNumber` writes it:
 * a minus or none, digits, then decimals or none; `+10` is none.
 */
const SHOWN_NUMBER = /^-?\d+(?:\.\d+)?$/u;

/**
 * Reads the number a text field acting as a number field shows (`SHOWN_NUMBER`), white space
 * around it ignored, as a number field reads its own.
 *
 * @kind procedure
 * @param shown - the text it shows
 * @returns the number; `NaN` for anything else, a value with a sign included
 * @see docs/domain/interaction.md#_2-selector
 */
export function readShownNumber(shown: string): number {
  const text = shown.trim();

  return SHOWN_NUMBER.test(text) ? Number(text) : NaN;
}
