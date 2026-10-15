import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { characterUnits } from "../line-characters";
import { lineWidth } from "../text-block";

import { shownLines } from "./shown-lines";
import { wrappedLines } from "./wrapped-lines";

import type { Text, TextLongWord } from "../text";

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

/** A word too long for the box: broken at a character or moved whole. */
type LongWordCase = {
  /** The option chosen. */
  readonly longWord: TextLongWord;
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

  it("[F14.AC2] shows no more lines than the text wraps into, and no ellipsis when all fit", () => {
    // h(3) = 55.19531 ≤ 60, but "Hello World" wraps into 2 lines in a box 50 wide.
    const text = { ...BOXED, boxHeight: 60, boxWidth: 50 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello", "World"]);
    expect(shownLines(ARIAL_BOX_METRICS, { ...text, content: "" })).toEqual([]);
  });

  it("[F14.AC2] cuts a word moved whole that overflows with an ellipsis", () => {
    // "runn…" 6147 / 128 = 48.02344 ≤ 50; "runni…" 51.57813 > 50.
    const text: Text = { ...BOXED, boxWidth: 50, content: PUMP, longWord: "move" };

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

  it.each<LongWordCase>([{ longWord: "break" }, { longWord: "move" }])(
    "[F14.AC2] keeps the first character and its ellipsis in a box too narrow, words $longWord",
    ({ longWord }) => {
      // (1479 + 2048) / 128 = 27.55469 > 10: both drawn, past the box's edge (provisional).
      expect(shown({ ...BOXED, boxWidth: 10, longWord, maximumLines: 1 })).toEqual({
        lines: ["H…"],
        widths: [27.5546875],
      });
    },
  );

  it("[F14.AC2] never counts the spaces opening a line as the character kept", () => {
    const text: Text = {
      ...BOXED,
      boxWidth: 10,
      content: "  Hello World",
      longWord: "move",
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

    // A word moved whole is drawn whole, 6832 / 128 = 53.375 wide, past the box (provisional).
    expect(shown({ ...off, boxWidth: 50, content: PUMP, longWord: "move" })).toEqual({
      lines: ["Pump", "P-101", "is", "running"],
      widths: [41.796875, 42.6953125, 11.5546875, 53.375],
    });
  });

  it("[F14.AC2] leaves a line of one character wider than the box without an ellipsis", () => {
    // Broken at a character in a box 5: every letter alone, each wider than 5; nothing to remove,
    // the first character is clipped rather than ellipsed (provisional, VAL-015).
    const text: Text = { ...BOXED, boxWidth: 5, content: "Hello" };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["H", "e", "l", "l", "o"]);

    // Moved whole: "W" 15.10 and "a" (glyph 0) 12 are each wider than 5.
    expect(shownLines(ARIAL_BOX_METRICS, { ...text, content: "W a", longWord: "move" })).toEqual([
      "W",
      "a",
    ]);
  });

  it("[F14.AC2] adds no ellipsis when the lines left out are empty", () => {
    // "Hello" then an empty line, at most 1 line: nothing hidden but the empty line (provisional).
    const text: Text = { ...BOXED, content: "Hello\n", maximumLines: 1 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello"]);

    expect(shownLines(ARIAL_BOX_METRICS, { ...text, content: "Hello\n\nWorld" })).toEqual([
      "Hello…",
    ]);
  });

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
    longWord: fc.constantFrom<TextLongWord>("break", "move"),
    maximumLines: fc.integer({ max: 5, min: 0 }),
  })(
    "[F14.AC2] shows at most the maximum, each line fitting with the ellipsis on unless one character is left",
    (values) => {
      const text = { ...BOXED, ...values };
      const lines = shownLines(ARIAL_BOX_METRICS, text);
      const wrapped = wrappedLines(ARIAL_BOX_METRICS, text);

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
