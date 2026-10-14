import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isValidText } from "./is-valid-text";

import type { Text } from "./text";

/** "Hello" in Inter 16, regular, black, at the origin. */
const HELLO: Text = {
  alignment: "left",
  angle: 0,
  color: "#000000",
  content: "Hello",
  direction: "ltr",
  fallback: "sans-serif",
  font: "Inter",
  isItalic: false,
  isStruckThrough: false,
  isUnderlined: false,
  letterSpacing: 0,
  opacity: 100,
  size: 16,
  textCase: "none",
  weight: 400,
  wordSpacing: 0,
  x: 0,
  y: 0,
};

describe("isValidText (F05)", () => {
  it("[F05.AC2] accepts a text with every option set", () => {
    expect(
      isValidText({
        ...HELLO,
        alignment: "center",
        angle: 359,
        color: "#1A2b3C",
        content: "שלום\nWorld",
        direction: "rtl",
        isItalic: true,
        isStruckThrough: true,
        isUnderlined: true,
        letterSpacing: -2,
        opacity: 0,
        textCase: "kebab",
        weight: 900,
        wordSpacing: 4,
        x: -20,
        y: 35,
      }),
    ).toBe(true);
  });

  it("[F05.AC2] accepts an empty content, which draws nothing", () => {
    expect(isValidText({ ...HELLO, content: "" })).toBe(true);
  });

  it.each([1, 100_000])("[F05.AC2] accepts a size of %d, an end of 1 .. 100 000", (size) => {
    expect(isValidText({ ...HELLO, size })).toBe(true);
  });

  it.each([0, 100_001, 16.5, -16, NaN])(
    "[F05.AC2] refuses a size of %d: not an integer from 1 to 100 000",
    (size) => {
      expect(isValidText({ ...HELLO, size })).toBe(false);
    },
  );

  it.each([
    { name: "a weight under 100", text: { weight: 99 } },
    { name: "a weight over 900", text: { weight: 901 } },
    { name: "a fractional weight", text: { weight: 450.5 } },
    { name: "an opacity over 100", text: { opacity: 101 } },
    { name: "a negative opacity", text: { opacity: -1 } },
    { name: "a color of three digits", text: { color: "#000" } },
    { name: "a color without #", text: { color: "000000" } },
    { name: "a color with a letter past f", text: { color: "#00000g" } },
    { name: "a color with an alpha", text: { color: "#00000080" } },
    { name: "a fractional letter spacing", text: { letterSpacing: 0.5 } },
    { name: "a fractional word spacing", text: { wordSpacing: 1.5 } },
    { name: "an angle of 360", text: { angle: 360 } },
    { name: "a fractional position", text: { x: 0.5 } },
    { name: "a position not a number", text: { y: NaN } },
  ])("[F05.AC2] refuses $name", ({ text }) => {
    expect(isValidText({ ...HELLO, ...text })).toBe(false);
  });

  it.each([-100_000, 100_000])(
    "[F05.AC2] accepts a letter and word spacing of %d, an end of −100 000 .. 100 000",
    (spacing) => {
      expect(isValidText({ ...HELLO, letterSpacing: spacing, wordSpacing: spacing })).toBe(true);
    },
  );

  it.each([
    { name: "a letter spacing under −100 000", text: { letterSpacing: -100_001 } },
    { name: "a word spacing over 100 000", text: { wordSpacing: 100_001 } },
    { name: "an empty font name", text: { font: "" } },
    { name: "a font name of spaces only", text: { font: "  " } },
  ])("[F05.AC2] refuses $name", ({ text }) => {
    expect(isValidText({ ...HELLO, ...text })).toBe(false);
  });

  it.each([
    { field: "textCase", value: "shout" },
    { field: "alignment", value: "justify" },
    { field: "direction", value: "ttb" },
  ])("[F05.AC2] refuses a $field of '$value', read from outside", ({ field, value }) => {
    const text = { ...HELLO, [field]: value };

    expect(isValidText(text)).toBe(false);
  });

  test.prop({
    opacity: fc.integer({ max: 100, min: 0 }),
    size: fc.integer({ max: 100_000, min: 1 }),
    weight: fc.integer({ max: 900, min: 100 }),
  })("accepts every integer size, weight and opacity of their ranges", (values) => {
    expect(isValidText({ ...HELLO, ...values })).toBe(true);
  });
});
