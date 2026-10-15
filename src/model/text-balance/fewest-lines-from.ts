import { minimum, scan } from "../../math";

import { longestLineSpan } from "./longest-line-span";

import type { LinesEnding } from "./lines-ending";
import type { LinesEndingAhead } from "./lines-ending-ahead";

/** A line that fits, by its first token and the token after its last. */
type LineSpan = {
  /** The index of its first token. */
  readonly start: number;
  /** The index after its last token. */
  readonly end: number;
};

/** The lines that fit, by the token they start at, and how many tokens a line holds at most. */
type LineStarts = {
  /** The lines, by their first token. */
  readonly ends: ReadonlyMap<number, readonly LineSpan[]>;
  /** The most tokens a line holds, the number of later counts kept. */
  readonly span: number;
  /** The index of the paragraph's end. */
  readonly last: number;
};

/** An index read backwards: its lines ending there, and the index itself. */
type IndexedEnding = LinesEnding & {
  /** The index. */
  readonly index: number;
};

/** The paragraph read backwards to an index: its count, and the counts of the indices just after it. */
type FewestScan = {
  /** The index's lines, with the fewest lines after it. */
  readonly ahead: LinesEndingAhead;
  /** The fewest lines after it and after the next indices, nearest first, at most `span`. */
  readonly window: readonly number[];
};

/**
 * The fewest lines after a token, from the fewest after the ends of the lines starting at it: one
 * more than the least of them; none at the paragraph's end. A line that does not end after its
 * start reaches nothing.
 *
 * @kind domain
 * @param after - the fewest lines after the next indices, nearest first
 * @param starts - the lines by their start, and the paragraph's end
 * @param index - the token's index
 * @returns the fewest lines that hold the tokens from it to the end; infinite for none
 * @see DERIV-text-box
 */
function fewestAfter(after: readonly number[], starts: LineStarts, index: number): number {
  const reached = (starts.ends.get(index) ?? []).map(
    ({ end }) => after[end - index - 1] ?? Infinity,
  );

  return index === starts.last ? 0 : 1 + minimum(reached, Infinity);
}

/**
 * The paragraph read backwards with one more index: its fewest lines after it, and the window of
 * counts moved by one.
 *
 * @kind domain
 * @param state - the paragraph read back to the index after
 * @param ending - the index and its lines
 * @param starts - the lines by their start, the longest span and the paragraph's end
 * @returns the paragraph read back to the index
 * @see DERIV-text-box
 */
function fewestScanWith(state: FewestScan, ending: IndexedEnding, starts: LineStarts): FewestScan {
  const fewest = fewestAfter(state.window, starts, ending.index);

  return {
    ahead: { fewestAfter: fewest, lines: ending.lines },
    window: [fewest, ...state.window.slice(0, starts.span - 1)],
  };
}

/**
 * The fewest lines that fit and hold a paragraph's tokens from each index to its end, by dynamic
 * programming over the lines starting there, read backwards: none from the end, otherwise one more
 * than the fewest after the end of a line starting there (`DERIV-text-box`, step 9). Only the
 * counts of the next `m` indices are kept, `m` the most tokens a line holds (`longestLineSpan`).
 *
 * Formula: g(n) = 0; g(i) = 1 + min{ g(j) : j > i, the tokens i to j − 1 fit on a line }.
 *
 * @kind domain
 * @param ending - for each index from 0 to the number of tokens, the lines that fit and end there
 * @returns for each index, its lines with the fewest lines after it; infinite when no cut of
 * fitting lines reaches the end
 * @see DERIV-text-box
 */
export function fewestLinesFrom(ending: readonly LinesEnding[]): readonly LinesEndingAhead[] {
  const starts: LineStarts = {
    ends: Map.groupBy(
      ending.flatMap(({ lines }, end) => lines.map(({ start }) => ({ end, start }))),
      ({ start }) => start,
    ),
    last: ending.length - 1,
    span: longestLineSpan(ending),
  };
  const indexed = ending.map(({ lines }, index) => ({ index, lines })).toReversed();

  return scan<FewestScan, IndexedEnding>(
    indexed,
    (state, item) => fewestScanWith(state, item, starts),
    { ahead: { fewestAfter: Infinity, lines: [] }, window: [] },
  )
    .map(({ ahead }) => ahead)
    .toReversed();
}
