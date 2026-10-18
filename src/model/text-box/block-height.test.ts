import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { blockHeight } from "./block-height";

/** Arial 16's automatic line height, 2355 / 128, and its cap height at 16, 1467 / 128. */
const ARIAL_16 = { capHeight: 11.4609375, line: 18.3984375 };

describe("blockHeight (DERIV-text-box step 5)", () => {
  it("[F14.AC6] stacks two automatic lines, as in F05", () => {
    // 2 × 18.3984375 = 36.796875.
    expect(blockHeight({ firstLine: ARIAL_16.line, line: ARIAL_16.line }, 2)).toBe(36.796875);
  });

  it("[F14.AC3] stacks two lines of a line height of 24 into 48", () => {
    expect(blockHeight({ firstLine: 24, line: 24 }, 2)).toBe(48);
  });

  it("[F14.AC5] starts a trimmed block at its cap height", () => {
    const heights = { firstLine: ARIAL_16.capHeight, line: ARIAL_16.line };

    // 11.4609375 alone; then 18.3984375 + 11.4609375 = 29.859375.
    expect(blockHeight(heights, 1)).toBe(11.4609375);
    expect(blockHeight(heights, 2)).toBe(29.859375);
  });

  it("gives 0 for a block of no line, an empty text", () => {
    expect(blockHeight({ firstLine: ARIAL_16.capHeight, line: ARIAL_16.line }, 0)).toBe(0);
  });

  test.prop({
    count: fc.integer({ max: 1000, min: 1 }),
    firstLine: fc.integer({ max: 100_000, min: 1 }),
    line: fc.integer({ max: 100_000, min: 1 }),
  })("grows by one line height with each line", ({ count, firstLine, line }) => {
    const heights = { firstLine, line };

    expect(blockHeight(heights, count + 1) - blockHeight(heights, count)).toBeCloseTo(line, 6);
  });
});
