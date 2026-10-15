import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";

import { textToAttributes } from "./text-to-attributes";

describe("textToAttributes (ADR-0052, decision 4)", () => {
  it("[F05.AC4] writes the font, size, weight, style and color of a plain text", () => {
    expect(textToAttributes(HELLO_TEXT)).toEqual([
      ["font-family", '"Inter", sans-serif'],
      ["font-size", "16"],
      ["font-weight", "400"],
      ["font-style", "normal"],
      ["fill", "#000000"],
      ["fill-opacity", "1"],
    ]);
  });

  it("[F05.AC4] adds the spacing, italic, opacity and direction typed", () => {
    const text = {
      ...HELLO_TEXT,
      color: "#1a2b3c",
      direction: "rtl",
      isItalic: true,
      letterSpacing: 2,
      opacity: 40,
      weight: 700,
      wordSpacing: -4,
    } as const;

    expect(textToAttributes(text)).toEqual([
      ["font-family", '"Inter", sans-serif'],
      ["font-size", "16"],
      ["font-weight", "700"],
      ["font-style", "italic"],
      ["fill", "#1a2b3c"],
      ["fill-opacity", "0.4"],
      ["letter-spacing", "2"],
      ["word-spacing", "-4"],
      ["direction", "rtl"],
    ]);
  });

  it("[F05.AC4] writes no text-anchor, dominant-baseline nor text-decoration", () => {
    const names = textToAttributes({
      ...HELLO_TEXT,
      isStruckThrough: true,
      isUnderlined: true,
    }).map(([name]) => name);

    expect(names).not.toContain("text-anchor");
    expect(names).not.toContain("dominant-baseline");
    expect(names).not.toContain("text-decoration");
  });
});
