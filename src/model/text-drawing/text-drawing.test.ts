import { describe, expect, it } from "vitest";

import { DECORATED_METRICS, HELLO_TEXT } from "../../../tests";

import { textDrawing } from "./text-drawing";

describe("textDrawing (ADR-0052, DERIV-text-width steps 4 to 6)", () => {
  it("[F05.AC4] underlines each line that has a length, an empty line in between", () => {
    const text = { ...HELLO_TEXT, content: "Hello\n\nWorld", isUnderlined: true };
    const drawn = textDrawing(DECORATED_METRICS, text);

    // Baselines 14.484375 and 14.484375 + 2 × 18.3984375 = 51.28125; underlines 2 below them.
    expect(drawn.lines).toHaveLength(3);

    expect(drawn.decorations).toEqual([
      { height: 1, width: 36.4609375, x: 0, y: 16.484375 },
      { height: 1, width: 41.78125, x: 0, y: 53.28125 },
    ]);
  });

  it("[F05.AC4] keeps a trailing empty line: a second baseline, no stroke, two lines tall", () => {
    const drawn = textDrawing(DECORATED_METRICS, {
      ...HELLO_TEXT,
      content: "Hello\n",
      isUnderlined: true,
    });

    // 14.484375 + 18.3984375 = 32.8828125; block 2 × 18.3984375 = 36.796875 tall.
    expect(drawn.lines.map((line) => line.y)).toEqual([14.484375, 32.8828125]);
    expect(drawn.decorations).toHaveLength(1);
    expect(drawn.block).toEqual({ height: 36.796875, width: 36.4609375 });
  });

  it("[F05.AC4] starts each stroke of a right-to-left text at its line's left edge", () => {
    const drawn = textDrawing(DECORATED_METRICS, {
      ...HELLO_TEXT,
      alignment: "right",
      content: "Hello\nWorld",
      direction: "rtl",
      isUnderlined: true,
    });

    // 'Hello' from 41.78125 − 36.4609375 = 5.3203125 to 41.78125, where its x is written.
    expect(drawn.lines.map((line) => line.x)).toEqual([41.78125, 41.78125]);
    expect(drawn.decorations.map((stroke) => stroke.x)).toEqual([5.3203125, 0]);
  });

  it("draws an empty text as nothing", () => {
    expect(
      textDrawing(DECORATED_METRICS, { ...HELLO_TEXT, content: "", isUnderlined: true }),
    ).toEqual({ block: { height: 0, width: 0 }, decorations: [], lines: [] });
  });
});
