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
});
