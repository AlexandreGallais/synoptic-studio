import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";
import { DEFAULT_FONT } from "../font";

import { lineHeight } from "./line-height";
import { lineWidth } from "./line-width";
import { textBlock } from "./text-block";

describe("textBlock (DERIV-text-width step 4)", () => {
  it("[F05.AC3] measures 'Hello' and 'World' in Arial 16 as 41.78125 × 36.79688", () => {
    const text = { ...HELLO_TEXT, content: "Hello\nWorld" };

    // Widths (1479 + 1139 + 910 + 1139) × 16 / 2048 = 36.4609375 and
    // (1933 + 1139 + 682 + 455 + 1139) × 16 / 2048 = 41.78125; two lines of 18.3984375.
    expect(textBlock(ARIAL_UNKERNED, text)).toEqual({ height: 36.796875, width: 41.78125 });
    // Kerned, W o −37: 5311 × 16 / 2048 = 41.4921875.
    expect(textBlock(ARIAL_METRICS, text)).toEqual({ height: 36.796875, width: 41.4921875 });
  });

  it("[F05.AC3] measures one line as its width by one line height", () => {
    expect(textBlock(ARIAL_METRICS, HELLO_TEXT)).toEqual({
      height: 18.3984375,
      width: 82.3984375,
    });
  });

  it("[F05.AC3] measures an empty content as a block 0 wide and 0 tall", () => {
    expect(textBlock(ARIAL_METRICS, { ...HELLO_TEXT, content: "" })).toEqual({
      height: 0,
      width: 0,
    });
  });

  it("cuts lines at CR LF too: 'Hello' and 'World' in Inter Regular 16", () => {
    const regular = DEFAULT_FONT.faces.find((face) => face.weight === 400 && !face.isItalic);
    const metrics = regular?.metrics ?? ARIAL_METRICS;

    // "World": W 2018 + o 1228 + r 771 + l 496 + d 1254 = 5767, W o −104, r l +32: 5695 units,
    // 5695 × 16 / 2048 = 44.4921875 ("Hello" 4936 units, 38.5625); two lines of 19.359375.
    expect(textBlock(metrics, { ...HELLO_TEXT, content: "Hello\r\nWorld" })).toEqual({
      height: 38.71875,
      width: 44.4921875,
    });
  });

  it("counts an empty last line in the height", () => {
    // "Hello" then an empty line: 36.4609375 wide, two lines tall.
    expect(textBlock(ARIAL_METRICS, { ...HELLO_TEXT, content: "Hello\n" })).toEqual({
      height: 36.796875,
      width: 36.4609375,
    });
  });

  it("[F05.AC4] measures the content in its case", () => {
    // "hello" in sentence case is "Hello", 36.4609375; as typed, h takes glyph 0:
    // (1536 + 1139 + 910 + 1139) × 16 / 2048 = 36.90625.
    const text = { ...HELLO_TEXT, content: "hello" };

    expect(textBlock(ARIAL_METRICS, { ...text, textCase: "sentence" }).width).toBe(36.4609375);
    expect(textBlock(ARIAL_METRICS, text).width).toBe(36.90625);
  });

  test.prop({
    lines: fc.array(fc.stringMatching(/^[HeloWrdAVTR ]{1,12}$/u), { maxLength: 5, minLength: 1 }),
    size: fc.integer({ max: 1000, min: 1 }),
  })("is as wide as its widest line and as tall as its lines", ({ lines, size }) => {
    const text = { ...HELLO_TEXT, content: lines.join("\n"), size };
    const widths = lines.map((line) => lineWidth(ARIAL_METRICS, line, text));

    expect(textBlock(ARIAL_METRICS, text)).toEqual({
      height: lines.length * lineHeight(ARIAL_METRICS, size),
      width: Math.max(...widths),
    });
  });
});
