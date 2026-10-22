import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isLevelCrossed } from "./is-level-crossed";

/** An ordinate, an integer. */
const ORDINATE = fc.integer({ max: 1000, min: -1000 });

describe("isLevelCrossed (DERIV-painted-area step 2)", () => {
  it.each([
    // Going up the axis, from 0 to 100: its start counts, its end does not.
    { expected: true, from: 0, level: 0, to: 100 },
    { expected: true, from: 0, level: 50, to: 100 },
    { expected: false, from: 0, level: 100, to: 100 },
    // Going down, from 100 to 0: its end counts, its start does not.
    { expected: true, from: 100, level: 0, to: 0 },
    { expected: false, from: 100, level: 100, to: 0 },
    // Beyond its ends.
    { expected: false, from: 0, level: -1, to: 100 },
    { expected: false, from: 0, level: 101, to: 100 },
  ])("is $expected for a piece from $from to $to at the level $level", (row) => {
    expect(isLevelCrossed(row.from, row.to, row.level)).toBe(row.expected);
  });

  it("is false for a flat piece, even on its own level", () => {
    // min(50, 50) ≤ 50 < max(50, 50) fails: horizontal edges are excluded.
    expect(isLevelCrossed(50, 50, 50)).toBe(false);
  });
});

describe("isLevelCrossed (properties)", () => {
  test.prop({ from: ORDINATE, level: ORDINATE, to: ORDINATE })(
    "is the same read backwards",
    ({ from, level, to }) => {
      expect(isLevelCrossed(to, from, level)).toBe(isLevelCrossed(from, to, level));
    },
  );

  test.prop({ from: ORDINATE, level: ORDINATE, to: ORDINATE })(
    "holds exactly from the lower end, included, to the upper one, excluded",
    ({ from, level, to }) => {
      expect(isLevelCrossed(from, to, level)).toBe(
        Math.min(from, to) <= level && level < Math.max(from, to),
      );
    },
  );
});
