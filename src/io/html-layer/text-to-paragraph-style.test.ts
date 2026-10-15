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

  it("[F16.AC4] strikes each paragraph through in the text's color when none was chosen", () => {
    const text = { ...HELLO_TEXT, color: "#0000ff", isStruckThrough: true, opacity: 40 };

    // 40 %: 255 · 40 / 100 = 102, written 66.
    expect(textToParagraphStyle(text)).toEqual([
      ["text-decoration-line", "line-through"],
      ["text-decoration-thickness", "from-font"],
      ["text-decoration-color", "#0000ff66"],
    ]);
  });

  it("[F16.AC4] strikes it through in its own color, #ff0000, after the background", () => {
    const text = {
      ...HELLO_TEXT,
      backgroundColor: "#22c55e",
      hasBackground: true,
      hasStrikeThroughColor: true,
      isStruckThrough: true,
      strikeThroughColor: "#ff0000",
    };

    expect(textToParagraphStyle(text)).toEqual([
      ["background-color", "#22c55eff"],
      ["text-decoration-line", "line-through"],
      ["text-decoration-thickness", "from-font"],
      ["text-decoration-color", "#ff0000ff"],
    ]);
  });

  it("[F16.AC4] writes no strike-through on the spans for a text only underlined", () => {
    expect(textToParagraphStyle({ ...HELLO_TEXT, isUnderlined: true })).toEqual([]);
  });
});
