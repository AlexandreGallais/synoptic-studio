import { integerRange, minimum, scan } from "../../math";

import { fewestLinesFrom } from "./fewest-lines-from";
import { longestLineSpan } from "./longest-line-span";

import type { LinesEnding } from "./lines-ending";
import type { LinesEndingAhead } from "./lines-ending-ahead";

/** The least cost of a paragraph's first tokens on some lines, and where its last line starts. */
type BreakCell = {
  /** The least sum of the squares of the rooms left; infinite when no cut reaches it. */
  readonly cost: number;
  /** The first token of the last line of that cut; −1 for none. */
  readonly start: number;
};

/** The least costs of a paragraph's first tokens on each number of lines a whole cut may give them. */
type BreakColumn = {
  /** The fewest lines of the column, that of its first cell. */
  readonly least: number;
  /** By the number of lines, from the fewest up. */
  readonly cells: readonly BreakCell[];
};

/** Where a column stands: its end, the columns just before it, and the paragraph's counts. */
type BreakPlace = {
  /** The number of tokens before it. */
  readonly end: number;
  /** The columns of the ends just before it, nearest first, as many as a line holds tokens. */
  readonly window: readonly BreakColumn[];
  /** The number of lines of the paragraph. */
  readonly lineCount: number;
  /** The number of tokens of the paragraph. */
  readonly tokenCount: number;
};

/** The paragraph's counts that every column reads. */
type BreakCounts = {
  /** The number of lines of the paragraph. */
  readonly lineCount: number;
  /** The number of tokens of the paragraph. */
  readonly tokenCount: number;
  /** The most tokens a line holds. */
  readonly span: number;
};

/** An end read: its lines with the fewest lines after it, and the end itself. */
type IndexedAhead = LinesEndingAhead & {
  /** The number of tokens before it. */
  readonly end: number;
};

/** The paragraph read to an end: its column, and the columns just before, nearest first. */
type BreakScan = {
  /** The end's column. */
  readonly column: BreakColumn;
  /** The columns of this end and the ends just before, nearest first, as many as a line holds tokens. */
  readonly window: readonly BreakColumn[];
};

/** A column that no cut reaches. */
const NO_COLUMN: BreakColumn = { cells: [], least: Infinity };

/** No token on no line: nothing left, at no cost. */
const NOTHING: BreakColumn = { cells: [{ cost: 0, start: -1 }], least: 0 };

/** First tokens that no cut on that number of lines reaches. */
const UNREACHED: BreakCell = { cost: Infinity, start: -1 };

/**
 * The raggedness of a line: the square of the room it leaves, so that one large gap costs more
 * than two small ones of the same total (minimum raggedness, `REF-WIKI-LINE-WRAP`).
 *
 * Formula: (W − w)².
 *
 * @kind domain
 * @param room - the room the line leaves in its box
 * @returns its square
 * @see REF-WIKI-LINE-WRAP
 */
function lineRaggedness(room: number): number {
  return room ** 2;
}

/**
 * The least cost of a paragraph's first tokens on a number of lines, from their column; first
 * tokens that no whole cut gives that number of lines are unreached.
 *
 * @kind domain
 * @param column - the least costs of the first tokens, or none past the window
 * @param count - the number of lines
 * @returns the cell, or `UNREACHED`
 * @see DERIV-text-box
 */
function cellAt(column: BreakColumn | undefined, count: number): BreakCell {
  return column?.cells[count - column.least] ?? UNREACHED;
}

/**
 * The column of a line's start, read from the window of a column's place: none for a line that
 * does not end after its start.
 *
 * @kind domain
 * @param place - the end and the columns just before it
 * @param start - the line's first token
 * @returns the column, or none
 * @see DERIV-text-box
 */
function columnAt(place: BreakPlace, start: number): BreakColumn | undefined {
  return place.window[place.end - start - 1];
}

/**
 * The least cost of the tokens before an end on a number of lines: over the lines ending there,
 * the least cost of the tokens before the line's start on one line less, plus the line's
 * raggedness; on equal cost the later start, tried last, is kept (`DERIV-text-box`, step 9).
 *
 * @kind domain
 * @param place - the end and the columns just before it
 * @param ahead - the lines that fit and end there, by their start in order
 * @param count - the number of lines
 * @returns the least cost and the start of the last line
 * @see DERIV-text-box
 */
function leastCell(place: BreakPlace, ahead: LinesEndingAhead, count: number): BreakCell {
  return ahead.lines.reduce<BreakCell>((best, line) => {
    const cost = cellAt(columnAt(place, line.start), count - 1).cost + lineRaggedness(line.room);

    return cost <= best.cost && Number.isFinite(cost) ? { cost, start: line.start } : best;
  }, UNREACHED);
}

/**
 * The fewest lines a cut may have reached a column with, infinite for no column.
 *
 * @kind domain
 * @param column - the column of an end, or none
 * @returns its fewest lines
 * @see DERIV-text-box
 */
function columnLeast(column: BreakColumn | undefined): number {
  return column?.least ?? Infinity;
}

/**
 * The column of an end: its least cost on each number of lines a whole cut may give the tokens
 * before it — at least one more than the fewest of the columns its lines start at, and the lines
 * the rest of the tokens leave, one each; at most one a token, and the lines left once the rest
 * has its fewest (`fewestLinesFrom`).
 *
 * @kind domain
 * @param place - the end, the columns just before it and the paragraph's counts
 * @param ahead - the lines that fit and end there, with the fewest lines after it
 * @returns its fewest lines and its cells
 * @see DERIV-text-box
 */
function breakColumn(place: BreakPlace, ahead: LinesEndingAhead): BreakColumn {
  const before = minimum(
    ahead.lines.map(({ start }) => columnLeast(columnAt(place, start))),
    Infinity,
  );
  const least = Math.max(1 + before, place.lineCount - place.tokenCount + place.end);
  const most = Math.min(place.end, place.lineCount - ahead.fewestAfter);

  return { cells: integerRange(least, most).map((count) => leastCell(place, ahead, count)), least };
}

/**
 * The paragraph read to one more end: its column, and the window of columns moved by one; the
 * first end, before any token, holds nothing at no cost.
 *
 * @kind domain
 * @param state - the paragraph read to the end before
 * @param ahead - the end, its lines and the fewest lines after it
 * @param counts - the paragraph's numbers of lines and tokens, and the most tokens a line holds
 * @returns the paragraph read to the end
 * @see DERIV-text-box
 */
function breakScanWith(state: BreakScan, ahead: IndexedAhead, counts: BreakCounts): BreakScan {
  const column =
    ahead.end === 0
      ? NOTHING
      : breakColumn({ ...counts, end: ahead.end, window: state.window }, ahead);

  return { column, window: [column, ...state.window.slice(0, counts.span - 1)] };
}

/**
 * The breaks of a paragraph on a given number of lines that leave the least sum of the squares of
 * the rooms, by dynamic programming over the break points: the least cost of the first `j` tokens
 * on `k` lines is the least, over the lines from `i` to `j` that fit, of that of the first `i`
 * tokens on `k − 1` lines plus the square of the line's room; on equal cost the later break is
 * kept (`DERIV-text-box`, step 9; `REF-WIKI-KNUTH-PLASS`). Each end keeps only the numbers of
 * lines a whole cut may give the tokens before it, and reads only the columns of the `m` ends
 * before it, `m` the most tokens a line holds; the ends are read by a scan (`scan`), and so are the
 * breaks, back from the last; a line that does not end after its start is never taken.
 *
 * @kind domain
 * @param ending - for each index from 0 to the number of tokens, the lines that fit and end there
 * @param lineCount - the number of lines, 1 or more, that some cut of fitting lines reaches
 * @returns the first token of each line after the first, in order
 * @see REF-WIKI-KNUTH-PLASS
 */
export function leastRaggedBreaks(
  ending: readonly LinesEnding[],
  lineCount: number,
): readonly number[] {
  const counts = { lineCount, span: longestLineSpan(ending), tokenCount: ending.length - 1 };
  const columns = scan<BreakScan, IndexedAhead>(
    fewestLinesFrom(ending).map((ahead, end) => ({ ...ahead, end })),
    (state, ahead) => breakScanWith(state, ahead, counts),
    { column: NO_COLUMN, window: [] },
  ).map(({ column }) => column);

  return scan<number, number>(
    integerRange(2, lineCount).toReversed(),
    (end, count) => cellAt(columns[end], count).start,
    counts.tokenCount,
  ).toReversed();
}
