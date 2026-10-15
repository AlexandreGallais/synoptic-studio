import { describe, expect, it } from "vitest";

import { fewestLinesFrom } from "./fewest-lines-from";

import type { LinesEnding } from "./lines-ending";

/**
 * The lines that fit, ending at each index, from their starts; the rooms do not count here.
 *
 * @param starts - for each index, the starts of the lines ending there
 * @returns the table of lines ending at each index
 */
function endingFrom(starts: readonly (readonly number[])[]): readonly LinesEnding[] {
  return starts.map((row) => ({ lines: row.map((start) => ({ room: 0, start })) }));
}

describe("fewestLinesFrom (DERIV-text-box step 9)", () => {
  it("[F14.AC3] counts the fewest lines from each token of 'AAA BB CC DDDDD' in a box 6", () => {
    // Lines: AAA (0–1), AAA BB (0–2), BB (1–2), BB CC (1–3), CC (2–3), DDDDD (3–4). From 3: one
    // line; from 2: CC then DDDDD, 2; from 1: BB CC then DDDDD, 2; from 0: AAA BB, CC, DDDDD, 3.
    const ending = endingFrom([[], [0], [0, 1], [1, 2], [3]]);

    expect(fewestLinesFrom(ending).map(({ fewestAfter }) => fewestAfter)).toEqual([3, 2, 2, 1, 0]);

    expect(fewestLinesFrom(ending).map(({ lines }) => lines)).toEqual(
      ending.map(({ lines }) => lines),
    );
  });

  it("gives an infinite count from a token that starts no line, and before it", () => {
    // Token 1 starts no line: nothing from it reaches the end, nor from 0, whose only line ends at 1.
    expect(
      fewestLinesFrom(endingFrom([[], [0], [], [2]])).map(({ fewestAfter }) => fewestAfter),
    ).toEqual([Infinity, Infinity, 1, 0]);
  });

  it("never takes a line that does not end after its start", () => {
    // A line from 1 "ending" at 1 reaches nothing; the line 0–1 and 1–2 give 2 from 0.
    expect(
      fewestLinesFrom(endingFrom([[], [0, 1], [1]])).map(({ fewestAfter }) => fewestAfter),
    ).toEqual([2, 1, 0]);
  });

  it("gives none for an empty paragraph", () => {
    expect(fewestLinesFrom(endingFrom([[]]))).toEqual([{ fewestAfter: 0, lines: [] }]);
  });
});
