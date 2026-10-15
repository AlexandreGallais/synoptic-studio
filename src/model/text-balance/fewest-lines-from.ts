import { minimum } from "../../math";

import type { LinesEnding } from "./lines-ending";
import type { LinesEndingAhead } from "./lines-ending-ahead";

/** A line that fits, by its first token and the token after its last. */
type LineSpan = {
  /** The index of its first token. */
  readonly start: number;
  /** The index after its last token. */
  readonly end: number;
};

/** The lines that fit, by the token they start at. */
type EndsByStart = ReadonlyMap<number, readonly LineSpan[]>;

/**
 * The fewest lines after a token, from the fewest after the ends of the lines starting at it: one
 * more than the least of them; none at the paragraph's end. A line that does not end after its
 * start reaches nothing.
 *
 * @kind domain
 * @param after - the indices after it, with their fewest lines, in order
 * @param ends - the ends of the lines that fit, by their start
 * @param index - the token's index
 * @returns the fewest lines that hold the tokens from it to the end; infinite for none
 * @see REF-WIKI-KNUTH-PLASS
 */
function fewestAfter(after: readonly LinesEndingAhead[], ends: EndsByStart, index: number): number {
  const reached = (ends.get(index) ?? []).map(
    ({ end }) => after[end - index - 1]?.fewestAfter ?? Infinity,
  );

  return after.length === 0 ? 0 : 1 + minimum(reached, Infinity);
}

/**
 * The fewest lines that fit and hold a paragraph's tokens from each index to its end, by dynamic
 * programming over the lines starting there: none from the end, otherwise one more than the fewest
 * after the end of a line starting there (`DERIV-text-box`, step 9).
 *
 * Formula: g(n) = 0; g(i) = 1 + min{ g(j) : j > i, the tokens i to j − 1 fit on a line }.
 *
 * @kind domain
 * @param ending - for each index from 0 to the number of tokens, the lines that fit and end there
 * @returns for each index, its lines with the fewest lines after it; infinite when no cut of
 * fitting lines reaches the end
 * @see REF-WIKI-KNUTH-PLASS
 */
export function fewestLinesFrom(ending: readonly LinesEnding[]): readonly LinesEndingAhead[] {
  const ends = Map.groupBy(
    ending.flatMap(({ lines }, end) => lines.map(({ start }) => ({ end, start }))),
    ({ start }) => start,
  );

  return ending.reduceRight<readonly LinesEndingAhead[]>(
    (after, { lines }, index) => [
      { fewestAfter: fewestAfter(after, ends, index), lines },
      ...after,
    ],
    [],
  );
}
