/**
 * The lines of a text's content, cut at each line feed: none for an empty content, which draws
 * nothing; a content ending with a line feed ends with an empty line.
 *
 * @kind domain
 * @param content - the content, a line feed (U+000A) between two lines
 * @returns its lines, in order
 * @see docs/domain/shapes.md#text
 */
export function textLines(content: string): readonly string[] {
  return content === "" ? [] : content.split("\n");
}
