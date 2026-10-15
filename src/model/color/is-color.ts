/** A stored color: `#` and six lower-case hexadecimal digits. */
const STORED_COLOR = /^#[\da-f]{6}$/u;

/**
 * Whether a color can enter the model: `#` and six lower-case hexadecimal digits, two for each of
 * red, green and blue, the HTML-compatible serialization of an 8-bit sRGB color (ADR-0056). The
 * check is shared by every color of the model, the text's and its background's today; a color
 * typed in another form, upper case among them, is turned to this one before it is checked.
 *
 * @kind domain
 * @param color - the color to check
 * @returns `true` for `#` and six lower-case hexadecimal digits
 * @see REF-CSS-COLOR-4
 */
export function isColor(color: string): boolean {
  return STORED_COLOR.test(color);
}
