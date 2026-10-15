import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textHtmlLayerStrokes } from "./text-html-layer-strokes";

/** "Hello World", underlined and struck through, both following its color. */
const BOTH = { ...HELLO_TEXT, isStruckThrough: true, isUnderlined: true } as const;

describe("textHtmlLayerStrokes (US-041)", () => {
  it("[F16.AC4] puts both strokes on the content's div while they follow the text's color", () => {
    expect(textHtmlLayerStrokes(BOTH)).toEqual({
      content: ["underline", "strikeThrough"],
      paragraphs: [],
    });
  });

  it("[F16.AC4] keeps both on the div when both are chosen #ff0000 at 40 %", () => {
    const text = {
      ...BOTH,
      hasStrikeThroughColor: true,
      hasUnderlineColor: true,
      strikeThroughColor: "#ff0000",
      strikeThroughOpacity: 40,
      underlineColor: "#ff0000",
      underlineOpacity: 40,
    };

    expect(textHtmlLayerStrokes(text).paragraphs).toEqual([]);
  });

  it.each([
    { underlineColor: "#ff0000", underlineOpacity: 100 },
    { underlineColor: "#000000", underlineOpacity: 50 },
  ])(
    "[F16.AC4] strikes the div and underlines the spans for an underline $underlineColor at $underlineOpacity %",
    (underline) => {
      const text = { ...BOTH, hasUnderlineColor: true, ...underline };

      expect(textHtmlLayerStrokes(text)).toEqual({
        content: ["strikeThrough"],
        paragraphs: ["underline"],
      });
    },
  );

  it("[F16.AC4] keeps a lone underline of its own color on the div, so that … carries it", () => {
    const text = {
      ...HELLO_TEXT,
      hasUnderlineColor: true,
      isUnderlined: true,
      underlineColor: "#ff0000",
    };

    expect(textHtmlLayerStrokes(text)).toEqual({ content: ["underline"], paragraphs: [] });
  });

  it("[F16.AC4] writes no stroke for a text with none", () => {
    expect(textHtmlLayerStrokes(HELLO_TEXT)).toEqual({ content: [], paragraphs: [] });
  });

  test.prop({ isStruckThrough: fc.boolean(), isUnderlined: fc.boolean() })(
    "[F16.AC4] places every stroke drawn once, on the div or on the spans",
    (strokes) => {
      const text = {
        ...HELLO_TEXT,
        ...strokes,
        hasUnderlineColor: true,
        underlineColor: "#ff0000",
      };
      const { content, paragraphs } = textHtmlLayerStrokes(text);

      const placed = [...content, ...paragraphs];

      expect(placed).toHaveLength(Number(strokes.isUnderlined) + Number(strokes.isStruckThrough));

      expect(new Set(placed)).toEqual(
        new Set([
          ...(strokes.isUnderlined ? ["underline"] : []),
          ...(strokes.isStruckThrough ? ["strikeThrough"] : []),
        ]),
      );
    },
  );
});
