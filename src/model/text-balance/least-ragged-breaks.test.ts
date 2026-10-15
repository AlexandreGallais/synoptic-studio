import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { leastRaggedBreaks } from "./least-ragged-breaks";

import type { LinesEnding } from "./lines-ending";

/** Tokens of given widths in a box, a gap of 1 between two tokens on a line. */
type Row = {
  /** The width of each token. */
  readonly widths: readonly number[];
  /** The box's width. */
  readonly box: number;
};

/**
 * How wide a line of whole tokens is: their widths and a gap of 1 between two of them.
 *
 * @param widths - each token's width
 * @param start - the first token
 * @param end - the token after the last
 * @returns their widths and the gaps between them
 */
function rowWidth(widths: readonly number[], start: number, end: number): number {
  return widths.slice(start, end).reduce((sum, width) => sum + width, 0) + (end - start - 1);
}

/**
 * The lines that fit and end before each token, by hand: from each start, lines grow until one
 * does not fit.
 *
 * @param row - the tokens' widths and the box
 * @returns for each index from 0 to the number of tokens, the lines ending there
 */
function endingOf(row: Row): readonly LinesEnding[] {
  const { box, widths } = row;

  return Array.from({ length: widths.length + 1 }, (unused, end) => ({
    lines: Array.from({ length: end }, (alsoUnused, start) => start)
      .filter((start) =>
        Array.from({ length: end - start }, (unusedToo, count) => start + count + 1).every(
          (stop) => rowWidth(widths, start, stop) <= box,
        ),
      )
      .map((start) => ({ room: box - rowWidth(widths, start, end), start })),
  }));
}

/**
 * The raggedness of a cut, by hand: the room each line leaves, squared and added up.
 *
 * @param row - the tokens' widths and the box
 * @param breaks - the first token of each line after the first
 * @returns the sum of the squares of the rooms
 */
function cutCost(row: Row, breaks: readonly number[]): number {
  const starts = [0, ...breaks, row.widths.length];

  return starts
    .slice(1)
    .reduce(
      (sum, end, index) => sum + (row.box - rowWidth(row.widths, starts[index] ?? 0, end)) ** 2,
      0,
    );
}

/**
 * Every way to cut the tokens into lines that all fit, tried one by one: each subset of the
 * places between two tokens is a cut.
 *
 * @param row - the tokens' widths and the box
 * @param lineCount - the number of lines
 * @returns the breaks of each cut
 */
function everyCut(row: Row, lineCount: number): readonly (readonly number[])[] {
  const count = row.widths.length;
  const cuts = Array.from({ length: 2 ** Math.max(count - 1, 0) }, (unused, mask) =>
    Array.from({ length: count - 1 }, (unusedToo, index) => index + 1).filter(
      (index) => Math.floor(mask / 2 ** (index - 1)) % 2 === 1,
    ),
  );

  return cuts.filter(
    (breaks) =>
      breaks.length === lineCount - 1 &&
      [0, ...breaks]
        .map((start, index) => [start, [...breaks, count][index] ?? count] as const)
        .every(([start, end]) => rowWidth(row.widths, start, end) <= row.box),
  );
}

/** "AAA BB CC DDDDD", every character 1 wide, in a box 6 wide (`REF-WIKI-KNUTH-PLASS`). */
const KNUTH_PLASS: Row = { box: 6, widths: [3, 2, 2, 5] };

describe("leastRaggedBreaks (DERIV-text-box step 9, REF-WIKI-KNUTH-PLASS)", () => {
  it("[F14.AC3] cuts 'AAA BB CC DDDDD' in a box 6 into 'AAA' / 'BB CC' / 'DDDDD'", () => {
    // Greedy "AAA BB" / "CC" / "DDDDD": 0² + 4² + 1² = 17; balanced 3² + 1² + 1² = 11.
    const breaks = leastRaggedBreaks(endingOf(KNUTH_PLASS), 3);

    expect(breaks).toEqual([1, 3]);
    expect(cutCost(KNUTH_PLASS, breaks)).toBe(11);
    expect(cutCost(KNUTH_PLASS, [2, 3])).toBe(17);
  });

  it("[F14.AC3] keeps the later break on equal cost", () => {
    // Three tokens 2 wide in a box 5: "2 | 2 2" costs 3² + 0² = 9, "2 2 | 2" costs 0² + 3² = 9.
    expect(leastRaggedBreaks(endingOf({ box: 5, widths: [2, 2, 2] }), 2)).toEqual([2]);
  });

  it("never takes a line that does not end after its start", () => {
    // Lines 0–1 and 1–2 (rooms 1 and 2), and a line from 2 "ending" at 2, which is left aside.
    const ending = [
      { lines: [] },
      { lines: [{ room: 1, start: 0 }] },
      {
        lines: [
          { room: 2, start: 1 },
          { room: 0, start: 2 },
        ],
      },
    ];

    expect(leastRaggedBreaks(ending, 2)).toEqual([1]);
  });

  it("cuts 6000 tokens into a line each within a second", { timeout: 1000 }, () => {
    // Only the line of each token fits: the breaks are 1 to 5999, read back one by one.
    const ending = Array.from({ length: 6001 }, (unused, end) => ({
      lines: end === 0 ? [] : [{ room: 0, start: end - 1 }],
    }));
    const breaks = leastRaggedBreaks(ending, 6000);

    expect(breaks).toHaveLength(5999);
    expect([breaks.at(0), breaks.at(-1)]).toEqual([1, 5999]);
  });

  it("gives no break for one line", () => {
    expect(leastRaggedBreaks(endingOf({ box: 9, widths: [3, 2] }), 1)).toEqual([]);
  });

  it("puts each token on its own line when there are as many lines as tokens", () => {
    expect(leastRaggedBreaks(endingOf(KNUTH_PLASS), 4)).toEqual([1, 2, 3]);
  });

  test.prop({
    extra: fc.nat({ max: 8 }),
    lineCount: fc.integer({ max: 8, min: 1 }),
    widths: fc.array(fc.integer({ max: 6, min: 1 }), { maxLength: 8, minLength: 1 }),
  })(
    "[F14.AC3] finds the least sum of squares of every cut on that number of lines",
    ({ extra, lineCount, widths }) => {
      const row = { box: Math.max(...widths) + extra, widths };
      const cuts = everyCut(row, lineCount);

      fc.pre(cuts.length > 0);

      const least = Math.min(...cuts.map((cut) => cutCost(row, cut)));
      const breaks = leastRaggedBreaks(endingOf(row), lineCount);

      expect(breaks).toHaveLength(lineCount - 1);
      expect(cuts).toContainEqual(breaks);
      expect(cutCost(row, breaks)).toBe(least);
    },
  );
});
