import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textToParagraphStyle } from "./text-to-paragraph-style";

describe("textToParagraphStyle (ADR-0053, EN-041)", () => {
  it("[F15.AC3] paints a background behind each paragraph in its color and opacity", () => {
    // 40 %: 255 · 40 / 100 = 102, written 66.
    const text = {
      ...HELLO_TEXT,
      backgroundColor: "#22c55e",
      backgroundOpacity: 40,
      hasBackground: true,
    };

    expect(textToParagraphStyle(text)).toEqual([["background-color", "#22c55e66"]]);
  });

  it("[F15.AC3] paints nothing without a background, whatever its color", () => {
    expect(textToParagraphStyle({ ...HELLO_TEXT, backgroundColor: "#22c55e" })).toEqual([]);
  });

  it("[F16.AC4] writes no stroke on the spans while both share the text's color", () => {
    const text = { ...HELLO_TEXT, isStruckThrough: true, isUnderlined: true };

    expect(textToParagraphStyle(text)).toEqual([]);
  });

  it("[F16.AC4] underlines each span in #ff0000 at 50 %, after the background, the strokes of two colors", () => {
    const text = {
      ...HELLO_TEXT,
      backgroundColor: "#22c55e",
      hasBackground: true,
      hasUnderlineColor: true,
      isStruckThrough: true,
      isUnderlined: true,
      underlineColor: "#ff0000",
      underlineOpacity: 50,
    };

    // 50 %: 255 · 50 / 100 = 127.5, rounded 128, written 80.
    expect(textToParagraphStyle(text)).toEqual([
      ["background-color", "#22c55eff"],
      ["text-decoration-line", "underline"],
      ["text-decoration-thickness", "from-font"],
      ["text-decoration-color", "#ff000080"],
    ]);
  });
});
