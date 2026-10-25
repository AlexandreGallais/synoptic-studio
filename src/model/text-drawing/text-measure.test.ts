import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";

import { measuredTextDrawing } from "./measured-text-drawing";
import { textDrawing } from "./text-drawing";
import { textMeasure } from "./text-measure";

describe("textMeasure and measuredTextDrawing (DERIV-text-box)", () => {
  it("[F17.AC5] measures 'Hello World' in a box 50 wide: two lines, the widest 41.78125", () => {
    // Arial 16: "Hello" 36.4609375, "World" 41.78125 wide; two lines of 18.3984375.
    expect(
      textMeasure(ARIAL_BOX_METRICS, { ...HELLO_TEXT, boxHeight: 60, boxWidth: 50, isInBox: true }),
    ).toEqual({
      block: { height: 36.796875, width: 41.78125 },
      lines: ["Hello", "World"],
    });
  });

  it("[F17.AC16] gives no height to the empty lines dropped after the ellipsis", () => {
    // 4 lines, the 3rd empty, at most 3: "Hello" over "World…" (57.78125), 2 × 18.3984375 tall.
    const text = {
      ...HELLO_TEXT,
      boxHeight: 100,
      boxWidth: 60,
      content: "Hello\nWorld\n\nHello",
      isInBox: true,
      maximumLines: 3,
    };

    expect(textMeasure(ARIAL_BOX_METRICS, text)).toEqual({
      block: { height: 36.796875, width: 57.78125 },
      lines: ["Hello", "World…"],
    });
  });

  it("[F17.AC5] draws from the measure what textDrawing draws", () => {
    const text = { ...HELLO_TEXT, boxHeight: 60, boxWidth: 50, isInBox: true };

    expect(
      measuredTextDrawing(ARIAL_BOX_METRICS, text, textMeasure(ARIAL_BOX_METRICS, text)),
    ).toEqual(textDrawing(ARIAL_BOX_METRICS, text));
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    opacity: fc.integer({ max: 100, min: 0 }),
    x: fc.integer({ max: 1000, min: -1000 }),
    y: fc.integer({ max: 1000, min: -1000 }),
  })("[F17.AC5] measures the same whatever the position, angle and opacity", (moved) => {
    expect(textMeasure(ARIAL_BOX_METRICS, { ...HELLO_TEXT, ...moved, color: "#ff0000" })).toEqual(
      textMeasure(ARIAL_BOX_METRICS, HELLO_TEXT),
    );
  });
});
