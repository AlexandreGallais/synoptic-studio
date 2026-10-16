import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textFrame } from "./text-frame";

/** "Hello" over "World" in Arial 16 without kerning: 41.78125 wide, 36.796875 tall. */
const BLOCK = { height: 36.796875, width: 41.78125 };

describe("textFrame (DERIV-text-box step 8)", () => {
  it("[F14.AC3] lays a text in a box out in its box, whatever its block", () => {
    const text = { ...HELLO_TEXT, boxHeight: 40, boxWidth: 60, isInBox: true, x: 10, y: 20 };

    expect(textFrame(text, BLOCK)).toEqual({ height: 40, width: 60, x: 10, y: 20 });
  });

  it("[F14.AC6] lays a text that fits its content out in its block, as in F05", () => {
    const text = { ...HELLO_TEXT, boxHeight: 40, boxWidth: 60, x: 10, y: 20 };

    expect(textFrame(text, BLOCK)).toEqual({ ...BLOCK, x: 10, y: 20 });
  });

  test.prop({
    block: fc.record({
      height: fc.double({ max: 500, min: 0, noNaN: true }),
      width: fc.double({ max: 500, min: 0, noNaN: true }),
    }),
    box: fc.record({
      boxHeight: fc.integer({ max: 500, min: 0 }),
      boxWidth: fc.integer({ max: 500, min: 0 }),
    }),
    isInBox: fc.boolean(),
    place: fc.record({
      x: fc.integer({ max: 100, min: -100 }),
      y: fc.integer({ max: 100, min: -100 }),
    }),
  })(
    "lays any text out at its position, its box's size in a box, its block's otherwise",
    (values) => {
      const text = { ...HELLO_TEXT, ...values.box, ...values.place, isInBox: values.isInBox };
      const frame = textFrame(text, values.block);
      const size = values.isInBox
        ? { height: values.box.boxHeight, width: values.box.boxWidth }
        : values.block;

      expect(frame).toEqual({ ...size, ...values.place });
    },
  );
});
