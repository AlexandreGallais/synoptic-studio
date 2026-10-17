/** One or more word separators of the programmers' cases: spaces, hyphens and underscores. */
const CASE_SEPARATORS = /[ _-]+/u;

/**
 * The words of a line for the programmers' cases (Q36): the line cut at spaces, hyphens and
 * underscores, the empty pieces left out.
 *
 * @kind domain
 * @param line - one line of a text
 * @returns its words, in order; none for a line of separators only
 * @see docs/domain/shapes.md#text
 */
export function caseWords(line: string): readonly string[] {
  return line.split(CASE_SEPARATORS).filter((word) => word !== "");
}
