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
});
