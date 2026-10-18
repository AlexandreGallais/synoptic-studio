import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { blockOffsetInBox } from "./block-offset-in-box";

/** "Hello" over "World" in Arial 16: two lines of 2355 / 128, 36.796875 tall. */
const TWO_LINES = 36.796875;

describe("blockOffsetInBox (DERIV-text-box step 8)", () => {
  it.each([
    // Room 60 − 36.796875 = 23.203125: none, half or all of it above the block.
    { offset: 0, verticalAlignment: "top" },
    { offset: 11.6015625, verticalAlignment: "middle" },
    { offset: 23.203125, verticalAlignment: "bottom" },
  ] as const)(
    "[F14.AC3] puts the block $offset below a box 60 tall's top, aligned $verticalAlignment",
    ({ offset, verticalAlignment }) => {
      expect(blockOffsetInBox(60, TWO_LINES, verticalAlignment)).toBe(offset);
    },
  );

  it("[F14.AC3] centers a block taller than its box over both edges", () => {
    // One line of 18.3984375 in a box 10 tall: (10 − 18.3984375) / 2 = −4.19921875.
    expect(blockOffsetInBox(10, 18.3984375, "middle")).toBe(-4.19921875);
  });

  test.prop({
    block: fc.integer({ max: 100_000, min: 0 }),
    box: fc.integer({ max: 100_000, min: 0 }),
  })("ends a bottom block at the box's bottom, a middle one halfway", ({ block, box }) => {
    // −0 for a block taller than its box, (H − h) × 0 of a negative room: the box's y + −0 is y.
    expect(blockOffsetInBox(box, block, "top")).toBeCloseTo(0, 9);
    expect(blockOffsetInBox(box, block, "bottom") + block).toBeCloseTo(box, 6);

    expect(2 * blockOffsetInBox(box, block, "middle")).toBeCloseTo(
      blockOffsetInBox(box, block, "bottom"),
      6,
    );
  });
});
