/** The first letter of a string, a whole code point of the Unicode category L. */
const FIRST_LETTER = /\p{L}/u;

/**
 * A word or a line with its first letter in upper case, whatever comes before it, the rest kept
 * as typed (Q36): " pump" gives " Pump", "(pump) on" gives "(Pump) on".
 *
 * @kind domain
 * @param word - a word or a line
 * @returns it, its first letter in upper case; unchanged without a letter
 * @see RULE-shapes-text
 */
export function capitalized(word: string): string {
  return word.replace(FIRST_LETTER, (letter) => letter.toUpperCase());
}
