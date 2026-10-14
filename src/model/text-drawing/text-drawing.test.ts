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

  it("draws an empty text as nothing", () => {
    expect(
      textDrawing(DECORATED_METRICS, { ...HELLO_TEXT, content: "", isUnderlined: true }),
    ).toEqual({ decorations: [], lines: [] });
  });
});
