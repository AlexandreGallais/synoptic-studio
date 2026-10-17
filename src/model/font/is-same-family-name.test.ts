import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isSameFamilyName } from "./is-same-family-name";

describe("isSameFamilyName (REF-CSS-FONTS-4, §5.1)", () => {
  it.each([
    { isSame: true, name: "arial", other: "Arial" },
    { isSame: true, name: "ARIAL", other: "Arial" },
    // Case folding maps ß to ss.
    { isSame: true, name: "Straße", other: "STRASSE" },
    // The Kelvin sign U+212A folds to k: upper case keeps it apart from K, lower case gives k.
    { isSame: true, name: "\u{212A}elvin Sans", other: "kelvin sans" },
    { isSame: false, name: "Arial", other: "Arial Black" },
    // Without normalization: a + combining ring is not å.
    { isSame: false, name: "å", other: "å" },
  ])("tells '$name' and '$other' alike: $isSame", ({ isSame, name, other }) => {
    expect(isSameFamilyName(name, other)).toBe(isSame);
  });

  test.prop({ name: fc.string({ maxLength: 20, unit: "grapheme-ascii" }) })(
    "matches a name with itself in upper and lower case",
    ({ name }) => {
      expect(isSameFamilyName(name.toUpperCase(), name.toLowerCase())).toBe(true);
    },
  );
});
