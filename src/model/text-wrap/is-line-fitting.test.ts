import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";

import { isLineFitting } from "./is-line-fitting";

/** Arial 16 without kerning in a box 50 wide. */
const BOXED = { ...HELLO_TEXT, boxWidth: 50, isInBox: true };

describe("isLineFitting (DERIV-text-box step 2)", () => {
  it("[F14.AC1] refuses 'Hello World' in a box 50 wide", () => {
    // 4667 + 569 + 5348 = 10 584 units, 10 584 / 128 = 82.6875 > 50.
    expect(isLineFitting(ARIAL_BOX_METRICS, "Hello World", BOXED)).toBe(false);
  });

  it("[F14.AC1] fits 'Hello' and its trailing spaces, which are not measured for fit", () => {
    // 4667 / 128 = 36.46094 ≤ 50; with its two spaces 5805 / 128 = 45.35156, and with ten
    // 10 357 / 128 = 80.91406 — neither counts.
    expect(isLineFitting(ARIAL_BOX_METRICS, "Hello  ", BOXED)).toBe(true);
    expect(isLineFitting(ARIAL_BOX_METRICS, `Hello${" ".repeat(10)}`, BOXED)).toBe(true);
  });

  it("fits a line exactly as wide as the box, and no wider", () => {
    // At size 2048 one font unit is one user unit: "Hello" is 4667 wide, which a box 4667 wide
    // holds and one 4666 wide does not.
    const large = { ...BOXED, size: 2048 };

    expect(isLineFitting(ARIAL_BOX_METRICS, "Hello", { ...large, boxWidth: 4667 })).toBe(true);
    expect(isLineFitting(ARIAL_BOX_METRICS, "Hello", { ...large, boxWidth: 4666 })).toBe(false);
    // An empty line, or one of spaces, is 0 wide: it fits a box 0 wide.
    expect(isLineFitting(ARIAL_BOX_METRICS, "  ", { ...BOXED, boxWidth: 0 })).toBe(true);
  });

  it("counts the letter and word spacing of the line", () => {
    // "Hello World" 82.6875 plus 10 letter spacings of −2 and one word spacing of −3: 59.6875.
    const text = { ...BOXED, boxWidth: 60, letterSpacing: -2, wordSpacing: -3 };

    expect(isLineFitting(ARIAL_BOX_METRICS, "Hello World", text)).toBe(true);
  });
});
