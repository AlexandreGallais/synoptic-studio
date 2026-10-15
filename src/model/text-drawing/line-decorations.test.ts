import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, DECORATED_METRICS, HELLO_TEXT } from "../../../tests";

import { lineDecorations } from "./line-decorations";

/** 'Hello' in the decorated test font 16, aligned left, its baseline at 14.484375. */
const HELLO_LINE = { content: "Hello", left: 0, width: 36.4609375, x: 0, y: 14.484375 };

/** The text's color, black and opaque, which a stroke follows until its own is chosen. */
const BLACK = { color: "#000000", opacity: 100 } as const;

/** Underlined and struck through. */
const DECORATED = { ...HELLO_TEXT, isStruckThrough: true, isUnderlined: true };

describe("lineDecorations (ADR-0052, DERIV-text-width step 6)", () => {
  it("[F05.AC4] draws the underline from `post` and the strike-through from `OS/2`", () => {
    // Underline: top −256 × 16 / 2048 = −2 from the baseline, 128 × 16 / 2048 = 1 thick, so
    // y = 14.484375 + 2; strike-out: top 512 × 16 / 2048 = 4 above it, y = 14.484375 − 4,
    // 64 × 16 / 2048 = 0.5 thick.
    expect(lineDecorations(DECORATED_METRICS, DECORATED, HELLO_LINE)).toEqual([
      { ...BLACK, height: 1, width: 36.4609375, x: 0, y: 16.484375 },
      { ...BLACK, height: 0.5, width: 36.4609375, x: 0, y: 10.484375 },
    ]);
  });

  it("draws only what the text asks", () => {
    expect(lineDecorations(DECORATED_METRICS, HELLO_TEXT, HELLO_LINE)).toEqual([]);

    expect(
      lineDecorations(DECORATED_METRICS, { ...HELLO_TEXT, isStruckThrough: true }, HELLO_LINE),
    ).toEqual([{ ...BLACK, height: 0.5, width: 36.4609375, x: 0, y: 10.484375 }]);
  });

  it("draws nothing along a line with no length, empty or tightened past nothing", () => {
    expect(lineDecorations(DECORATED_METRICS, DECORATED, { ...HELLO_LINE, width: 0 })).toEqual([]);
    expect(lineDecorations(DECORATED_METRICS, DECORATED, { ...HELLO_LINE, width: -8 })).toEqual([]);
  });

  it("draws no stroke the font gives no thickness, 0 or negative", () => {
    // ARIAL_UNKERNED reads no decoration: both thicknesses 0.
    expect(lineDecorations(ARIAL_UNKERNED, DECORATED, HELLO_LINE)).toEqual([]);

    expect(
      lineDecorations({ ...DECORATED_METRICS, strikeoutSize: -64 }, DECORATED, HELLO_LINE),
    ).toEqual([{ ...BLACK, height: 1, width: 36.4609375, x: 0, y: 16.484375 }]);
  });

  it("[F16.AC4] draws an underline chosen #ff0000 at 50 % in it, the strike-through in the text's color", () => {
    const text = {
      ...DECORATED,
      hasUnderlineColor: true,
      underlineColor: "#ff0000",
      underlineOpacity: 50,
    };

    expect(lineDecorations(DECORATED_METRICS, text, HELLO_LINE)).toEqual([
      { color: "#ff0000", height: 1, opacity: 50, width: 36.4609375, x: 0, y: 16.484375 },
      { ...BLACK, height: 0.5, width: 36.4609375, x: 0, y: 10.484375 },
    ]);
  });

  it("[F16.AC4] draws both strokes in the text's #0000ff when neither color was chosen", () => {
    const text = { ...DECORATED, color: "#0000ff", opacity: 60, underlineColor: "#ff0000" };

    expect(
      lineDecorations(DECORATED_METRICS, text, HELLO_LINE).map(({ color, opacity }) => [
        color,
        opacity,
      ]),
    ).toEqual([
      ["#0000ff", 60],
      ["#0000ff", 60],
    ]);
  });
});
