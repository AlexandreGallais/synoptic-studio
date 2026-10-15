import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { textDrawing } from "../text-drawing";
import { blockOffsetInBox } from "../text-layout";

import { cssFirstBaselineOffset } from "./css-first-baseline-offset";
import { htmlLayerFrame } from "./html-layer-frame";

import type { Text } from "../text";

/** "Hello" over "World" in Arial 16 without kerning, at (0, 0): block 41.78125 × 36.796875. */
const HELLO_WORLD: Text = { ...HELLO_TEXT, content: "Hello\nWorld", font: "Arial" };

/** The same text in a box 50 × 40 at (0, 0). */
const BOXED: Text = { ...HELLO_WORLD, boxHeight: 40, boxWidth: 50, isInBox: true };

/**
 * The layer of a text drawn in Arial without kerning.
 *
 * @param text - its content, box, line height and trim
 * @returns its layer's rectangle
 */
function frameOf(text: Text): ReturnType<typeof htmlLayerFrame> {
  return htmlLayerFrame(ARIAL_UNKERNED, text, textDrawing(ARIAL_UNKERNED, text));
}

describe("htmlLayerFrame (DERIV-html-layer, step 4 and checks)", () => {
  it("[F15.AC3] raises the block of an automatic line height by half the line gap", () => {
    // k · G / 2 = 0.0078125 · 33.5 = 0.26171875; 2 · 18.3984375 = 36.796875 tall; bottom
    // 36.796875 − 0.26171875 = 36.53515625.
    expect(frameOf(HELLO_WORLD)).toEqual({
      height: 36.796875,
      width: 41.78125,
      x: 0,
      y: -0.26171875,
    });
  });

  it("[F15.AC3] places the layer at the text's x, its y raised", () => {
    // At (47, 61): y 61 − 0.26171875 = 60.73828125.
    expect(frameOf({ ...HELLO_WORLD, x: 47, y: 61 })).toEqual({
      height: 36.796875,
      width: 41.78125,
      x: 47,
      y: 60.73828125,
    });

    expect(frameOf({ ...BOXED, x: -12, y: 5 }).x).toBe(-12);
  });

  it("[F15.AC3] raises Arial 160 by 2.6171875", () => {
    // 0.078125 · 33.5 = 2.6171875.
    expect(frameOf({ ...HELLO_WORLD, size: 160 }).y).toBeCloseTo(-2.6171875, 12);
  });

  it("[F15.AC3] does not move a line height of 24, nor a face without line gap", () => {
    expect(frameOf({ ...HELLO_WORLD, lineHeight: 24 })).toEqual({
      height: 48,
      width: 41.78125,
      x: 0,
      y: 0,
    });

    const noGap = { ...ARIAL_UNKERNED, lineGap: 0 };

    expect(htmlLayerFrame(noGap, HELLO_WORLD, textDrawing(noGap, HELLO_WORLD)).y).toBe(0);
  });

  it("[F15.AC3] raises a trimmed block by 3.28515625, its height untrimmed", () => {
    // Untrimmed baseline 14.484375, trimmed 0.0078125 · 1467 = 11.4609375: 3.0234375 higher,
    // plus 0.26171875.
    expect(frameOf({ ...HELLO_WORLD, isTrimmed: true })).toEqual({
      height: 36.796875,
      width: 41.78125,
      x: 0,
      y: -3.28515625,
    });
  });

  it("[F15.AC3] raises a box 50 × 40 by 0.26171875, its bottom at 39.73828125", () => {
    const frame = frameOf({ ...BOXED, verticalAlignment: "bottom" });

    expect(frame).toEqual({ height: 40, width: 50, x: 0, y: -0.26171875 });
    expect(frame.y + frame.height).toBe(39.73828125);
  });

  it("[F15.AC3] lowers a trimmed box aligned at the bottom by 3.65234375", () => {
    // λ − f = 18.3984375 − 11.4609375 = 6.9375; 6.9375 + 11.4609375 − 14.74609375 = 3.65234375
    // (DERIV-html-layer, a trimmed box): both last baselines at 40.
    expect(frameOf({ ...BOXED, isTrimmed: true, verticalAlignment: "bottom" }).y).toBe(3.65234375);
  });

  it("places an empty text at its position, 0 tall", () => {
    expect(frameOf({ ...HELLO_WORLD, content: "" })).toEqual({
      height: 0,
      width: 0,
      x: 0,
      y: -0.26171875,
    });
  });

  test.prop({
    isInBox: fc.boolean(),
    isTrimmed: fc.boolean(),
    lineHeight: fc.integer({ max: 60, min: 0 }),
    verticalAlignment: fc.constantFrom("top" as const, "middle" as const, "bottom" as const),
    y: fc.integer({ max: 50, min: -50 }),
  })("[F15.AC3] puts the browser's first baseline on the library's", (options) => {
    const text = { ...BOXED, ...options };
    const drawn = textDrawing(ARIAL_UNKERNED, text);
    const frame = htmlLayerFrame(ARIAL_UNKERNED, text, drawn);
    const lineHeight = text.lineHeight === 0 ? 18.3984375 : text.lineHeight;
    const browserBaseline =
      frame.y +
      blockOffsetInBox(frame.height, drawn.lines.length * lineHeight, text.verticalAlignment) +
      cssFirstBaselineOffset(ARIAL_UNKERNED, text);

    expect(browserBaseline).toBeCloseTo(drawn.lines[0]?.y ?? NaN, 9);
  });
});
