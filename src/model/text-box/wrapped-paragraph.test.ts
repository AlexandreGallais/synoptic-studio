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

  it("[F14.AC1] breaks a word too long at a character when the text says so", () => {
    // As moved whole, but "running" 53.375 > 50 starts its own line, then breaks after "runnin",
    // 5693 / 128 = 44.47656 ≤ 50, without a hyphen; "g" goes on the next line.
    const text = { ...BOXED, longWord: "break" } as const;
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, "Pump P-101 is running", text);

    expect(lines).toEqual(["Pump", "P-101", "is", "runnin", "g"]);
    expect(lineWidth(ARIAL_BOX_METRICS, "runnin", text)).toBe(44.4765625);
  });

  it("lets the next words join the last part of a word broken", () => {
    // A box 30 wide: "Hello" 36.46 > 30 breaks after "Hell" (3528 / 128 = 27.5625); "o is",
    // (1139 + 569 + 1479) / 128 = 24.89844 ≤ 30, takes "is"; "World" 41.78 breaks after "Wor"
    // (3754 / 128 = 29.32813), "ld" 12.45 left.
    const text = { ...BOXED, boxWidth: 30, longWord: "break" } as const;

    expect(wrappedParagraph(ARIAL_BOX_METRICS, "Hello is World", text)).toEqual([
      "Hell",
      "o is",
      "Wor",
      "ld",
    ]);
  });

  it("breaks a first word too long, after the spaces that open the paragraph", () => {
    // A box 20 wide: "  Hel" 2 × 569 + 3073 = 4211 units, 32.9 > 20; "  He" 3756 units, 29.34 > 20;
    // "  H" 2617 units, 20.45 > 20, kept as the one character a line takes.
    const text = { ...BOXED, boxWidth: 20, longWord: "break" } as const;

    expect(wrappedParagraph(ARIAL_BOX_METRICS, "  Hello", text)).toEqual(["  H", "ell", "o"]);
  });

  test.prop({
    boxWidth: fc.integer({ max: 150, min: 0 }),
    longWord: fc.constantFrom("break", "move"),
    paragraph: PARAGRAPH,
  })(
    "[F14.AC1] gives back the paragraph joined, but for the spaces dropped at each wrap",
    ({ boxWidth, longWord, paragraph }) => {
      const lines = wrappedParagraph(ARIAL_BOX_METRICS, paragraph, {
        ...BOXED,
        boxWidth,
        longWord,
      });
      // Characters of the alphabet need no escape: the lines joined by any run of spaces.
      const joined = new RegExp(`^${lines.join(" *")}$`, "u");

      expect(joined.test(paragraph)).toBe(true);
      expect(lines.slice(0, -1).every((line) => !line.endsWith(" "))).toBe(true);
    },
  );

  test.prop({ boxWidth: fc.integer({ max: 150, min: 0 }), paragraph: PARAGRAPH })(
    "[F14.AC1] breaking at a character, makes no line wider than the box but one of one character",
    ({ boxWidth, paragraph }) => {
      const text = { ...BOXED, boxWidth, longWord: "break" } as const;

      for (const line of wrappedParagraph(ARIAL_BOX_METRICS, paragraph, text)) {
        expect(isLineFitting(ARIAL_BOX_METRICS, line, text) || line.trim().length === 1).toBe(true);
      }
    },
  );

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
