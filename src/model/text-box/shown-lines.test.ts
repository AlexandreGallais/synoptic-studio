import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, HELLO_TEXT } from "../../../tests";
import { characterUnits } from "../line-characters";
import { lineWidth } from "../text-block";
import { wrappedLines } from "../text-wrap";

import { shownLines } from "./shown-lines";
import { textEllipsis } from "./text-ellipsis";

import type { Text } from "../text";

/** Arial 16 without kerning in a box 60 wide and 1000 tall, its ellipsis on. */
const BOXED: Text = { ...HELLO_TEXT, boxHeight: 1000, boxWidth: 60, isInBox: true };

/** Four lines of one word each in a box 60 wide. */
const TWICE = "Hello World Hello World";

/** A word too long for a box 50 wide, "running" being 53.375 wide. */
const PUMP = "Pump P-101 is running";

/** A box's values drawn at random: its size, its maximum, its ellipsis, and a content of few letters. */
const BOX_VALUES = {
  boxHeight: fc.integer({ max: 120, min: 0 }),
  boxWidth: fc.integer({ max: 80, min: 0 }),
  content: fc.stringMatching(/^[ \nHWdelor]{0,40}$/u),
  hasEllipsis: fc.boolean(),
  maximumLines: fc.integer({ max: 5, min: 0 }),
};

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

  it("[F17.AC16] puts the ellipsis of a text cut by its maximum on its last line shown holding text", () => {
    // "Hello", "World", "", "Hello", at most 3 lines: the 3rd shown is empty, a line holding text
    // is left out; "World…" (5348 + 2048) / 128 = 57.78125 ≤ 60, the empty line dropped.
    const text: Text = { ...BOXED, content: "Hello\nWorld\n\nHello", maximumLines: 3 };

    expect(shown(text)).toEqual({ lines: ["Hello", "World…"], widths: [36.4609375, 57.78125] });

    // Two empty lines shown after "Hello": both dropped, "Hello…" 52.46094.
    expect(shownLines(ARIAL_BOX_METRICS, { ...text, content: "Hello\n\n\nWorld" })).toEqual([
      "Hello…",
    ]);
  });

  it("[F17.AC16] puts the ellipsis of a text cut by its box's height on its last line holding text", () => {
    // h(3) = 3 × 18.39844 = 55.19531 ≤ 60 < h(4) = 73.59375: 3 lines fit, the 3rd empty.
    const text: Text = { ...BOXED, boxHeight: 60, content: "Hello\nWorld\n\nHello" };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello", "World…"]);
  });

  it("[F17.AC16] reads a line of spaces as empty, its spaces trimmed as at every line's end", () => {
    // "   " wraps into "" (DERIV-text-box, step 2): it holds no text, so it is dropped as well.
    const text: Text = { ...BOXED, content: "Hello\nWorld\n   \nHello", maximumLines: 3 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello", "World…"]);
  });

  it("[F17.AC16] keeps the empty lines shown when the ellipsis is off, or nothing is cut", () => {
    const text: Text = { ...BOXED, content: "Hello\nWorld\n\nHello", maximumLines: 3 };

    expect(shownLines(ARIAL_BOX_METRICS, { ...text, hasEllipsis: false })).toEqual([
      "Hello",
      "World",
      "",
    ]);

    // Only an empty line left out: no ellipsis, the empty line shown kept.
    expect(shownLines(ARIAL_BOX_METRICS, { ...text, content: "Hello\n\n\n" })).toEqual([
      "Hello",
      "",
      "",
    ]);
  });

  it("[F17.AC16] ends a cut whose lines shown are all empty with the ellipsis on its last", () => {
    // No line shown holds text: the ellipsis stays on the last one, as before (provisional).
    const text: Text = { ...BOXED, content: "\n\nHello", maximumLines: 2 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["", "…"]);
  });

  it("[F17.AC1] wraps every text automatically, as many words a line as fit", () => {
    // "Pump P-101 is" (5350 + 569 + 5465 + 569 + 1479) / 128 = 13 432 / 128 = 104.9375 ≤ 110;
    // with " running" 104.9375 + 4.44531 + 53.375 > 110: "running" alone.
    expect(shown({ ...BOXED, boxWidth: 110, content: PUMP })).toEqual({
      lines: ["Pump P-101 is", "running"],
      widths: [104.9375, 53.375],
    });
  });

  it("[F17.AC1] wraps automatically a text read with a wrap style", () => {
    // A text saved before US-043 still holds `wrapStyle: "balance"`: the field is no longer read,
    // and the lines are not the balanced "Pump P-101" / "is running".
    const read = { ...BOXED, boxWidth: 110, content: PUMP, wrapStyle: "balance" };

    expect(shownLines(ARIAL_BOX_METRICS, read)).toEqual(["Pump P-101 is", "running"]);
  });

  it("[F14.AC6] keeps every line of a text that fits its content", () => {
    // Not in a box: no maximum, no ellipsis, the lines of F05.
    const text = { ...BOXED, content: "Hello World\nHello", isInBox: false, maximumLines: 1 };

    expect(shownLines(ARIAL_BOX_METRICS, text)).toEqual(["Hello World", "Hello"]);
  });

  test.prop(BOX_VALUES)(
    "[F14.AC2] shows at most the maximum, each line fitting with the ellipsis on unless one character is left",
    (values) => {
      const text = { ...BOXED, ...values };
      const lines = shownLines(ARIAL_BOX_METRICS, text);
      const wrapped = wrappedLines(ARIAL_BOX_METRICS, text);

      const maximum = values.maximumLines === 0 ? wrapped.length : values.maximumLines;

      expect(lines.length).toBeLessThanOrEqual(maximum);
      expect(lines.length === 0).toBe(values.content === "");

      // With the ellipsis on, an empty last line shown means nothing but empty lines was cut.
      expect(
        !values.hasEllipsis ||
          lines.at(-1) !== "" ||
          wrapped.slice(lines.length).every((line) => line === ""),
      ).toBe(true);

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

  test.prop(BOX_VALUES)(
    "[F17.AC16] drops only empty lines from those the box shows, each line kept its wrapped line or a start of it with the ellipsis",
    (values) => {
      const text = { ...BOXED, ...values };
      const lines = shownLines(ARIAL_BOX_METRICS, text);
      const wrapped = wrappedLines(ARIAL_BOX_METRICS, text);
      // With the ellipsis off, a box shows its first lines as they wrap, as many as fit.
      const count = shownLines(ARIAL_BOX_METRICS, { ...text, hasEllipsis: false }).length;
      const ellipsis = textEllipsis(ARIAL_BOX_METRICS);

      expect(wrapped.slice(lines.length, count).every((line) => line === "")).toBe(true);

      expect(
        lines.every(
          (line, index) =>
            line === wrapped[index] ||
            (line.endsWith(ellipsis) &&
              wrapped[index]?.startsWith(line.slice(0, -ellipsis.length)) === true),
        ),
      ).toBe(true);
    },
  );
});
