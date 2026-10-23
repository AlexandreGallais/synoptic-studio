import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textPanelValues } from "./text-panel-values";

import type { PanelValues } from "./panel-values";

/**
 * Asserts each value to 5 decimals, the precision the panel shows them with (Q10).
 *
 * @param actual - values computed
 * @param expected - values computed by hand
 */
function expectValues(actual: PanelValues, expected: PanelValues): void {
  expect(actual.x).toBeCloseTo(expected.x, 5);
  expect(actual.y).toBeCloseTo(expected.y, 5);
  expect(actual.width).toBeCloseTo(expected.width, 5);
  expect(actual.height).toBeCloseTo(expected.height, 5);
}

/** A block measured 80 × 30. */
const BLOCK = { height: 30, width: 80 };

describe("textPanelValues (F38)", () => {
  it("[F38.AC1] gives a text in a box 40 × 40 at (200, 20) its box, whatever its block", () => {
    const text = { ...HELLO_TEXT, boxHeight: 40, boxWidth: 40, isInBox: true, x: 200, y: 20 };

    expect(textPanelValues(text, BLOCK)).toEqual({ height: 40, width: 40, x: 200, y: 20 });
  });

  it("[F38.AC1] gives a text not in its box its block 80 × 30 at (300, 20)", () => {
    const text = { ...HELLO_TEXT, boxHeight: 40, boxWidth: 40, x: 300, y: 20 };

    expect(textPanelValues(text, BLOCK)).toEqual({ height: 30, width: 80, x: 300, y: 20 });
  });

  it("[F38.AC1] gives the block 80 × 30 at (300, 20) turned by 90° X 325, Y −5, W 80, H 30", () => {
    // About its middle (340, 35), the frame turned by 90° is 30 wide and 80 tall:
    // x = 340 − 15 = 325, y = 35 − 40 = −5; its own size kept.
    expectValues(textPanelValues({ ...HELLO_TEXT, angle: 90, x: 300, y: 20 }, BLOCK), {
      height: 30,
      width: 80,
      x: 325,
      y: -5,
    });
  });

  it("gives a block 40 × 20 at (0, 0) turned by 45° its straight box's corner", () => {
    // Half sizes 20 cos 45° + 10 sin 45° = 30 · √2 / 2 = 21.21320 about (20, 10), as
    // `textStraightBox`'s example: x = −1.21320, y = −11.21320.
    expectValues(
      textPanelValues({ ...HELLO_TEXT, angle: 45, x: 0, y: 0 }, { height: 20, width: 40 }),
      {
        height: 20,
        width: 40,
        x: -1.2132,
        y: -11.2132,
      },
    );
  });

  it("gives an empty block its position and no size", () => {
    expect(textPanelValues({ ...HELLO_TEXT, x: 7, y: 9 }, { height: 0, width: 0 })).toEqual({
      height: 0,
      width: 0,
      x: 7,
      y: 9,
    });
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    height: fc.integer({ max: 500, min: 0 }),
    isInBox: fc.boolean(),
    width: fc.integer({ max: 500, min: 0 }),
    x: fc.integer({ max: 500, min: -500 }),
    y: fc.integer({ max: 500, min: -500 }),
  })("gives a text its frame's size, and its straight box about its frame's middle", (frame) => {
    const { angle, height, isInBox, width, x, y } = frame;
    const text = { ...HELLO_TEXT, angle, boxHeight: height, boxWidth: width, isInBox, x, y };
    const block = isInBox ? BLOCK : { height, width };
    const values = textPanelValues(text, block);

    expect(values.width).toBe(width);
    expect(values.height).toBe(height);
    expect(values.x).toBeLessThanOrEqual(x + width / 2 + 1e-9);
    expect(values.y).toBeLessThanOrEqual(y + height / 2 + 1e-9);
  });
});
