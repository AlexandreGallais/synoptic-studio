import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { EPSILON } from "../../math";

import { boxAroundBlock } from "./box-around-block";

describe("boxAroundBlock (DERIV-text-box, step 11)", () => {
  it.each([
    // "Pump P-101" in Inter 16: 87.953125 × 19.359375, each side rounded up.
    { block: { height: 19.359375, width: 87.953125 }, expected: { boxHeight: 20, boxWidth: 88 } },
    // Two lines, 2 × 19.359375 = 38.71875 tall.
    { block: { height: 38.71875, width: 87.953125 }, expected: { boxHeight: 39, boxWidth: 88 } },
    // Integers stay; a side 1e-10 above an integer is that integer within the rounding error.
    { block: { height: 20, width: 88 }, expected: { boxHeight: 20, boxWidth: 88 } },
    {
      block: { height: 20.0000000001, width: 88.0000000001 },
      expected: { boxHeight: 20, boxWidth: 88 },
    },
    // 2e-9 above an integer is beyond the error: one more.
    {
      block: { height: 20.000000002, width: 88.000000002 },
      expected: { boxHeight: 21, boxWidth: 89 },
    },
  ])("[F17.AC13] puts the block $block in the box $expected", ({ block, expected }) => {
    expect(boxAroundBlock(block)).toEqual(expected);
  });

  it("[F17.AC13] gives an empty text's block 0 × 0 the box 0 × 0, not −0", () => {
    expect(boxAroundBlock({ height: 0, width: 0 })).toEqual({ boxHeight: 0, boxWidth: 0 });
    expect(Object.is(boxAroundBlock({ height: 0, width: 0 }).boxWidth, 0)).toBe(true);
  });

  test.prop({
    height: fc.double({ max: 100_000, min: 0, noNaN: true }),
    width: fc.double({ max: 100_000, min: 0, noNaN: true }),
  })("is the smallest box of integer sides that holds the block within EPSILON", (block) => {
    const box = boxAroundBlock(block);

    for (const [side, measured] of [
      [box.boxWidth, block.width],
      [box.boxHeight, block.height],
    ] as const) {
      expect(Number.isSafeInteger(side)).toBe(true);
      expect(measured).toBeLessThanOrEqual(side + EPSILON);
      expect(measured).toBeGreaterThan(side - 1 + EPSILON);
    }
  });
});
