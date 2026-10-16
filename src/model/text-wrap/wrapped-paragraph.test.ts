import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { lineWidth } from "../text-block";

import { isLineFitting } from "./is-line-fitting";
import { paragraphTokens } from "./paragraph-tokens";
import { wrappedParagraph } from "./wrapped-paragraph";

import type { Text } from "../text";

/** Arial 16 without kerning in a box 50 wide. */
const BOXED: Text = {
  ...HELLO_TEXT,
  boxHeight: 1000,
  boxWidth: 50,
  isInBox: true,
};

/**
 * A paragraph of the fixture's Arial characters, spaces, no-break spaces and combining acutes, in
 * a box 0 to 60 wide, letter and word spacing from −15 to 5.
 */
const PARAGRAPH_IN_BOX = {
  boxWidth: fc.integer({ max: 60, min: 0 }),
  letterSpacing: fc.integer({ max: 5, min: -15 }),
  paragraph: fc.stringMatching(/^(?:[ HWdelor\u{A0}]|\u{301}){0,40}$/u),
  wordSpacing: fc.integer({ max: 5, min: -15 }),
};

/**
 * The text a drawn paragraph sits in.
 *
 * @param values - its box width and spacing
 * @returns the text in a box 50 wide with them
 */
function boxedText(values: Pick<Text, "boxWidth" | "letterSpacing" | "wordSpacing">): Text {
  const { boxWidth, letterSpacing, wordSpacing } = values;

  return { ...BOXED, boxWidth, letterSpacing, wordSpacing };
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

  it("[F17.AC1] moves a word too long whole to its own line, wider than the box", () => {
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

  it("[F17.AC6] fits a line by its letters, its trailing space left out", () => {
    // Box 85: "Hello World" 82.6875 fits; with its trailing space, 82.6875 + 569 / 128 = 87.13281
    // would not, and "World Hello" (82.6875) would take the second line instead.
    expect(
      wrappedParagraph(ARIAL_BOX_METRICS, "Hello World Hello", { ...BOXED, boxWidth: 85 }),
    ).toEqual(["Hello World", "Hello"]);
  });

  it("[F17.AC6] keeps the spaces that open the paragraph, and drops those that end it", () => {
    // "  Hello" 2 × 569 + 4667 = 5805 units, 45.35156 ≤ 50; the last line loses its trailing
    // spaces as every other line does (US-049).
    expect(wrappedParagraph(ARIAL_BOX_METRICS, "  Hello World  ", BOXED)).toEqual([
      "  Hello",
      "World",
    ]);
  });

  it("[F17.AC6] drops the space typed at the end of 'Hello World '", () => {
    expect(wrappedParagraph(ARIAL_BOX_METRICS, "Hello World ", BOXED)).toEqual(["Hello", "World"]);
  });

  it("gives one empty line for an empty paragraph, and for a paragraph of spaces", () => {
    expect(wrappedParagraph(ARIAL_BOX_METRICS, "", BOXED)).toEqual([""]);

    expect(wrappedParagraph(ARIAL_BOX_METRICS, " ".repeat(4), { ...BOXED, boxWidth: 0 })).toEqual([
      "",
    ]);
  });

  it("[F17.AC1] keeps a word too long whole, never broken at a character", () => {
    // "running" (682 + 1139 + 1139 + 1139 + 455 + 1139 + 1139 = 6832) / 128 = 53.375 > 50:
    // one line, overflowing the box by 3.375, not "runnin" 44.47656 over "g".
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, "running", BOXED);

    expect(lines).toEqual(["running"]);
    expect(lineWidth(ARIAL_BOX_METRICS, "running", BOXED) - BOXED.boxWidth).toBe(3.375);
  });

  it("keeps each word whole in a box 0 wide, and a first word after its opening spaces", () => {
    // "H l" 2503 / 128 = 19.55 > 0: "H" and "l" on a line each, overflowing; "  Hello" 45.35 > 20
    // stays one line.
    expect(wrappedParagraph(ARIAL_BOX_METRICS, "H l", { ...BOXED, boxWidth: 0 })).toEqual([
      "H",
      "l",
    ]);

    expect(wrappedParagraph(ARIAL_BOX_METRICS, "  Hello", { ...BOXED, boxWidth: 20 })).toEqual([
      "  Hello",
    ]);
  });

  it("places whole a word that a negative letter spacing makes fit", () => {
    // "Hel" (1479 + 1139 + 455) / 128 = 24.00781, two letter spacings of −10: 4.00781 ≤ 5, so it
    // is no word too long, though "He" is wider: 2618 / 128 − 10 = 10.45313 > 5.
    const text = { ...BOXED, boxWidth: 5, letterSpacing: -10 };

    expect(wrappedParagraph(ARIAL_BOX_METRICS, "Hel", text)).toEqual(["Hel"]);
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
    "[F14.AC1] gives back the paragraph joined, but for the spaces dropped at each wrap and at its end",
    (values) => {
      const lines = wrappedParagraph(ARIAL_BOX_METRICS, values.paragraph, boxedText(values));
      // Characters of the alphabet need no escape: the lines joined by any run of spaces.
      const joined = new RegExp(`^${lines.join(" *")} *$`, "u");

      expect(joined.test(values.paragraph)).toBe(true);
      expect(lines.every((line) => !line.endsWith(" "))).toBe(true);
      expect(lines.slice(0, -1).every((line) => line !== "")).toBe(true);
    },
  );

  test.prop(PARAGRAPH_IN_BOX)(
    "[F17.AC1] makes no line wider than the box but one of a long word, kept whole",
    (values) => {
      const text = boxedText(values);

      for (const line of wrappedParagraph(ARIAL_BOX_METRICS, values.paragraph, text)) {
        const isOneWord = paragraphTokens(line).length === 1;

        expect(isLineFitting(ARIAL_BOX_METRICS, line, text) || isOneWord).toBe(true);
      }
    },
  );

  test.prop({
    boxWidth: fc.integer({ max: 60, min: 0 }),
    words: fc.array(fc.stringMatching(/^[HWdelor]{1,6}$/u), { maxLength: 12, minLength: 1 }),
  })("[F14.AC1] ends a line only where its next word would not fit after it, greedy", (values) => {
    const text = boxedText({ boxWidth: values.boxWidth, letterSpacing: 0, wordSpacing: 0 });
    const lines = wrappedParagraph(ARIAL_BOX_METRICS, values.words.join(" "), text);

    // Words one space apart: the next line's first word, after the line and its space, overflows.
    const ended = lines.slice(0, -1);

    for (const [index, line] of ended.entries()) {
      const [next = ""] = paragraphTokens(lines[index + 1] ?? "");

      expect(isLineFitting(ARIAL_BOX_METRICS, `${line} ${next}`, text)).toBe(false);
    }
  });
});
