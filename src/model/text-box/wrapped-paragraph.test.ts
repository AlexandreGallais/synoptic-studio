import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { characterUnits } from "../line-characters";
import { lineWidth } from "../text-block";

import { isLineFitting } from "./is-line-fitting";
import { paragraphTokens } from "./paragraph-tokens";
import { wrappedParagraph } from "./wrapped-paragraph";

import type { Text, TextLongWord } from "../text";

/** Arial 16 without kerning in a box 50 wide, a word too long moved whole. */
const BOXED: Text = {
  ...HELLO_TEXT,
  boxHeight: 1000,
  boxWidth: 50,
  isInBox: true,
  longWord: "move",
};

/**
 * A paragraph of the fixture's Arial characters, spaces, no-break spaces and combining acutes, in
 * a box 0 to 60 wide, either long-word option, letter and word spacing from −15 to 5.
 */
const PARAGRAPH_IN_BOX = {
  boxWidth: fc.integer({ max: 60, min: 0 }),
  letterSpacing: fc.integer({ max: 5, min: -15 }),
  longWord: fc.constantFrom<TextLongWord>("break", "move"),
  paragraph: fc.stringMatching(/^(?:[ HWdelor\u{A0}]|\u{301}){0,40}$/u),
  wordSpacing: fc.integer({ max: 5, min: -15 }),
};

/**
 * The text a drawn paragraph sits in.
 *
 * @param values - its box width, spacing and long-word option
 * @returns the text in a box 50 wide with them
 */
function boxedText(
  values: Pick<Text, "boxWidth" | "letterSpacing" | "longWord" | "wordSpacing">,
): Text {
  const { boxWidth, letterSpacing, longWord, wordSpacing } = values;

  return { ...BOXED, boxWidth, letterSpacing, longWord, wordSpacing };
}

describe("wrappedParagraph (DERIV-text-box steps 1 to 4)", () => {
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

  it("breaks words separated by a space into a line each in a box 0 wide", () => {
    // "H " cannot share a line with "l": "H l" 2503 / 128 = 19.55 > 0; each alone overflows.
    const text = { ...BOXED, boxWidth: 0, longWord: "break" } as const;

    expect(wrappedParagraph(ARIAL_BOX_METRICS, "H l", text)).toEqual(["H", "l"]);
  });

  it("places whole a word that a negative letter spacing makes fit", () => {
    // "Hel" (1479 + 1139 + 455) / 128 = 24.00781, two letter spacings of −10: 4.00781 ≤ 5, so it
    // is no word too long, though "He" is wider: 2618 / 128 − 10 = 10.45313 > 5.
    const text = { ...BOXED, boxWidth: 5, letterSpacing: -10, longWord: "break" } as const;

    expect(wrappedParagraph(ARIAL_BOX_METRICS, "Hel", text)).toEqual(["Hel"]);
  });

  it("[F14.AC1] never cuts a word after a no-break space, breaking at a character", () => {
    // In a box 45: "Hello" + no-break space (569) = 40.91 would fit, but with "W" 7169 / 128 =
    // 56.01 > 45: the cut comes before the no-break space, which stays with "W"; then
    // "\u{A0}Worl" 4778 / 128 = 37.33 ≤ 45, "\u{A0}World" 5917 / 128 = 46.23 > 45. No line ends
    // with the no-break space.
    const text = { ...BOXED, boxWidth: 45, longWord: "break" } as const;
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, "Hello\u{A0}World", text);

    expect(lines).toEqual(["Hello", "\u{A0}Worl", "d"]);
    expect(lines.some((line) => line.endsWith("\u{A0}"))).toBe(false);

    expect(wrappedParagraph(ARIAL_BOX_METRICS, "\u{A0}r", { ...text, boxWidth: 0 })).toEqual([
      "\u{A0}r",
    ]);
  });

  it("wraps 16 000 words in well under a second, each measured once", { timeout: 1000 }, () => {
    // 8000 times "Hello World ": in a box 100 wide, 8000 lines "Hello World". In a box 100 000
    // wide, 12 800 000 units: "Hello World " is 11 153 units, so about 2295 words a line, 7 lines
    // for 16 000; measuring each line again at each word would read some 4 × 10⁷ characters.
    const paragraph = "Hello World ".repeat(8000);
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, paragraph, { ...BOXED, boxWidth: 100 });
    const wide = wrappedParagraph(ARIAL_BOX_METRICS, paragraph, { ...BOXED, boxWidth: 100_000 });

    expect(lines).toHaveLength(8000);
    expect(wide).toHaveLength(7);
  });

  test.prop(PARAGRAPH_IN_BOX)(
    "[F14.AC1] gives back the paragraph joined, but for the spaces dropped at each wrap",
    (values) => {
      const lines = wrappedParagraph(ARIAL_BOX_METRICS, values.paragraph, boxedText(values));
      // Characters of the alphabet need no escape: the lines joined by any run of spaces.
      const joined = new RegExp(`^${lines.join(" *")}$`, "u");

      expect(joined.test(values.paragraph)).toBe(true);
      expect(lines.slice(0, -1).every((line) => !line.endsWith(" "))).toBe(true);
      expect(lines.slice(0, -1).every((line) => line !== "")).toBe(true);
    },
  );

  test.prop(PARAGRAPH_IN_BOX)(
    "[F14.AC1] makes no line wider than the box but one of a long word, or of one character",
    (values) => {
      const text = boxedText(values);

      for (const line of wrappedParagraph(ARIAL_BOX_METRICS, values.paragraph, text)) {
        const isKeptWhole =
          text.longWord === "break"
            ? characterUnits(line.replaceAll(/[ \u{A0}]/gu, "")).length <= 1
            : paragraphTokens(line).length === 1;

        expect(isLineFitting(ARIAL_BOX_METRICS, line, text) || isKeptWhole).toBe(true);
      }
    },
  );
});
