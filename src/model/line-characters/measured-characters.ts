/** Invisible zero-width formatting characters: the Unicode category Cf. */
const FORMAT_CHARACTERS = /\p{Cf}/gu;

/**
 * The characters of a line as they are measured: composed into Normalization Form C
 * (`String.prototype.normalize`, `REF-ECMA-262-NORMALIZE`), so that "e" and a combining acute
 * are measured as the font's "é"; then without the invisible zero-width formatting characters
 * (category Cf), measured and spaced as if absent, as letter spacing treats them.
 *
 * @kind domain
 * @param line - one line of a text, in its case, as typed with its marks and formatting characters
 * @returns its composed characters, without formatting characters
 * @see REF-CSS-TEXT-3
 */
export function measuredCharacters(line: string): string {
  return line.normalize("NFC").replaceAll(FORMAT_CHARACTERS, "");
}
