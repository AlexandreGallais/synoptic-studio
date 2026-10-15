/** The space U+0020. */
const SPACE = " ";

/**
 * A line without the spaces U+0020 at its end: "spaces at the end of a line are ordinarily not
 * measured for fit" (UAX #14, SP), and a line ended by a wrap is drawn without them
 * (`DERIV-text-box`, step 2, `trim(t)`). Other white space, a no-break space among it, stays. The
 * line is read once from its end, so that a long run of spaces costs no more than its length.
 *
 * @kind domain
 * @param line - a line, or a candidate line, of a paragraph
 * @returns the line without its trailing spaces
 * @see REF-UAX14
 */
export function withoutTrailingSpaces(line: string): string {
  // A space is one UTF-16 code unit: cutting after the last other code unit splits no character.
  return line.slice(0, line.split("").findLastIndex((unit) => unit !== SPACE) + 1);
}
