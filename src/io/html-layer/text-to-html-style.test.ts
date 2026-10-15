import { describe, expect, it } from "vitest";

import { ARIAL_HTML_LAYER, HELLO_TEXT } from "../../../tests";

import { textToHtmlStyle } from "./text-to-html-style";

/** "Hello" over "World" in Arial 16. */
const ARIAL_TEXT = { ...HELLO_TEXT, content: "Hello\nWorld", font: "Arial" } as const;

/** The same text in a box 50 × 40. */
const BOXED = { ...ARIAL_TEXT, boxHeight: 40, boxWidth: 50, isInBox: true } as const;

/**
 * The declarations of a text's layer from `white-space` on.
 *
 * @param text - its options, in a box or not
 * @param lineLimit - the most lines its box shows
 * @returns each declaration as `property: value`
 */
function wrapping(text: typeof BOXED | typeof ARIAL_TEXT, lineLimit: number): readonly string[] {
  const style = textToHtmlStyle(text, { ...ARIAL_HTML_LAYER, lineLimit });
  const start = style.findIndex(([property]) => property === "white-space");

  return style.slice(start).map(([property, value]) => `${property}: ${value}`);
}

describe("textToHtmlStyle (ADR-0053, EN-041)", () => {
  it("[F15.AC3] writes the font list, size, weight, style, color and the library's line height", () => {
    // Line height 18.3984375 at 5 decimals: 18.39844 (Q10); opaque black: alpha ff.
    expect(textToHtmlStyle(ARIAL_TEXT, ARIAL_HTML_LAYER)).toEqual([
      ["font-family", '"Arial", sans-serif'],
      ["font-size", "16px"],
      ["font-weight", "400"],
      ["font-style", "normal"],
      ["color", "#000000ff"],
      ["line-height", "18.39844px"],
      ["text-align", "left"],
      ["white-space", "pre"],
    ]);
  });

  it("[F15.AC3] adds the spacing, direction, alignment and the underline typed", () => {
    const text = {
      ...ARIAL_TEXT,
      alignment: "center",
      color: "#1a2b3c",
      direction: "rtl",
      isItalic: true,
      isStruckThrough: true,
      isUnderlined: true,
      letterSpacing: 2,
      opacity: 40,
      weight: 700,
      wordSpacing: -4,
    } as const;

    // 40 %: 255 · 40 / 100 = 102, written 66.
    expect(textToHtmlStyle(text, ARIAL_HTML_LAYER)).toEqual([
      ["font-family", '"Arial", sans-serif'],
      ["font-size", "16px"],
      ["font-weight", "700"],
      ["font-style", "italic"],
      ["color", "#1a2b3c66"],
      ["line-height", "18.39844px"],
      ["letter-spacing", "2px"],
      ["word-spacing", "-4px"],
      ["direction", "rtl"],
      ["text-align", "center"],
      ["text-decoration-line", "underline"],
      ["text-decoration-thickness", "from-font"],
      ["text-decoration-color", "#1a2b3c66"],
      ["white-space", "pre"],
    ]);
  });

  it("[F16.AC4] writes the underline in its own color, #ff0000 at 50 %, the strike-through left to the spans", () => {
    const text = {
      ...ARIAL_TEXT,
      hasUnderlineColor: true,
      isStruckThrough: true,
      isUnderlined: true,
      underlineColor: "#ff0000",
      underlineOpacity: 50,
    };

    // 50 %: 255 · 50 / 100 = 127.5, rounded 128, written 80.
    expect(
      textToHtmlStyle(text, ARIAL_HTML_LAYER).filter(([property]) =>
        property.startsWith("text-decoration"),
      ),
    ).toEqual([
      ["text-decoration-line", "underline"],
      ["text-decoration-thickness", "from-font"],
      ["text-decoration-color", "#ff000080"],
    ]);
  });

  it("[F16.AC4] writes no decoration on the content's div for a text only struck through", () => {
    const style = textToHtmlStyle({ ...ARIAL_TEXT, isStruckThrough: true }, ARIAL_HTML_LAYER);

    expect(style.filter(([property]) => property.startsWith("text-decoration"))).toEqual([]);
  });

  it("[F15.AC3] escapes the font's name as the <text> does", () => {
    const [family] = textToHtmlStyle({ ...ARIAL_TEXT, font: 'My "Font"' }, ARIAL_HTML_LAYER);

    // A double quote in a CSS string is escaped by a backslash (`formatFontFamilyName`).
    expect(family).toEqual(["font-family", String.raw`"My \"Font\"", sans-serif`]);
  });

  it("[F15.AC3] wraps a text in a box, a word too long broken at a character", () => {
    expect(wrapping(BOXED, 0)).toEqual([
      "white-space: pre-wrap",
      "overflow-wrap: anywhere",
      "overflow: hidden",
      "text-overflow: ellipsis",
      "flex-shrink: 0",
    ]);
  });

  it("[F15.AC3] moves a word too long whole, and balances the lines when asked", () => {
    expect(wrapping({ ...BOXED, longWord: "move", wrapStyle: "balance" }, 0)).toEqual([
      "white-space: pre-wrap",
      "overflow-wrap: normal",
      "text-wrap: balance",
      "overflow: hidden",
      "text-overflow: ellipsis",
      "flex-shrink: 0",
    ]);
  });

  it("[F15.AC3] clamps a box showing at most 2 lines, the ellipsis on", () => {
    expect(wrapping(BOXED, 2)).toEqual([
      "white-space: pre-wrap",
      "overflow-wrap: anywhere",
      "overflow: hidden",
      "text-overflow: ellipsis",
      "display: -webkit-box",
      "-webkit-box-orient: vertical",
      "-webkit-line-clamp: 2",
      "flex-shrink: 0",
    ]);
  });

  it("[F15.AC3] cuts a box showing at most 2 lines at 2 line heights, down only, the ellipsis off", () => {
    // 2 · 18.3984375 = 36.796875, at 5 decimals 36.79688; clipped down, visible across, so that a
    // word moved whole overflows sideways.
    expect(wrapping({ ...BOXED, hasEllipsis: false, longWord: "move" }, 2)).toEqual([
      "white-space: pre-wrap",
      "overflow-wrap: normal",
      "overflow-x: visible",
      "overflow-y: clip",
      "max-height: 36.79688px",
      "flex-shrink: 0",
    ]);
  });

  it("[F15.AC3] neither cuts nor ends with an ellipsis a box without limit, the ellipsis off", () => {
    expect(wrapping({ ...BOXED, hasEllipsis: false }, 0)).toEqual([
      "white-space: pre-wrap",
      "overflow-wrap: anywhere",
      "flex-shrink: 0",
    ]);
  });

  it("[F15.AC3] never shrinks the content of a box, a block taller than its box keeping its place", () => {
    // A box 50 × 10 holding 2 lines 36.796875 tall: without flex-shrink 0, the item would shrink.
    expect(textToHtmlStyle({ ...BOXED, boxHeight: 10 }, ARIAL_HTML_LAYER).at(-1)).toEqual([
      "flex-shrink",
      "0",
    ]);
  });

  it("[F15.AC3] keeps a text that fits its content whole, whatever its box options", () => {
    expect(
      wrapping(
        { ...ARIAL_TEXT, maximumLines: 1, wrapStyle: "balance" },
        ARIAL_HTML_LAYER.lineLimit,
      ),
    ).toEqual(["white-space: pre"]);
  });
});
