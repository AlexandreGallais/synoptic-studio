import { minimum } from "../../math";

import { fewestLinesFrom } from "./fewest-lines-from";

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

/** Where a column stands: its end, and the numbers of lines and tokens of the paragraph. */
type BreakPlace = {
  /** The number of tokens before it. */
  readonly end: number;
  /** The number of lines of the paragraph. */
  readonly lineCount: number;
  /** The number of tokens of the paragraph. */
  readonly tokenCount: number;
};

/** The fewest and the most lines a whole cut may give a paragraph's first tokens. */
type LineCountRange = {
  /** The fewest. */
  readonly least: number;
  /** The most. */
  readonly most: number;
};

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
 * @param column - the least costs of the first tokens, or none past the paragraph
 * @param count - the number of lines
 * @returns the cell, or `UNREACHED`
 * @see REF-WIKI-KNUTH-PLASS
 */
function cellAt(column: BreakColumn | undefined, count: number): BreakCell {
  return column?.cells[count - column.least] ?? UNREACHED;
}

/**
 * The least cost of the tokens before an end on a number of lines: over the lines ending there,
 * the least cost of the tokens before the line's start on one line less, plus the line's
 * raggedness; on equal cost the later start, tried last, is kept (`DERIV-text-box`, step 9).
 *
 * @kind domain
 * @param columns - the columns of the ends before
 * @param ending - the lines that fit and end there, by their start in order
 * @param count - the number of lines
 * @returns the least cost and the start of the last line
 * @see REF-WIKI-KNUTH-PLASS
 */
function leastCell(
  columns: readonly BreakColumn[],
  ending: LinesEnding | LinesEndingAhead,
  count: number,
): BreakCell {
  return ending.lines.reduce<BreakCell>((best, line) => {
    const cost = cellAt(columns[line.start], count - 1).cost + lineRaggedness(line.room);

    return cost <= best.cost && Number.isFinite(cost) ? { cost, start: line.start } : best;
  }, UNREACHED);
}

/**
 * The fewest lines a cut may have reached a column with, infinite for no column.
 *
 * @kind domain
 * @param column - the column of an end, or none
 * @returns its fewest lines
 * @see REF-WIKI-KNUTH-PLASS
 */
function columnLeast(column: BreakColumn | undefined): number {
  return column?.least ?? Infinity;
}

/**
 * The numbers of lines a whole cut may give the tokens before an end: at least one more than the
 * fewest of the columns its lines start at, and the lines the rest of the tokens leave, one each;
 * at most one a token, and the lines left once the rest has its fewest (`fewestLinesFrom`).
 *
 * @kind domain
 * @param columns - the columns of the ends before
 * @param ahead - the lines ending there, with the fewest lines after it
 * @param place - the end, the number of lines and of tokens
 * @returns the fewest and the most lines
 * @see REF-WIKI-KNUTH-PLASS
 */
function lineCountRange(
  columns: readonly BreakColumn[],
  ahead: LinesEndingAhead,
  place: BreakPlace,
): LineCountRange {
  const before = minimum(
    ahead.lines.map(({ start }) => columnLeast(columns[start])),
    Infinity,
  );

  return {
    least: Math.max(1 + before, place.lineCount - place.tokenCount + place.end),
    most: Math.min(place.end, place.lineCount - ahead.fewestAfter),
  };
}

/**
 * The column of an end: its least cost on each number of lines a whole cut may give the tokens
 * before it (`lineCountRange`).
 *
 * @kind domain
 * @param columns - the columns of the ends before
 * @param ahead - the lines that fit and end there, with the fewest lines after it
 * @param place - the end, the number of lines and of tokens
 * @returns its fewest lines and its cells
 * @see REF-WIKI-KNUTH-PLASS
 */
function breakColumn(
  columns: readonly BreakColumn[],
  ahead: LinesEndingAhead,
  place: BreakPlace,
): BreakColumn {
  const range = lineCountRange(columns, ahead, place);

  return {
    cells: Array.from({ length: Math.max(range.most - range.least + 1, 0) }, (unused, offset) =>
      leastCell(columns, ahead, range.least + offset),
    ),
    least: range.least,
  };
}

/**
 * The breaks of a paragraph on a given number of lines that leave the least sum of the squares of
 * the rooms, by dynamic programming over the break points: the least cost of the first `j` tokens
 * on `k` lines is the least, over the lines from `i` to `j` that fit, of that of the first `i`
 * tokens on `k − 1` lines plus the square of the line's room; on equal cost the later break is
 * kept (`DERIV-text-box`, step 9; `REF-WIKI-KNUTH-PLASS`). Each end keeps only the numbers of
 * lines a whole cut may give the tokens before it (`lineCountRange`), so that the work follows
 * the number of lines that fit, not that times the number of lines; a line that does not end
 * after its start is never taken.
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
  const tokenCount = ending.length - 1;
  const columns = fewestLinesFrom(ending).reduce<readonly BreakColumn[]>(
    (done, ahead, end) => [
      ...done,
      end === 0 ? NOTHING : breakColumn(done, ahead, { end, lineCount, tokenCount }),
    ],
    [],
  );

  return Array.from({ length: lineCount - 1 }).reduce<readonly number[]>(
    (starts, unused, index) => [
      cellAt(columns[starts.at(0) ?? tokenCount], lineCount - index).start,
      ...starts,
    ],
    [],
  );
}
