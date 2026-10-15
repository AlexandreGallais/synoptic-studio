/** The spaces U+0020 that end a line. */
const TRAILING_SPACES = / +$/u;

/**
 * A line without the spaces U+0020 at its end: "spaces at the end of a line are ordinarily not
 * measured for fit" (UAX #14, SP), and a line ended by a wrap is drawn without them
 * (`DERIV-text-box`, step 2, `trim(t)`). Other white space, a no-break space among it, stays.
 *
 * @kind domain
 * @param line - a line, or a candidate line, of a paragraph
 * @returns the line without its trailing spaces
 * @see REF-UAX14
 */
export function withoutTrailingSpaces(line: string): string {
  return line.replace(TRAILING_SPACES, "");
}
