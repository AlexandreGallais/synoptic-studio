import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { lineWidth } from "../text-block";
import { withoutTrailingSpaces, wrappedParagraph } from "../text-wrap";

import { balancedParagraph } from "./balanced-paragraph";

import type { FontMetrics } from "../font";
import type { Text, TextLongWord } from "../text";

/** Arial 16 without kerning in a box 110 wide, balanced, a word too long moved whole. */
const BOXED: Text = {
  ...HELLO_TEXT,
  boxHeight: 1000,
  boxWidth: 110,
  isInBox: true,
  longWord: "move",
  wrapStyle: "balance",
};

/** A face whose space and `A` to `D` are 1 unit of 16 per em: 1 wide at size 16. */
const UNIT_METRICS: FontMetrics = {
  ...ARIAL_BOX_METRICS,
  advances: { 32: 1, 65: 1, 66: 1, 67: 1, 68: 1 },
  unitsPerEm: 16,
};

/** A word too long for the box: broken at a character or moved whole. */
type LongWordCase = {
  /** The option chosen. */
  readonly longWord: TextLongWord;
};

/**
 * The sum of the squares of the room each line leaves, its trailing spaces left out.
 *
 * @param metrics - the face's metrics
 * @param lines - the lines of a paragraph
 * @param text - the text they sit in
 * @returns the sum of the squares of the rooms
 */
function raggedness(metrics: FontMetrics, lines: readonly string[], text: Text): number {
  return lines.reduce(
    (sum, line) =>
      sum + (text.boxWidth - lineWidth(metrics, withoutTrailingSpaces(line), text)) ** 2,
    0,
  );
}

describe("balancedParagraph (DERIV-text-box step 9)", () => {
  it("[F14.AC3] balances 'Pump P-101 is running' in a box 110 wide", () => {
    // Greedy "Pump P-101 is" 104.9375 / "running" 53.375: 5.0625² + 56.625² = 3232.01953;
    // balanced "Pump P-101" 88.9375 / "is running" 69.375: 21.0625² + 40.625² = 2094.01953.
    const lines = balancedParagraph(ARIAL_BOX_METRICS, "Pump P-101 is running", BOXED);

    expect(lines).toEqual(["Pump P-101", "is running"]);

    expect(lines.map((line) => lineWidth(ARIAL_BOX_METRICS, line, BOXED))).toEqual([
      88.9375, 69.375,
    ]);

    expect(raggedness(ARIAL_BOX_METRICS, lines, BOXED)).toBeCloseTo(2094.01953, 5);

    expect(
      raggedness(
        ARIAL_BOX_METRICS,
        wrappedParagraph(ARIAL_BOX_METRICS, "Pump P-101 is running", BOXED),
        BOXED,
      ),
    ).toBeCloseTo(3232.01953, 5);
  });

  it("[F14.AC3] cuts 'AAA BB CC DDDDD' in a box 6 as the source's optimum", () => {
    // Every character 1 wide: greedy "AAA BB" / "CC" / "DDDDD" costs 0² + 4² + 1² = 17; balanced
    // "AAA" / "BB CC" / "DDDDD" costs 3² + 1² + 1² = 11 (REF-WIKI-KNUTH-PLASS).
    const text = { ...BOXED, boxWidth: 6 };
    const lines = balancedParagraph(UNIT_METRICS, "AAA BB CC DDDDD", text);

    expect(lines).toEqual(["AAA", "BB CC", "DDDDD"]);
    expect(raggedness(UNIT_METRICS, lines, text)).toBe(11);

    expect(
      raggedness(UNIT_METRICS, wrappedParagraph(UNIT_METRICS, "AAA BB CC DDDDD", text), text),
    ).toBe(17);
  });

  it("[F14.AC3] keeps the later break of two cuts of equal cost", () => {
    // In a box 90: "Hello World" / "Hello" and "Hello" / "World Hello" both cost
    // 7.3125² + 53.53906², "World Hello" being 82.6875 wide as "Hello World".
    expect(
      balancedParagraph(ARIAL_BOX_METRICS, "Hello World Hello", { ...BOXED, boxWidth: 90 }),
    ).toEqual(["Hello World", "Hello"]);
  });

  it("drops the spaces at each break and keeps those typed at the end", () => {
    // "Pump  P-101 is running  ": the two spaces after "P-101" dropped, the last two kept.
    expect(balancedParagraph(ARIAL_BOX_METRICS, "Pump P-101  is running  ", BOXED)).toEqual([
      "Pump P-101",
      "is running  ",
    ]);
  });

  it.each<LongWordCase>([{ longWord: "move" }, { longWord: "break" }])(
    "[F14.AC3] keeps the greedy lines of a paragraph holding a word too long ($longWord)",
    ({ longWord }) => {
      // Every character 1 wide, a box 6: "DDDDDDD" 7 > 6 is a word too long; greedy "AAA BB" /
      // "CC" / "DDDDDDD" (or "DDDDDD" / "D" broken) stays, though "AAA" / "BB CC" is less ragged.
      const text = { ...BOXED, boxWidth: 6, longWord };
      const lines = balancedParagraph(UNIT_METRICS, "AAA BB CC DDDDDDD", text);

      expect(lines.slice(0, 2)).toEqual(["AAA BB", "CC"]);
      expect(lines).toEqual(wrappedParagraph(UNIT_METRICS, "AAA BB CC DDDDDDD", text));
    },
  );

  it("keeps one line, and gives one empty line for an empty paragraph", () => {
    expect(balancedParagraph(ARIAL_BOX_METRICS, "Pump", BOXED)).toEqual(["Pump"]);
    expect(balancedParagraph(ARIAL_BOX_METRICS, "", BOXED)).toEqual([""]);
  });

  it("balances 2000 words within a second", { timeout: 1000 }, () => {
    // 1000 times "Hello World ": in a box 100 wide, "Hello World" 82.6875 a line, 1000 lines; in a
    // box 1000 wide, "Hello World " is 87.13281, so about 22 words a line, 91 lines as the greedy
    // wrap gives. The work grows with the number of words times the most a line holds.
    const paragraph = "Hello World ".repeat(1000);

    expect(
      balancedParagraph(ARIAL_BOX_METRICS, paragraph, { ...BOXED, boxWidth: 100 }),
    ).toHaveLength(1000);

    expect(
      balancedParagraph(ARIAL_BOX_METRICS, paragraph, { ...BOXED, boxWidth: 1000 }),
    ).toHaveLength(91);
  });

  test.prop({
    boxWidth: fc.integer({ max: 120, min: 0 }),
    letterSpacing: fc.integer({ max: 5, min: -15 }),
    longWord: fc.constantFrom<TextLongWord>("break", "move"),
    paragraph: fc.stringMatching(/^(?:[ HWdelor\u{A0}]|\u{301}){0,40}$/u),
    wordSpacing: fc.integer({ max: 5, min: -15 }),
  })(
    "[F14.AC3] keeps the greedy number of lines and is never more ragged than it",
    ({ paragraph, ...values }) => {
      const text = { ...BOXED, ...values };
      const greedy = wrappedParagraph(ARIAL_BOX_METRICS, paragraph, text);
      const balanced = balancedParagraph(ARIAL_BOX_METRICS, paragraph, text);
      const joined = new RegExp(`^${balanced.join(" *")}$`, "u");

      expect(balanced).toHaveLength(greedy.length);
      expect(joined.test(paragraph)).toBe(true);

      expect(raggedness(ARIAL_BOX_METRICS, balanced, text)).toBeLessThanOrEqual(
        raggedness(ARIAL_BOX_METRICS, greedy, text) + 1e-6,
      );
    },
  );
});
