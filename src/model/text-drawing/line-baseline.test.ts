import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, HELLO_TEXT } from "../../../tests";
import { lineHeight } from "../text-block";

import { lineBaseline } from "./line-baseline";

describe("lineBaseline (DERIV-text-width step 4)", () => {
  it("[F05.AC4] puts the first baseline of Arial 16 at 14.484375 below the top", () => {
    // 1854 × 16 / 2048 = 14.484375.
    expect(lineBaseline(ARIAL_METRICS, HELLO_TEXT, 0)).toBe(14.484375);
  });

  it("[F05.AC4] puts the second one a line lower, from a block at y = 20", () => {
    // 20 + 18.3984375 + 14.484375 = 52.8828125.
    expect(lineBaseline(ARIAL_METRICS, { ...HELLO_TEXT, y: 20 }, 1)).toBe(52.8828125);
  });

  test.prop({
    index: fc.integer({ max: 1000, min: 0 }),
    size: fc.integer({ max: 100_000, min: 1 }),
    y: fc.integer({ max: 100_000, min: -100_000 }),
  })("steps down by one line height from line to line", ({ index, size, y }) => {
    const text = { ...HELLO_TEXT, size, y };

    expect(
      lineBaseline(ARIAL_METRICS, text, index + 1) - lineBaseline(ARIAL_METRICS, text, index),
    ).toBeCloseTo(lineHeight(ARIAL_METRICS, size), 3);
  });
});
