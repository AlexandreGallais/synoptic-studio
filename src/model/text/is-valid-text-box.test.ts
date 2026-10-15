import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { TEXT_BOX_DEFAULTS } from "../text-box";

import { isValidText } from "./is-valid-text";
import { isValidTextBox } from "./is-valid-text-box";

/** "Hello World" in a box 60 × 40, every option of its box set away from its default. */
const BOXED = {
  ...HELLO_TEXT,
  backgroundColor: "#1a2b3c",
  backgroundOpacity: 40,
  boxHeight: 40,
  boxWidth: 60,
  hasBackground: true,
  hasEllipsis: false,
  isInBox: true,
  isTrimmed: true,
  lineHeight: 24,
  maximumLines: 2,
  verticalAlignment: "middle",
  wrapStyle: "balance",
} as const;

describe("isValidTextBox (F14)", () => {
  it("[F14.AC1] accepts a text in a box with every option set", () => {
    expect(isValidTextBox(BOXED)).toBe(true);
    expect(isValidText(BOXED)).toBe(true);
  });

  it("[F14.AC6] gives a text of F05 the defaults: fits its content, automatic line height, no trim, no background, ellipsis on", () => {
    // The others, read only in a box (provisional, VAL-015): docs/domain/shapes.md, Box.
    expect(TEXT_BOX_DEFAULTS).toEqual({
      backgroundColor: "#ffff00",
      backgroundOpacity: 100,
      boxHeight: 0,
      boxWidth: 0,
      hasBackground: false,
      hasEllipsis: true,
      isInBox: false,
      isTrimmed: false,
      lineHeight: 0,
      maximumLines: 0,
      verticalAlignment: "top",
      wrapStyle: "auto",
    });

    expect(isValidText({ ...HELLO_TEXT, ...TEXT_BOX_DEFAULTS })).toBe(true);
  });

  it.each([0, 100_000])(
    "[F14.AC1] accepts %d, an end of 0 .. 100 000, as each side, maximum and line height",
    (value) => {
      const text = {
        ...BOXED,
        boxHeight: value,
        boxWidth: value,
        lineHeight: value,
        maximumLines: value,
      };

      expect(isValidTextBox(text)).toBe(true);
    },
  );

  it.each([
    { name: "a negative width", text: { boxWidth: -1 } },
    { name: "a width over 100 000", text: { boxWidth: 100_001 } },
    { name: "a fractional height", text: { boxHeight: 40.5 } },
    { name: "a height not a number", text: { boxHeight: NaN } },
    { name: "a negative maximum of lines", text: { maximumLines: -1 } },
    { name: "a maximum of lines over 100 000", text: { maximumLines: 100_001 } },
    { name: "a fractional maximum of lines", text: { maximumLines: 1.5 } },
    { name: "a negative line height", text: { lineHeight: -1 } },
    { name: "a line height over 100 000", text: { lineHeight: 100_001 } },
    { name: "a fractional line height", text: { lineHeight: 18.4 } },
    { name: "a background opacity over 100", text: { backgroundOpacity: 101 } },
    { name: "a negative background opacity", text: { backgroundOpacity: -1 } },
    { name: "a background color of three digits", text: { backgroundColor: "#ff0" } },
    { name: "a background color without #", text: { backgroundColor: "ffff00" } },
    { name: "a background color in upper case", text: { backgroundColor: "#FF8000" } },
  ])("[F14.AC1] refuses $name", ({ text }) => {
    expect(isValidTextBox({ ...BOXED, ...text })).toBe(false);
    expect(isValidText({ ...BOXED, ...text })).toBe(false);
  });

  it.each([
    { field: "verticalAlignment", value: "baseline" },
    { field: "wrapStyle", value: "pretty" },
    { field: "isInBox", value: "true" },
    { field: "hasEllipsis", value: 1 },
    { field: "isTrimmed", value: undefined },
    { field: "hasBackground", value: "false" },
  ])("[F14.AC1] refuses a $field of '$value', read from outside", ({ field, value }) => {
    const text = { ...BOXED, [field]: value };

    expect(isValidTextBox(text)).toBe(false);
  });

  test.prop({
    boxHeight: fc.integer({ max: 100_000, min: 0 }),
    boxWidth: fc.integer({ max: 100_000, min: 0 }),
    hasBackground: fc.boolean(),
    hasEllipsis: fc.boolean(),
    isInBox: fc.boolean(),
    isTrimmed: fc.boolean(),
    lineHeight: fc.integer({ max: 100_000, min: 0 }),
    maximumLines: fc.integer({ max: 100_000, min: 0 }),
    verticalAlignment: fc.constantFrom("top", "middle", "bottom"),
    wrapStyle: fc.constantFrom("auto", "balance"),
  })("accepts every option of a box within its range or among its values", (options) => {
    expect(isValidTextBox({ ...BOXED, ...options })).toBe(true);
  });
});
