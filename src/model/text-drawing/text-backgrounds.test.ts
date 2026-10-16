import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";

import { textBackgrounds } from "./text-backgrounds";

/** 'Hello' in Arial 16, aligned left, its baseline at 14.484375. */
const HELLO_LINE = { content: "Hello", left: 0, width: 36.4609375, x: 0, y: 14.484375 };

/** 'World' under it, its baseline 18.3984375 lower. */
const WORLD_LINE = { content: "World", left: 0, width: 41.78125, x: 0, y: 32.8828125 };

/** A background turned on. */
const HIGHLIGHTED = { ...HELLO_TEXT, hasBackground: true };

describe("textBackgrounds (DERIV-text-box, step 10)", () => {
  it("[F14.AC4] covers each line from the ascender to the descender, as long as the line", () => {
    // Ascent 1854 × 16 / 2048 = 14.484375; height (1854 + 434) × 16 / 2048 = 17.875; tops
    // 14.484375 − 14.484375 = 0 and 32.8828125 − 14.484375 = 18.3984375.
    expect(textBackgrounds(ARIAL_BOX_METRICS, HIGHLIGHTED, [HELLO_LINE, WORLD_LINE])).toEqual([
      { height: 17.875, width: 36.4609375, x: 0, y: 0 },
      { height: 17.875, width: 41.78125, x: 0, y: 18.3984375 },
    ]);
  });

  it("[F14.AC4] starts a right-to-left line's background at its left edge, not at its x", () => {
    const line = { ...HELLO_LINE, left: 5.3203125, x: 41.78125 };

    expect(textBackgrounds(ARIAL_BOX_METRICS, HIGHLIGHTED, [line])).toEqual([
      { height: 17.875, width: 36.4609375, x: 5.3203125, y: 0 },
    ]);
  });

  it("[F14.AC4] gives no background to a line with no length, empty or tightened past nothing", () => {
    const lines = [
      HELLO_LINE,
      { ...HELLO_LINE, content: "", width: 0 },
      { ...WORLD_LINE, width: -8 },
    ];

    expect(textBackgrounds(ARIAL_BOX_METRICS, HIGHLIGHTED, lines)).toHaveLength(1);
  });

  it("[F14.AC6] draws no background when the text has none", () => {
    expect(textBackgrounds(ARIAL_BOX_METRICS, HELLO_TEXT, [HELLO_LINE, WORLD_LINE])).toEqual([]);
  });

  test.prop({
    size: fc.integer({ max: 100_000, min: 1 }),
    width: fc.double({ max: 1000, min: 0.001, noNaN: true }),
    y: fc.integer({ max: 100_000, min: -100_000 }),
  })(
    "[F14.AC4] sits the background's top an ascent above the baseline, its height ascent plus depth",
    ({ size, width, y }) => {
      const text = { ...HIGHLIGHTED, size };
      const [background = { height: NaN, width: NaN, x: NaN, y: NaN }] = textBackgrounds(
        ARIAL_BOX_METRICS,
        text,
        [{ ...HELLO_LINE, width, y }],
      );
      const ascent = (1854 * size) / 2048;
      const depth = (434 * size) / 2048;

      expect(background.width).toBe(width);
      expect(background.y + ascent).toBeCloseTo(y, 6);
      expect(background.height).toBeCloseTo(ascent + depth, 6);
    },
  );
});
