import { describe, expect, it } from "vitest";

import { GENERIC_FONT_FAMILIES } from "./generic-font-families";
import { isGenericFamilyName } from "./is-generic-family-name";

describe("isGenericFamilyName (REF-CSS-FONTS-4 §2.1.2)", () => {
  it.each(GENERIC_FONT_FAMILIES)("tells %s generic", (name) => {
    expect(isGenericFamilyName(name)).toBe(true);
  });

  it("tells a generic name in any case, white space around it, generic", () => {
    expect(isGenericFamilyName("Monospace")).toBe(true);
    expect(isGenericFamilyName(" SERIF ")).toBe(true);
  });

  it("tells a family name not generic, one that holds a keyword among them", () => {
    expect(isGenericFamilyName("Arial")).toBe(false);
    expect(isGenericFamilyName("Liberation Serif")).toBe(false);
  });
});
