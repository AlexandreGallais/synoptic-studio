import { describe, expect, it } from "vitest";

import { GENERIC_FONT_FAMILIES } from "../../model";

import { formatFontFamilyName } from "./format-font-family-name";

describe("formatFontFamilyName (REF-CSS-FONTS-4 §2.1.1, §2.1.2)", () => {
  it("quotes a family name, spaces and digits included", () => {
    expect(formatFontFamilyName("Inter")).toBe('"Inter"');
    expect(formatFontFamilyName("Hawaii 5-0")).toBe('"Hawaii 5-0"');
  });

  it.each(GENERIC_FONT_FAMILIES)("writes the generic family %s as its keyword", (family) => {
    expect(formatFontFamilyName(family)).toBe(family);
  });

  it("writes a generic family in any case, white space around it, as its keyword", () => {
    expect(formatFontFamilyName("Serif")).toBe("serif");
    expect(formatFontFamilyName(" serif ")).toBe("serif");
  });

  it("escapes a double quote and a backslash in a name (REF-CSS-VALUES-4 §4.4)", () => {
    expect(formatFontFamilyName(String.raw`My "Font" \ 2`)).toBe(String.raw`"My \"Font\" \\ 2"`);
  });

  it("escapes a control character as its hexadecimal code and a space (REF-CSS2-SYNDATA)", () => {
    // Line feed U+000A, tab U+0009: a quoted string holds neither as typed.
    expect(formatFontFamilyName("Inter\nBold\tB")).toBe(String.raw`"Inter\A Bold\9 B"`);
  });
});
