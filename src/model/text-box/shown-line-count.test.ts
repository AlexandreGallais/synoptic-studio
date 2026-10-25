import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { integerRange } from "../../math";

import { shownLineCount } from "./shown-line-count";

import type { BlockLineHeights } from "./block-line-heights";
import type { Text } from "../text";

/** Arial 16's automatic line height, 2355 / 128, its block untrimmed. */
const AUTOMATIC: BlockLineHeights = { firstLine: 18.3984375, line: 18.3984375 };

/** The same, trimmed: the first line is the cap height, 1467 / 128 = 11.46094. */
const TRIMMED: BlockLineHeights = { firstLine: 11.4609375, line: 18.3984375 };

/** A text in a box 60 wide, tall enough for every line, with no maximum. */
const BOXED: Text = { ...HELLO_TEXT, boxHeight: 1000, boxWidth: 60, isInBox: true };

/**
 * The text in a box of a height and a maximum number of lines.
 *
 * @param boxHeight - how tall the box is, in user units
 * @param maximumLines - the most lines shown, 0 for none
 * @returns the text
 */
function boxed(boxHeight: number, maximumLines: number): Text {
  return { ...BOXED, boxHeight, maximumLines };
}

/**
 * The height of a block of lines (`DERIV-text-box`, step 5).
 *
 * @param heights - its first line's height and its line height
 * @param count - its number of lines, at least 1
 * @returns h(n) = first line + (n − 1) × line height
 */
function blockHeight(heights: BlockLineHeights, count: number): number {
  return heights.firstLine + (count - 1) * heights.line;
}

describe("shownLineCount (DERIV-text-box step 5)", () => {
  it("[F14.AC2] shows at most the maximum number of lines", () => {
    // "Hello World Hello World" wraps into 4 lines in a box 60 wide; at most 1, then 2.
    expect(shownLineCount(boxed(1000, 1), AUTOMATIC, 4)).toBe(1);
    expect(shownLineCount(boxed(1000, 2), AUTOMATIC, 4)).toBe(2);
  });

  it("[F14.AC2] shows the lines whose block fits the box's height", () => {
    // h(2) = 2 × 18.3984375 = 36.79688 ≤ 40 < h(3) = 55.19531.
    expect(shownLineCount(boxed(40, 0), AUTOMATIC, 4)).toBe(2);
    // A box exactly 2 lines tall shows them: 36.796875 ≤ 36.796875.
    expect(shownLineCount(boxed(36.796875, 0), AUTOMATIC, 4)).toBe(2);
  });

  it("[F14.AC2] shows one line in a box shorter than a line", () => {
    // No n ≥ 1 fits 10: max ∅ = 0, raised to 1 (provisional, VAL-015).
    expect(shownLineCount(boxed(10, 0), AUTOMATIC, 4)).toBe(1);
    expect(shownLineCount(boxed(0, 0), AUTOMATIC, 4)).toBe(1);
  });

  it("[F14.AC2] never shows more lines than the text wraps into", () => {
    // In a box 60 tall, h(3) = 55.19531 ≤ 60, but "Hello" over "World" is 2 lines.
    expect(shownLineCount(boxed(60, 0), AUTOMATIC, 2)).toBe(2);
    expect(shownLineCount(boxed(60, 5), AUTOMATIC, 2)).toBe(2);
  });

  it("[F14.AC2] shows no line of an empty text", () => {
    expect(shownLineCount(boxed(60, 0), AUTOMATIC, 0)).toBe(0);
    expect(shownLineCount(boxed(0, 1), AUTOMATIC, 0)).toBe(0);
  });

  it("[F14.AC5] counts a trimmed block from the cap height of its first line", () => {
    // Trimmed, h(n) = (n − 1) × 18.3984375 + 11.4609375: h(2) = 29.85938 ≤ 30 < h(3) = 48.25781;
    // untrimmed, h(2) = 36.79688 > 30, so only 1 line.
    expect(shownLineCount(boxed(30, 0), TRIMMED, 4)).toBe(2);
    expect(shownLineCount(boxed(30, 0), AUTOMATIC, 4)).toBe(1);
  });

  it("[F14.AC2] counts a block that fits within a rounding error", () => {
    // λ = 9.4: (94 − 9.4) / 9.4 = 9 exactly, 8.999999999999998 in doubles; 10 lines fit 94.
    expect(shownLineCount(boxed(94, 0), { firstLine: 9.4, line: 9.4 }, 20)).toBe(10);
  });

  it("shows every line wrapped when a line takes no height", () => {
    // A font whose ascender, descender and line gap add up to 0 gives an automatic line height
    // of 0: in a box 0 tall, f = 0 ≤ 0 and each further line adds 0, so all 3 lines fit;
    // (0 − 0) / 0 would be NaN and show none.
    expect(shownLineCount(boxed(0, 0), { firstLine: 0, line: 0 }, 3)).toBe(3);
    expect(shownLineCount(boxed(0, 2), { firstLine: 0, line: 0 }, 3)).toBe(2);
  });

  it("counts a chosen line height", () => {
    // A line height of 24: h(2) = 48 ≤ 50 < h(3) = 72.
    expect(shownLineCount(boxed(50, 0), { firstLine: 24, line: 24 }, 10)).toBe(2);
  });

  test.prop({
    boxHeight: fc.integer({ max: 1000, min: 0 }),
    firstLine: fc.integer({ max: 200, min: 1 }),
    line: fc.integer({ max: 200, min: 1 }),
    maximumLines: fc.integer({ max: 20, min: 0 }),
    wrapped: fc.integer({ max: 30, min: 0 }),
  })(
    "[F14.AC2] shows the most lines that fit, at least one, never past the maximum or the lines wrapped",
    ({ boxHeight, firstLine, line, maximumLines, wrapped }) => {
      const heights = { firstLine, line };
      const limit = Math.min(maximumLines === 0 ? wrapped : maximumLines, wrapped);
      // Every count from 1 to the limit whose block fits, the first always kept: h grows with n,
      // so the last of them is the most lines shown.
      const fitting = integerRange(1, limit).filter(
        (count) => count === 1 || blockHeight(heights, count) <= boxHeight,
      );

      expect(shownLineCount(boxed(boxHeight, maximumLines), heights, wrapped)).toBe(
        fitting.at(-1) ?? 0,
      );
    },
  );

  test.prop({
    boxHeight: fc.integer({ max: 1000, min: 0 }),
    firstTenths: fc.integer({ max: 2000, min: 1 }),
    lineTenths: fc.integer({ max: 2000, min: 1 }),
  })(
    "[F14.AC2] counts heights in tenths as exact integers would",
    ({ boxHeight, firstTenths, lineTenths }) => {
      const heights = { firstLine: firstTenths / 10, line: lineTenths / 10 };
      // In tenths, h(n) ≤ H reads f + (n − 1) × λ ≤ 10 H, in integers.
      const fitting = integerRange(1, 100).filter(
        (count) => count === 1 || firstTenths + (count - 1) * lineTenths <= 10 * boxHeight,
      );

      expect(shownLineCount(boxed(boxHeight, 0), heights, 100)).toBe(fitting.at(-1) ?? 0);
    },
  );
});
