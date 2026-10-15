import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_BOX_METRICS, ARIAL_METRICS, ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { DEFAULT_FONT } from "../font";
import { lineHeight, lineWidth } from "../text-block";
import { shownLines } from "../text-box";

import { textBlock } from "./text-block";

import type { FontMetrics } from "../font";
import type { Text } from "../text";

/**
 * The block of a text measured from its lines shown.
 *
 * @param metrics - the metrics of the face it is drawn with
 * @param text - its content and options
 * @returns its block
 */
function measured(metrics: FontMetrics, text: Text): ReturnType<typeof textBlock> {
  return textBlock(metrics, text, shownLines(metrics, text));
}

describe("textBlock (DERIV-text-width step 4)", () => {
  it("[F05.AC3] measures 'Hello' and 'World' in Arial 16 as 41.78125 × 36.79688", () => {
    const text = { ...HELLO_TEXT, content: "Hello\nWorld" };

    // Widths (1479 + 1139 + 910 + 1139) × 16 / 2048 = 36.4609375 and
    // (1933 + 1139 + 682 + 455 + 1139) × 16 / 2048 = 41.78125; two lines of 18.3984375.
    expect(measured(ARIAL_UNKERNED, text)).toEqual({ height: 36.796875, width: 41.78125 });
    // Kerned, W o −37: 5311 × 16 / 2048 = 41.4921875.
    expect(measured(ARIAL_METRICS, text)).toEqual({ height: 36.796875, width: 41.4921875 });
  });

  it("[F05.AC3] measures one line as its width by one line height", () => {
    expect(measured(ARIAL_METRICS, HELLO_TEXT)).toEqual({
      height: 18.3984375,
      width: 82.3984375,
    });
  });

  it("[F05.AC3] measures an empty content as a block 0 wide and 0 tall", () => {
    expect(measured(ARIAL_METRICS, { ...HELLO_TEXT, content: "" })).toEqual({
      height: 0,
      width: 0,
    });
  });

  it("cuts lines at CR LF too: 'Hello' and 'World' in Inter Regular 16", () => {
    const regular = DEFAULT_FONT.faces.find((face) => face.weight === 400 && !face.isItalic);
    const metrics = regular?.metrics ?? ARIAL_METRICS;

    // "World": W 2018 + o 1228 + r 771 + l 496 + d 1254 = 5767, W o −104, r l +32: 5695 units,
    // 5695 × 16 / 2048 = 44.4921875 ("Hello" 4936 units, 38.5625); two lines of 19.359375.
    expect(measured(metrics, { ...HELLO_TEXT, content: "Hello\r\nWorld" })).toEqual({
      height: 38.71875,
      width: 44.4921875,
    });
  });

  it("measures a line of spaces with its spacing", () => {
    // Three spaces, 3 × 569 = 1707 units: 13.3359375, plus 2 letter spacings of 2 and 3 word
    // spacings of 4: 13.3359375 + 4 + 12 = 29.3359375.
    const text = { ...HELLO_TEXT, content: " ".repeat(3), letterSpacing: 2, wordSpacing: 4 };

    expect(measured(ARIAL_METRICS, text).width).toBe(29.3359375);
  });

  it("floors the width at 0 when a negative spacing makes every line narrower than nothing", () => {
    // "HH": 2958 units, 23.109375, then one letter spacing of −100: −76.890625, floored.
    const text = { ...HELLO_TEXT, content: "HH\nHH", letterSpacing: -100 };

    expect(measured(ARIAL_METRICS, text)).toEqual({ height: 36.796875, width: 0 });
  });

  it("measures 200 000 lines, past what a call's arguments may hold", () => {
    const text = { ...HELLO_TEXT, content: Array.from({ length: 200_000 }, () => "H").join("\n") };

    // 1479 × 16 / 2048 = 11.5546875 wide, 200 000 lines of 18.3984375.
    expect(measured(ARIAL_METRICS, text)).toEqual({ height: 3_679_687.5, width: 11.5546875 });
  });

  it("counts an empty last line in the height", () => {
    // "Hello" then an empty line: 36.4609375 wide, two lines tall.
    expect(measured(ARIAL_METRICS, { ...HELLO_TEXT, content: "Hello\n" })).toEqual({
      height: 36.796875,
      width: 36.4609375,
    });
  });

  it("[F05.AC4] measures the content in its case", () => {
    // "hello" in sentence case is "Hello", 36.4609375; as typed, h takes glyph 0:
    // (1536 + 1139 + 910 + 1139) × 16 / 2048 = 36.90625.
    const text = { ...HELLO_TEXT, content: "hello" };

    expect(measured(ARIAL_METRICS, { ...text, textCase: "sentence" }).width).toBe(36.4609375);
    expect(measured(ARIAL_METRICS, text).width).toBe(36.90625);
  });

  it("[F14.AC2] measures the lines a box shows, with their ellipsis", () => {
    // "Hello World Hello World" in a box 60 wide, at most 2 lines: "Hello" over "World…",
    // (5348 + 2048) / 128 = 57.78125 wide, two lines of 18.3984375.
    const text = {
      ...HELLO_TEXT,
      boxHeight: 1000,
      boxWidth: 60,
      content: "Hello World Hello World",
      isInBox: true,
      maximumLines: 2,
    };

    expect(measured(ARIAL_BOX_METRICS, text)).toEqual({ height: 36.796875, width: 57.78125 });
  });

  it("[F14.AC6] measures a text that fits its content with the lines of F05", () => {
    // "Hello World" in a box 50 wide would wrap; fitting its content it stays one line, 82.6875
    // wide, over "Hello": two lines of 18.3984375.
    const text = {
      ...HELLO_TEXT,
      boxHeight: 1000,
      boxWidth: 50,
      content: "Hello World\nHello",
      maximumLines: 1,
    };

    expect(measured(ARIAL_BOX_METRICS, text)).toEqual({ height: 36.796875, width: 82.6875 });
  });

  it("[F14.AC3] measures 'Hello' over 'World' with a line height of 24 as 48 tall", () => {
    const text = { ...HELLO_TEXT, content: "Hello\nWorld", lineHeight: 24 };

    expect(measured(ARIAL_UNKERNED, text).height).toBe(48);
  });

  it("[F14.AC5] measures 'Hello' over 'World' trimmed from the cap height to the baseline", () => {
    // 18.3984375 + 1467 / 128 = 18.3984375 + 11.4609375 = 29.859375.
    const text = { ...HELLO_TEXT, content: "Hello\nWorld", isTrimmed: true };

    expect(measured(ARIAL_UNKERNED, text).height).toBe(29.859375);
  });

  test.prop({
    lines: fc.array(fc.stringMatching(/^[HeloWrdAVTR ]{1,12}$/u), { maxLength: 5, minLength: 1 }),
    size: fc.integer({ max: 1000, min: 1 }),
  })("is as wide as its widest line and as tall as its lines", ({ lines, size }) => {
    const text = { ...HELLO_TEXT, content: lines.join("\n"), size };
    const widths = lines.map((line) => lineWidth(ARIAL_METRICS, line, text));

    expect(measured(ARIAL_METRICS, text)).toEqual({
      height: lines.length * lineHeight(ARIAL_METRICS, size),
      width: Math.max(...widths),
    });
  });
});
