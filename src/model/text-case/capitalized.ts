/** The first character of a string, a whole code point. */
const FIRST_CHARACTER = /^./u;

/**
 * A word or a line with its first character in upper case, the rest kept as typed (Q36).
 *
 * @kind domain
 * @param word - a word or a line
 * @returns it, its first character in upper case
 * @see docs/domain/shapes.md#text
 */
export function capitalized(word: string): string {
  return word.replace(FIRST_CHARACTER, (character) => character.toUpperCase());
}
