import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { isValidText } from "../../model";

import { toStoredColors } from "./to-stored-colors";

describe("toStoredColors (ADR-0056)", () => {
  it("[F16.AC1] turns a text's color and background color in upper case to lower case, valid", () => {
    const text = { ...HELLO_TEXT, backgroundColor: "#22C55E", color: "#FF8000" };

    expect(isValidText(text)).toBe(false);
    expect(isValidText(toStoredColors(text))).toBe(true);

    expect(toStoredColors(text)).toEqual({
      ...text,
      backgroundColor: "#22c55e",
      color: "#ff8000",
    });
  });

  it("[F16.AC1] keeps the opacities when the colors have no alpha", () => {
    const text = { ...HELLO_TEXT, backgroundOpacity: 40, color: " f80 ", opacity: 70 };

    expect(toStoredColors(text)).toEqual({ ...text, color: "#ff8800" });
  });

  it("[F16.AC1] sets each opacity from its color's alpha: cc is 80 %, ec 93 %", () => {
    const text = { ...HELLO_TEXT, backgroundColor: "#000000ec", color: "#0000FFCC", opacity: 40 };

    expect(toStoredColors(text)).toEqual({
      ...text,
      backgroundColor: "#000000",
      backgroundOpacity: 93,
      color: "#0000ff",
      opacity: 80,
    });
  });

  it.each(["red", "#ff800", ""])("[F16.AC1] leaves '%s' as it is, still refused", (color) => {
    const text = { ...HELLO_TEXT, color };

    expect(toStoredColors(text)).toEqual(text);
    expect(isValidText(toStoredColors(text))).toBe(false);
  });

  it("[F16.AC1] keeps an underline typed F00 as #ff0000, a strike-through's alpha 80 as its opacity", () => {
    const text = { ...HELLO_TEXT, strikeThroughColor: "#00FF00CC", underlineColor: "F00" };

    // cc = 204, 100 · 204 / 255 = 80; the underline's opacity, without alpha, is kept.
    expect(toStoredColors(text)).toEqual({
      ...text,
      strikeThroughColor: "#00ff00",
      strikeThroughOpacity: 80,
      underlineColor: "#ff0000",
    });
  });

  it.each(["red", "#ff000"])(
    "[F16.AC1] leaves the underline's and the strike-through's '%s' as it is, still refused",
    (color) => {
      const text = { ...HELLO_TEXT, strikeThroughColor: color, underlineColor: color };

      expect(toStoredColors(text)).toEqual(text);
      expect(isValidText(toStoredColors(text))).toBe(false);
    },
  );

  test.prop({
    background: fc.stringMatching(/^#?[\dA-Fa-f]{3,8}$/u),
    color: fc.stringMatching(/^#?[\dA-Fa-f]{3,8}$/u),
  })("stores each color once: storing it again changes nothing", ({ background, color }) => {
    const stored = toStoredColors({ ...HELLO_TEXT, backgroundColor: background, color });

    expect(toStoredColors(stored)).toEqual(stored);
  });
});
