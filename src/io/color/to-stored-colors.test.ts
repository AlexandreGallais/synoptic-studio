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
});
