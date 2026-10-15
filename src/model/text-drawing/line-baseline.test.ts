import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { textLineHeight } from "../text-block";

import { lineBaseline } from "./line-baseline";

/** "Hello" over "World", the text of the derivation's placements. */
const HELLO_WORLD = { ...HELLO_TEXT, content: "Hello\nWorld" };

describe("lineBaseline (DERIV-text-box step 7)", () => {
  it("[F05.AC4] puts the first baseline of Arial 16 at 14.484375 below the top", () => {
    // 1854 × 16 / 2048 = 14.484375.
    expect(lineBaseline(ARIAL_METRICS, HELLO_TEXT, { index: 0, top: 0 })).toBe(14.484375);
  });

  it("[F05.AC4] puts the second one a line lower, from a block at y = 20", () => {
    // 20 + 18.3984375 + 14.484375 = 52.8828125.
    expect(lineBaseline(ARIAL_METRICS, HELLO_TEXT, { index: 1, top: 20 })).toBe(52.8828125);
  });

  it.each([
    // Automatic, as in F05: 14.484375, then 18.3984375 + 14.484375 = 32.8828125.
    { case: "an automatic line height", expected: [14.484375, 32.8828125], text: HELLO_WORLD },
    // 3.0625 + 14.484375 = 17.546875, then 24 more: 41.546875.
    {
      case: "a line height of 24",
      expected: [17.546875, 41.546875],
      text: { ...HELLO_WORLD, lineHeight: 24 },
    },
    // 1467 / 128 = 11.4609375, then 18.3984375 more: 29.859375.
    {
      case: "a trimmed block",
      expected: [11.4609375, 29.859375],
      text: { ...HELLO_WORLD, isTrimmed: true },
    },
  ])("[F14.AC3] places 'Hello' over 'World' with $case", ({ expected, text }) => {
    expect([0, 1].map((index) => lineBaseline(ARIAL_UNKERNED, text, { index, top: 0 }))).toEqual(
      expected,
    );
  });

  test.prop({
    index: fc.integer({ max: 1000, min: 0 }),
    isTrimmed: fc.boolean(),
    lineHeight: fc.integer({ max: 100_000, min: 0 }),
    size: fc.integer({ max: 100_000, min: 1 }),
    top: fc.integer({ max: 100_000, min: -100_000 }),
  })("[F14.AC3] puts consecutive baselines one line height apart", ({ index, top, ...options }) => {
    const text = { ...HELLO_TEXT, ...options };

    expect(
      lineBaseline(ARIAL_METRICS, text, { index: index + 1, top }) -
        lineBaseline(ARIAL_METRICS, text, { index, top }),
    ).toBeCloseTo(textLineHeight(ARIAL_METRICS, text), 3);
  });
});
