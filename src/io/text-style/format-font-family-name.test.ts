import { describe, expect, it } from "vitest";

import { formatFontFamilyName } from "./format-font-family-name";

describe("formatFontFamilyName (REF-CSS-FONTS-4 §2.1.1, §2.1.2)", () => {
  it("quotes a family name, spaces and digits included", () => {
    expect(formatFontFamilyName("Inter")).toBe('"Inter"');
    expect(formatFontFamilyName("Hawaii 5-0")).toBe('"Hawaii 5-0"');
  });

  it("writes a generic family as its keyword, in lower case", () => {
    expect(formatFontFamilyName("sans-serif")).toBe("sans-serif");
    expect(formatFontFamilyName("Monospace")).toBe("monospace");
  });

  it("escapes a double quote and a backslash in a name (REF-CSS-VALUES-4 §4.4)", () => {
    expect(formatFontFamilyName(String.raw`My "Font" \ 2`)).toBe(String.raw`"My \"Font\" \\ 2"`);
  });
});
