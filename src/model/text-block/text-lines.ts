/** A hard line break: a carriage return followed by a line feed, a carriage return alone, or a line feed. */
const LINE_BREAK = /\r\n|\r|\n/u;

/**
 * The lines of a text's content, cut at each hard line break: CR LF, CR or LF, three of the breaks
 * of UAX #14 (LB5), whose NL (U+0085) is not one here; none for an empty content, which draws nothing; a content ending with a line
 * break ends with an empty line.
 *
 * @kind domain
 * @param content - the content, a line break between two lines
 * @returns its lines, in order
 * @see REF-UAX14
 */
export function textLines(content: string): readonly string[] {
  return content === "" ? [] : content.split(LINE_BREAK);
}
