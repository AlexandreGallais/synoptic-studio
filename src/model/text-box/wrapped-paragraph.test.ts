import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { lineWidth } from "../text-block";

import { isLineFitting } from "./is-line-fitting";
import { paragraphTokens } from "./paragraph-tokens";
import { wrappedParagraph } from "./wrapped-paragraph";

import type { Text } from "../text";

/** Arial 16 without kerning in a box 50 wide, a word too long moved whole. */
const BOXED: Text = {
  ...HELLO_TEXT,
  boxHeight: 1000,
  boxWidth: 50,
  isInBox: true,
  longWord: "move",
};

/** A paragraph of words written with the characters of the fixture's Arial, and spaces. */
const PARAGRAPH = fc.stringMatching(/^[ HWdelor]{0,40}$/u);

describe("wrappedParagraph (DERIV-text-box steps 1 to 3)", () => {
  it("[F14.AC1] wraps 'Hello World' in a box 50 wide after the space", () => {
    // 4667 + 569 + 5348 = 10 584 units, 82.6875 > 50: "Hello" 36.46094 over "World" 41.78125.
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, "Hello World", BOXED);

    expect(lines).toEqual(["Hello", "World"]);

    expect(lines.map((line) => lineWidth(ARIAL_BOX_METRICS, line, BOXED))).toEqual([
      36.4609375, 41.78125,
    ]);
  });

  it("[F14.AC1] breaks after the second of two spaces, neither measured nor drawn", () => {
    // "Hello" 4667 / 128 = 36.4609375, its two spaces dropped at the wrap.
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, "Hello  World", BOXED);

    expect(lines).toEqual(["Hello", "World"]);
    expect(lineWidth(ARIAL_BOX_METRICS, lines[0] ?? "", BOXED)).toBe(36.4609375);
  });

  it("[F14.AC1] never breaks at a no-break space", () => {
    expect(wrappedParagraph(ARIAL_BOX_METRICS, "Hello\u{A0}World", BOXED)).toEqual([
      "Hello\u{A0}World",
    ]);
  });

  it("[F14.AC1] moves a word too long whole to its own line, wider than the box", () => {
    // "Pump" 5350 / 128 = 41.79688; "Pump P-101" 41.79688 + 4.44531 + 42.69531 = 88.9375 > 50;
    // "P-101 is" 42.69531 + 4.44531 + 11.55469 = 58.69531 > 50; "is running" 69.375 > 50;
    // "running" 6832 / 128 = 53.375 > 50 stays whole.
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, "Pump P-101 is running", BOXED);

    expect(lines).toEqual(["Pump", "P-101", "is", "running"]);
    expect(lineWidth(ARIAL_BOX_METRICS, "running", BOXED)).toBe(53.375);
  });

  it("puts on one line every token that fits", () => {
    // "Hello World Hello" in a box 90 wide: "Hello World" 82.6875 fits, "Hello World Hello" not.
    expect(
      wrappedParagraph(ARIAL_BOX_METRICS, "Hello World Hello", { ...BOXED, boxWidth: 90 }),
    ).toEqual(["Hello World", "Hello"]);
  });

  it("keeps the spaces that open the paragraph and those that end it", () => {
    // "  Hello" 2 × 569 + 4667 = 5805 units, 45.35156 ≤ 50; the last line keeps its spaces.
    expect(wrappedParagraph(ARIAL_BOX_METRICS, "  Hello World  ", BOXED)).toEqual([
      "  Hello",
      "World  ",
    ]);
  });

  it("gives one empty line for an empty paragraph, and keeps a line of spaces", () => {
    expect(wrappedParagraph(ARIAL_BOX_METRICS, "", BOXED)).toEqual([""]);

    expect(wrappedParagraph(ARIAL_BOX_METRICS, " ".repeat(4), { ...BOXED, boxWidth: 0 })).toEqual([
      " ".repeat(4),
    ]);
  });

  test.prop({ boxWidth: fc.integer({ max: 150, min: 0 }), paragraph: PARAGRAPH })(
    "[F14.AC1] makes no line wider than the box but a long word's",
    ({ boxWidth, paragraph }) => {
      const text = { ...BOXED, boxWidth };

      for (const line of wrappedParagraph(ARIAL_BOX_METRICS, paragraph, text)) {
        expect(
          isLineFitting(ARIAL_BOX_METRICS, line, text) || paragraphTokens(line).length === 1,
        ).toBe(true);
      }
    },
  );
});
