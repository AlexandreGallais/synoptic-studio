/** A color: `#` and six hexadecimal digits, upper or lower case. */
const HEXADECIMAL_COLOR = /^#[\da-f]{6}$/iu;

/**
 * Whether a text's color can enter the model: `#` followed by six hexadecimal digits, two for
 * each of red, green and blue, as the designer copies and pastes it (F05 refinement, round 2).
 *
 * @kind domain
 * @param color - the color typed
 * @returns `true` for `#` and six hexadecimal digits
 * @see docs/domain/shapes.md#text
 */
export function isTextColor(color: string): boolean {
  return HEXADECIMAL_COLOR.test(color);
}
