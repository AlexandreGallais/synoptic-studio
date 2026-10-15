import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { characterUnits } from "../line-characters";
import { lineWidth } from "../text-block";
import { wrappedLines, wrappedParagraph } from "../text-wrap";

import { shownLines } from "./shown-lines";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** Arial 16 without kerning in a box 60 wide and 1000 tall, its ellipsis on. */
const BOXED: Text = { ...HELLO_TEXT, boxHeight: 1000, boxWidth: 60, isInBox: true };

/** Four lines of one word each in a box 60 wide. */
const TWICE = "Hello World Hello World";

/** A word too long for a box 50 wide, "running" being 53.375 wide. */
const PUMP = "Pump P-101 is running";

/** The lines a text shows, and their widths. */
type ShownText = {
  /** Its lines shown. */
  readonly lines: readonly string[];
  /** Their widths in user units. */
  readonly widths: readonly number[];
};

/**
 * The lines a text shows, and their widths.
 *
 * @param text - a text in Arial 16 without kerning
 * @returns its lines shown and their widths
 */
function shown(text: Text): ShownText {
  const lines = shownLines(ARIAL_BOX_METRICS, text);

  return { lines, widths: lines.map((line) => lineWidth(ARIAL_BOX_METRICS, line, text)) };
}

describe("shownLines (DERIV-text-box steps 5 and 6)", () => {
  it("[F14.AC2] ends a text cut by its maximum number of lines with an ellipsis", () => {
    // "Hello…" (4667 + 2048) / 128 = 52.46094 ≤ 60.
    expect(shown({ ...BOXED, maximumLines: 1 })).toEqual({
      lines: ["Hello…"],
      widths: [52.4609375],
    });

    // "World…" (5348 + 2048) / 128 = 57.78125.
    expect(shown({ ...BOXED, content: TWICE, maximumLines: 2 })).toEqual({
      lines: ["Hello", "World…"],
      widths: [36.4609375, 57.78125],
    });
  });

  it("[F14.AC2] removes the last character when the line and its ellipsis do not fit", () => {
    // "Hello…" 52.46094 > 50; "Hell…" (3528 + 2048) / 128 = 43.5625.
    expect(shown({ ...BOXED, boxWidth: 50, maximumLines: 1 })).toEqual({
      lines: ["Hell…"],
      widths: [43.5625],
    });
  });

  it("[F14.AC2] shows the lines that fit the box's height", () => {
    // h(2) = 36.79688 ≤ 40 < h(3) = 55.19531: 2 of the 4 lines.
    expect(shownLines(ARIAL_BOX_METRICS, { ...BOXED, boxHeight: 40, content: TWICE })).toEqual([
      "Hello",
      "World…",
    ]);

    // Shorter than a line: one line still shows (provisional, VAL-015).
    expect(shownLines(ARIAL_BOX_METRICS, { ...BOXED, boxHeight: 10, content: TWICE })).toEqual([
      "Hello…",
    ]);
  });

  it("[F14.AC2] counts with a chosen line height", () => {
    // A line height of 24 in a box 50 tall: h(2) = 48 ≤ 50 < h(3) = 72.
    const text = { ...BOXED, boxHeight: 50, content: TWICE, lineHeight: 24 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello", "World…"]);
  });

  it("[F14.AC5] counts a trimmed first line at the cap height", () => {
    // Trimmed, h(2) = 11.46094 + 18.39844 = 29.85938 ≤ 30 < h(3) = 48.25781: 2 lines;
    // untrimmed, h(2) = 36.79688 > 30: 1 line.
    const text = { ...BOXED, boxHeight: 30, content: TWICE };

    expect(shownLines(ARIAL_BOX_METRICS, { ...text, isTrimmed: true })).toEqual([
      "Hello",
      "World…",
    ]);

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello…"]);
  });

  it("[F14.AC2] shows no more lines than the text wraps into, and no ellipsis when all fit", () => {
    // h(3) = 55.19531 ≤ 60, but "Hello World" wraps into 2 lines in a box 50 wide.
    const text = { ...BOXED, boxHeight: 60, boxWidth: 50 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello", "World"]);
    expect(shownLines(ARIAL_BOX_METRICS, { ...text, content: "" })).toEqual([]);
  });

  it("[F17.AC1] cuts a word too long, kept whole, with an ellipsis", () => {
    // "running" 6832 / 128 = 53.375 > 50 stays whole; "runn…" (6832 − 1139 − 1139 − 455 + 2048)
    // / 128 = 6147 / 128 = 48.02344 ≤ 50; "runni…" 51.57813 > 50.
    expect(shown({ ...BOXED, boxWidth: 50, content: "running" })).toEqual({
      lines: ["runn…"],
      widths: [48.0234375],
    });

    const text: Text = { ...BOXED, boxWidth: 50, content: PUMP };

    expect(shown(text)).toEqual({
      lines: ["Pump", "P-101", "is", "runn…"],
      widths: [41.796875, 42.6953125, 11.5546875, 48.0234375],
    });

    // At most 2 lines: "P-101…" 58.69531 > 50, "P-10…" 6374 / 128 = 49.79688.
    expect(shown({ ...text, maximumLines: 2 })).toEqual({
      lines: ["Pump", "P-10…"],
      widths: [41.796875, 49.796875],
    });
  });

  it("[F14.AC2] keeps the first character and its ellipsis in a box too narrow", () => {
    // (1479 + 2048) / 128 = 27.55469 > 10: both drawn, past the box's edge (provisional).
    expect(shown({ ...BOXED, boxWidth: 10, maximumLines: 1 })).toEqual({
      lines: ["H…"],
      widths: [27.5546875],
    });
  });

  it("[F14.AC2] never counts the spaces opening a line as the character kept", () => {
    const text: Text = {
      ...BOXED,
      boxWidth: 10,
      content: "  Hello World",
      maximumLines: 1,
    };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["  H…"]);
  });

  it("[F14.AC2] stops a text at its last line shown when the ellipsis is off", () => {
    const off: Text = { ...BOXED, hasEllipsis: false };

    expect(shown({ ...off, maximumLines: 1 })).toEqual({ lines: ["Hello"], widths: [36.4609375] });

    expect(shownLines(ARIAL_BOX_METRICS, { ...off, content: TWICE, maximumLines: 2 })).toEqual([
      "Hello",
      "World",
    ]);
  });

  it("[F17.AC1] draws a word too long whole, past the box, when the ellipsis is off", () => {
    // "running" 6832 / 128 = 53.375, 3.375 past the box 50 wide.
    const off: Text = { ...BOXED, boxWidth: 50, hasEllipsis: false };

    expect(shown({ ...off, content: "running" })).toEqual({ lines: ["running"], widths: [53.375] });

    expect(shown({ ...off, content: PUMP })).toEqual({
      lines: ["Pump", "P-101", "is", "running"],
      widths: [41.796875, 42.6953125, 11.5546875, 53.375],
    });

    // "Hello World" 82.6875 > 50 wraps as before: "Hello" 36.46094 over "World" 41.78125.
    expect(shown({ ...off, content: "Hello World" })).toEqual({
      lines: ["Hello", "World"],
      widths: [36.4609375, 41.78125],
    });
  });

  it("[F14.AC2] leaves a line of one character wider than the box without an ellipsis", () => {
    // In a box 5: "W" 15.10 and "a" (glyph 0) 12 are each wider than 5; nothing to remove, the
    // first character is clipped rather than ellipsed (provisional, VAL-015).
    const text: Text = { ...BOXED, boxWidth: 5, content: "W a" };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["W", "a"]);
  });

  it("[F14.AC2] adds no ellipsis when the lines left out are empty", () => {
    // "Hello" then an empty line, at most 1 line: nothing hidden but the empty line (provisional).
    const text: Text = { ...BOXED, content: "Hello\n", maximumLines: 1 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello"]);

    expect(shownLines(ARIAL_BOX_METRICS, { ...text, content: "Hello\n\nWorld" })).toEqual([
      "Hello…",
    ]);
  });

  it("[F14.AC3] shows the balanced lines of a text that is not cut", () => {
    // "Pump P-101" 88.9375 / "is running" 69.375 rather than the greedy "Pump P-101 is" / "running".
    expect(shown({ ...BOXED, boxWidth: 110, content: PUMP, wrapStyle: "balance" })).toEqual({
      lines: ["Pump P-101", "is running"],
      widths: [88.9375, 69.375],
    });
  });

  it.each([{ hasEllipsis: true }, { hasEllipsis: false }])(
    "[F14.AC3] keeps the greedy lines of a text cut, the ellipsis on: $hasEllipsis",
    ({ hasEllipsis }) => {
      // Every character 1 wide, a box 6, lines 1 apart, at most 2 lines: greedy "AAA BB" / "CC" / "DDDDD" is cut
      // after "CC"; balanced "AAA" / "BB CC" would move what the cut hides. With the ellipsis on,
      // "CC..." is 5 ≤ 6, the face having no "…".
      const metrics: FontMetrics = {
        ...ARIAL_BOX_METRICS,
        advances: { 32: 1, 46: 1, 65: 1, 66: 1, 67: 1, 68: 1 },
        unitsPerEm: 16,
      };
      const text: Text = {
        ...BOXED,
        boxWidth: 6,
        content: "AAA BB CC DDDDD",
        hasEllipsis,
        lineHeight: 1,
        maximumLines: 2,
        wrapStyle: "balance",
      };

      expect(shownLines(metrics, text)).toEqual(["AAA BB", hasEllipsis ? "CC..." : "CC"]);
      expect(shownLines(metrics, { ...text, maximumLines: 0 })).toEqual(["AAA", "BB CC", "DDDDD"]);
    },
  );

  it("[F14.AC6] keeps every line of a text that fits its content", () => {
    // Not in a box: no maximum, no ellipsis, the lines of F05.
    const text = { ...BOXED, content: "Hello World\nHello", isInBox: false, maximumLines: 1 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello World", "Hello"]);
  });

  test.prop({
    boxHeight: fc.integer({ max: 120, min: 0 }),
    boxWidth: fc.integer({ max: 80, min: 0 }),
    content: fc.stringMatching(/^[ \nHWdelor]{0,40}$/u),
    hasEllipsis: fc.boolean(),
    maximumLines: fc.integer({ max: 5, min: 0 }),
  })(
    "[F14.AC2] shows at most the maximum, each line fitting with the ellipsis on unless one character is left",
    (values) => {
      const text = { ...BOXED, ...values };
      const lines = shownLines(ARIAL_BOX_METRICS, text);
      const wrapped = wrappedLines(ARIAL_BOX_METRICS, text, wrappedParagraph);

      const maximum = values.maximumLines === 0 ? wrapped.length : values.maximumLines;

      expect(lines.length).toBeLessThanOrEqual(maximum);
      expect(lines.length === 0).toBe(values.content === "");

      expect(
        lines.every(
          (line) =>
            !values.hasEllipsis ||
            characterUnits(line.trim()).length <= 2 ||
            lineWidth(ARIAL_BOX_METRICS, line.trimEnd(), text) <= values.boxWidth,
        ),
      ).toBe(true);
    },
  );
});
