import { describe, expect, it } from "vitest";

import { DECORATED_METRICS, HELLO_TEXT } from "../../../tests";

import { lineDecorations } from "./line-decorations";

/** 'Hello' in the decorated test font 16, aligned left, its baseline at 14.484375. */
const HELLO_LINE = { content: "Hello", left: 0, width: 36.4609375, x: 0, y: 14.484375 };

describe("lineDecorations (ADR-0052, DERIV-text-width step 6)", () => {
  it("[F05.AC4] draws the underline from `post` and the strike-through from `OS/2`", () => {
    const text = { ...HELLO_TEXT, isStruckThrough: true, isUnderlined: true };

    // Underline: top −256 × 16 / 2048 = −2 from the baseline, 128 × 16 / 2048 = 1 thick, so
    // y = 14.484375 + 2; strike-out: top 512 × 16 / 2048 = 4 above it, y = 14.484375 − 4.
    expect(lineDecorations(DECORATED_METRICS, text, HELLO_LINE)).toEqual([
      { height: 1, width: 36.4609375, x: 0, y: 16.484375 },
      { height: 1, width: 36.4609375, x: 0, y: 10.484375 },
    ]);
  });

  it("draws only what the text asks", () => {
    expect(lineDecorations(DECORATED_METRICS, HELLO_TEXT, HELLO_LINE)).toEqual([]);

    expect(
      lineDecorations(DECORATED_METRICS, { ...HELLO_TEXT, isStruckThrough: true }, HELLO_LINE),
    ).toEqual([{ height: 1, width: 36.4609375, x: 0, y: 10.484375 }]);
  });

  it("draws nothing along a line with no length, empty or tightened past nothing", () => {
    const text = { ...HELLO_TEXT, isStruckThrough: true, isUnderlined: true };

    expect(lineDecorations(DECORATED_METRICS, text, { ...HELLO_LINE, width: 0 })).toEqual([]);
    expect(lineDecorations(DECORATED_METRICS, text, { ...HELLO_LINE, width: -8 })).toEqual([]);
  });
});
